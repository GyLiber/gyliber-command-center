use super::{ArtifactReference, DomainError, Instant, Notes};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionPlan {
    pub instruction: Notes,
    pub expected_artifact: Option<Notes>,
    pub verification_method: Option<Notes>,
    pub start_reference: Option<ArtifactReference>,
}

impl ActionPlan {
    fn ensure_executable(&self) -> Result<(), DomainError> {
        if self.expected_artifact.is_none() || self.verification_method.is_none() {
            return Err(DomainError::IncompleteAction);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionStatus {
    New,
    Ready,
    InProgress,
    Completed,
    Blocked,
    Cancelled,
}

/// State is mutated through guarded transitions, never by a JSON status field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Action {
    plan: ActionPlan,
    status: ActionStatus,
    started_at: Option<Instant>,
    completed_at: Option<Instant>,
    artifact: Option<ArtifactReference>,
    updated_at: Instant,
}

impl Action {
    pub fn new(plan: ActionPlan, at: Instant) -> Self {
        Self {
            plan,
            status: ActionStatus::New,
            started_at: None,
            completed_at: None,
            artifact: None,
            updated_at: at,
        }
    }

    pub fn status(&self) -> ActionStatus {
        self.status
    }

    pub fn update_plan(&mut self, plan: ActionPlan, at: Instant) -> Result<(), DomainError> {
        at.ensure_after(self.updated_at)?;
        if !matches!(
            self.status,
            ActionStatus::New | ActionStatus::Ready | ActionStatus::Blocked
        ) {
            return Err(DomainError::InvalidTransition);
        }
        self.plan = plan;
        self.status = ActionStatus::New;
        self.started_at = None;
        self.updated_at = at;
        Ok(())
    }

    pub fn ready(&mut self, at: Instant) -> Result<(), DomainError> {
        at.ensure_after(self.updated_at)?;
        if !matches!(self.status, ActionStatus::New | ActionStatus::Blocked) {
            return Err(DomainError::InvalidTransition);
        }
        self.plan.ensure_executable()?;
        self.status = ActionStatus::Ready;
        self.updated_at = at;
        Ok(())
    }

    pub fn start(&mut self, at: Instant) -> Result<(), DomainError> {
        at.ensure_after(self.updated_at)?;
        if self.status != ActionStatus::Ready {
            return Err(DomainError::InvalidTransition);
        }
        self.plan.ensure_executable()?;
        self.status = ActionStatus::InProgress;
        self.started_at = Some(at);
        self.updated_at = at;
        Ok(())
    }

    pub fn complete(
        &mut self,
        artifact: ArtifactReference,
        at: Instant,
    ) -> Result<(), DomainError> {
        at.ensure_after(self.updated_at)?;
        if self.status != ActionStatus::InProgress {
            return Err(DomainError::InvalidTransition);
        }
        self.status = ActionStatus::Completed;
        self.completed_at = Some(at);
        self.artifact = Some(artifact);
        self.updated_at = at;
        Ok(())
    }

    pub fn block(&mut self, at: Instant) -> Result<(), DomainError> {
        at.ensure_after(self.updated_at)?;
        if !matches!(
            self.status,
            ActionStatus::New | ActionStatus::Ready | ActionStatus::InProgress
        ) {
            return Err(DomainError::InvalidTransition);
        }
        self.status = ActionStatus::Blocked;
        self.updated_at = at;
        Ok(())
    }

    pub fn cancel(&mut self, at: Instant) -> Result<(), DomainError> {
        at.ensure_after(self.updated_at)?;
        if matches!(
            self.status,
            ActionStatus::Completed | ActionStatus::Cancelled
        ) {
            return Err(DomainError::InvalidTransition);
        }
        self.status = ActionStatus::Cancelled;
        self.updated_at = at;
        Ok(())
    }

    /// The adapter must archive the previous state in the same transaction.
    pub fn reopen(&mut self, at: Instant) -> Result<(), DomainError> {
        at.ensure_after(self.updated_at)?;
        if !matches!(
            self.status,
            ActionStatus::Completed | ActionStatus::Cancelled
        ) {
            return Err(DomainError::InvalidTransition);
        }
        self.status = ActionStatus::New;
        self.started_at = None;
        self.completed_at = None;
        self.artifact = None;
        self.updated_at = at;
        Ok(())
    }
}
