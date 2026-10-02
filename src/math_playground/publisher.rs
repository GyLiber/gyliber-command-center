use super::model::Exhibit;
use base64::Engine;
use reqwest::{Client, Method, StatusCode};
use serde_json::{Value, json};
use std::time::Duration;

const API: &str = "https://api.github.com/repos/GyLiber/gyliber-command-center";
pub(super) const BRANCH: &str = "math-playground-artifacts";

pub(super) fn manifest(exhibit: &Exhibit) -> Value {
    // This public manifest contains only reviewed, generic engine configuration.
    // No title, course excerpt, formal source statement, hash of coursework or member identity.
    json!({"format": 1, "id": exhibit.id, "engine_version": exhibit.version,
        "kind": exhibit.concept.kind, "palette": exhibit.concept.palette,
        "engine_sha256": exhibit.engine_sha256, "renderer_sha256": exhibit.renderer_sha256})
}

async fn call(
    client: &Client,
    token: &str,
    method: Method,
    path: &str,
    body: Option<Value>,
) -> anyhow::Result<(StatusCode, Value)> {
    let mut request = client
        .request(method, format!("{API}/{path}"))
        .bearer_auth(token)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .timeout(Duration::from_secs(20));
    if let Some(body) = body {
        request = request.json(&body);
    }
    let response = request.send().await?;
    let status = response.status();
    let value = super::bounded_json(response, 256 * 1024).await?;
    Ok((status, value))
}

fn sha(value: &Value, pointer: &str) -> anyhow::Result<String> {
    let sha = value.pointer(pointer).and_then(Value::as_str).unwrap_or("");
    anyhow::ensure!(
        sha.len() == 40 && sha.bytes().all(|byte| byte.is_ascii_hexdigit()),
        "invalid Git object response"
    );
    Ok(sha.into())
}

pub(super) async fn publish(
    client: &Client,
    token: &str,
    exhibit: &Exhibit,
    engine: &str,
    renderer: &str,
) -> anyhow::Result<String> {
    let ref_path = format!("git/ref/heads/{BRANCH}");
    let (mut status, mut reference) = call(client, token, Method::GET, &ref_path, None).await?;
    if status == StatusCode::NOT_FOUND {
        let (base_status, main) =
            call(client, token, Method::GET, "git/ref/heads/main", None).await?;
        anyhow::ensure!(base_status.is_success(), "unable to resolve artifact base");
        let main_sha = sha(&main, "/object/sha")?;
        let (create_status, _) = call(
            client,
            token,
            Method::POST,
            "git/refs",
            Some(json!({"ref": format!("refs/heads/{BRANCH}"), "sha": main_sha})),
        )
        .await?;
        // Concurrent first publishers can race to create the branch; fetch authoritative state again.
        anyhow::ensure!(
            create_status.is_success() || create_status == StatusCode::UNPROCESSABLE_ENTITY,
            "unable to create artifact branch"
        );
        (status, reference) = call(client, token, Method::GET, &ref_path, None).await?;
    }
    anyhow::ensure!(status.is_success(), "unable to read artifact branch");
    let parent = sha(&reference, "/object/sha")?;
    let path = format!(
        "math-playground/exhibits/{}/{}",
        exhibit.id, exhibit.version
    );
    let public_manifest = manifest(exhibit);
    let (status, existing) = call(
        client,
        token,
        Method::GET,
        &format!("contents/{path}/manifest.json?ref={BRANCH}"),
        None,
    )
    .await?;
    if status.is_success() {
        let encoded = existing["content"].as_str().unwrap_or("").replace('\n', "");
        let bytes = base64::engine::general_purpose::STANDARD.decode(encoded)?;
        let value: Value = serde_json::from_slice(&bytes)?;
        anyhow::ensure!(
            value == public_manifest,
            "existing artifact does not match this immutable release"
        );
        for (name, expected) in [("engine.mjs", engine), ("renderer.mjs", renderer)] {
            let (status, file) = call(
                client,
                token,
                Method::GET,
                &format!("contents/{path}/{name}?ref={parent}"),
                None,
            )
            .await?;
            anyhow::ensure!(status.is_success(), "existing artifact file is missing");
            let encoded = file["content"].as_str().unwrap_or("").replace('\n', "");
            let bytes = base64::engine::general_purpose::STANDARD.decode(encoded)?;
            anyhow::ensure!(
                bytes == expected.as_bytes(),
                "existing artifact code differs from accepted release"
            );
        }
        // The returned snapshot contains this exact artifact even after other exhibits were appended.
        return Ok(parent);
    }
    anyhow::ensure!(
        status == StatusCode::NOT_FOUND,
        "unable to check artifact identity"
    );
    let (status, commit) = call(
        client,
        token,
        Method::GET,
        &format!("git/commits/{parent}"),
        None,
    )
    .await?;
    anyhow::ensure!(status.is_success(), "unable to read artifact tree");
    let base_tree = sha(&commit, "/tree/sha")?;
    let entries = [
        ("engine.mjs", engine.to_owned()),
        ("renderer.mjs", renderer.to_owned()),
        (
            "manifest.json",
            serde_json::to_string_pretty(&public_manifest)?,
        ),
    ];
    let tree = entries.into_iter().map(|(name, content)| json!({
        "path": format!("{path}/{name}"), "mode": "100644", "type": "blob", "content": content
    })).collect::<Vec<_>>();
    let (status, tree) = call(
        client,
        token,
        Method::POST,
        "git/trees",
        Some(json!({"base_tree": base_tree, "tree": tree})),
    )
    .await?;
    anyhow::ensure!(status.is_success(), "unable to write artifact tree");
    let tree_sha = sha(&tree, "/sha")?;
    let (status, commit) = call(client, token, Method::POST, "git/commits", Some(json!({
        "message": format!("feat: preserve mathematics exhibit {} v{}", exhibit.id, exhibit.version),
        "tree": tree_sha, "parents": [parent]
    }))).await?;
    anyhow::ensure!(status.is_success(), "unable to write artifact commit");
    let commit_sha = sha(&commit, "/sha")?;
    let (status, _) = call(
        client,
        token,
        Method::PATCH,
        &format!("git/refs/heads/{BRANCH}"),
        Some(json!({"sha": commit_sha, "force": false})),
    )
    .await?;
    anyhow::ensure!(
        status.is_success(),
        "artifact branch advanced concurrently; retry safely"
    );
    Ok(commit_sha)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn public_manifest_has_no_course_data_or_member_metadata() {
        let concept = serde_json::from_value(json!({"title":"PRIVATE TITLE","kind":"metaphor",
            "concept_type":"theorem","notation":"PRIVATE NOTATION","hypotheses":"PRIVATE ASSUMPTION",
            "statement":"PRIVATE STATEMENT","visual_mapping":"PRIVATE MAPPING","limitations":"PRIVATE LIMIT",
            "source_file":"PRIVATE FILE","source_quote":"PRIVATE QUOTE","palette":"candy"})).unwrap();
        let exhibit = Exhibit {
            id: "0".repeat(32),
            version: "0.1.0".into(),
            formal_version: "0.2.0".into(),
            concept,
            source_sha256: "PRIVATE HASH".into(),
            engine_sha256: "engine".into(),
            renderer_sha256: "renderer".into(),
            created_at: 0,
            review: "PRIVATE MEMBER".into(),
            repository_commit: None,
        };
        let value = manifest(&exhibit).to_string();
        assert!(!value.contains("PRIVATE"));
        assert!(!value.contains("source"));
    }
}
