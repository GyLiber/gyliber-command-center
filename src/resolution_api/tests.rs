use super::*;
use axum::{
    body::{Body, to_bytes},
    http::Request,
};
use gyliber_command_center::resolution_control::*;
use model::{Meta, Operation};
use serde_json::{Value, json};
use std::sync::Arc;
use tower::ServiceExt;
use tower_sessions::cookie::{Cookie, CookieJar, Key};

fn id(s: &str) -> Identifier {
    s.to_owned().try_into().unwrap()
}
fn instant(s: &str) -> Instant {
    serde_json::from_value(json!(s)).unwrap()
}
fn create_resolution() -> Command {
    serde_json::from_value(json!({"type":"create_resolution","id":"r","spec":{"title":"Synthetic outcome","objective":null,"client_reference":null}})).unwrap()
}
fn create_commitment() -> Command {
    serde_json::from_value(json!({"type":"create_commitment","id":"c","resolution":"r","spec":{"title":"Synthetic proof","area":null,"kind":"assessment","source":null},"schedule":{"deadline":null,"earliest_finish":null,"buffer_minutes":null}})).unwrap()
}
fn create_action(name: &str) -> Command {
    serde_json::from_value(json!({"type":"create_action","id":name,"commitment":"c","plan":{"instruction":"Write one proof","expected_artifact":"proof.tex","verification_method":"Check all implications","start_reference":null}})).unwrap()
}
fn operation(token: &str, ack: Option<&store::Ack>, command: Command) -> Operation {
    Operation::Mutation {
        meta: meta(token, ack),
        command,
    }
}
fn meta(token: &str, ack: Option<&store::Ack>) -> Meta {
    Meta {
        operation_id: id(token),
        generation: ack.map(|a| a.generation.clone()),
        expected_revision: ack.map_or(0, |a| a.revision),
    }
}
async fn database() -> Option<PgPool> {
    let url = std::env::var("DATABASE_URL").ok()?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("configured test database must connect");
    store::migrate(&pool).await.unwrap();
    Some(pool)
}
fn owner() -> String {
    format!(
        "test-{}",
        &store::digest(oauth2::CsrfToken::new_random().secret().as_bytes())[..32]
    )
}
async fn cleanup(pool: &PgPool, owners: &[&str]) {
    for owner in owners {
        for statement in [
            "DELETE FROM gyliber_resolution_events WHERE owner=$1",
            "DELETE FROM gyliber_resolution_operations WHERE owner=$1",
            "DELETE FROM gyliber_resolution_sources WHERE owner=$1",
            "DELETE FROM gyliber_resolution_deletions WHERE owner=$1",
        ] {
            sqlx::query(statement)
                .bind(owner)
                .execute(pool)
                .await
                .unwrap();
        }
        sqlx::query("DELETE FROM gyliber_resolution_workspaces WHERE owner=$1")
            .bind(owner)
            .execute(pool)
            .await
            .unwrap();
        sqlx::query("DELETE FROM gyliber_resolution_audit WHERE actor=$1")
            .bind(owner)
            .execute(pool)
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn postgres_retry_conflict_isolation_and_reconnection() {
    let Some(pool) = database().await else { return };
    let a = owner();
    let b = owner();
    let first = store::operate(&pool, &a, operation("one", None, create_resolution()))
        .await
        .unwrap();
    let retry = store::operate(&pool, &a, operation("one", None, create_resolution()))
        .await
        .unwrap();
    assert!(retry.replayed);
    assert_eq!(retry.revision, 1);
    assert_eq!(
        store::operate(
            &pool,
            &a,
            operation("one", None, Command::SelectAction { id: None })
        )
        .await
        .unwrap_err(),
        store::Error::Conflict
    );
    assert_eq!(
        store::operate(&pool, &a, operation("stale", None, create_commitment()))
            .await
            .unwrap_err(),
        store::Error::Conflict
    );
    assert!(
        store::view(&pool, &b)
            .await
            .unwrap()
            .workspace
            .resolutions
            .is_empty()
    );
    assert_eq!(
        store::operate(&pool, &b, operation("foreign", None, create_commitment()))
            .await
            .unwrap_err(),
        store::Error::Rule(WorkspaceError::NotFound)
    );
    assert!(store::view(&pool, &b).await.unwrap().generation.is_none());
    let other_pool = database().await.unwrap();
    let second = store::operate(
        &other_pool,
        &a,
        operation("two", Some(&first), create_commitment()),
    )
    .await
    .unwrap();
    assert_eq!(second.revision, 2);
    let third = store::operate(
        &other_pool,
        &a,
        operation("three", Some(&second), create_action("a")),
    )
    .await
    .unwrap();
    assert_eq!(third.revision, 3);
    assert_eq!(
        store::view(&pool, &a)
            .await
            .unwrap()
            .workspace
            .actions
            .len(),
        1
    );
    assert_eq!(store::history(&pool, &a, 0).await.unwrap().len(), 3);
    assert_eq!(store::history(&pool, &b, 0).await.unwrap().len(), 0);
    assert_eq!(store::export(&pool, &a).await.unwrap().events.len(), 3);
    cleanup(&pool, &[&a, &b]).await;
}

#[tokio::test]
async fn postgres_concurrent_writes_and_rejected_transition_are_atomic() {
    let Some(pool) = database().await else { return };
    let owner = owner();
    let ack = store::operate(&pool, &owner, operation("one", None, create_resolution()))
        .await
        .unwrap();
    let (a, b) = tokio::join!(
        store::operate(
            &pool,
            &owner,
            operation("two", Some(&ack), create_commitment())
        ),
        store::operate(
            &pool,
            &owner,
            operation("three", Some(&ack), create_commitment())
        )
    );
    assert_eq!(usize::from(a.is_ok()) + usize::from(b.is_ok()), 1);
    assert_eq!(
        a.as_ref().err().or(b.as_ref().err()),
        Some(&store::Error::Conflict)
    );
    let ack = a.or(b).unwrap();
    let bad = store::operate(
        &pool,
        &owner,
        operation(
            "bad",
            Some(&ack),
            Command::ConfirmReadiness {
                commitment: id("c"),
            },
        ),
    )
    .await;
    assert!(matches!(bad, Err(store::Error::Rule(_))));
    assert_eq!(store::view(&pool, &owner).await.unwrap().revision, 2);
    cleanup(&pool, &[&owner]).await;
}

#[tokio::test]
async fn postgres_export_restore_and_deletion_barriers_survive_fresh_owner() {
    let Some(pool) = database().await else { return };
    let a = owner();
    let b = owner();
    let fresh = owner();
    let ack = store::operate(&pool, &a, operation("one", None, create_resolution()))
        .await
        .unwrap();
    let backup = store::export(&pool, &a).await.unwrap();
    let metadata = store::recovery_metadata(&pool, &a).await.unwrap();
    let restored = store::operate(
        &pool,
        &b,
        Operation::Restore {
            meta: meta("restore", None),
            backup: backup.clone(),
            recovery_metadata: metadata.clone(),
            confirm_recovery_metadata: true,
        },
    )
    .await
    .unwrap();
    assert_ne!(restored.generation, ack.generation);
    let view = store::view(&pool, &b).await.unwrap();
    assert!(view.imported_history);
    assert_eq!(view.revision, 1);
    assert_eq!(
        view.workspace,
        store::view(&pool, &a).await.unwrap().workspace
    );
    assert_eq!(
        store::operate(
            &pool,
            &a,
            Operation::Purge {
                meta: meta("bad", Some(&ack)),
                confirmation: "wrong".into()
            }
        )
        .await
        .unwrap_err(),
        store::Error::ConfirmationRequired
    );
    assert_eq!(store::view(&pool, &a).await.unwrap().revision, 1);
    let purged = store::operate(
        &pool,
        &a,
        Operation::Purge {
            meta: meta("purge", Some(&ack)),
            confirmation: "DELETE MY RESOLUTION CONTROL DATA".into(),
        },
    )
    .await
    .unwrap();
    assert_eq!(purged.revision, 0);
    assert_eq!(
        store::operate(&pool, &a, operation("one", None, create_resolution()))
            .await
            .unwrap_err(),
        store::Error::Gone
    );
    assert_eq!(
        store::operate(
            &pool,
            &a,
            Operation::Restore {
                meta: meta("blocked", Some(&purged)),
                backup: backup.clone(),
                recovery_metadata: metadata,
                confirm_recovery_metadata: true
            }
        )
        .await
        .unwrap_err(),
        store::Error::Gone
    );
    let newest = purged.recovery_metadata.unwrap();
    assert!(newest.deleted_generations.contains(&ack.generation));
    assert_eq!(
        store::operate(
            &pool,
            &fresh,
            Operation::Restore {
                meta: meta("blocked", None),
                backup,
                recovery_metadata: newest,
                confirm_recovery_metadata: true
            }
        )
        .await
        .unwrap_err(),
        store::Error::Gone
    );
    assert!(
        store::view(&pool, &fresh)
            .await
            .unwrap()
            .generation
            .is_none()
    );
    assert_eq!(store::view(&pool, &a).await.unwrap().revision, 0);
    let root_backup = store::export(&pool, &b).await.unwrap();
    let purged_b = store::operate(
        &pool,
        &b,
        Operation::Purge {
            meta: meta("purge", Some(&restored)),
            confirmation: "DELETE MY RESOLUTION CONTROL DATA".into(),
        },
    )
    .await
    .unwrap();
    assert_eq!(
        store::operate(
            &pool,
            &b,
            Operation::Restore {
                meta: meta("blocked-again", Some(&purged_b)),
                backup: root_backup,
                recovery_metadata: store::recovery_metadata(&pool, &b).await.unwrap(),
                confirm_recovery_metadata: true
            }
        )
        .await
        .unwrap_err(),
        store::Error::Gone
    );
    cleanup(&pool, &[&a, &b, &fresh]).await;
}

#[tokio::test]
async fn postgres_invalid_restore_never_commits_and_requires_confirmation() {
    let Some(pool) = database().await else { return };
    let a = owner();
    let b = owner();
    store::operate(&pool, &a, operation("one", None, create_resolution()))
        .await
        .unwrap();
    let backup = store::export(&pool, &a).await.unwrap();
    let metadata = store::recovery_metadata(&pool, &a).await.unwrap();
    let mut invalid = backup.clone();
    invalid.checksum = "0".repeat(64);
    assert_eq!(
        store::operate(
            &pool,
            &b,
            Operation::Restore {
                meta: meta("bad", None),
                backup: invalid,
                recovery_metadata: metadata.clone(),
                confirm_recovery_metadata: true
            }
        )
        .await
        .unwrap_err(),
        store::Error::InvalidBackup
    );
    assert_eq!(
        store::operate(
            &pool,
            &b,
            Operation::Restore {
                meta: meta("unconfirmed", None),
                backup,
                recovery_metadata: metadata,
                confirm_recovery_metadata: false
            }
        )
        .await
        .unwrap_err(),
        store::Error::ConfirmationRequired
    );
    assert!(store::view(&pool, &b).await.unwrap().generation.is_none());
    cleanup(&pool, &[&a, &b]).await;
}

fn events() -> Vec<WorkspaceEvent> {
    vec![
        WorkspaceEvent {
            revision: 1,
            at: instant("2026-10-08T21:59:59Z"),
            origin: EventOrigin::Live,
            command: create_resolution(),
        },
        WorkspaceEvent {
            revision: 2,
            at: instant("2026-10-08T22:00:00Z"),
            origin: EventOrigin::Imported,
            command: create_commitment(),
        },
    ]
}
fn query(date: &str, cutoff: Option<u64>) -> report::ReportQuery {
    report::ReportQuery {
        date: serde_json::from_value(json!(date)).unwrap(),
        offset_minutes: 120,
        cutoff_revision: cutoff,
    }
}
#[test]
fn daily_report_boundaries_cutoffs_and_empty_days_are_explicit() {
    let events = events();
    let now = instant("2026-10-10T12:00:00Z");
    let previous = report::build(&events, id("actor"), query("2026-10-08", None), now).unwrap();
    assert_eq!(previous.changes.len(), 1);
    assert_eq!(previous.state.commitments.len(), 0);
    assert!(!previous.imported_history);
    let today = report::build(&events, id("actor"), query("2026-10-09", None), now).unwrap();
    assert_eq!(today.changes.len(), 1);
    assert_eq!(today.changes[0].revision, 2);
    assert!(today.imported_history);
    let cutoff = report::build(&events, id("actor"), query("2026-10-09", Some(1)), now).unwrap();
    assert!(cutoff.empty_day);
    assert!(cutoff.state.commitments.is_empty());
    let empty = report::build(&events, id("actor"), query("2026-10-10", None), now).unwrap();
    assert!(empty.empty_day);
    assert_eq!(empty.state.commitments.len(), 1);
    let mut invalid = query("2026-10-09", None);
    invalid.offset_minutes = 841;
    assert!(matches!(
        report::build(&events, id("actor"), invalid, now),
        Err(store::Error::InvalidRequest)
    ));
    assert!(matches!(
        report::build(&events, id("actor"), query("2026-10-09", Some(3)), now),
        Err(store::Error::InvalidRequest)
    ));
}

#[test]
fn command_json_cannot_override_identity_status_or_clock() {
    for field in ["owner", "actor", "timestamp", "status", "verified_finish"] {
        let mut value = serde_json::to_value(create_resolution()).unwrap();
        value[field] = json!("forged");
        assert!(serde_json::from_value::<Command>(value).is_err());
    }
    let mut value = json!({"meta":{"operation_id":"one","generation":null,"expected_revision":0},"command":create_resolution()});
    value["owner"] = json!("other");
    assert!(serde_json::from_value::<model::Mutation>(value).is_err());
}

async fn app(
    member_id: Option<Option<u64>>,
    enabled: bool,
    pool: Option<PgPool>,
) -> (Router, Option<String>) {
    let sessions = tower_sessions::MemoryStore::default();
    let key = Key::generate();
    let cookie = if let Some(id) = member_id {
        let session = Session::new(None, Arc::new(sessions.clone()), None);
        session
            .insert(
                auth::MEMBER_KEY,
                auth::GitHubUser {
                    id,
                    login: "synthetic".into(),
                    name: None,
                    avatar_url: None,
                },
            )
            .await
            .unwrap();
        session.insert(CSRF, "test-csrf").await.unwrap();
        session.save().await.unwrap();
        let mut jar = CookieJar::new();
        jar.private_mut(&key).add(Cookie::new(
            "gyliber.sid",
            session.id().unwrap().to_string(),
        ));
        Some(jar.get("gyliber.sid").unwrap().to_string())
    } else {
        None
    };
    let github = crate::config::GitHubConfig {
        client_id: oauth2::ClientId::new("synthetic".into()),
        client_secret: oauth2::ClientSecret::new("synthetic".into()),
        redirect_url: oauth2::RedirectUrl::new("http://localhost/auth/github/callback".into())
            .unwrap(),
        allowed_logins: vec!["synthetic".into()],
    };
    let state = AppState {
        github: Some(Arc::new(github)),
        http: reqwest::Client::new(),
        math: Arc::new(crate::math_playground::Service::default()),
        resolution: Arc::new(Service { enabled, pool }),
    };
    (
        crate::build_app(
            state,
            key,
            false,
            crate::session_store::SessionStoreBackend::Memory(sessions),
        ),
        cookie,
    )
}
async fn request(
    app: Router,
    path: &str,
    method: &str,
    cookie: Option<&str>,
    csrf: Option<&str>,
    origin: Option<&str>,
    body: &str,
) -> Response {
    let mut request = Request::builder()
        .uri(path)
        .method(method)
        .header(header::CONTENT_TYPE, "application/json");
    if let Some(cookie) = cookie {
        request = request.header(header::COOKIE, cookie);
    }
    if let Some(csrf) = csrf {
        request = request.header("x-resolution-csrf", csrf);
    }
    if let Some(origin) = origin {
        request = request.header(header::ORIGIN, origin);
    }
    app.oneshot(request.body(Body::from(body.to_owned())).unwrap())
        .await
        .unwrap()
}
async fn response_json(response: Response) -> Value {
    serde_json::from_slice(
        &to_bytes(response.into_body(), 5 * 1024 * 1024)
            .await
            .unwrap(),
    )
    .unwrap()
}

#[tokio::test]
async fn http_all_endpoints_require_membership_even_for_bad_inputs() {
    let (app, _) = app(None, true, None).await;
    for (path, method) in [
        ("", "GET"),
        ("/commands", "POST"),
        ("/history?owner=forged", "GET"),
        ("/report?date=invalid", "GET"),
        ("/export", "GET"),
        ("/recovery-metadata", "GET"),
        ("/restore", "POST"),
        ("/purge", "POST"),
    ] {
        let response = request(
            app.clone(),
            &format!("/api/resolution-control{path}"),
            method,
            None,
            None,
            None,
            "invalid",
        )
        .await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED, "{path}");
        assert_eq!(response.headers()[header::CACHE_CONTROL], "no-store");
    }
}
#[tokio::test]
async fn http_old_sessions_csrf_origin_disabled_and_missing_storage_fail_closed() {
    let (old, cookie) = app(Some(None), true, None).await;
    let response = request(
        old,
        "/api/resolution-control",
        "GET",
        cookie.as_deref(),
        None,
        None,
        "",
    )
    .await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        response_json(response).await["error"],
        "reauthentication_required"
    );
    let (app, cookie) = app(Some(Some(1)), true, None).await;
    for csrf in [None, Some("wrong")] {
        assert_eq!(
            request(
                app.clone(),
                "/api/resolution-control/commands",
                "POST",
                cookie.as_deref(),
                csrf,
                None,
                "invalid"
            )
            .await
            .status(),
            StatusCode::FORBIDDEN
        );
    }
    assert_eq!(
        request(
            app.clone(),
            "/api/resolution-control",
            "GET",
            cookie.as_deref(),
            None,
            Some("https://evil.invalid"),
            ""
        )
        .await
        .status(),
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        request(
            app,
            "/api/resolution-control/commands",
            "POST",
            cookie.as_deref(),
            Some("test-csrf"),
            Some("http://localhost"),
            "invalid"
        )
        .await
        .status(),
        StatusCode::SERVICE_UNAVAILABLE
    );
    let (disabled, cookie) = self::app(Some(Some(1)), false, None).await;
    let response = request(
        disabled,
        "/api/resolution-control",
        "GET",
        cookie.as_deref(),
        None,
        None,
        "",
    )
    .await;
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(response_json(response).await["error"], "feature_disabled");
}

#[tokio::test]
async fn postgres_http_valid_owner_workflow_body_limits_and_closed_pool() {
    let Some(pool) = database().await else { return };
    let bytes = oauth2::CsrfToken::new_random()
        .secret()
        .bytes()
        .fold(1_u64, |a, b| a.wrapping_mul(31).wrapping_add(u64::from(b)));
    let id_number = bytes.max(1);
    let owner = format!("github-{id_number}");
    let (app, cookie) = app(Some(Some(id_number)), true, Some(pool.clone())).await;
    let bootstrap = request(
        app.clone(),
        "/api/resolution-control",
        "GET",
        cookie.as_deref(),
        None,
        None,
        "",
    )
    .await;
    assert_eq!(bootstrap.status(), StatusCode::OK);
    let bootstrap = response_json(bootstrap).await;
    assert_eq!(bootstrap["csrf"], "test-csrf");
    assert_eq!(bootstrap["view"]["generation"], Value::Null);
    let body =
        serde_json::to_string(&json!({"meta":meta("one", None),"command":create_resolution()}))
            .unwrap();
    let response = request(
        app.clone(),
        "/api/resolution-control/commands",
        "POST",
        cookie.as_deref(),
        Some("test-csrf"),
        Some("http://localhost"),
        &body,
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response_json(response).await["revision"], 1);
    let foreign = request(
        app.clone(),
        "/api/resolution-control/history?owner=forged",
        "GET",
        cookie.as_deref(),
        None,
        None,
        "",
    )
    .await;
    assert_eq!(foreign.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        request(
            app.clone(),
            "/api/resolution-control/commands",
            "POST",
            cookie.as_deref(),
            Some("test-csrf"),
            None,
            "{}"
        )
        .await
        .status(),
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        request(
            app.clone(),
            "/api/resolution-control/commands",
            "POST",
            cookie.as_deref(),
            Some("test-csrf"),
            None,
            &"x".repeat(65537)
        )
        .await
        .status(),
        StatusCode::PAYLOAD_TOO_LARGE
    );
    assert_eq!(
        request(
            app.clone(),
            "/api/resolution-control/restore",
            "POST",
            cookie.as_deref(),
            Some("test-csrf"),
            None,
            &"x".repeat(4 * 1024 * 1024 + 1)
        )
        .await
        .status(),
        StatusCode::PAYLOAD_TOO_LARGE
    );
    let download = request(
        app.clone(),
        "/api/resolution-control/export",
        "GET",
        cookie.as_deref(),
        None,
        None,
        "",
    )
    .await;
    assert_eq!(download.status(), StatusCode::OK);
    assert!(download.headers().contains_key(header::CONTENT_DISPOSITION));
    cleanup(&pool, &[&owner]).await;
    pool.close().await;
    assert_eq!(
        request(
            app,
            "/api/resolution-control",
            "GET",
            cookie.as_deref(),
            None,
            None,
            ""
        )
        .await
        .status(),
        StatusCode::SERVICE_UNAVAILABLE
    );
}
