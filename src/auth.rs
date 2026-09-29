use anyhow::Result;
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope};
use oauth2_reqwest::ReqwestClient;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use tracing::info;

use crate::config::{self, AppState};

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
    let client = config::github_client(&state.github);

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

    let token = match config::github_client(&state.github)
        .exchange_code(AuthorizationCode::new(query.code))
        .set_pkce_verifier(PkceCodeVerifier::new(verifier))
        .request_async(&ReqwestClient::from(state.http.clone()))
        .await
    {
        Ok(token) => token,
        Err(_) => {
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
                return (StatusCode::UNAUTHORIZED, "Unable to read member identity")
                    .into_response();
            }
        },
        Err(_) => {
            return (StatusCode::UNAUTHORIZED, "Unable to verify member identity").into_response();
        }
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

    if session.cycle_id().await.is_err() || session.insert(MEMBER_KEY, &user).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to establish secure session",
        )
            .into_response();
    }

    Redirect::to("/command").into_response()
}

pub(crate) async fn logout(session: Session) -> Response {
    let _ = session.clear().await;
    Redirect::to("/").into_response()
}

pub(crate) async fn member_from_session(session: &Session) -> Option<GitHubUser> {
    session.get::<GitHubUser>(MEMBER_KEY).await.ok().flatten()
}

pub(crate) fn is_allowed_member(login: &str, allowed_logins: &[String]) -> bool {
    let normalized = login.trim();
    allowed_logins
        .iter()
        .any(|allowed| allowed.trim().eq_ignore_ascii_case(normalized))
}

#[allow(dead_code)]
fn _keep_reqwest_type(_: &Client) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::is_allowed_member;

    #[test]
    fn member_allowlist_is_case_insensitive_and_trimmed() {
        let allowed = vec!["GyLiber".to_string(), "ExampleMember".to_string()];
        assert!(is_allowed_member("gyliber", &allowed));
        assert!(is_allowed_member(" EXAMPLEMEMBER ", &allowed));
        assert!(!is_allowed_member("intruder", &allowed));
    }
}
