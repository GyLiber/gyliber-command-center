use std::{env, net::SocketAddr};

use anyhow::{Context, Result};
use axum::{
    Json, Router,
    extract::State,
    http::{HeaderValue, StatusCode, header},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use serde::Serialize;
use time::Duration;
use tower_http::{
    limit::RequestBodyLimitLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    services::ServeDir,
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};
use tower_sessions::{Expiry, Session, SessionManagerLayer, cookie::Key};
use tracing::info;

mod audit;
mod auth;
mod config;
mod modules;
mod repository;
mod resources;
mod session_store;
mod state;

const RELEASE: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize)]
struct Health {
    service: &'static str,
    version: &'static str,
    status: &'static str,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(env::var("RUST_LOG").unwrap_or_else(|_| "info".into()))
        .init();

    let state = config::load_state()?;
    let session_key = config::load_session_key()?;
    config::validate_runtime_security()?;
    let secure_cookie = match env::var("COOKIE_SECURE") {
        Ok(value) => config::parse_bool("COOKIE_SECURE", &value)?,
        Err(env::VarError::NotPresent) => false,
        Err(error) => return Err(error.into()),
    };

    let session_store = session_store::SessionStoreBackend::from_environment().await?;
    let app = build_app(state, session_key, secure_cookie, session_store);

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".into())
        .parse::<u16>()
        .context("PORT must be a valid u16")?;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!(%addr, "GyLiber Command Center starting");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn build_app(
    state: config::AppState,
    session_key: Key,
    secure_cookie: bool,
    session_store: session_store::SessionStoreBackend,
) -> Router {
    let sessions = SessionManagerLayer::new(session_store)
        .with_name("gyliber.sid")
        .with_http_only(true)
        .with_secure(secure_cookie)
        // Lax permits the top-level GET OAuth callback while blocking cross-site subrequests.
        .with_same_site(tower_sessions::cookie::SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::hours(8)))
        .with_private(session_key);

    let hsts = if secure_cookie {
        HeaderValue::from_static("max-age=31536000; includeSubDomains")
    } else {
        HeaderValue::from_static("max-age=0")
    };

    Router::new()
        .route("/", get(public_home))
        .route("/about", get(public_about))
        .route("/work", get(public_work))
        .route("/links", get(public_links))
        .route("/login", get(login))
        .route("/auth/github/start", get(auth::github_start))
        .route("/auth/github/callback", get(auth::github_callback))
        .route("/logout", post(auth::logout))
        .route("/command", get(command_center))
        .route("/command/state", get(command_state))
        .route("/command/repository", get(command_repository))
        .route("/command/resources", get(command_resources))
        .route("/api/health", get(health))
        .route("/api/state", get(protected_state))
        .route("/api/modules", get(protected_modules))
        .route("/api/repository", get(protected_repository))
        .route("/api/resources", get(protected_resources))
        .nest_service("/static", ServeDir::new("static"))
        .fallback(not_found)
        .layer(RequestBodyLimitLayer::new(64 * 1024))
        .layer(SetRequestIdLayer::new(
            header::HeaderName::from_static("x-request-id"),
            MakeRequestUuid,
        ))
        .layer(PropagateRequestIdLayer::new(
            header::HeaderName::from_static("x-request-id"),
        ))
        .layer(TraceLayer::new_for_http())
        .layer(SetResponseHeaderLayer::if_not_present(
            header::CONTENT_SECURITY_POLICY,
            HeaderValue::from_static(
                "default-src 'self'; img-src 'self' https://avatars.githubusercontent.com; style-src 'self'; script-src 'self'; base-uri 'none'; form-action 'self' https://github.com; frame-ancestors 'none'; object-src 'none'; connect-src 'self'",
            ),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::REFERRER_POLICY,
            HeaderValue::from_static("no-referrer"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-store"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::HeaderName::from_static("permissions-policy"),
            HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::HeaderName::from_static("cross-origin-opener-policy"),
            HeaderValue::from_static("same-origin"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::HeaderName::from_static("x-permitted-cross-domain-policies"),
            HeaderValue::from_static("none"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::HeaderName::from_static("cross-origin-resource-policy"),
            HeaderValue::from_static("same-origin"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::STRICT_TRANSPORT_SECURITY,
            hsts,
        ))
        .layer(sessions)
        .with_state(state)
}

async fn public_home() -> Html<&'static str> {
    Html(include_str!("../static/home.html"))
}

async fn public_about() -> Html<&'static str> {
    Html(include_str!("../static/about.html"))
}

async fn public_work() -> Html<&'static str> {
    Html(include_str!("../static/work.html"))
}

async fn public_links() -> Html<&'static str> {
    Html(include_str!("../static/links.html"))
}

async fn login(State(state): State<config::AppState>) -> Html<String> {
    Html(render_login(state.github.is_some()))
}

async fn protected_state(session: Session) -> Response {
    let _member = match auth::require_api_member(&session).await {
        Ok(member) => member,
        Err(failure) => return failure.into_response(),
    };
    Json(state::snapshot()).into_response()
}

async fn protected_resources(session: Session) -> Response {
    let _member = match auth::require_api_member(&session).await {
        Ok(member) => member,
        Err(failure) => return failure.into_response(),
    };

    Json(resources::catalog()).into_response()
}

async fn protected_repository(State(state): State<config::AppState>, session: Session) -> Response {
    let _member = match auth::require_api_member(&session).await {
        Ok(member) => member,
        Err(failure) => return failure.into_response(),
    };

    match repository::snapshot(&state.http).await {
        Ok(snapshot) => Json(snapshot).into_response(),
        Err(_) => (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({"error": "repository_source_unavailable"})),
        )
            .into_response(),
    }
}

async fn protected_modules(session: Session) -> Response {
    let _member = match auth::require_api_member(&session).await {
        Ok(member) => member,
        Err(failure) => return failure.into_response(),
    };

    Json(modules::catalog()).into_response()
}

async fn command_resources(session: Session) -> Response {
    let _member = match auth::require_page_member(&session).await {
        Ok(member) => member,
        Err(failure) => return failure.into_response(),
    };

    Html(include_str!("../static/resources.html")).into_response()
}

async fn command_repository(session: Session) -> Response {
    let _member = match auth::require_page_member(&session).await {
        Ok(member) => member,
        Err(failure) => return failure.into_response(),
    };

    Html(include_str!("../static/repository.html")).into_response()
}

async fn command_state(session: Session) -> Response {
    let _member = match auth::require_page_member(&session).await {
        Ok(member) => member,
        Err(failure) => return failure.into_response(),
    };

    Html(include_str!("../static/state.html")).into_response()
}

async fn command_center(session: Session) -> Response {
    let user = match auth::require_page_member(&session).await {
        Ok(user) => user,
        Err(failure) => return failure.into_response(),
    };

    Html(render_command_center(&user)).into_response()
}

async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Html(include_str!("../static/404.html")),
    )
}

async fn health() -> Json<Health> {
    Json(Health {
        service: "gyliber-command-center",
        version: RELEASE,
        status: "ok",
    })
}

fn render_login(authentication_configured: bool) -> String {
    let action = if authentication_configured {
        r#"<a class="primary-btn" href="/auth/github/start">Continue with GitHub</a>"#
    } else {
        r#"<span class="muted">Member authentication is not configured for this deployment yet.</span>"#
    };
    let status = if authentication_configured {
        "GitHub OAuth with PKCE is enabled for authorized member access."
    } else {
        "Public deployment mode is active. Protected routes remain unavailable until GitHub OAuth is configured."
    };

    include_str!("../static/login.html")
        .replace("<!-- MEMBER_ACTION -->", action)
        .replace("<!-- AUTH_STATUS -->", status)
}

fn render_command_center(user: &auth::GitHubUser) -> String {
    let name = html_escape(user.name.as_deref().unwrap_or(&user.login));
    let login = html_escape(&user.login);
    let avatar = html_escape(user.avatar_url.as_deref().unwrap_or(""));
    let release = RELEASE;

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>GyLiber Command Center</title><link rel="stylesheet" href="/static/app.css"><script src="/static/app.js" defer></script>
</head>
<body>
<main class="shell">
<header class="topbar"><div><span class="eyebrow">GYLIBER / INTERNAL</span><h1>Command Center</h1></div>
<form action="/logout" method="post"><button class="ghost" type="submit">Sign out</button></form></header>
<section class="hero-grid">
<article class="panel primary"><div class="status-line"><span class="pulse"></span>SYSTEM OPERATIONAL</div>
<h2>Welcome, {name}</h2><p>Authenticated member surface. Sensitive company data is intentionally disabled in {release}.</p>
<div class="metrics"><div><span>Release</span><strong id="release">{release}</strong></div><div><span>API</span><strong id="api-status">ONLINE</strong></div><div><span>Data</span><strong id="data-status">GATED</strong></div></div>
<p class="muted live-readout">Last state observation: <span id="state-seen">checking…</span></p>
</article>
<article class="panel"><span class="eyebrow">MEMBER</span><div class="member"><img src="{avatar}" alt=""><div><strong>@{login}</strong><span>GitHub identity verified</span></div></div>
<p class="muted">Authorization is enforced server-side through the configured GyLiber member allowlist.</p></article>
</section>
<section>
<div class="section-heading"><span class="eyebrow">MODULE REGISTRY</span><span class="muted small" id="module-summary">loading…</span></div>
<div class="module-grid" id="module-grid"></div>
</section>
</main></body></html>"#
    )
}

fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use reqwest::Client;
    use tower::ServiceExt;
    use tower_sessions::cookie::Key;

    use super::{build_app, config, html_escape};
    use crate::auth;
    use oauth2::{ClientId, ClientSecret, RedirectUrl};

    fn test_app() -> axum::Router {
        let github = config::GitHubConfig {
            client_id: ClientId::new("test-client".into()),
            client_secret: ClientSecret::new("test-secret".into()),
            redirect_url: RedirectUrl::new("http://localhost:3000/auth/github/callback".into())
                .expect("test callback URL is valid"),
            allowed_logins: vec!["gyliber".into()],
        };

        let state = config::AppState {
            github: Some(std::sync::Arc::new(github)),
            http: Client::new(),
        };

        build_app(
            state,
            Key::generate(),
            false,
            session_store::SessionStoreBackend::Memory(tower_sessions::MemoryStore::default()),
        )
    }

    #[tokio::test]
    async fn github_login_starts_oauth_flow() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .uri("/auth/github/start")
                    .body(Body::empty())
                    .expect("request builds"),
            )
            .await
            .expect("response is produced");

        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        let location = response
            .headers()
            .get("location")
            .and_then(|value| value.to_str().ok())
            .expect("OAuth location header exists");
        assert!(location.starts_with("https://github.com/login/oauth/authorize"));
        assert!(response.headers().contains_key("set-cookie"));
    }

    fn public_only_test_app() -> axum::Router {
        let state = config::AppState {
            github: None,
            http: Client::new(),
        };

        build_app(
            state,
            Key::generate(),
            false,
            session_store::SessionStoreBackend::Memory(tower_sessions::MemoryStore::default()),
        )
    }

    #[tokio::test]
    async fn member_login_reports_unavailable_when_oauth_is_unconfigured() {
        let response = public_only_test_app()
            .oneshot(
                Request::builder()
                    .uri("/auth/github/start")
                    .body(Body::empty())
                    .expect("request builds"),
            )
            .await
            .expect("response is produced");

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn public_only_login_page_is_honest_about_authentication_state() {
        let response = public_only_test_app()
            .oneshot(
                Request::builder()
                    .uri("/login")
                    .body(Body::empty())
                    .expect("request builds"),
            )
            .await
            .expect("response is produced");

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), 16 * 1024)
            .await
            .expect("body reads");
        let body = String::from_utf8(body.to_vec()).expect("body is UTF-8");

        assert!(body.contains("Member authentication is not configured for this deployment yet."));
        assert!(!body.contains("Continue with GitHub"));
    }

    #[tokio::test]
    async fn public_home_is_accessible() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .uri("/")
                    .body(Body::empty())
                    .expect("request builds"),
            )
            .await
            .expect("response is produced");

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn security_headers_are_present() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .uri("/")
                    .body(Body::empty())
                    .expect("request builds"),
            )
            .await
            .expect("response is produced");

        assert_eq!(
            response
                .headers()
                .get("x-content-type-options")
                .and_then(|value| value.to_str().ok()),
            Some("nosniff")
        );
        assert_eq!(
            response
                .headers()
                .get("x-frame-options")
                .and_then(|value| value.to_str().ok()),
            Some("DENY")
        );
        assert_eq!(
            response
                .headers()
                .get("cache-control")
                .and_then(|value| value.to_str().ok()),
            Some("no-store")
        );
        assert!(response.headers().contains_key("content-security-policy"));
        assert_eq!(
            response
                .headers()
                .get("permissions-policy")
                .and_then(|value| value.to_str().ok()),
            Some("camera=(), microphone=(), geolocation=()")
        );
        assert_eq!(
            response
                .headers()
                .get("cross-origin-opener-policy")
                .and_then(|value| value.to_str().ok()),
            Some("same-origin")
        );
        assert_eq!(
            response
                .headers()
                .get("strict-transport-security")
                .and_then(|value| value.to_str().ok()),
            Some("max-age=0")
        );
    }

    #[tokio::test]
    async fn unknown_routes_use_controlled_not_found_page() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .uri("/does-not-exist")
                    .body(Body::empty())
                    .expect("request builds"),
            )
            .await
            .expect("response is produced");

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn health_is_public() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .uri("/api/health")
                    .body(Body::empty())
                    .expect("request builds"),
            )
            .await
            .expect("response is produced");

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn command_center_redirects_anonymous_visitors() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .uri("/command")
                    .body(Body::empty())
                    .expect("request builds"),
            )
            .await
            .expect("response is produced");

        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        assert_eq!(
            response
                .headers()
                .get("location")
                .and_then(|value| value.to_str().ok()),
            Some("/login")
        );
    }

    #[tokio::test]
    async fn module_catalog_rejects_anonymous_requests() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .uri("/api/modules")
                    .body(Body::empty())
                    .expect("request builds"),
            )
            .await
            .expect("response is produced");

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn resource_registry_redirects_anonymous_visitors() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .uri("/command/resources")
                    .body(Body::empty())
                    .expect("request builds"),
            )
            .await
            .expect("response is produced");

        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        assert_eq!(
            response
                .headers()
                .get("location")
                .and_then(|value| value.to_str().ok()),
            Some("/login")
        );
    }

    #[tokio::test]
    async fn resource_api_rejects_anonymous_requests() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .uri("/api/resources")
                    .body(Body::empty())
                    .expect("request builds"),
            )
            .await
            .expect("response is produced");

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn repository_monitor_redirects_anonymous_visitors() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .uri("/command/repository")
                    .body(Body::empty())
                    .expect("request builds"),
            )
            .await
            .expect("response is produced");

        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        assert_eq!(
            response
                .headers()
                .get("location")
                .and_then(|value| value.to_str().ok()),
            Some("/login")
        );
    }

    #[tokio::test]
    async fn live_state_page_redirects_anonymous_visitors() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .uri("/command/state")
                    .body(Body::empty())
                    .expect("request builds"),
            )
            .await
            .expect("response is produced");

        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        assert_eq!(
            response
                .headers()
                .get("location")
                .and_then(|value| value.to_str().ok()),
            Some("/login")
        );
    }

    #[tokio::test]
    async fn live_state_rejects_anonymous_api_requests() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .uri("/api/state")
                    .body(Body::empty())
                    .expect("request builds"),
            )
            .await
            .expect("response is produced");

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert!(response.headers().get("location").is_none());
    }

    #[test]
    fn boolean_configuration_accepts_only_true_or_false() {
        assert!(config::parse_bool("COOKIE_SECURE", "true").expect("true parses"));
        assert!(!config::parse_bool("COOKIE_SECURE", "false").expect("false parses"));
        assert!(config::parse_bool("COOKIE_SECURE", "enabled").is_err());
    }

    #[test]
    fn member_allowlist_is_case_insensitive_and_trimmed() {
        let allowed = vec!["GyLiber".to_string(), "ExampleMember".to_string()];
        assert!(auth::is_allowed_member("gyliber", &allowed));
        assert!(auth::is_allowed_member(" EXAMPLEMEMBER ", &allowed));
        assert!(!auth::is_allowed_member("intruder", &allowed));
    }

    #[test]
    fn html_escape_blocks_markup_characters() {
        let escaped = html_escape("<script>alert('x')</script>");
        assert_eq!(escaped, "&lt;script&gt;alert(&#39;x&#39;)&lt;/script&gt;");
    }
}
