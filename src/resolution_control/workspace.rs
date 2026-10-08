//! Bounded command replay is also the recovery validation boundary. Incoming
//! JSON cannot set a status, actor, timestamp, owner or verified-finish token.
use super::*;
use serde::{Deserialize, Serialize};

pub const MAX_RESOLUTIONS: usize = 16;
pub const MAX_COMMITMENTS: usize = 64;
pub const MAX_ACTIONS: usize = 128;
pub const MAX_THREATS: usize = 128;
pub const MAX_EVENTS: usize = 2048;
pub const MAX_BACKUP_BYTES: usize = 4 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Command {
    CreateResolution {
        id: Identifier,
        spec: ResolutionSpec,
    },
    UpdateResolution {
        id: Identifier,
        spec: ResolutionSpec,
    },
    CreateCommitment {
        id: Identifier,
        resolution: Identifier,
        spec: CommitmentSpec,
        schedule: Schedule,
    },
    UpdateCommitment {
        id: Identifier,
        spec: CommitmentSpec,
        schedule: Schedule,
    },
    CreateAction {
        id: Identifier,
        commitment: Identifier,
        plan: ActionPlan,
    },
    UpdateAction {
        id: Identifier,
        plan: ActionPlan,
    },
    SelectAction {
        id: Option<Identifier>,
    },
    ReadyAction {
        id: Identifier,
    },
    StartAction {
        id: Identifier,
    },
    CompleteAction {
        id: Identifier,
        artifact: ArtifactReference,
    },
    BlockAction {
        id: Identifier,
    },
    CancelAction {
        id: Identifier,
    },
    ReopenAction {
        id: Identifier,
    },
    IdentifyScope {
        commitment: Identifier,
        items: Vec<ScopeKey>,
    },
    MarkScopeUnknown {
        commitment: Identifier,
    },
    MapMaterial {
        commitment: Identifier,
        key: ScopeKey,
        artifact: ArtifactReference,
    },
    DeployMaterial {
        commitment: Identifier,
        key: ScopeKey,
        artifact: ArtifactReference,
    },
    RecordEvidence {
        commitment: Identifier,
        input: EvidenceInput,
    },
    ConfirmReadiness {
        commitment: Identifier,
    },
    ReopenReadiness {
        commitment: Identifier,
    },
    AddThreat {
        id: Identifier,
        commitment: Identifier,
        spec: ThreatSpec,
    },
    ResolveThreat {
        id: Identifier,
    },
}

impl Command {
    pub fn code(&self) -> &'static str {
        match self {
            Self::CreateResolution { .. } => "resolution.created",
            Self::UpdateResolution { .. } => "resolution.updated",
            Self::CreateCommitment { .. } => "commitment.created",
            Self::UpdateCommitment { .. } => "commitment.updated",
            Self::CreateAction { .. } => "action.created",
            Self::UpdateAction { .. } => "action.updated",
            Self::SelectAction { .. } => "action.selected",
            Self::ReadyAction { .. } => "action.ready",
            Self::StartAction { .. } => "action.started",
            Self::CompleteAction { .. } => "action.completed",
            Self::BlockAction { .. } => "action.blocked",
            Self::CancelAction { .. } => "action.cancelled",
            Self::ReopenAction { .. } => "action.reopened",
            Self::IdentifyScope { .. } => "scope.identified",
            Self::MarkScopeUnknown { .. } => "scope.unknown",
            Self::MapMaterial { .. } => "material.mapped",
            Self::DeployMaterial { .. } => "material.deployed",
            Self::RecordEvidence { input, .. } => match (input.stage, input.outcome) {
                (EvidenceStage::StressTest, Outcome::Pass) => "stress_test.passed",
                (EvidenceStage::StressTest, Outcome::Fail) => "stress_test.failed",
                (EvidenceStage::Verification, Outcome::Pass) => "verification.passed",
                (EvidenceStage::Verification, Outcome::Fail) => "verification.failed",
            },
            Self::ConfirmReadiness { .. } => "readiness.confirmed",
            Self::ReopenReadiness { .. } => "readiness.reopened",
            Self::AddThreat { .. } => "threat.created",
            Self::ResolveThreat { .. } => "threat.resolved",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResolutionRecord {
    pub id: Identifier,
    pub spec: ResolutionSpec,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommitmentRecord {
    pub id: Identifier,
    pub resolution: Identifier,
    pub spec: CommitmentSpec,
    pub schedule: Schedule,
    pub readiness: Readiness,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ActionRecord {
    pub id: Identifier,
    pub commitment: Identifier,
    pub action: Action,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ThreatRecord {
    pub id: Identifier,
    pub commitment: Identifier,
    pub spec: ThreatSpec,
    pub resolved: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct Workspace {
    pub resolutions: Vec<ResolutionRecord>,
    pub commitments: Vec<CommitmentRecord>,
    pub actions: Vec<ActionRecord>,
    pub threats: Vec<ThreatRecord>,
    pub current_action: Option<Identifier>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceError {
    NotFound,
    DuplicateIdentifier,
    CapacityReached,
    CurrentActionRequired,
    Domain(DomainError),
}

impl From<DomainError> for WorkspaceError {
    fn from(value: DomainError) -> Self {
        Self::Domain(value)
    }
}

impl Workspace {
    fn commitment_mut(&mut self, id: &Identifier) -> Result<&mut CommitmentRecord, WorkspaceError> {
        self.commitments
            .iter_mut()
            .find(|c| &c.id == id)
            .ok_or(WorkspaceError::NotFound)
    }

    fn action_mut(&mut self, id: &Identifier) -> Result<&mut ActionRecord, WorkspaceError> {
        self.actions
            .iter_mut()
            .find(|a| &a.id == id)
            .ok_or(WorkspaceError::NotFound)
    }

    fn ensure_unused(&self, id: &Identifier) -> Result<(), WorkspaceError> {
        if self.resolutions.iter().any(|r| &r.id == id)
            || self.commitments.iter().any(|c| &c.id == id)
            || self.actions.iter().any(|a| &a.id == id)
            || self.threats.iter().any(|t| &t.id == id)
        {
            return Err(WorkspaceError::DuplicateIdentifier);
        }
        Ok(())
    }

    /// Failure leaves every aggregate unchanged, including multi-record edits.
    pub fn apply(
        &mut self,
        command: &Command,
        actor: Identifier,
        at: Instant,
    ) -> Result<(), WorkspaceError> {
        let mut next = self.clone();
        next.apply_inner(command.clone(), actor, at)?;
        *self = next;
        Ok(())
    }

    fn apply_inner(
        &mut self,
        command: Command,
        actor: Identifier,
        at: Instant,
    ) -> Result<(), WorkspaceError> {
        match command {
            Command::CreateResolution { id, spec } => {
                self.ensure_unused(&id)?;
                if self.resolutions.len() >= MAX_RESOLUTIONS {
                    return Err(WorkspaceError::CapacityReached);
                }
                self.resolutions.push(ResolutionRecord { id, spec });
            }
            Command::UpdateResolution { id, spec } => {
                self.resolutions
                    .iter_mut()
                    .find(|r| r.id == id)
                    .ok_or(WorkspaceError::NotFound)?
                    .spec = spec;
            }
            Command::CreateCommitment {
                id,
                resolution,
                spec,
                schedule,
            } => {
                self.ensure_unused(&id)?;
                if !self.resolutions.iter().any(|r| r.id == resolution) {
                    return Err(WorkspaceError::NotFound);
                }
                if self.commitments.len() >= MAX_COMMITMENTS {
                    return Err(WorkspaceError::CapacityReached);
                }
                self.commitments.push(CommitmentRecord {
                    id,
                    resolution,
                    spec,
                    schedule,
                    readiness: Readiness::unknown(at),
                });
            }
            Command::UpdateCommitment { id, spec, schedule } => {
                let commitment = self.commitment_mut(&id)?;
                // A changed objective/source/kind may invalidate the known scope.
                if commitment.spec != spec {
                    commitment.readiness.mark_scope_unknown(at)?;
                }
                commitment.spec = spec;
                commitment.schedule = schedule;
            }
            Command::CreateAction {
                id,
                commitment,
                plan,
            } => {
                self.ensure_unused(&id)?;
                self.commitment_mut(&commitment)?;
                if self.actions.len() >= MAX_ACTIONS {
                    return Err(WorkspaceError::CapacityReached);
                }
                self.actions.push(ActionRecord {
                    id,
                    commitment,
                    action: Action::new(plan, at),
                });
            }
            Command::SelectAction { id } => {
                if let Some(id) = &id {
                    let status = self.action_mut(id)?.action.status();
                    if matches!(status, ActionStatus::Completed | ActionStatus::Cancelled) {
                        return Err(DomainError::InvalidTransition.into());
                    }
                }
                self.current_action = id;
            }
            Command::UpdateAction { id, plan } => {
                self.action_mut(&id)?.action.update_plan(plan, at)?
            }
            Command::ReadyAction { id } => self.action_mut(&id)?.action.ready(at)?,
            Command::StartAction { id } => {
                if self.current_action.as_ref() != Some(&id) {
                    return Err(WorkspaceError::CurrentActionRequired);
                }
                // There can be only one running action, not just one UI focus.
                if self
                    .actions
                    .iter()
                    .any(|a| a.action.status() == ActionStatus::InProgress)
                {
                    return Err(DomainError::InvalidTransition.into());
                }
                self.action_mut(&id)?.action.start(at)?;
            }
            Command::CompleteAction { id, artifact } => {
                self.action_mut(&id)?.action.complete(artifact, at)?;
                if self.current_action.as_ref() == Some(&id) {
                    self.current_action = None;
                }
            }
            Command::BlockAction { id } => self.action_mut(&id)?.action.block(at)?,
            Command::CancelAction { id } => {
                self.action_mut(&id)?.action.cancel(at)?;
                if self.current_action.as_ref() == Some(&id) {
                    self.current_action = None;
                }
            }
            Command::ReopenAction { id } => self.action_mut(&id)?.action.reopen(at)?,
            Command::IdentifyScope { commitment, items } => self
                .commitment_mut(&commitment)?
                .readiness
                .identify(items, at)?,
            Command::MarkScopeUnknown { commitment } => self
                .commitment_mut(&commitment)?
                .readiness
                .mark_scope_unknown(at)?,
            Command::MapMaterial {
                commitment,
                key,
                artifact,
            } => self
                .commitment_mut(&commitment)?
                .readiness
                .map(&key, artifact, at)?,
            Command::DeployMaterial {
                commitment,
                key,
                artifact,
            } => self
                .commitment_mut(&commitment)?
                .readiness
                .deploy(&key, artifact, at)?,
            Command::RecordEvidence { commitment, input } => {
                let readiness = &mut self.commitment_mut(&commitment)?.readiness;
                let evidence = Evidence::new(input, actor, readiness.revision(), at);
                readiness.record(evidence)?;
            }
            Command::ConfirmReadiness { commitment } => {
                self.commitment_mut(&commitment)?.readiness.confirm(at)?;
            }
            Command::ReopenReadiness { commitment } => {
                self.commitment_mut(&commitment)?.readiness.reopen(at)?
            }
            Command::AddThreat {
                id,
                commitment,
                spec,
            } => {
                self.ensure_unused(&id)?;
                self.commitment_mut(&commitment)?;
                if self.threats.len() >= MAX_THREATS {
                    return Err(WorkspaceError::CapacityReached);
                }
                self.threats.push(ThreatRecord {
                    id,
                    commitment: commitment.clone(),
                    spec,
                    resolved: false,
                });
                self.refresh_blockers(&commitment, at)?;
            }
            Command::ResolveThreat { id } => {
                let threat = self
                    .threats
                    .iter_mut()
                    .find(|t| t.id == id)
                    .ok_or(WorkspaceError::NotFound)?;
                if threat.resolved {
                    return Err(DomainError::InvalidTransition.into());
                }
                threat.resolved = true;
                let commitment = threat.commitment.clone();
                self.refresh_blockers(&commitment, at)?;
            }
        }
        Ok(())
    }

    fn refresh_blockers(&mut self, id: &Identifier, at: Instant) -> Result<(), WorkspaceError> {
        let count = self
            .threats
            .iter()
            .filter(|t| &t.commitment == id && t.spec.blocking && !t.resolved)
            .count();
        self.commitment_mut(id)?
            .readiness
            .set_blocking_threats(count as u16, at)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventOrigin {
    Live,
    Imported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceEvent {
    pub revision: u64,
    pub at: Instant,
    pub origin: EventOrigin,
    pub command: Command,
}

pub fn replay(
    events: &[WorkspaceEvent],
    actor: Identifier,
    cutoff: Instant,
) -> Result<Workspace, WorkspaceError> {
    if events.len() > MAX_EVENTS {
        return Err(WorkspaceError::CapacityReached);
    }
    let mut state = Workspace::default();
    let mut previous = None;
    for (index, event) in events.iter().enumerate() {
        if event.revision != index as u64 + 1
            || event.at > cutoff
            || previous.is_some_and(|prior| event.at < prior)
        {
            return Err(DomainError::InvalidTime.into());
        }
        state.apply(&event.command, actor.clone(), event.at)?;
        previous = Some(event.at);
    }
    Ok(state)
}
