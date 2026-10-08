//! Private owner-bound API. Command history is persisted transactionally; no
//! browser-selected owner, actor, timestamp or readiness status is accepted.
use crate::{auth, config::AppState};
use axum::{
    Json, Router,
    extract::{
        DefaultBodyLimit, Query, State,
        rejection::{JsonRejection, QueryRejection},
    },
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use tower_http::limit::RequestBodyLimitLayer;
use tower_sessions::Session;

mod model;
mod report;
mod store;
#[cfg(test)]
mod tests;

const CSRF: &str = "resolution_csrf";
#[derive(Default)]
pub(crate) struct Service {
    enabled: bool,
    pool: Option<PgPool>,
}
impl Service {
    pub(crate) fn from_environment() -> anyhow::Result<Self> {
        let enabled = match std::env::var("RESOLUTION_CONTROL_ENABLED") {
            Ok(value) => crate::config::parse_bool("RESOLUTION_CONTROL_ENABLED", &value)?,
            Err(std::env::VarError::NotPresent) => false,
            Err(error) => return Err(error.into()),
        };
        Ok(Self {
            enabled,
            pool: None,
        })
    }
    pub(crate) async fn initialize(&mut self, pool: Option<PgPool>) -> anyhow::Result<()> {
        if let Some(pool) = &pool {
            store::migrate(pool)
                .await
                .map_err(|_| anyhow::anyhow!("unable to migrate Resolution Control"))?;
        }
        self.pool = pool;
        Ok(())
    }
}

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/resolution-control", get(bootstrap))
        .route("/api/resolution-control/commands", post(mutate))
        .route("/api/resolution-control/history", get(history))
        .route("/api/resolution-control/report", get(report))
        .route("/api/resolution-control/export", get(export))
        .route(
            "/api/resolution-control/recovery-metadata",
            get(recovery_metadata),
        )
        .route("/api/resolution-control/purge", post(purge))
        .layer(DefaultBodyLimit::max(64 * 1024))
        .layer(RequestBodyLimitLayer::new(64 * 1024))
        .merge(
            Router::new()
                .route("/api/resolution-control/restore", post(restore))
                .layer(DefaultBodyLimit::max(4 * 1024 * 1024))
                .layer(RequestBodyLimitLayer::new(4 * 1024 * 1024)),
        )
}

fn failure(status: StatusCode, code: &'static str) -> Response {
    (status, Json(json!({"error": code}))).into_response()
}
fn error(error: store::Error) -> Response {
    use store::Error::*;
    let (status, code) = match error {
        Unavailable => (StatusCode::SERVICE_UNAVAILABLE, "storage_unavailable"),
        Conflict => (StatusCode::CONFLICT, "refresh_required"),
        Gone => (StatusCode::GONE, "deleted_generation"),
        InvalidBackup => (StatusCode::BAD_REQUEST, "invalid_backup"),
        InvalidRequest => (StatusCode::BAD_REQUEST, "invalid_request"),
        ConfirmationRequired => (StatusCode::BAD_REQUEST, "confirmation_required"),
        Capacity
        | Rule(gyliber_command_center::resolution_control::WorkspaceError::CapacityReached) => {
            (StatusCode::CONFLICT, "capacity_reached")
        }
        NotFound | Rule(gyliber_command_center::resolution_control::WorkspaceError::NotFound) => {
            (StatusCode::NOT_FOUND, "not_found")
        }
        Rule(_) => (StatusCode::BAD_REQUEST, "transition_rejected"),
    };
    failure(status, code)
}
async fn denied(
    state: &AppState,
    owner: Option<&str>,
    status: StatusCode,
    code: &'static str,
) -> Response {
    if let Some(pool) = &state.resolution.pool
        && store::audit(pool, owner, code).await.is_err()
    {
        return error(store::Error::Unavailable);
    }
    failure(status, code)
}
async fn access(
    state: &AppState,
    session: &Session,
    headers: &HeaderMap,
    writing: bool,
) -> Result<String, Box<Response>> {
    let Some(user) = auth::member_from_session(session).await else {
        return Err(Box::new(
            denied(
                state,
                None,
                StatusCode::UNAUTHORIZED,
                "authentication_required",
            )
            .await,
        ));
    };
    let Some(id) = user.id.filter(|id| *id > 0) else {
        return Err(Box::new(
            denied(
                state,
                None,
                StatusCode::UNAUTHORIZED,
                "reauthentication_required",
            )
            .await,
        ));
    };
    let owner = format!("github-{id}");
    let cross_site = headers
        .get("sec-fetch-site")
        .is_some_and(|v| v != "same-origin" && v != "none");
    let invalid_origin = headers.get(header::ORIGIN).is_some_and(|origin| {
        let expected = state
            .github
            .as_ref()
            .map(|g| g.redirect_url.url().origin().ascii_serialization());
        origin
            .to_str()
            .ok()
            .zip(expected.as_deref())
            .is_none_or(|(actual, expected)| actual != expected)
    });
    if cross_site || invalid_origin {
        return Err(Box::new(
            denied(
                state,
                Some(&owner),
                StatusCode::FORBIDDEN,
                "same_origin_required",
            )
            .await,
        ));
    }
    if writing {
        let token = session
            .get::<String>(CSRF)
            .await
            .map_err(|_| Box::new(error(store::Error::Unavailable)))?;
        if token
            .as_deref()
            .zip(
                headers
                    .get("x-resolution-csrf")
                    .and_then(|v| v.to_str().ok()),
            )
            .is_none_or(|(expected, actual)| expected != actual)
        {
            return Err(Box::new(
                denied(state, Some(&owner), StatusCode::FORBIDDEN, "csrf_required").await,
            ));
        }
    }
    if !state.resolution.enabled {
        return Err(Box::new(failure(
            StatusCode::SERVICE_UNAVAILABLE,
            "feature_disabled",
        )));
    }
    if state.resolution.pool.is_none() {
        return Err(Box::new(error(store::Error::Unavailable)));
    }
    Ok(owner)
}
async fn bootstrap(
    State(state): State<AppState>,
    session: Session,
    headers: HeaderMap,
) -> Response {
    let owner = match access(&state, &session, &headers, false).await {
        Ok(owner) => owner,
        Err(response) => return *response,
    };
    let pool = state.resolution.pool.as_ref().expect("access checked");
    let view = match store::view(pool, &owner).await {
        Ok(view) => view,
        Err(e) => return error(e),
    };
    let token = match session.get::<String>(CSRF).await {
        Ok(Some(token)) => token,
        Ok(None) => {
            let token = oauth2::CsrfToken::new_random().secret().clone();
            if session.insert(CSRF, &token).await.is_err() {
                return error(store::Error::Unavailable);
            }
            token
        }
        Err(_) => return error(store::Error::Unavailable),
    };
    Json(json!({"csrf": token, "view": view, "limits": {"events": 2048, "resolutions": 16, "commitments": 64, "actions": 128, "threats": 128}, "import_notice": "Imported history is a user-supplied account, not independently authenticated evidence."})).into_response()
}
async fn execute(state: &AppState, owner: &str, operation: model::Operation) -> Response {
    let pool = state.resolution.pool.as_ref().expect("access checked");
    match store::operate(pool, owner, operation).await {
        Ok(ack) => Json(ack).into_response(),
        Err(e) => {
            if e != store::Error::Unavailable
                && store::audit(pool, Some(owner), "resolution.operation_rejected")
                    .await
                    .is_err()
            {
                return error(store::Error::Unavailable);
            }
            error(e)
        }
    }
}
async fn mutate(
    State(state): State<AppState>,
    session: Session,
    headers: HeaderMap,
    input: Result<Json<model::Mutation>, JsonRejection>,
) -> Response {
    let owner = match access(&state, &session, &headers, true).await {
        Ok(owner) => owner,
        Err(response) => return *response,
    };
    match input {
        Ok(Json(model::Mutation { meta, command })) => {
            execute(&state, &owner, model::Operation::Mutation { meta, command }).await
        }
        Err(rejection) => {
            if rejection.status() == StatusCode::PAYLOAD_TOO_LARGE {
                failure(StatusCode::PAYLOAD_TOO_LARGE, "payload_too_large")
            } else {
                error(store::Error::InvalidRequest)
            }
        }
    }
}
async fn restore(
    State(state): State<AppState>,
    session: Session,
    headers: HeaderMap,
    input: Result<Json<model::Restore>, JsonRejection>,
) -> Response {
    let owner = match access(&state, &session, &headers, true).await {
        Ok(owner) => owner,
        Err(response) => return *response,
    };
    match input {
        Ok(Json(model::Restore {
            meta,
            backup,
            recovery_metadata,
            confirm_recovery_metadata,
        })) => {
            execute(
                &state,
                &owner,
                model::Operation::Restore {
                    meta,
                    backup,
                    recovery_metadata,
                    confirm_recovery_metadata,
                },
            )
            .await
        }
        Err(rejection) => {
            if rejection.status() == StatusCode::PAYLOAD_TOO_LARGE {
                failure(StatusCode::PAYLOAD_TOO_LARGE, "payload_too_large")
            } else {
                error(store::Error::InvalidRequest)
            }
        }
    }
}
async fn purge(
    State(state): State<AppState>,
    session: Session,
    headers: HeaderMap,
    input: Result<Json<model::Purge>, JsonRejection>,
) -> Response {
    let owner = match access(&state, &session, &headers, true).await {
        Ok(owner) => owner,
        Err(response) => return *response,
    };
    match input {
        Ok(Json(model::Purge { meta, confirmation })) => {
            execute(
                &state,
                &owner,
                model::Operation::Purge { meta, confirmation },
            )
            .await
        }
        Err(rejection) => {
            if rejection.status() == StatusCode::PAYLOAD_TOO_LARGE {
                failure(StatusCode::PAYLOAD_TOO_LARGE, "payload_too_large")
            } else {
                error(store::Error::InvalidRequest)
            }
        }
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct HistoryQuery {
    #[serde(default)]
    after_revision: u64,
}
async fn history(
    State(state): State<AppState>,
    session: Session,
    headers: HeaderMap,
    query: Result<Query<HistoryQuery>, QueryRejection>,
) -> Response {
    let owner = match access(&state, &session, &headers, false).await {
        Ok(owner) => owner,
        Err(response) => return *response,
    };
    let query = match query {
        Ok(Query(query)) => query,
        Err(_) => return error(store::Error::InvalidRequest),
    };
    match store::history(
        state.resolution.pool.as_ref().expect("access checked"),
        &owner,
        query.after_revision,
    )
    .await
    {
        Ok(events) => Json(json!({"events": events})).into_response(),
        Err(e) => error(e),
    }
}
async fn report(
    State(state): State<AppState>,
    session: Session,
    headers: HeaderMap,
    query: Result<Query<report::ReportQuery>, QueryRejection>,
) -> Response {
    let owner = match access(&state, &session, &headers, false).await {
        Ok(owner) => owner,
        Err(response) => return *response,
    };
    let query = match query {
        Ok(Query(query)) => query,
        Err(_) => return error(store::Error::InvalidRequest),
    };
    match report::generate(
        state.resolution.pool.as_ref().expect("access checked"),
        &owner,
        query,
    )
    .await
    {
        Ok(report) => Json(report).into_response(),
        Err(e) => error(e),
    }
}
fn attachment<T: serde::Serialize>(value: T, filename: &'static str) -> Response {
    ([(header::CONTENT_DISPOSITION, filename)], Json(value)).into_response()
}
async fn export(State(state): State<AppState>, session: Session, headers: HeaderMap) -> Response {
    let owner = match access(&state, &session, &headers, false).await {
        Ok(owner) => owner,
        Err(response) => return *response,
    };
    match store::export(
        state.resolution.pool.as_ref().expect("access checked"),
        &owner,
    )
    .await
    {
        Ok(backup) => attachment(
            backup,
            "attachment; filename=resolution-control-backup.json",
        ),
        Err(e) => error(e),
    }
}
async fn recovery_metadata(
    State(state): State<AppState>,
    session: Session,
    headers: HeaderMap,
) -> Response {
    let owner = match access(&state, &session, &headers, false).await {
        Ok(owner) => owner,
        Err(response) => return *response,
    };
    match store::recovery_metadata(
        state.resolution.pool.as_ref().expect("access checked"),
        &owner,
    )
    .await
    {
        Ok(metadata) => attachment(
            metadata,
            "attachment; filename=resolution-control-recovery-metadata.json",
        ),
        Err(e) => error(e),
    }
}
