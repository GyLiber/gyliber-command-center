use std::{env, net::SocketAddr, sync::Arc};

use anyhow::{Context, Result};
use axum::{
    extract::{Query, State},
    http::{header, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
    Json, Router,
};
use oauth2::{
    basic::BasicClient,
    AuthorizationCode, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use time::Duration;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    services::ServeDir,
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};
use tower_sessions::{cookie::Key, Expiry, MemoryStore, Session, SessionManagerLayer};
use tracing::info;

const MEMBER_KEY: &str = "member";
const OAUTH_STATE_KEY: &str = "oauth_state";
const OAUTH_VERIFIER_KEY: &str = "oauth_pkce_verifier";

#[derive(Clone)]
struct AppState {
    github: Arc<GitHubConfig>,
    http: Client,
}

#[derive(Clone)]
struct GitHubConfig {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_url: RedirectUrl,
    allowed_logins: Vec<String>,
}

#[derive(Deserialize)]
struct OAuthCallback {
    code: String,
    state: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct GitHubUser {
    login: String,
    name: Option<String>,
    avatar_url: Option<String>,
}

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

    let state = load_state()?;
    let session_key = load_session_key()?;
    let secure_cookie = env::var("COOKIE_SECURE")
        .map(|value| value != "false")
        .unwrap_or(true);

    let sessions = SessionManagerLayer::new(MemoryStore::default())
        .with_name("gyliber.sid")
        .with_secure(secure_cookie)
        .with_same_site(tower_sessions::cookie::SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::hours(8)))
        .with_private(session_key);

    let app = Router::new()
        .route("/", get(public_home))
        .route("/about", get(public_about))
        .route("/work", get(public_work))
        .route("/links", get(public_links))
        .route("/login", get(login))
        .route("/auth/github/start", get(github_start))
        .route("/auth/github/callback", get(github_callback))
        .route("/logout", post(logout))
        .route("/command", get(command_center))
        .route("/api/health", get(health))
        .route("/api/state", get(protected_state))
        .nest_service("/static", ServeDir::new("static"))
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
        .layer(sessions)
        .with_state(state);

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

fn load_state() -> Result<AppState> {
    let client_id = ClientId::new(
        env::var("GITHUB_CLIENT_ID").context("GITHUB_CLIENT_ID is required")?,
    );
    let client_secret = ClientSecret::new(
        env::var("GITHUB_CLIENT_SECRET").context("GITHUB_CLIENT_SECRET is required")?,
    );
    let redirect_url = RedirectUrl::new(
        env::var("GITHUB_REDIRECT_URL").context("GITHUB_REDIRECT_URL is required")?,
    )?;
    let allowed_logins = env::var("GYLIBER_ALLOWED_GITHUB_LOGINS")
        .context("GYLIBER_ALLOWED_GITHUB_LOGINS is required")?
        .split(',')
        .map(|value| value.trim().to_ascii_lowercase())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();

    anyhow::ensure!(!allowed_logins.is_empty(), "GYLIBER_ALLOWED_GITHUB_LOGINS cannot be empty");

    Ok(AppState {
        github: Arc::new(GitHubConfig {
            client_id,
            client_secret,
            redirect_url,
            allowed_logins,
        }),
        http: Client::builder()
            .user_agent("GyLiber-Command-Center/0.1.0")
            .build()?,
    })
}

fn load_session_key() -> Result<Key> {
    let master = env::var("SESSION_MASTER_KEY")
        .context("SESSION_MASTER_KEY is required and must contain at least 64 random bytes")?;
    Key::try_from(master.as_bytes())
        .context("SESSION_MASTER_KEY must contain at least 64 bytes for a private session key")
}

fn public_home() -> Html<&'static str> {
    Html(include_str!("../static/home.html"))
}

fn public_about() -> Html<&'static str> {
    Html(include_str!("../static/about.html"))
}

fn public_work() -> Html<&'static str> {
    Html(include_str!("../static/work.html"))
}

fn public_links() -> Html<&'static str> {
    Html(include_str!("../static/links.html"))
}

fn login() -> Html<&'static str> {
    Html(include_str!("../static/login.html"))
}

async fn github_start(State(state): State<AppState>, session: Session) -> Response {
    let client = github_client(&state.github);

    let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("read:user".into()))
        .set_pkce_challenge(challenge)
        .url();

    if session
        .insert(OAUTH_STATE_KEY, csrf_state.secret())
        .await
        .is_err()
        || session
            .insert(OAUTH_VERIFIER_KEY, verifier.secret())
            .await
            .is_err()
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Unable to initialize login").into_response();
    }

    Redirect::to(auth_url.as_str()).into_response()
}

async fn github_callback(
    State(state): State<AppState>,
    Query(query): Query<OAuthCallback>,
    session: Session,
) -> Response {
    let expected_state = session.get::<String>(OAUTH_STATE_KEY).await.ok().flatten();
    let verifier = session
        .get::<String>(OAUTH_VERIFIER_KEY)
        .await
        .ok()
        .flatten();

    let _ = session.remove::<String>(OAUTH_STATE_KEY).await;
    let _ = session.remove::<String>(OAUTH_VERIFIER_KEY).await;

    if expected_state.as_deref() != Some(query.state.as_str()) {
        return (StatusCode::UNAUTHORIZED, "Login state was invalid").into_response();
    }

    let verifier = match verifier {
        Some(value) => value,
        None => return (StatusCode::UNAUTHORIZED, "Login state was invalid").into_response(),
    };

    let token = match github_client(&state.github)
        .exchange_code(AuthorizationCode::new(query.code))
        .set_pkce_verifier(PkceCodeVerifier::new(verifier))
        .request_async(&state.http)
        .await
    {
        Ok(token) => token,
        Err(_) => return (StatusCode::UNAUTHORIZED, "GitHub authentication failed").into_response(),
    };

    let user = match state
        .http
        .get("https://api.github.com/user")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .and_then(|response| response.error_for_status())
    {
        Ok(response) => match response.json::<GitHubUser>().await {
            Ok(user) => user,
            Err(_) => return (StatusCode::UNAUTHORIZED, "Unable to read member identity").into_response(),
        },
        Err(_) => return (StatusCode::UNAUTHORIZED, "Unable to verify member identity").into_response(),
    };

    if !is_allowed_member(&user.login, &state.github.allowed_logins) {
        info!(github_login = %user.login, "Rejected non-member login");
        let _ = session.clear().await;
        return (
            StatusCode::FORBIDDEN,
            "This GitHub account is not authorized for GyLiber Command Center",
        )
            .into_response();
    }

    if session.cycle_id().await.is_err()
        || session.insert(MEMBER_KEY, &user).await.is_err()
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to establish secure session",
        )
            .into_response();
    }

    Redirect::to("/command").into_response()
}

async fn logout(session: Session) -> Response {
    let _ = session.clear().await;
    Redirect::to("/").into_response()
}

async fn protected_state(session: Session) -> Response {
    if session.get::<GitHubUser>(MEMBER_KEY).await.ok().flatten().is_none() {
        return Redirect::to("/login").into_response();
    }
    Json(serde_json::json!({
        "release": "0.1.0",
        "public_surface": "operational",
        "authenticated_surface": "protected",
        "sensitive_data": "disabled",
        "data_freshness": "live-process",
    })).into_response()
}

async fn command_center(session: Session) -> Response {
    let user = session.get::<GitHubUser>(MEMBER_KEY).await.ok().flatten();

    match user {
        Some(user) => Html(render_command_center(&user)).into_response(),
        None => Redirect::to("/login").into_response(),
    }
}

fn health() -> Json<Health> {
    Json(Health {
        service: "gyliber-command-center",
        version: "0.1.0",
        status: "ok",
    })
}


fn github_client(config: &GitHubConfig) -> BasicClient {
    BasicClient::new(config.client_id.clone())
        .set_client_secret(config.client_secret.clone())
        .set_auth_uri(
            AuthUrl::new("https://github.com/login/oauth/authorize".into())
                .expect("static GitHub authorization URL is valid"),
        )
        .set_token_uri(
            TokenUrl::new("https://github.com/login/oauth/access_token".into())
                .expect("static GitHub token URL is valid"),
        )
        .set_redirect_uri(config.redirect_url.clone())
}

fn is_allowed_member(login: &str, allowed_logins: &[String]) -> bool {
    let normalized = login.trim().to_ascii_lowercase();
    allowed_logins.iter().any(|allowed| allowed == &normalized)
}

fn render_command_center(user: &GitHubUser) -> String {
    let name = html_escape(user.name.as_deref().unwrap_or(&user.login));
    let login = html_escape(&user.login);
    let avatar = html_escape(user.avatar_url.as_deref().unwrap_or(""));

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>GyLiber Command Center</title><link rel="stylesheet" href="/static/app.css">
</head>
<body>
<main class="shell">
<header class="topbar"><div><span class="eyebrow">GYLIBER / INTERNAL</span><h1>Command Center</h1></div>
<form action="/logout" method="post"><button class="ghost" type="submit">Sign out</button></form></header>
<section class="hero-grid">
<article class="panel primary"><div class="status-line"><span class="pulse"></span>SYSTEM OPERATIONAL</div>
<h2>Welcome, {name}</h2><p>Authenticated member surface. Sensitive company data is intentionally disabled in v0.1.0.</p>
<div class="metrics"><div><span>Release</span><strong>0.1.0</strong></div><div><span>API</span><strong>ONLINE</strong></div><div><span>Data</span><strong>GATED</strong></div></div>
</article>
<article class="panel"><span class="eyebrow">MEMBER</span><div class="member"><img src="{avatar}" alt=""><div><strong>@{login}</strong><span>GitHub identity verified</span></div></div>
<p class="muted">Authorization is enforced server-side through the configured GyLiber member allowlist.</p></article>
</section>
<section class="module-grid">
<a class="module" href="/api/state"><span>01</span><h3>Live System State</h3><p>Inspect the current safe operational signal.</p></a>
<article class="module locked"><span>02</span><h3>Finance</h3><p>Reserved for a later security-gated release.</p></article>
<article class="module locked"><span>03</span><h3>Intellectual Property</h3><p>Reserved for classified document and rights controls.</p></article>
<article class="module locked"><span>04</span><h3>People</h3><p>Reserved for private staff information controls.</p></article>
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
    use super::is_allowed_member;

    #[test]
    fn member_allowlist_is_case_insensitive() {
        let allowed = vec!["GyLiber".to_string(), "ExampleMember".to_string()];
        assert!(is_allowed_member("gyliber", &allowed));
        assert!(is_allowed_member(" EXAMPLEMEMBER ", &allowed));
        assert!(!is_allowed_member("intruder", &allowed));
    }
}
