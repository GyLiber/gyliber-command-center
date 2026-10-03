use crate::{auth, config::AppState};
use anyhow::{Context, Result};
use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Path, State},
    http::{HeaderMap, StatusCode, header},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use model::{Exhibit, Intake, digest};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::PgPool;
use std::{env, time::Duration};
use tokio::sync::Semaphore;
use tower_sessions::Session;

mod generation;
mod model;
mod publisher;
mod store;

#[derive(Default)]
struct Settings {
    provider: generation::Provider,
    key: Option<String>,
    model: Option<String>,
    publisher: Option<String>,
}

pub(crate) struct Service {
    settings: Settings,
    pool: Option<PgPool>,
    http: reqwest::Client,
    jobs: Semaphore,
    publishing: Semaphore,
}

impl Default for Service {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            pool: None,
            http: reqwest::Client::builder()
                .user_agent("GyLiber-Math-Playground")
                .redirect(reqwest::redirect::Policy::none())
                .timeout(Duration::from_secs(5))
                .build()
                .expect("static HTTP client configuration is valid"),
            jobs: Semaphore::new(2),
            publishing: Semaphore::new(1),
        }
    }
}

impl Service {
    pub(crate) fn from_environment() -> Result<Self> {
        fn optional(name: &str) -> Result<Option<String>> {
            match env::var(name) {
                Ok(value) => {
                    anyhow::ensure!(!value.trim().is_empty(), "{name} cannot be empty");
                    Ok(Some(value))
                }
                Err(env::VarError::NotPresent) => Ok(None),
                Err(error) => Err(error.into()),
            }
        }
        let provider =
            generation::Provider::from_setting(optional("MATH_AI_PROVIDER")?.as_deref())?;
        let (key_name, model_name) = provider.environment_names();
        let key = optional(key_name)?;
        let model = optional(model_name)?;
        anyhow::ensure!(
            key.is_some() == model.is_some(),
            "set both {key_name} and {model_name} for the selected AI provider"
        );
        if let Some(model) = &model {
            anyhow::ensure!(
                generation::valid_model(model),
                "invalid mathematics model identifier"
            );
        }
        Ok(Self {
            settings: Settings {
                provider,
                key,
                model,
                publisher: optional("MATH_GITHUB_TOKEN")?,
            },
            ..Self::default()
        })
    }

    pub(crate) async fn initialize(&mut self, pool: Option<PgPool>) -> Result<()> {
        if let Some(pool) = &pool {
            sqlx::raw_sql(include_str!("../../migrations/0002_math_playground.sql"))
                .execute(pool)
                .await
                .context("unable to migrate mathematics playground")?;
            store::clean(pool).await?;
        }
        self.pool = pool;
        Ok(())
    }
}

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/command/math-playground", get(page))
        .route("/api/math-playground", get(catalog))
        .route("/api/math-playground/demos/{id}", get(demo))
        .route(
            "/api/math-playground/demos/{id}/engine.mjs",
            get(demo_engine),
        )
        .route(
            "/api/math-playground/runtime/renderer.mjs",
            get(demo_renderer),
        )
        .route(
            "/api/math-playground/runtime/{version}/renderer.mjs",
            get(versioned_demo_renderer),
        )
        .route("/api/math-playground/drafts", post(create))
        .route(
            "/api/math-playground/exhibits/{id}",
            get(detail).delete(remove),
        )
        .route(
            "/api/math-playground/exhibits/{id}/engine.mjs",
            get(engine_code),
        )
        .route(
            "/api/math-playground/exhibits/{id}/renderer.mjs",
            get(renderer_code),
        )
        .route("/api/math-playground/exhibits/{id}/publish", post(publish))
        .layer(DefaultBodyLimit::max(512 * 1024))
        .layer(tower_http::limit::RequestBodyLimitLayer::new(512 * 1024))
}

fn failure(status: StatusCode, code: &str) -> Response {
    (status, Json(json!({"error": code}))).into_response()
}
struct AccessError(StatusCode, &'static str);
impl IntoResponse for AccessError {
    fn into_response(self) -> Response {
        failure(self.0, self.1)
    }
}

fn unavailable() -> Response {
    failure(
        StatusCode::SERVICE_UNAVAILABLE,
        "playground_storage_unavailable",
    )
}
fn storage_failure() -> Response {
    // Intentionally avoid serializing database/provider errors, connection URLs or source text.
    tracing::warn!(
        event_code = "MATH_STORAGE_UNAVAILABLE",
        "Mathematics storage operation failed"
    );
    failure(
        StatusCode::SERVICE_UNAVAILABLE,
        "playground_storage_unavailable",
    )
}

async fn page(session: Session) -> Response {
    if let Err(error) = auth::require_page_member(&session).await {
        return error.into_response();
    }
    Html(include_str!("../../static/math-playground.html")).into_response()
}

const CSRF: &str = "math_csrf";
async fn authorize_write(
    session: &Session,
    headers: &HeaderMap,
) -> Result<auth::GitHubUser, AccessError> {
    let member = auth::require_api_member(session)
        .await
        .map_err(|_| AccessError(StatusCode::UNAUTHORIZED, "authentication_required"))?;
    let expected = session.get::<String>(CSRF).await.ok().flatten();
    if expected.is_none()
        || headers
            .get("x-math-csrf")
            .and_then(|value| value.to_str().ok())
            != expected.as_deref()
    {
        return Err(AccessError(StatusCode::FORBIDDEN, "invalid_request_token"));
    }
    Ok(member)
}

async fn catalog(State(state): State<AppState>, session: Session) -> Response {
    let member = match auth::require_api_member(&session).await {
        Ok(member) => member,
        Err(error) => return error.into_response(),
    };
    let token = match session.get::<String>(CSRF).await {
        Ok(Some(token)) => token,
        Ok(None) => {
            let token = oauth2::CsrfToken::new_random().secret().clone();
            if session.insert(CSRF, &token).await.is_err() {
                return storage_failure();
            }
            token
        }
        Err(_) => return storage_failure(),
    };
    let service = &state.math;
    let exhibits = if let Some(pool) = &service.pool {
        if store::clean(pool).await.is_err() {
            return storage_failure();
        }
        match store::list(pool, &member.login.to_ascii_lowercase()).await {
            Ok(value) => value,
            Err(_) => return storage_failure(),
        }
    } else {
        Vec::new()
    };
    Json(json!({"csrf": token, "ai_ready": service.pool.is_some() && service.settings.key.is_some(),
        "ai_provider": service.settings.provider.name(),
        "publishing_ready": service.pool.is_some() && service.settings.publisher.is_some(),
        "storage_ready": service.pool.is_some(), "exhibits": exhibits,
        "limits": {"files": 8, "file_bytes": 32768, "total_bytes": 65536, "requests_per_24_hours": 8},
        "proof_status": "deferred_to_0.3.0"})).into_response()
}

async fn create(
    State(state): State<AppState>,
    session: Session,
    headers: HeaderMap,
    Json(intake): Json<Intake>,
) -> Response {
    let member = match authorize_write(&session, &headers).await {
        Ok(value) => value,
        Err(error) => return error.into_response(),
    };
    if let Err(error) = intake.validate() {
        return failure(StatusCode::UNPROCESSABLE_ENTITY, error);
    }
    let service = &state.math;
    if intake.ai_provider != service.settings.provider {
        return failure(StatusCode::CONFLICT, "ai_provider_changed_refresh_consent");
    }
    let (Some(pool), Some(key), Some(model)) = (
        &service.pool,
        &service.settings.key,
        &service.settings.model,
    ) else {
        return failure(
            StatusCode::SERVICE_UNAVAILABLE,
            "ai_authoring_not_configured",
        );
    };
    let Ok(_permit) = service.jobs.try_acquire() else {
        return failure(StatusCode::TOO_MANY_REQUESTS, "authoring_busy_retry_later");
    };
    let owner = member.login.to_ascii_lowercase();
    match store::reserve_attempt(pool, &owner).await {
        Ok(true) => {}
        Ok(false) => {
            return failure(
                StatusCode::TOO_MANY_REQUESTS,
                "daily_authoring_limit_reached",
            );
        }
        Err(_) => return storage_failure(),
    }
    let concept = match generation::generate(
        &service.http,
        service.settings.provider,
        key,
        model,
        &intake,
    )
    .await
    {
        Ok(value) => value,
        Err(error) => {
            tracing::warn!(
                event_code = "MATH_AI_FAILED",
                reason = error,
                "Mathematics authoring failed"
            );
            return failure(StatusCode::BAD_GATEWAY, error);
        }
    };
    let nonce = oauth2::CsrfToken::new_random();
    let id = digest(nonce.secret().as_bytes())[..32].to_owned();
    let code = model::engine(&id, &concept);
    let source = serde_json::to_vec(&intake.files).expect("source serializes");
    let exhibit = Exhibit {
        id,
        version: "0.2.0".into(),
        formal_version: "0.2.0".into(),
        concept,
        source_sha256: digest(&source),
        engine_sha256: digest(code.as_bytes()),
        renderer_sha256: digest(model::RENDERER.as_bytes()),
        created_at: time::OffsetDateTime::now_utc().unix_timestamp(),
        review: "ai_draft_unverified".into(),
        repository_commit: None,
    };
    if store::save(pool, &owner, &exhibit, &code).await.is_err() {
        return storage_failure();
    }
    (StatusCode::CREATED, Json(exhibit)).into_response()
}

async fn load(
    state: &AppState,
    session: &Session,
    id: &str,
) -> Result<(Exhibit, String, String), AccessError> {
    let member = auth::require_api_member(session)
        .await
        .map_err(|_| AccessError(StatusCode::UNAUTHORIZED, "authentication_required"))?;
    if !model::valid_id(id) {
        return Err(AccessError(StatusCode::NOT_FOUND, "exhibit_not_found"));
    }
    let pool = state.math.pool.as_ref().ok_or(AccessError(
        StatusCode::SERVICE_UNAVAILABLE,
        "playground_storage_unavailable",
    ))?;
    store::get(pool, &member.login.to_ascii_lowercase(), id)
        .await
        .map_err(|_| {
            AccessError(
                StatusCode::SERVICE_UNAVAILABLE,
                "playground_storage_unavailable",
            )
        })?
        .ok_or(AccessError(StatusCode::NOT_FOUND, "exhibit_not_found"))
}

async fn detail(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
) -> Response {
    match load(&state, &session, &id).await {
        Ok((exhibit, _, _)) => Json(exhibit).into_response(),
        Err(error) => error.into_response(),
    }
}
fn javascript(code: String) -> Response {
    (
        [(header::CONTENT_TYPE, "text/javascript; charset=utf-8")],
        code,
    )
        .into_response()
}
async fn engine_code(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
) -> Response {
    match load(&state, &session, &id).await {
        Ok((_, code, _)) => javascript(code),
        Err(error) => error.into_response(),
    }
}
async fn renderer_code(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<String>,
) -> Response {
    match load(&state, &session, &id).await {
        Ok((_, _, code)) => javascript(code),
        Err(error) => error.into_response(),
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Review {
    mathematics_reviewed: bool,
    public_code_consent: bool,
}

async fn publish(
    State(state): State<AppState>,
    session: Session,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(review): Json<Review>,
) -> Response {
    let member = match authorize_write(&session, &headers).await {
        Ok(value) => value,
        Err(error) => return error.into_response(),
    };
    if !review.mathematics_reviewed || !review.public_code_consent {
        return failure(
            StatusCode::UNPROCESSABLE_ENTITY,
            "mathematics_review_and_public_code_consent_required",
        );
    }
    let (exhibit, engine, renderer) = match load(&state, &session, &id).await {
        Ok(value) => value,
        Err(error) => return error.into_response(),
    };
    let service = &state.math;
    let Some(token) = &service.settings.publisher else {
        return failure(
            StatusCode::SERVICE_UNAVAILABLE,
            "repository_publishing_not_configured",
        );
    };
    if let Some(commit) = &exhibit.repository_commit {
        return Json(json!({"commit": commit})).into_response();
    }
    let Ok(_permit) = service.publishing.try_acquire() else {
        return failure(StatusCode::TOO_MANY_REQUESTS, "publishing_busy_retry_later");
    };
    let commit = match publisher::publish(&service.http, token, &exhibit, &engine, &renderer).await
    {
        Ok(commit) => commit,
        Err(_) => {
            tracing::warn!(
                event_code = "MATH_PUBLISH_FAILED",
                "Mathematics repository publish failed"
            );
            return failure(
                StatusCode::BAD_GATEWAY,
                "repository_publish_failed_retry_safely",
            );
        }
    };
    let Some(pool) = &service.pool else {
        return unavailable();
    };
    if store::publish(pool, &member.login.to_ascii_lowercase(), &id, &commit)
        .await
        .is_err()
    {
        return storage_failure();
    }
    Json(json!({"commit": commit})).into_response()
}

async fn remove(
    State(state): State<AppState>,
    session: Session,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    let member = match authorize_write(&session, &headers).await {
        Ok(value) => value,
        Err(error) => return error.into_response(),
    };
    if !model::valid_id(&id) {
        return failure(StatusCode::NOT_FOUND, "exhibit_not_found");
    }
    let Some(pool) = &state.math.pool else {
        return unavailable();
    };
    match store::delete(pool, &member.login.to_ascii_lowercase(), &id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => storage_failure(),
    }
}

pub(super) async fn bounded_json(mut response: reqwest::Response, limit: usize) -> Result<Value> {
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await? {
        anyhow::ensure!(
            bytes.len() + chunk.len() <= limit,
            "upstream response exceeds limit"
        );
        bytes.extend_from_slice(&chunk);
    }
    Ok(serde_json::from_slice(&bytes)?)
}

fn demo_artifact(id: &str, code: bool) -> Option<&'static str> {
    match (id, code) {
        ("giant-pi", false) => Some(include_str!(
            "../../math-playground/exhibits/giant-pi/0.2.0/manifest.json"
        )),
        ("giant-pi", true) => Some(include_str!(
            "../../math-playground/exhibits/giant-pi/0.2.0/engine.mjs"
        )),
        ("snug-tails", false) => Some(include_str!(
            "../../math-playground/exhibits/snug-tails/0.2.0/manifest.json"
        )),
        ("snug-tails", true) => Some(include_str!(
            "../../math-playground/exhibits/snug-tails/0.2.0/engine.mjs"
        )),
        ("creature-shuffle", false) => Some(include_str!(
            "../../math-playground/exhibits/creature-shuffle/0.2.0/manifest.json"
        )),
        ("creature-shuffle", true) => Some(include_str!(
            "../../math-playground/exhibits/creature-shuffle/0.2.0/engine.mjs"
        )),
        ("memory-cloud", false) => Some(include_str!(
            "../../math-playground/exhibits/memory-cloud/0.2.0/manifest.json"
        )),
        ("memory-cloud", true) => Some(include_str!(
            "../../math-playground/exhibits/memory-cloud/0.2.0/engine.mjs"
        )),
        _ => None,
    }
}
async fn demo(session: Session, Path(id): Path<String>) -> Response {
    if let Err(error) = auth::require_api_member(&session).await {
        return error.into_response();
    }
    match demo_artifact(&id, false) {
        Some(content) => ([(header::CONTENT_TYPE, "application/json")], content).into_response(),
        None => failure(StatusCode::NOT_FOUND, "exhibit_not_found"),
    }
}
async fn demo_engine(session: Session, Path(id): Path<String>) -> Response {
    if let Err(error) = auth::require_api_member(&session).await {
        return error.into_response();
    }
    match demo_artifact(&id, true) {
        Some(content) => javascript(content.to_owned()),
        None => failure(StatusCode::NOT_FOUND, "exhibit_not_found"),
    }
}
async fn demo_renderer(session: Session) -> Response {
    if let Err(error) = auth::require_api_member(&session).await {
        return error.into_response();
    }
    javascript(model::RENDERER.to_owned())
}

async fn versioned_demo_renderer(session: Session, Path(version): Path<String>) -> Response {
    if let Err(error) = auth::require_api_member(&session).await {
        return error.into_response();
    }
    match version.as_str() {
        "0.1.0" => javascript(model::LEGACY_RENDERER.to_owned()),
        "0.2.0" => javascript(model::RENDERER.to_owned()),
        _ => failure(StatusCode::NOT_FOUND, "exhibit_not_found"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{Body, to_bytes},
        http::Request,
    };
    use std::sync::Arc;
    use tower::ServiceExt;
    use tower_sessions::cookie::{Cookie, CookieJar, Key};

    async fn app_with_member() -> (Router, String) {
        let store = tower_sessions::MemoryStore::default();
        let session = Session::new(None, Arc::new(store.clone()), None);
        session
            .insert(
                auth::MEMBER_KEY,
                auth::GitHubUser {
                    login: "test-member".into(),
                    name: None,
                    avatar_url: None,
                },
            )
            .await
            .unwrap();
        session.insert(CSRF, "test-request-token").await.unwrap();
        session.save().await.unwrap();
        let key = Key::generate();
        let mut jar = CookieJar::new();
        jar.private_mut(&key).add(
            Cookie::build(("gyliber.sid", session.id().unwrap().to_string()))
                .secure(true)
                .build(),
        );
        let cookie = jar.get("gyliber.sid").unwrap().to_string();
        let state = AppState {
            github: None,
            http: reqwest::Client::new(),
            math: Arc::new(Service::default()),
        };
        (
            crate::build_app(
                state,
                key,
                false,
                crate::session_store::SessionStoreBackend::Memory(store),
            ),
            cookie,
        )
    }

    #[tokio::test]
    async fn all_playground_data_and_code_routes_require_membership() {
        let (app, _) = app_with_member().await;
        for path in [
            "/api/math-playground",
            "/api/math-playground/demos/giant-pi",
            "/api/math-playground/demos/giant-pi/engine.mjs",
            "/api/math-playground/runtime/renderer.mjs",
            "/api/math-playground/runtime/0.1.0/renderer.mjs",
            "/api/math-playground/runtime/0.2.0/renderer.mjs",
            "/api/math-playground/exhibits/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "/api/math-playground/exhibits/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa/engine.mjs",
        ] {
            let response = app
                .clone()
                .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::UNAUTHORIZED, "{path}");
        }
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/command/math-playground")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::SEE_OTHER);
    }

    #[tokio::test]
    async fn member_can_play_reviewed_demos_without_ai_credentials() {
        let (app, cookie) = app_with_member().await;
        for path in [
            "/command/math-playground",
            "/api/math-playground/demos/giant-pi/engine.mjs",
            "/api/math-playground/runtime/renderer.mjs",
            "/api/math-playground/runtime/0.1.0/renderer.mjs",
            "/api/math-playground/runtime/0.2.0/renderer.mjs",
        ] {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri(path)
                        .header("cookie", &cookie)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK, "{path}");
            assert_eq!(response.headers()[header::CACHE_CONTROL], "no-store");
        }
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/math-playground")
                    .header("cookie", cookie)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let bytes = to_bytes(response.into_body(), 16384).await.unwrap();
        let body: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(body["ai_ready"], false);
        assert_eq!(body["publishing_ready"], false);
        assert_eq!(body["csrf"], "test-request-token");
    }

    #[tokio::test]
    async fn demo_and_versioned_renderers_preserve_package_provenance() {
        let (app, cookie) = app_with_member().await;
        for (version, expected) in [
            ("0.1.0", model::LEGACY_RENDERER),
            ("0.2.0", model::RENDERER),
        ] {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri(format!(
                            "/api/math-playground/runtime/{version}/renderer.mjs"
                        ))
                        .header("cookie", &cookie)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            let bytes = to_bytes(response.into_body(), 64 * 1024).await.unwrap();
            assert_eq!(bytes.as_ref(), expected.as_bytes());
        }
        let document: Value =
            serde_json::from_str(demo_artifact("giant-pi", false).unwrap()).unwrap();
        assert_eq!(document["version"], "0.2.0");
        assert_eq!(document["formal_version"], "0.2.0");
        assert_eq!(
            document["renderer_sha256"],
            digest(model::RENDERER.as_bytes())
        );
        assert_eq!(
            document["engine_sha256"],
            digest(demo_artifact("giant-pi", true).unwrap().as_bytes())
        );
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/math-playground/runtime/unknown/renderer.mjs")
                    .header("cookie", &cookie)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn writes_require_request_token_and_fail_honestly_without_provider() {
        let (app, cookie) = app_with_member().await;
        let body = json!({"files": [{"name": "x.tex", "content": "C = 2\\pi r"}], "provider_consent": true}).to_string();
        for (token, expected) in [
            ("wrong", StatusCode::FORBIDDEN),
            ("test-request-token", StatusCode::SERVICE_UNAVAILABLE),
        ] {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/api/math-playground/drafts")
                        .method("POST")
                        .header("cookie", &cookie)
                        .header("content-type", "application/json")
                        .header("x-math-csrf", token)
                        .body(Body::from(body.clone()))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), expected);
        }
    }
}
