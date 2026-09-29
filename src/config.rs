use std::{env, sync::Arc, time::Duration as StdDuration};

use anyhow::{Context, Result};
use oauth2::{
    AuthUrl, ClientId, ClientSecret, EndpointNotSet, EndpointSet, RedirectUrl, TokenUrl,
    basic::BasicClient,
};
use reqwest::Client;
use tower_sessions::cookie::Key;

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) github: Option<Arc<GitHubConfig>>,
    pub(crate) http: Client,
}

#[derive(Clone)]
pub(crate) struct GitHubConfig {
    pub(crate) client_id: ClientId,
    pub(crate) client_secret: ClientSecret,
    pub(crate) redirect_url: RedirectUrl,
    pub(crate) allowed_logins: Vec<String>,
}

pub(crate) type GitHubClient =
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>;

pub(crate) fn github_client(config: &GitHubConfig) -> GitHubClient {
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

pub(crate) fn load_state() -> Result<AppState> {
    let github_configured = [
        "GITHUB_CLIENT_ID",
        "GITHUB_CLIENT_SECRET",
        "GYLIBER_ALLOWED_GITHUB_LOGINS",
    ]
    .iter()
    .any(|name| env::var(name).is_ok());

    let github = if github_configured {
        let client_id =
            ClientId::new(env::var("GITHUB_CLIENT_ID").context("GITHUB_CLIENT_ID is required")?);
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

        anyhow::ensure!(
            !allowed_logins.is_empty(),
            "GYLIBER_ALLOWED_GITHUB_LOGINS cannot be empty"
        );

        Some(Arc::new(GitHubConfig {
            client_id,
            client_secret,
            redirect_url,
            allowed_logins,
        }))
    } else {
        None
    };

    Ok(AppState {
        github,
        http: Client::builder()
            .user_agent("GyLiber-Command-Center/0.1.0")
            .redirect(reqwest::redirect::Policy::none())
            .timeout(StdDuration::from_secs(5))
            .build()?,
    })
}

pub(crate) fn parse_bool(name: &str, value: &str) -> Result<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => anyhow::bail!("{name} must be true or false"),
    }
}

pub(crate) fn validate_runtime_security() -> Result<()> {
    let production = env::var("APP_ENV")
        .map(|value| value.eq_ignore_ascii_case("production"))
        .unwrap_or(false);

    if production {
        let cookie_secure =
            env::var("COOKIE_SECURE").context("COOKIE_SECURE is required in production")?;
        anyhow::ensure!(
            cookie_secure.eq_ignore_ascii_case("true"),
            "COOKIE_SECURE must be true in production"
        );

        if let Ok(callback) = env::var("GITHUB_REDIRECT_URL") {
            let parsed = url::Url::parse(&callback)
                .context("GITHUB_REDIRECT_URL must be a valid URL in production")?;
            anyhow::ensure!(
                parsed.scheme() == "https",
                "GITHUB_REDIRECT_URL must use HTTPS in production"
            );
        }
    }

    Ok(())
}

pub(crate) fn load_session_key() -> Result<Key> {
    let master = env::var("SESSION_MASTER_KEY")
        .context("SESSION_MASTER_KEY is required and must contain at least 64 random bytes")?;
    Key::try_from(master.as_bytes())
        .context("SESSION_MASTER_KEY must contain at least 64 bytes for a private session key")
}
