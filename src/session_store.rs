use std::time::Duration as StdDuration;

use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::{PgPool, Row, postgres::PgPoolOptions};
use time::OffsetDateTime;
use tower_sessions::{
    MemoryStore, SessionStore,
    session::{Id, Record},
    session_store,
};

const MIGRATION: &str = include_str!("../migrations/0001_sessions.sql");
const MAX_CONNECTIONS: u32 = 5;

#[derive(Clone, Debug)]
pub(crate) struct PostgresSessionStore {
    pool: PgPool,
}

impl PostgresSessionStore {
    pub(crate) async fn connect(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(MAX_CONNECTIONS)
            .acquire_timeout(StdDuration::from_secs(5))
            .connect(database_url)
            .await
            .context("unable to connect to PostgreSQL session store")?;

        let store = Self { pool };
        store.migrate().await?;
        Ok(store)
    }

    async fn migrate(&self) -> Result<()> {
        sqlx::raw_sql(MIGRATION)
            .execute(&self.pool)
            .await
            .context("unable to migrate PostgreSQL session store")?;
        Ok(())
    }

    fn encode(record: &Record) -> session_store::Result<Vec<u8>> {
        rmp_serde::to_vec(record).map_err(|error| session_store::Error::Encode(error.to_string()))
    }

    fn decode(data: Vec<u8>) -> session_store::Result<Record> {
        rmp_serde::from_slice(&data)
            .map_err(|error| session_store::Error::Decode(error.to_string()))
    }

    fn is_unique_violation(error: &sqlx::Error) -> bool {
        matches!(
            error,
            sqlx::Error::Database(database_error)
                if database_error.code().as_deref() == Some("23505")
        )
    }
}
#[async_trait]
impl SessionStore for PostgresSessionStore {
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        let data = Self::encode(record)?;

        loop {
            let result = sqlx::query(
                "INSERT INTO gyliber_sessions (id, data, expiry_date) VALUES ($1, $2, $3)",
            )
            .bind(record.id.to_string())
            .bind(&data)
            .bind(record.expiry_date)
            .execute(&self.pool)
            .await;

            match result {
                Ok(_) => return Ok(()),
                Err(error) if Self::is_unique_violation(&error) => {
                    record.id = Id::default();
                }
                Err(error) => return Err(session_store::Error::Backend(error.to_string())),
            }
        }
    }

    async fn save(&self, record: &Record) -> session_store::Result<()> {
        let data = Self::encode(record)?;

        sqlx::query(
            "INSERT INTO gyliber_sessions (id, data, expiry_date)
             VALUES ($1, $2, $3)
             ON CONFLICT (id) DO UPDATE SET
                 data = EXCLUDED.data,
                 expiry_date = EXCLUDED.expiry_date",
        )
        .bind(record.id.to_string())
        .bind(data)
        .bind(record.expiry_date)
        .execute(&self.pool)
        .await
        .map_err(|error| session_store::Error::Backend(error.to_string()))?;

        Ok(())
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        let row = sqlx::query(
            "SELECT data FROM gyliber_sessions
             WHERE id = $1 AND expiry_date > $2",
        )
        .bind(session_id.to_string())
        .bind(OffsetDateTime::now_utc())
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| session_store::Error::Backend(error.to_string()))?;

        match row {
            Some(row) => {
                let data: Vec<u8> = row
                    .try_get("data")
                    .map_err(|error| session_store::Error::Backend(error.to_string()))?;
                Ok(Some(Self::decode(data)?))
            }
            None => Ok(None),
        }
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        sqlx::query("DELETE FROM gyliber_sessions WHERE id = $1")
            .bind(session_id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|error| session_store::Error::Backend(error.to_string()))?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(crate) enum SessionStoreBackend {
    Memory(MemoryStore),
    Postgres(PostgresSessionStore),
}

impl SessionStoreBackend {
    pub(crate) fn requires_database(app_env: Option<&str>, database_url: Option<&str>) -> bool {
        let production = app_env
            .map(|value| value.eq_ignore_ascii_case("production"))
            .unwrap_or(false);

        production
            && !database_url
                .map(|value| !value.trim().is_empty())
                .unwrap_or(false)
    }

    pub(crate) async fn from_environment() -> Result<Self> {
        match std::env::var("DATABASE_URL") {
            Ok(database_url) if !database_url.trim().is_empty() => Ok(Self::Postgres(
                PostgresSessionStore::connect(&database_url).await?,
            )),
            Ok(_) => anyhow::bail!("DATABASE_URL cannot be empty when provided"),
            Err(std::env::VarError::NotPresent) => {
                if Self::requires_database(std::env::var("APP_ENV").ok().as_deref(), None) {
                    anyhow::bail!(
                        "DATABASE_URL is required in production; refusing to start with in-memory sessions"
                    );
                }

                tracing::warn!(
                    "DATABASE_URL is not configured; using in-memory sessions for local development"
                );
                Ok(Self::Memory(MemoryStore::default()))
            }
            Err(error) => Err(error.into()),
        }
    }
}

#[async_trait]
impl SessionStore for SessionStoreBackend {
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        match self {
            Self::Memory(store) => store.create(record).await,
            Self::Postgres(store) => store.create(record).await,
        }
    }

    async fn save(&self, record: &Record) -> session_store::Result<()> {
        match self {
            Self::Memory(store) => store.save(record).await,
            Self::Postgres(store) => store.save(record).await,
        }
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        match self {
            Self::Memory(store) => store.load(session_id).await,
            Self::Postgres(store) => store.load(session_id).await,
        }
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        match self {
            Self::Memory(store) => store.delete(session_id).await,
            Self::Postgres(store) => store.delete(session_id).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PostgresSessionStore, SessionStoreBackend};
    use std::{env, sync::Arc};

    use time::Duration;
    use tower_sessions::Session;

    #[tokio::test]
    async fn postgres_store_round_trips_sessions_when_database_is_available() {
        let Ok(database_url) = env::var("DATABASE_URL") else {
            return;
        };

        let store = PostgresSessionStore::connect(&database_url)
            .await
            .expect("PostgreSQL store connects and migrates");

        let session = Session::new(
            None,
            Arc::new(store.clone()),
            Some(tower_sessions::Expiry::OnInactivity(Duration::minutes(5))),
        );
        session
            .insert("test-key", "test-value")
            .await
            .expect("session value inserts");
        session.save().await.expect("new session persists");

        let id = session.id().expect("persisted session has an id");
        let loaded = Session::new(Some(id), Arc::new(store), None);
        assert_eq!(
            loaded
                .get::<String>("test-key")
                .await
                .expect("session loads")
                .as_deref(),
            Some("test-value")
        );

        loaded.delete().await.expect("session deletes");
    }

    #[test]
    fn production_requires_database_persistence() {
        assert!(SessionStoreBackend::requires_database(
            Some("production"),
            None
        ));
        assert!(SessionStoreBackend::requires_database(
            Some("PRODUCTION"),
            Some("   ")
        ));
        assert!(!SessionStoreBackend::requires_database(
            Some("production"),
            Some("postgres://example")
        ));
        assert!(!SessionStoreBackend::requires_database(
            Some("development"),
            None
        ));
    }
}
