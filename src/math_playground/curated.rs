// Only reviewed, catalogued compile-time bytes are promoted to application trust.
use super::{failure, javascript};
use crate::auth;
use axum::{
    Json,
    extract::Path,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde_json::{Value, json};
use tower_sessions::Session;

include!(concat!(env!("OUT_DIR"), "/curated_files.rs"));
const CATALOG: &str = include_str!("../../math-playground/catalog.json");
const TEMPLATE: &str =
    include_str!("../../docs/operations/prompts/MATH_PLAYGROUND_EXTERNAL_AUTHORING.md");

pub(super) async fn catalog(session: Session) -> Response {
    if let Err(error) = auth::require_api_member(&session).await {
        return error.into_response();
    }
    let mut catalog: Value = serde_json::from_str(CATALOG).expect("build validated catalog");
    catalog["deployment_commit"] = json!(std::env::var("RENDER_GIT_COMMIT").ok().filter(|value| {
        value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
    }));
    Json(catalog).into_response()
}
pub(super) async fn template(session: Session) -> Response {
    if let Err(error) = auth::require_api_member(&session).await {
        return error.into_response();
    }
    (
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        TEMPLATE,
    )
        .into_response()
}
pub(super) async fn artifact(
    session: Session,
    Path((id, version, name)): Path<(String, String, String)>,
) -> Response {
    if let Err(error) = auth::require_api_member(&session).await {
        return error.into_response();
    }
    let Some(content) = file(&id, &version, &name) else {
        return failure(StatusCode::NOT_FOUND, "exhibit_not_found");
    };
    let kind = if name.ends_with(".json") {
        "application/json"
    } else if name.ends_with(".html") {
        "text/html; charset=utf-8"
    } else if name.ends_with(".css") {
        "text/css; charset=utf-8"
    } else {
        "text/plain; charset=utf-8"
    };
    if name.ends_with(".mjs") {
        return javascript(content.to_owned());
    }
    ([(header::CONTENT_TYPE, kind)], content).into_response()
}
pub(super) async fn package(
    session: Session,
    Path((id, version)): Path<(String, String)>,
) -> Response {
    if let Err(error) = auth::require_api_member(&session).await {
        return error.into_response();
    }
    let Some(content) = file(&id, &version, "manifest.json") else {
        return failure(StatusCode::NOT_FOUND, "exhibit_not_found");
    };
    let manifest: Value = serde_json::from_str(content).expect("build validated curated manifest");
    let mut files = serde_json::Map::new();
    for name in manifest["files"]
        .as_object()
        .expect("build validated files")
        .keys()
    {
        files.insert(
            name.clone(),
            json!(file(&id, &version, name).expect("build embedded every manifest file")),
        );
    }
    files.insert("manifest.json".into(), json!(content));
    Json(json!({"manifest": manifest, "files": files})).into_response()
}
