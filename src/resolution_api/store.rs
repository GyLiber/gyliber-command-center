use super::model::*;
use gyliber_command_center::resolution_control::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Postgres, Row, Transaction};

const MIGRATION: &str = include_str!("../../migrations/0003_resolution_control.sql");
const MAX_PRIVATE_BYTES: usize = 3 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Error {
    Unavailable,
    Conflict,
    Gone,
    InvalidBackup,
    InvalidRequest,
    ConfirmationRequired,
    Capacity,
    NotFound,
    Rule(WorkspaceError),
}

impl From<sqlx::Error> for Error {
    fn from(_: sqlx::Error) -> Self {
        Self::Unavailable
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Self::InvalidBackup
    }
}

impl From<WorkspaceError> for Error {
    fn from(value: WorkspaceError) -> Self {
        Self::Rule(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Backup {
    pub schema_version: u32,
    pub source_generation: Identifier,
    pub lineage: Vec<Identifier>,
    pub exported_at: Instant,
    pub events: Vec<WorkspaceEvent>,
    pub checksum: String,
}

impl Backup {
    fn new(
        generation: Identifier,
        lineage: Vec<Identifier>,
        events: Vec<WorkspaceEvent>,
        exported_at: Instant,
    ) -> Result<Self, Error> {
        let mut backup = Self {
            schema_version: 1,
            source_generation: generation,
            lineage,
            exported_at,
            events,
            checksum: String::new(),
        };
        backup.checksum = backup.expected_checksum()?;
        Ok(backup)
    }

    fn expected_checksum(&self) -> Result<String, Error> {
        Ok(digest(&serde_json::to_vec(&(
            self.schema_version,
            &self.source_generation,
            &self.lineage,
            self.exported_at,
            &self.events,
        ))?))
    }

    pub fn validate(&self, actor: Identifier, now: Instant) -> Result<(), Error> {
        if self.schema_version != 1
            || self.events.len() > MAX_EVENTS
            || self.exported_at > now
            || self.lineage.is_empty()
            || self.lineage.len() > 128
            || self
                .lineage
                .iter()
                .enumerate()
                .any(|(i, g)| self.lineage[..i].contains(g))
            || self.checksum.len() != 64
            || self.checksum != self.expected_checksum()?
            || serde_json::to_vec(self)?.len() > MAX_BACKUP_BYTES
            || command_bytes(&self.events)? > MAX_PRIVATE_BYTES
        {
            return Err(Error::InvalidBackup);
        }
        replay(&self.events, actor, self.exported_at).map_err(|_| Error::InvalidBackup)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct View {
    pub generation: Option<Identifier>,
    pub revision: u64,
    pub observed_at: Instant,
    pub workspace: Workspace,
    pub buffers: Vec<CommitmentBuffer>,
    pub imported_history: bool,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct CommitmentBuffer {
    pub commitment: Identifier,
    pub unfinished: Option<BufferSnapshot>,
    pub finished: Option<FinishOutcome>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Ack {
    pub generation: Identifier,
    pub revision: u64,
    pub recorded_at: Instant,
    pub operation: String,
    pub replayed: bool,
    pub recovery_metadata: Option<RecoveryMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RecoveryMetadata {
    pub schema_version: u32,
    pub deleted_generations: Vec<Identifier>,
    pub checksum: String,
}

impl RecoveryMetadata {
    fn new(deleted_generations: Vec<Identifier>) -> Result<Self, Error> {
        let checksum = digest(&serde_json::to_vec(&(1_u32, &deleted_generations))?);
        Ok(Self {
            schema_version: 1,
            deleted_generations,
            checksum,
        })
    }

    fn validate(&self) -> Result<(), Error> {
        if self.schema_version != 1
            || self.deleted_generations.len() > 4096
            || self
                .deleted_generations
                .iter()
                .enumerate()
                .any(|(i, g)| self.deleted_generations[..i].contains(g))
            || self.checksum.len() != 64
            || self.checksum
                != digest(&serde_json::to_vec(&(
                    self.schema_version,
                    &self.deleted_generations,
                ))?)
        {
            return Err(Error::InvalidBackup);
        }
        Ok(())
    }
}

async fn recovery_tx(
    tx: &mut Transaction<'_, Postgres>,
    owner: &str,
) -> Result<RecoveryMetadata, Error> {
    let generations: Vec<String> = sqlx::query_scalar("SELECT generation FROM gyliber_resolution_deletions WHERE owner=$1 ORDER BY generation LIMIT 4097")
        .bind(owner).fetch_all(&mut **tx).await?;
    if generations.len() > 4096 {
        return Err(Error::Capacity);
    }
    RecoveryMetadata::new(
        generations
            .into_iter()
            .map(|g| g.try_into().map_err(|_| Error::Unavailable))
            .collect::<Result<_, _>>()?,
    )
}

pub(super) async fn recovery_metadata(
    pool: &PgPool,
    owner: &str,
) -> Result<RecoveryMetadata, Error> {
    let mut tx = lock(pool, owner).await?;
    let result = recovery_tx(&mut tx, owner).await?;
    tx.commit().await?;
    Ok(result)
}

pub(super) fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn generation() -> Identifier {
    // Random capability-independent identifier; not a credential or owner.
    digest(oauth2::CsrfToken::new_random().secret().as_bytes())
        .try_into()
        .expect("hex identifier")
}

fn command_bytes(events: &[WorkspaceEvent]) -> Result<usize, Error> {
    let mut bytes = 0;
    for event in events {
        bytes += serde_json::to_vec(&event.command)?.len();
    }
    Ok(bytes)
}

pub(super) async fn migrate(pool: &PgPool) -> Result<(), Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock(68439127)")
        .execute(&mut *tx)
        .await?;
    sqlx::raw_sql(MIGRATION).execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(())
}

async fn lock(pool: &PgPool, owner: &str) -> Result<Transaction<'static, Postgres>, Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1, 8132))")
        .bind(owner)
        .execute(&mut *tx)
        .await?;
    Ok(tx)
}

async fn clock(
    tx: &mut Transaction<'_, Postgres>,
    events: &[WorkspaceEvent],
) -> Result<Instant, Error> {
    let seconds: i64 =
        sqlx::query_scalar("SELECT floor(extract(epoch FROM clock_timestamp()))::bigint")
            .fetch_one(&mut **tx)
            .await?;
    // Database time is authoritative; preserve monotonic order during small
    // clock corrections without ever accepting a caller timestamp for a write.
    let seconds = events
        .last()
        .map_or(seconds, |e| seconds.max(e.at.unix_seconds()));
    Instant::from_unix_seconds(seconds).map_err(|_| Error::Unavailable)
}

async fn header(
    tx: &mut Transaction<'_, Postgres>,
    owner: &str,
) -> Result<Option<(Identifier, u64)>, Error> {
    let row = sqlx::query(
        "SELECT generation, revision FROM gyliber_resolution_workspaces WHERE owner = $1",
    )
    .bind(owner)
    .fetch_optional(&mut **tx)
    .await?;
    row.map(|r| {
        let generation: String = r.try_get("generation")?;
        let revision: i64 = r.try_get("revision")?;
        Ok((
            generation.try_into().map_err(|_| Error::Unavailable)?,
            revision as u64,
        ))
    })
    .transpose()
}

async fn events(
    tx: &mut Transaction<'_, Postgres>,
    owner: &str,
) -> Result<Vec<WorkspaceEvent>, Error> {
    let rows = sqlx::query("SELECT document FROM gyliber_resolution_events WHERE owner = $1 ORDER BY revision LIMIT 2049")
        .bind(owner).fetch_all(&mut **tx).await?;
    if rows.len() > MAX_EVENTS {
        return Err(Error::Unavailable);
    }
    rows.into_iter()
        .map(|r| {
            let document: String = r.try_get("document")?;
            serde_json::from_str(&document).map_err(|_| Error::Unavailable)
        })
        .collect()
}

fn actor(owner: &str) -> Result<Identifier, Error> {
    owner.to_owned().try_into().map_err(|_| Error::Unavailable)
}

fn make_view(
    generation: Option<Identifier>,
    events: &[WorkspaceEvent],
    owner: &str,
    now: Instant,
) -> Result<View, Error> {
    let workspace = replay(events, actor(owner)?, now).map_err(|_| Error::Unavailable)?;
    let buffers = workspace
        .commitments
        .iter()
        .map(|c| match c.readiness.verified_finish() {
            Some(finish) => CommitmentBuffer {
                commitment: c.id.clone(),
                unfinished: None,
                finished: Some(c.schedule.finish_outcome(finish)),
            },
            None => CommitmentBuffer {
                commitment: c.id.clone(),
                unfinished: Some(c.schedule.snapshot(now)),
                finished: None,
            },
        })
        .collect();
    Ok(View {
        generation,
        revision: events.len() as u64,
        observed_at: now,
        workspace,
        buffers,
        imported_history: events.iter().any(|e| e.origin == EventOrigin::Imported),
    })
}

pub(super) async fn view(pool: &PgPool, owner: &str) -> Result<View, Error> {
    let mut tx = lock(pool, owner).await?;
    let current = header(&mut tx, owner).await?;
    let events = events(&mut tx, owner).await?;
    if current.as_ref().map_or(0, |h| h.1) != events.len() as u64 {
        return Err(Error::Unavailable);
    }
    let now = clock(&mut tx, &events).await?;
    let view = make_view(current.map(|h| h.0), &events, owner, now)?;
    tx.commit().await?;
    Ok(view)
}

pub(super) async fn export(pool: &PgPool, owner: &str) -> Result<Backup, Error> {
    let mut tx = lock(pool, owner).await?;
    let (generation, revision) = header(&mut tx, owner).await?.ok_or(Error::NotFound)?;
    let events = events(&mut tx, owner).await?;
    if events.len() as u64 != revision {
        return Err(Error::Unavailable);
    }
    let lineage = sources(&mut tx, owner).await?;
    let now = clock(&mut tx, &events).await?;
    let backup = Backup::new(generation, lineage, events, now)?;
    backup.validate(actor(owner)?, now)?;
    tx.commit().await?;
    Ok(backup)
}

async fn sources(
    tx: &mut Transaction<'_, Postgres>,
    owner: &str,
) -> Result<Vec<Identifier>, Error> {
    let rows: Vec<String> = sqlx::query_scalar("SELECT generation FROM gyliber_resolution_sources WHERE owner=$1 ORDER BY generation LIMIT 129")
        .bind(owner).fetch_all(&mut **tx).await?;
    if rows.len() > 128 {
        return Err(Error::Capacity);
    }
    rows.into_iter()
        .map(|g| g.try_into().map_err(|_| Error::Unavailable))
        .collect()
}

pub(super) async fn history(
    pool: &PgPool,
    owner: &str,
    after: u64,
) -> Result<Vec<WorkspaceEvent>, Error> {
    let mut tx = lock(pool, owner).await?;
    let result = events(&mut tx, owner)
        .await?
        .into_iter()
        .filter(|e| e.revision > after)
        .take(100)
        .collect();
    tx.commit().await?;
    Ok(result)
}

pub(super) async fn audit(
    pool: &PgPool,
    owner: Option<&str>,
    code: &'static str,
) -> Result<(), Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM gyliber_resolution_audit WHERE minute < CURRENT_TIMESTAMP - INTERVAL '30 days'")
        .execute(&mut *tx).await?;
    audit_tx(&mut tx, owner.unwrap_or("anonymous"), code).await?;
    tx.commit().await?;
    Ok(())
}

async fn audit_tx(
    tx: &mut Transaction<'_, Postgres>,
    owner: &str,
    code: &str,
) -> Result<(), Error> {
    sqlx::query("INSERT INTO gyliber_resolution_audit (actor,event_code,minute) VALUES ($1,$2,date_trunc('minute',CURRENT_TIMESTAMP))
        ON CONFLICT (actor,event_code,minute) DO UPDATE SET occurrences = gyliber_resolution_audit.occurrences + 1")
        .bind(owner).bind(code).execute(&mut **tx).await?;
    Ok(())
}

pub(super) async fn operate(pool: &PgPool, owner: &str, request: Operation) -> Result<Ack, Error> {
    let mut tx = lock(pool, owner).await?;
    sqlx::query("DELETE FROM gyliber_resolution_operations WHERE owner=$1 AND created_at < CURRENT_TIMESTAMP - INTERVAL '30 days'").bind(owner).execute(&mut *tx).await?;
    sqlx::query("DELETE FROM gyliber_resolution_audit WHERE minute < CURRENT_TIMESTAMP - INTERVAL '30 days'").execute(&mut *tx).await?;
    let meta = request.meta();
    let hash = digest(&serde_json::to_vec(&request)?);
    let current = header(&mut tx, owner).await?;
    let previous = sqlx::query("SELECT request_digest,result FROM gyliber_resolution_operations WHERE owner=$1 AND operation_id=$2")
        .bind(owner).bind(meta.operation_id.as_str()).fetch_optional(&mut *tx).await?;
    if let Some(row) = previous {
        if row.try_get::<String, _>("request_digest")? != hash {
            return Err(Error::Conflict);
        }
        let mut ack: Ack = serde_json::from_str(&row.try_get::<String, _>("result")?)?;
        if current.as_ref().map(|h| &h.0) != Some(&ack.generation) {
            return Err(Error::Gone);
        }
        ack.replayed = true;
        tx.commit().await?;
        return Ok(ack);
    }
    match &current {
        Some((generation, revision))
            if meta.generation.as_ref() == Some(generation)
                && meta.expected_revision == *revision => {}
        None if meta.generation.is_none() && meta.expected_revision == 0 => {}
        _ => return Err(Error::Conflict),
    }
    let initial_generation = current
        .as_ref()
        .map(|h| h.0.clone())
        .unwrap_or_else(generation);
    if current.is_none() {
        sqlx::query("INSERT INTO gyliber_resolution_workspaces (owner,generation) VALUES ($1,$2)")
            .bind(owner)
            .bind(initial_generation.as_str())
            .execute(&mut *tx)
            .await?;
        sqlx::query("INSERT INTO gyliber_resolution_sources (owner,generation) VALUES ($1,$2)")
            .bind(owner)
            .bind(initial_generation.as_str())
            .execute(&mut *tx)
            .await?;
    }
    let mut log = events(&mut tx, owner).await?;
    if meta.expected_revision != log.len() as u64 {
        return Err(Error::Unavailable);
    }
    let now = clock(&mut tx, &log).await?;
    let mut new_generation = initial_generation;
    let kind = match &request {
        Operation::Mutation { command, .. } => {
            if log.len() >= MAX_EVENTS {
                return Err(Error::Capacity);
            }
            let mut state = replay(&log, actor(owner)?, now).map_err(|_| Error::Unavailable)?;
            state.apply(command, actor(owner)?, now)?;
            log.push(WorkspaceEvent {
                revision: log.len() as u64 + 1,
                at: now,
                origin: EventOrigin::Live,
                command: command.clone(),
            });
            if command_bytes(&log)? > MAX_PRIVATE_BYTES {
                return Err(Error::Capacity);
            }
            let event = log.last().expect("just pushed");
            sqlx::query(
                "INSERT INTO gyliber_resolution_events (owner,revision,document) VALUES ($1,$2,$3)",
            )
            .bind(owner)
            .bind(event.revision as i64)
            .bind(serde_json::to_string(event)?)
            .execute(&mut *tx)
            .await?;
            "mutation"
        }
        Operation::Restore {
            backup,
            recovery_metadata,
            confirm_recovery_metadata,
            ..
        } => {
            if !confirm_recovery_metadata {
                return Err(Error::ConfirmationRequired);
            }
            if !log.is_empty() {
                return Err(Error::Conflict);
            }
            backup.validate(actor(owner)?, now)?;
            recovery_metadata.validate()?;
            for source in &recovery_metadata.deleted_generations {
                sqlx::query("INSERT INTO gyliber_resolution_deletions (owner,generation) VALUES ($1,$2) ON CONFLICT DO NOTHING")
                    .bind(owner).bind(source.as_str()).execute(&mut *tx).await?;
            }
            for source in backup
                .lineage
                .iter()
                .chain(std::iter::once(&backup.source_generation))
            {
                let deleted: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM gyliber_resolution_deletions WHERE owner=$1 AND generation=$2)")
                    .bind(owner).bind(source.as_str()).fetch_one(&mut *tx).await?;
                if deleted {
                    return Err(Error::Gone);
                }
            }
            log = backup.events.clone();
            new_generation = generation();
            // Preserve data roots across restores, without growing one root
            // for every regenerated optimistic-concurrency token.
            sqlx::query("DELETE FROM gyliber_resolution_sources WHERE owner=$1")
                .bind(owner)
                .execute(&mut *tx)
                .await?;
            for source in &backup.lineage {
                sqlx::query("INSERT INTO gyliber_resolution_sources (owner,generation) VALUES ($1,$2) ON CONFLICT DO NOTHING")
                    .bind(owner).bind(source.as_str()).execute(&mut *tx).await?;
            }
            sources(&mut tx, owner).await?;
            for event in &mut log {
                event.origin = EventOrigin::Imported;
                sqlx::query("INSERT INTO gyliber_resolution_events (owner,revision,document) VALUES ($1,$2,$3)")
                    .bind(owner).bind(event.revision as i64).bind(serde_json::to_string(event)?).execute(&mut *tx).await?;
            }
            audit_tx(&mut tx, owner, "resolution.restored").await?;
            "restore"
        }
        Operation::Purge { confirmation, .. } => {
            if confirmation != "DELETE MY RESOLUTION CONTROL DATA" {
                return Err(Error::ConfirmationRequired);
            }
            sqlx::query("INSERT INTO gyliber_resolution_deletions (owner,generation) SELECT owner,generation FROM gyliber_resolution_sources WHERE owner=$1 ON CONFLICT DO NOTHING")
                .bind(owner).execute(&mut *tx).await?;
            sqlx::query("INSERT INTO gyliber_resolution_deletions (owner,generation) VALUES ($1,$2) ON CONFLICT DO NOTHING")
                .bind(owner).bind(new_generation.as_str()).execute(&mut *tx).await?;
            sqlx::query("DELETE FROM gyliber_resolution_sources WHERE owner=$1")
                .bind(owner)
                .execute(&mut *tx)
                .await?;
            sqlx::query("DELETE FROM gyliber_resolution_events WHERE owner=$1")
                .bind(owner)
                .execute(&mut *tx)
                .await?;
            log.clear();
            new_generation = generation();
            sqlx::query("INSERT INTO gyliber_resolution_sources (owner,generation) VALUES ($1,$2)")
                .bind(owner)
                .bind(new_generation.as_str())
                .execute(&mut *tx)
                .await?;
            audit_tx(&mut tx, owner, "resolution.purged").await?;
            "purge"
        }
    };
    let recovery_metadata = if kind == "purge" {
        Some(recovery_tx(&mut tx, owner).await?)
    } else {
        None
    };
    let ack = Ack {
        generation: new_generation,
        revision: log.len() as u64,
        recorded_at: now,
        operation: kind.into(),
        replayed: false,
        recovery_metadata,
    };
    sqlx::query("UPDATE gyliber_resolution_workspaces SET generation=$2,revision=$3,payload_bytes=$4 WHERE owner=$1")
        .bind(owner).bind(ack.generation.as_str()).bind(ack.revision as i64).bind(command_bytes(&log)? as i64).execute(&mut *tx).await?;
    sqlx::query("INSERT INTO gyliber_resolution_operations (owner,operation_id,request_digest,result) VALUES ($1,$2,$3,$4)")
        .bind(owner).bind(meta.operation_id.as_str()).bind(hash).bind(serde_json::to_string(&ack)?).execute(&mut *tx).await?;
    sqlx::query("DELETE FROM gyliber_resolution_operations WHERE owner=$1 AND created_at < CURRENT_TIMESTAMP - INTERVAL '30 days'")
        .bind(owner).execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(ack)
}

#[cfg(test)]
mod validation_tests {
    use super::*;
    use serde_json::json;
    fn id(value: &str) -> Identifier {
        value.to_owned().try_into().unwrap()
    }
    fn at(value: &str) -> Instant {
        serde_json::from_value(json!(value)).unwrap()
    }
    fn event() -> WorkspaceEvent {
        WorkspaceEvent { revision: 1, at: at("2026-10-08T12:00:00Z"), origin: EventOrigin::Live,
            command: serde_json::from_value(json!({"type":"create_resolution","id":"r","spec":{"title":"Synthetic","objective":null,"client_reference":null}})).unwrap() }
    }
    #[test]
    fn backup_integrity_is_not_a_substitute_for_guarded_replay() {
        let now = at("2026-10-08T12:00:01Z");
        let good = Backup::new(id("generation"), vec![id("root")], vec![event()], now).unwrap();
        assert!(good.validate(id("actor"), now).is_ok());
        for variant in 0..7 {
            let mut bad = good.clone();
            match variant {
                0 => bad.schema_version = 2,
                1 => bad.events[0].revision = 2,
                2 => bad.events[0].at = at("2026-10-08T12:00:02Z"),
                3 => bad.events[0].command = Command::StartAction { id: id("absent") },
                4 => bad.lineage.clear(),
                5 => bad.lineage.push(id("root")),
                _ => bad.events.push(bad.events[0].clone()),
            }
            bad.checksum = bad.expected_checksum().unwrap();
            assert_eq!(bad.validate(id("actor"), now), Err(Error::InvalidBackup));
        }
        let mut bad = good;
        bad.checksum = "0".repeat(64);
        assert_eq!(bad.validate(id("actor"), now), Err(Error::InvalidBackup));
    }
    #[test]
    fn recovery_metadata_is_bounded_checked_and_data_only() {
        let metadata = RecoveryMetadata::new(vec![id("deleted")]).unwrap();
        assert!(metadata.validate().is_ok());
        let duplicate = RecoveryMetadata::new(vec![id("deleted"), id("deleted")]).unwrap();
        assert_eq!(duplicate.validate(), Err(Error::InvalidBackup));
        let excess = RecoveryMetadata::new(vec![id("deleted"); 4097]).unwrap();
        assert_eq!(excess.validate(), Err(Error::InvalidBackup));
        let mut value = serde_json::to_value(metadata).unwrap();
        value["owner"] = json!("forged");
        assert!(serde_json::from_value::<RecoveryMetadata>(value).is_err());
    }
}
