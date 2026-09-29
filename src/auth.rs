use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use oauth2::{
    AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope, TokenResponse,
};
use oauth2_reqwest::ReqwestClient;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use tracing::info;

use crate::{
    audit,
    config::{self, AppState},
};

pub(crate) const MEMBER_KEY: &str = "member";
const OAUTH_STATE_KEY: &str = "oauth_state";
const OAUTH_VERIFIER_KEY: &str = "oauth_pkce_verifier";

#[derive(Deserialize)]
pub(crate) struct OAuthCallback {
    pub(crate) code: String,
    pub(crate) state: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct GitHubUser {
    pub(crate) login: String,
    pub(crate) name: Option<String>,
    pub(crate) avatar_url: Option<String>,
}

pub(crate) async fn github_start(State(state): State<AppState>, session: Session) -> Response {
    let github = match state.github.as_deref() {
        Some(github) => github,
        None => {
            audit::record(audit::AuditEvent::LoginUnavailable, None);
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({"error": "member_authentication_unavailable"})),
            )
                .into_response();
        }
    };

    audit::record(audit::AuditEvent::LoginStarted, None);
    let client = config::github_client(github);

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
        audit::record(audit::AuditEvent::LoginFailed, None);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to initialize login",
        )
            .into_response();
    }

    Redirect::to(auth_url.as_str()).into_response()
}

pub(crate) async fn github_callback(
    State(state): State<AppState>,
    Query(query): Query<OAuthCallback>,
    session: Session,
) -> Response {
    let github = match state.github.as_deref() {
        Some(github) => github,
        None => {
            audit::record(audit::AuditEvent::LoginUnavailable, None);
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({"error": "member_authentication_unavailable"})),
            )
                .into_response();
        }
    };

    let expected_state = session.get::<String>(OAUTH_STATE_KEY).await.ok().flatten();
    let verifier = session
        .get::<String>(OAUTH_VERIFIER_KEY)
        .await
        .ok()
        .flatten();

    let _ = session.remove::<String>(OAUTH_STATE_KEY).await;
    let _ = session.remove::<String>(OAUTH_VERIFIER_KEY).await;

    if expected_state.as_deref() != Some(query.state.as_str()) {
        audit::record(audit::AuditEvent::LoginFailed, None);
        return (StatusCode::UNAUTHORIZED, "Login state was invalid").into_response();
    }

    let verifier = match verifier {
        Some(value) => value,
        None => {
            audit::record(audit::AuditEvent::LoginFailed, None);
            return (StatusCode::UNAUTHORIZED, "Login state was invalid").into_response();
        }
    };

    let token = match config::github_client(github)
        .exchange_code(AuthorizationCode::new(query.code))
        .set_pkce_verifier(PkceCodeVerifier::new(verifier))
        .request_async(&ReqwestClient::from(state.http.clone()))
        .await
    {
        Ok(token) => token,
        Err(_) => {
            audit::record(audit::AuditEvent::LoginFailed, None);
            return (StatusCode::UNAUTHORIZED, "GitHub authentication failed").into_response();
        }
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
            Err(_) => {
                audit::record(audit::AuditEvent::LoginFailed, None);
                return (StatusCode::UNAUTHORIZED, "Unable to read member identity")
                    .into_response();
            }
        },
        Err(_) => {
            audit::record(audit::AuditEvent::LoginFailed, None);
            return (StatusCode::UNAUTHORIZED, "Unable to verify member identity").into_response();
        }
    };

    if !is_allowed_member(&user.login, &github.allowed_logins) {
        audit::record(audit::AuditEvent::LoginRejected, Some(&user.login));
        info!(github_login = %user.login, "Rejected non-member login");
        let _ = session.clear().await;
        return (
            StatusCode::FORBIDDEN,
            "This GitHub account is not authorized for GyLiber Command Center",
        )
            .into_response();
    }

    if session.cycle_id().await.is_err() || session.insert(MEMBER_KEY, &user).await.is_err() {
        audit::record(audit::AuditEvent::LoginFailed, Some(&user.login));
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to establish secure session",
        )
            .into_response();
    }

    audit::record(audit::AuditEvent::LoginSucceeded, Some(&user.login));
    Redirect::to("/command").into_response()
}

pub(crate) async fn logout(session: Session) -> Response {
    let member = member_from_session(&session).await;
    audit::record(
        audit::AuditEvent::Logout,
        member.as_ref().map(|user| user.login.as_str()),
    );
    let _ = session.clear().await;
    Redirect::to("/").into_response()
}

pub(crate) async fn member_from_session(session: &Session) -> Option<GitHubUser> {
    session.get::<GitHubUser>(MEMBER_KEY).await.ok().flatten()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AuthFailure {
    PageLogin,
    ApiUnauthorized,
}

impl IntoResponse for AuthFailure {
    fn into_response(self) -> Response {
        match self {
            Self::PageLogin => Redirect::to("/login").into_response(),
            Self::ApiUnauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "authentication_required"})),
            )
                .into_response(),
        }
    }
}

pub(crate) async fn require_page_member(session: &Session) -> Result<GitHubUser, AuthFailure> {
    member_from_session(session).await.ok_or_else(|| {
        audit::record(audit::AuditEvent::ProtectedAccessDenied, None);
        AuthFailure::PageLogin
    })
}

pub(crate) async fn require_api_member(session: &Session) -> Result<GitHubUser, AuthFailure> {
    member_from_session(session).await.ok_or_else(|| {
        audit::record(audit::AuditEvent::ProtectedAccessDenied, None);
        AuthFailure::ApiUnauthorized
    })
}

pub(crate) fn is_allowed_member(login: &str, allowed_logins: &[String]) -> bool {
    let normalized = login.trim();
    allowed_logins
        .iter()
        .any(|allowed| allowed.trim().eq_ignore_ascii_case(normalized))
}

#[cfg(test)]
mod tests {
    use super::{AuthFailure, is_allowed_member};
    use axum::{http::StatusCode, response::IntoResponse};

    #[test]
    fn page_auth_failure_redirects_to_login() {
        let response = AuthFailure::PageLogin.into_response();
        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        assert_eq!(
            response
                .headers()
                .get("location")
                .and_then(|value| value.to_str().ok()),
            Some("/login")
        );
    }

    #[test]
    fn api_auth_failure_returns_unauthorized() {
        let response = AuthFailure::ApiUnauthorized.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn member_allowlist_is_case_insensitive_and_trimmed() {
        let allowed = vec!["GyLiber".to_string(), "ExampleMember".to_string()];
        assert!(is_allowed_member("gyliber", &allowed));
        assert!(is_allowed_member(" EXAMPLEMEMBER ", &allowed));
        assert!(!is_allowed_member("intruder", &allowed));
    }
}
