use super::model::{Exhibit, RENDERER};
use sqlx::{PgPool, Row};

pub(super) async fn reserve_attempt(pool: &PgPool, owner: &str) -> Result<bool, sqlx::Error> {
    let mut tx = pool.begin().await?;
    // Serialize each member's count+reservation across processes, not just this instance.
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1, 0))")
        .bind(owner)
        .execute(&mut *tx)
        .await?;
    let count: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM gyliber_math_events WHERE owner = $1 AND event_code = 'AI_REQUESTED'
         AND occurred_at > CURRENT_TIMESTAMP - INTERVAL '24 hours'",
    )
    .bind(owner)
    .fetch_one(&mut *tx)
    .await?;
    if count >= 8 {
        return Ok(false);
    }
    sqlx::query("INSERT INTO gyliber_math_events (owner, event_code) VALUES ($1, 'AI_REQUESTED')")
        .bind(owner)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(true)
}

pub(super) async fn clean(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM gyliber_math_exhibits WHERE expires_at <= CURRENT_TIMESTAMP")
        .execute(pool)
        .await?;
    sqlx::query("DELETE FROM gyliber_math_events WHERE occurred_at < CURRENT_TIMESTAMP - INTERVAL '30 days'")
        .execute(pool).await?;
    Ok(())
}

pub(super) async fn save(
    pool: &PgPool,
    owner: &str,
    exhibit: &Exhibit,
    engine: &str,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    sqlx::query(
        "INSERT INTO gyliber_math_exhibits (id, owner, document, engine, renderer, expires_at)
        VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP + INTERVAL '24 hours')",
    )
    .bind(&exhibit.id)
    .bind(owner)
    .bind(serde_json::to_string(exhibit)?)
    .bind(engine)
    .bind(RENDERER)
    .execute(&mut *tx)
    .await?;
    sqlx::query("INSERT INTO gyliber_math_events (owner, exhibit_id, event_code) VALUES ($1, $2, 'DRAFT_CREATED')")
        .bind(owner).bind(&exhibit.id).execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(())
}

pub(super) async fn get(
    pool: &PgPool,
    owner: &str,
    id: &str,
) -> anyhow::Result<Option<(Exhibit, String, String)>> {
    let row = sqlx::query(
        "SELECT document, engine, renderer, repository_commit FROM gyliber_math_exhibits
        WHERE id = $1 AND owner = $2 AND (expires_at IS NULL OR expires_at > CURRENT_TIMESTAMP)",
    )
    .bind(id)
    .bind(owner)
    .fetch_optional(pool)
    .await?;
    row.map(|row| {
        let mut exhibit: Exhibit = serde_json::from_str(&row.try_get::<String, _>("document")?)?;
        exhibit.repository_commit = row.try_get("repository_commit")?;
        if exhibit.repository_commit.is_some() {
            exhibit.review = "member_reviewed".into();
        }
        Ok((exhibit, row.try_get("engine")?, row.try_get("renderer")?))
    })
    .transpose()
}

pub(super) async fn list(pool: &PgPool, owner: &str) -> anyhow::Result<Vec<Exhibit>> {
    let rows = sqlx::query("SELECT id FROM gyliber_math_exhibits WHERE owner = $1
        AND (expires_at IS NULL OR expires_at > CURRENT_TIMESTAMP) ORDER BY created_at DESC LIMIT 100")
        .bind(owner).fetch_all(pool).await?;
    let mut result = Vec::new();
    for row in rows {
        if let Some((exhibit, _, _)) = get(pool, owner, &row.try_get::<String, _>("id")?).await? {
            result.push(exhibit);
        }
    }
    Ok(result)
}

pub(super) async fn publish(
    pool: &PgPool,
    owner: &str,
    id: &str,
    commit: &str,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let updated = sqlx::query(
        "UPDATE gyliber_math_exhibits SET repository_commit = $1, expires_at = NULL
        WHERE id = $2 AND owner = $3",
    )
    .bind(commit)
    .bind(id)
    .bind(owner)
    .execute(&mut *tx)
    .await?;
    anyhow::ensure!(updated.rows_affected() == 1, "exhibit no longer exists");
    sqlx::query("INSERT INTO gyliber_math_events (owner, exhibit_id, event_code) VALUES ($1, $2, 'CODE_PUBLISHED')")
        .bind(owner).bind(id).execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(())
}

pub(super) async fn delete(pool: &PgPool, owner: &str, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM gyliber_math_exhibits WHERE id = $1 AND owner = $2")
        .bind(id)
        .bind(owner)
        .execute(pool)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math_playground::{Service, model};
    use std::env;
    #[tokio::test]
    async fn postgres_exhibits_preserve_code_and_enforce_owner_retention_and_budget() {
        let Ok(url) = env::var("DATABASE_URL") else {
            return;
        };
        let pool = PgPool::connect(&url).await.unwrap();
        Service::default()
            .initialize(Some(pool.clone()))
            .await
            .unwrap();
        let nonce = oauth2::CsrfToken::new_random();
        let owner = format!("fixture-{}", model::digest(nonce.secret().as_bytes()));
        let id = model::digest(owner.as_bytes())[..32].to_owned();
        let concept = serde_json::from_value(serde_json::json!({"title":"Fixture","kind":"metaphor",
            "concept_type":"definition","notation":"X","hypotheses":"X is a set","statement":"x ∈ X",
            "visual_mapping":"mnemonic only","limitations":"not a proof","source_file":"fixture.tex",
            "source_quote":"x \\in X","palette":"candy"})).unwrap();
        let engine = model::engine(&id, &concept);
        let exhibit = Exhibit {
            id: id.clone(),
            version: "0.1.0".into(),
            formal_version: "0.2.0".into(),
            concept,
            source_sha256: "fixture".into(),
            engine_sha256: model::digest(engine.as_bytes()),
            renderer_sha256: model::digest(RENDERER.as_bytes()),
            created_at: 0,
            review: "ai_draft_unverified".into(),
            repository_commit: None,
        };
        save(&pool, &owner, &exhibit, &engine).await.unwrap();
        assert!(get(&pool, "other-member", &id).await.unwrap().is_none());
        let loaded = get(&pool, &owner, &id).await.unwrap().unwrap();
        assert_eq!(loaded.1, engine);
        assert_eq!(loaded.2, RENDERER);
        assert_eq!(list(&pool, &owner).await.unwrap().len(), 1);
        for _ in 0..8 {
            assert!(reserve_attempt(&pool, &owner).await.unwrap());
        }
        assert!(!reserve_attempt(&pool, &owner).await.unwrap());
        publish(&pool, &owner, &id, &"a".repeat(40)).await.unwrap();
        assert_eq!(
            get(&pool, &owner, &id).await.unwrap().unwrap().0.review,
            "member_reviewed"
        );
        delete(&pool, "other-member", &id).await.unwrap();
        assert!(get(&pool, &owner, &id).await.unwrap().is_some());
        sqlx::query("UPDATE gyliber_math_exhibits SET expires_at = CURRENT_TIMESTAMP - INTERVAL '1 second' WHERE id = $1")
            .bind(&id).execute(&pool).await.unwrap();
        assert!(get(&pool, &owner, &id).await.unwrap().is_none());
        clean(&pool).await.unwrap();
        delete(&pool, &owner, &id).await.unwrap();
        sqlx::query("DELETE FROM gyliber_math_events WHERE owner = $1")
            .bind(&owner)
            .execute(&pool)
            .await
            .unwrap();
    }
}
