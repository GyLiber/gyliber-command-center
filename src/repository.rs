use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::Client;

const GITHUB_REPOSITORY_URL: &str =
    "https://api.github.com/repos/GyLiber/gyliber-command-center";

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RepositorySnapshot {
    pub source: &'static str,
    pub observed_at_unix: u64,
    pub full_name: String,
    pub html_url: String,
    pub default_branch: String,
    pub visibility: String,
    pub open_issues: u64,
    pub stars: u64,
    pub forks: u64,
    pub last_push_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GitHubRepository {
    full_name: String,
    html_url: String,
    default_branch: String,
    visibility: String,
    open_issues_count: u64,
    stargazers_count: u64,
    forks_count: u64,
    pushed_at: Option<String>,
}

pub async fn snapshot(client: &Client) -> Result<RepositorySnapshot, reqwest::Error> {
    let repository = client
        .get(GITHUB_REPOSITORY_URL)
        .header(reqwest::header::ACCEPT, "application/vnd.github+json")
        .send()
        .await?
        .error_for_status()?
        .json::<GitHubRepository>()
        .await?;

    Ok(RepositorySnapshot {
        source: GITHUB_REPOSITORY_URL,
        observed_at_unix: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock must be after Unix epoch")
            .as_secs(),
        full_name: repository.full_name,
        html_url: repository.html_url,
        default_branch: repository.default_branch,
        visibility: repository.visibility,
        open_issues: repository.open_issues_count,
        stars: repository.stargazers_count,
        forks: repository.forks_count,
        last_push_at: repository.pushed_at,
    })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn repository_payload_deserializes_into_expected_shape() {
        let payload = json!({
            "full_name": "GyLiber/gyliber-command-center",
            "html_url": "https://github.com/GyLiber/gyliber-command-center",
            "default_branch": "main",
            "visibility": "public",
            "open_issues_count": 0,
            "stargazers_count": 0,
            "forks_count": 0,
            "pushed_at": "2026-09-29T00:00:00Z"
        });

        let repository: super::GitHubRepository =
            serde_json::from_value(payload).expect("GitHub fixture deserializes");
        assert_eq!(repository.full_name, "GyLiber/gyliber-command-center");
        assert_eq!(repository.default_branch, "main");
        assert_eq!(repository.visibility, "public");
    }
}
