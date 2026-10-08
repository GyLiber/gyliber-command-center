use super::DomainError;
use serde::{Deserialize, Serialize};
use time::{
    Date, Duration, Month, OffsetDateTime, UtcOffset, format_description::well_known::Rfc3339,
};

/// Whole-second RFC3339 instant, normalized to UTC. No implicit timezone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Instant(OffsetDateTime);

impl TryFrom<String> for Instant {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 40 {
            return Err(DomainError::InvalidTime);
        }
        let parsed =
            OffsetDateTime::parse(&value, &Rfc3339).map_err(|_| DomainError::InvalidTime)?;
        if parsed.nanosecond() != 0 {
            return Err(DomainError::InvalidTime);
        }
        Self::from_unix_seconds(parsed.unix_timestamp())
    }
}

impl From<Instant> for String {
    fn from(value: Instant) -> Self {
        value
            .0
            .format(&Rfc3339)
            .expect("validated RFC3339 UTC instant")
    }
}

impl Instant {
    pub fn from_unix_seconds(seconds: i64) -> Result<Self, DomainError> {
        let value = OffsetDateTime::from_unix_timestamp(seconds)
            .map_err(|_| DomainError::InvalidTime)?
            .to_offset(UtcOffset::UTC);
        if !(1..=9999).contains(&value.year()) {
            return Err(DomainError::InvalidTime);
        }
        Ok(Self(value))
    }

    pub fn unix_seconds(self) -> i64 {
        self.0.unix_timestamp()
    }

    pub(crate) fn ensure_after(self, prior: Self) -> Result<(), DomainError> {
        if self < prior {
            return Err(DomainError::ClockWentBackwards);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct DateOnly(Date);

impl DateOnly {
    pub fn as_date(self) -> Date {
        self.0
    }
}

impl TryFrom<String> for DateOnly {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let bytes = value.as_bytes();
        if bytes.len() != 10
            || bytes[4] != b'-'
            || bytes[7] != b'-'
            || !bytes
                .iter()
                .enumerate()
                .all(|(i, b)| i == 4 || i == 7 || b.is_ascii_digit())
        {
            return Err(DomainError::InvalidTime);
        }
        let year = value[..4]
            .parse::<i32>()
            .map_err(|_| DomainError::InvalidTime)?;
        let month = value[5..7]
            .parse::<u8>()
            .map_err(|_| DomainError::InvalidTime)?;
        let day = value[8..]
            .parse::<u8>()
            .map_err(|_| DomainError::InvalidTime)?;
        if year == 0 {
            return Err(DomainError::InvalidTime);
        }
        let month = Month::try_from(month).map_err(|_| DomainError::InvalidTime)?;
        Date::from_calendar_date(year, month, day)
            .map(Self)
            .map_err(|_| DomainError::InvalidTime)
    }
}

impl From<DateOnly> for String {
    fn from(value: DateOnly) -> Self {
        format!(
            "{:04}-{:02}-{:02}",
            value.0.year(),
            value.0.month() as u8,
            value.0.day()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "precision", content = "value", rename_all = "snake_case")]
pub enum TimeFact {
    DateOnly(DateOnly),
    Instant(Instant),
}

impl TimeFact {
    fn instant(self) -> Option<Instant> {
        match self {
            Self::Instant(value) => Some(value),
            Self::DateOnly(_) => None,
        }
    }
}

/// A deliberately selected safety margin, capped at one 365-day year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u32", into = "u32")]
pub struct BufferMinutes(u32);

impl TryFrom<u32> for BufferMinutes {
    type Error = DomainError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > 525_600 {
            return Err(DomainError::InvalidBuffer);
        }
        Ok(Self(value))
    }
}

impl From<BufferMinutes> for u32 {
    fn from(value: BufferMinutes) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ScheduleInput {
    pub deadline: Option<TimeFact>,
    pub earliest_finish: Option<TimeFact>,
    pub buffer_minutes: Option<BufferMinutes>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "ScheduleInput", into = "ScheduleInput")]
pub struct Schedule(ScheduleInput);

impl TryFrom<ScheduleInput> for Schedule {
    type Error = DomainError;

    fn try_from(value: ScheduleInput) -> Result<Self, Self::Error> {
        let schedule = Self(value);
        if let (Some(deadline), Some(earliest)) = (schedule.deadline(), schedule.earliest())
            && earliest > deadline
        {
            return Err(DomainError::InvalidSchedule);
        }
        if let Some(protected) = schedule.protected_start()?
            && schedule
                .earliest()
                .is_some_and(|earliest| earliest > protected)
        {
            return Err(DomainError::InvalidSchedule);
        }
        Ok(schedule)
    }
}

impl From<Schedule> for ScheduleInput {
    fn from(value: Schedule) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BufferState {
    Healthy,
    Shrinking,
    Exhausted,
    Late,
    TargetUnknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct BufferSnapshot {
    pub state: BufferState,
    pub deadline_reached: bool,
    pub seconds_until_deadline: Option<i64>,
    pub protected_start: Option<Instant>,
}

/// Only the readiness aggregate can mint this token; action completion cannot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct VerifiedFinish(pub(super) Instant);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct FinishOutcome {
    pub verified_at: Instant,
    pub actual_buffer_seconds: Option<i64>,
    pub target_preserved: Option<bool>,
    pub late: Option<bool>,
}

impl Schedule {
    fn deadline(self) -> Option<Instant> {
        self.0.deadline.and_then(TimeFact::instant)
    }

    fn earliest(self) -> Option<Instant> {
        self.0.earliest_finish.and_then(TimeFact::instant)
    }

    fn protected_start(self) -> Result<Option<Instant>, DomainError> {
        let (Some(deadline), Some(buffer)) = (self.deadline(), self.0.buffer_minutes) else {
            return Ok(None);
        };
        let start = deadline
            .0
            .checked_sub(Duration::minutes(i64::from(buffer.0)))
            .ok_or(DomainError::InvalidSchedule)?;
        Instant::from_unix_seconds(start.unix_timestamp())
            .map(Some)
            .map_err(|_| DomainError::InvalidSchedule)
    }

    pub fn snapshot(self, now: Instant) -> BufferSnapshot {
        let deadline = self.deadline();
        let protected_start = self.protected_start().expect("validated schedule");
        let state = match (deadline, self.earliest(), protected_start) {
            (Some(d), Some(e), Some(p)) => {
                if now >= d {
                    BufferState::Late
                } else if now >= p {
                    BufferState::Exhausted
                } else if now >= e {
                    BufferState::Shrinking
                } else {
                    BufferState::Healthy
                }
            }
            _ => BufferState::TargetUnknown,
        };
        BufferSnapshot {
            state,
            deadline_reached: deadline.is_some_and(|d| now >= d),
            seconds_until_deadline: deadline.map(|d| d.unix_seconds() - now.unix_seconds()),
            protected_start,
        }
    }

    /// A frozen outcome: evaluating it later never consumes a completed buffer.
    pub fn finish_outcome(self, finish: VerifiedFinish) -> FinishOutcome {
        let deadline = self.deadline();
        FinishOutcome {
            verified_at: finish.0,
            actual_buffer_seconds: deadline.map(|d| d.unix_seconds() - finish.0.unix_seconds()),
            target_preserved: self
                .protected_start()
                .expect("validated schedule")
                .map(|p| finish.0 <= p),
            late: deadline.map(|d| finish.0 > d),
        }
    }
}
