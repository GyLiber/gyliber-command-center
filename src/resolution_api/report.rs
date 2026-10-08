use super::store::{self, Error};
use gyliber_command_center::resolution_control::*;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::{Time, UtcOffset};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ReportQuery {
    pub date: DateOnly,
    #[serde(default = "pilot_offset")]
    pub offset_minutes: i16,
    pub cutoff_revision: Option<u64>,
}

fn pilot_offset() -> i16 {
    120
}

#[derive(Debug, Serialize)]
pub(super) struct DailyReport {
    pub date: DateOnly,
    pub offset_minutes: i16,
    pub cutoff_revision: u64,
    pub state_at: Instant,
    pub changes: Vec<Change>,
    pub state: Workspace,
    pub empty_day: bool,
    pub imported_history: bool,
}

#[derive(Debug, Serialize)]
pub(super) struct Change {
    pub revision: u64,
    pub at: Instant,
    pub origin: EventOrigin,
    pub code: &'static str,
    pub detail: Command,
}

pub(super) fn build(
    events: &[WorkspaceEvent],
    actor: Identifier,
    query: ReportQuery,
    now: Instant,
) -> Result<DailyReport, Error> {
    if !(-840..=840).contains(&query.offset_minutes) {
        return Err(Error::InvalidRequest);
    }
    let offset = UtcOffset::from_whole_seconds(i32::from(query.offset_minutes) * 60)
        .map_err(|_| Error::InvalidRequest)?;
    let date = query.date.as_date();
    let start = date
        .with_time(Time::MIDNIGHT)
        .assume_offset(offset)
        .unix_timestamp();
    let end = date
        .next_day()
        .ok_or(Error::InvalidRequest)?
        .with_time(Time::MIDNIGHT)
        .assume_offset(offset)
        .unix_timestamp();
    let cutoff_revision = query.cutoff_revision.unwrap_or(events.len() as u64);
    if cutoff_revision > events.len() as u64 {
        return Err(Error::InvalidRequest);
    }
    let state_at = Instant::from_unix_seconds((end - 1).min(now.unix_seconds()))
        .map_err(|_| Error::InvalidRequest)?;
    let prefix: Vec<_> = events
        .iter()
        .filter(|e| e.revision <= cutoff_revision && e.at <= state_at)
        .cloned()
        .collect();
    let state = replay(&prefix, actor, state_at).map_err(|_| Error::Unavailable)?;
    let changes: Vec<_> = prefix
        .iter()
        .filter(|e| e.at.unix_seconds() >= start)
        .map(|e| Change {
            revision: e.revision,
            at: e.at,
            origin: e.origin,
            code: e.command.code(),
            detail: e.command.clone(),
        })
        .collect();
    let empty_day = changes.is_empty();
    let imported_history = prefix.iter().any(|e| e.origin == EventOrigin::Imported);
    Ok(DailyReport {
        date: query.date,
        offset_minutes: query.offset_minutes,
        cutoff_revision,
        state_at,
        changes,
        state,
        empty_day,
        imported_history,
    })
}

pub(super) async fn generate(
    pool: &PgPool,
    owner: &str,
    query: ReportQuery,
) -> Result<DailyReport, Error> {
    // Export locks the owner and obtains one consistent, validated history.
    let backup = match store::export(pool, owner).await {
        Ok(backup) => Some(backup),
        Err(Error::NotFound) => None,
        Err(error) => return Err(error),
    };
    let now = match &backup {
        Some(backup) => backup.exported_at,
        None => store::view(pool, owner).await?.observed_at,
    };
    let events = backup.as_ref().map_or(&[][..], |b| b.events.as_slice());
    build(
        events,
        owner
            .to_owned()
            .try_into()
            .map_err(|_| Error::Unavailable)?,
        query,
        now,
    )
}
