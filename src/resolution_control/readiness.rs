use super::{ArtifactReference, DomainError, Identifier, Instant, Notes, VerifiedFinish};
use serde::{Deserialize, Serialize};

pub const MAX_SCOPE_ITEMS: usize = 128;
pub const MAX_BLOCKING_THREATS: u16 = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Dimension {
    Written,
    Oral,
    Board,
    Justification,
    Research,
    Practical,
    TestCase,
}

/// One identified scope item in one explicitly applicable dimension.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScopeKey {
    pub item: Identifier,
    pub dimension: Dimension,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceStage {
    StressTest,
    Verification,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Pass,
    Fail,
}

/// Actor, revision and timestamp must be supplied by the server adapter.
/// This records a human attestation; it does not certify mathematics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Evidence {
    key: ScopeKey,
    stage: EvidenceStage,
    method: Notes,
    artifact: ArtifactReference,
    actor: Identifier,
    scope_revision: u64,
    outcome: Outcome,
    recorded_at: Instant,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceInput {
    pub key: ScopeKey,
    pub stage: EvidenceStage,
    pub method: Notes,
    pub artifact: ArtifactReference,
    pub outcome: Outcome,
}

impl Evidence {
    pub fn new(input: EvidenceInput, actor: Identifier, scope_revision: u64, at: Instant) -> Self {
        Self {
            key: input.key,
            stage: input.stage,
            method: input.method,
            artifact: input.artifact,
            actor,
            scope_revision,
            outcome: input.outcome,
            recorded_at: at,
        }
    }

    fn passes(&self, revision: u64) -> bool {
        self.scope_revision == revision && self.outcome == Outcome::Pass
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageStage {
    Identified,
    Mapped,
    Deployed,
    StressTestFailed,
    StressTested,
    VerificationFailed,
    Verified,
    Stale,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct ScopeItem {
    key: ScopeKey,
    mapped: Option<ArtifactReference>,
    deployed: Option<ArtifactReference>,
    stress_test: Option<Evidence>,
    verification: Option<Evidence>,
}

impl ScopeItem {
    fn stage(&self, revision: u64) -> CoverageStage {
        if self
            .stress_test
            .as_ref()
            .is_some_and(|e| e.scope_revision == revision && e.outcome == Outcome::Fail)
        {
            return CoverageStage::StressTestFailed;
        }
        if self
            .verification
            .as_ref()
            .is_some_and(|e| e.scope_revision == revision && e.outcome == Outcome::Fail)
        {
            return CoverageStage::VerificationFailed;
        }
        if self
            .stress_test
            .as_ref()
            .is_some_and(|e| e.passes(revision))
        {
            if self
                .verification
                .as_ref()
                .is_some_and(|e| e.passes(revision))
            {
                return CoverageStage::Verified;
            }
            return CoverageStage::StressTested;
        }
        if self.stress_test.is_some() || self.verification.is_some() {
            return CoverageStage::Stale;
        }
        if self.deployed.is_some() {
            CoverageStage::Deployed
        } else if self.mapped.is_some() {
            CoverageStage::Mapped
        } else {
            CoverageStage::Identified
        }
    }
}

/// Latest state only. The persistence adapter must atomically preserve every
/// superseded state/evidence in history before storing the next state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Readiness {
    scope_revision: u64,
    scope_identified: bool,
    items: Vec<ScopeItem>,
    blocking_threats: u16,
    verified_finish: Option<VerifiedFinish>,
    updated_at: Instant,
}

impl Readiness {
    pub fn unknown(at: Instant) -> Self {
        Self {
            scope_revision: 0,
            scope_identified: false,
            items: Vec::new(),
            blocking_threats: 0,
            verified_finish: None,
            updated_at: at,
        }
    }

    pub fn revision(&self) -> u64 {
        self.scope_revision
    }

    pub fn verified_finish(&self) -> Option<VerifiedFinish> {
        self.verified_finish
    }

    pub fn mark_scope_unknown(&mut self, at: Instant) -> Result<(), DomainError> {
        self.advance(at)?;
        self.scope_identified = false;
        Ok(())
    }

    fn index(&self, key: &ScopeKey) -> Result<usize, DomainError> {
        self.items
            .iter()
            .position(|item| &item.key == key)
            .ok_or(DomainError::UnknownScopeItem)
    }

    fn advance(&mut self, at: Instant) -> Result<(), DomainError> {
        at.ensure_after(self.updated_at)?;
        let next = self
            .scope_revision
            .checked_add(1)
            .ok_or(DomainError::RevisionExhausted)?;
        self.scope_revision = next;
        self.verified_finish = None;
        self.updated_at = at;
        Ok(())
    }

    /// Re-identification conservatively makes all prior evidence stale.
    pub fn identify(&mut self, keys: Vec<ScopeKey>, at: Instant) -> Result<(), DomainError> {
        if keys.is_empty()
            || keys.len() > MAX_SCOPE_ITEMS
            || keys
                .iter()
                .enumerate()
                .any(|(i, key)| keys[..i].contains(key))
        {
            return Err(DomainError::InvalidScope);
        }
        at.ensure_after(self.updated_at)?;
        // Check revision overflow before changing the aggregate.
        self.scope_revision
            .checked_add(1)
            .ok_or(DomainError::RevisionExhausted)?;
        let items = keys
            .into_iter()
            .map(|key| {
                self.items
                    .iter()
                    .find(|item| item.key == key)
                    .cloned()
                    .unwrap_or(ScopeItem {
                        key,
                        mapped: None,
                        deployed: None,
                        stress_test: None,
                        verification: None,
                    })
            })
            .collect();
        self.advance(at)?;
        self.items = items;
        self.scope_identified = true;
        Ok(())
    }

    pub fn map(
        &mut self,
        key: &ScopeKey,
        artifact: ArtifactReference,
        at: Instant,
    ) -> Result<(), DomainError> {
        let index = self.index(key)?;
        self.advance(at)?;
        self.items[index].mapped = Some(artifact);
        self.items[index].deployed = None;
        Ok(())
    }

    pub fn deploy(
        &mut self,
        key: &ScopeKey,
        artifact: ArtifactReference,
        at: Instant,
    ) -> Result<(), DomainError> {
        let index = self.index(key)?;
        if self.items[index].mapped.is_none() {
            return Err(DomainError::MissingPrerequisite);
        }
        self.advance(at)?;
        self.items[index].deployed = Some(artifact);
        Ok(())
    }

    pub fn record(&mut self, evidence: Evidence) -> Result<(), DomainError> {
        evidence.recorded_at.ensure_after(self.updated_at)?;
        if evidence.scope_revision != self.scope_revision {
            return Err(DomainError::StaleEvidence);
        }
        let index = self.index(&evidence.key)?;
        let item = &mut self.items[index];
        if item.deployed.is_none() {
            return Err(DomainError::MissingPrerequisite);
        }
        if evidence.stage == EvidenceStage::Verification
            && evidence.outcome == Outcome::Pass
            && !item
                .stress_test
                .as_ref()
                .is_some_and(|e| e.passes(self.scope_revision))
        {
            return Err(DomainError::MissingPrerequisite);
        }
        self.updated_at = evidence.recorded_at;
        match evidence.stage {
            EvidenceStage::StressTest => {
                item.stress_test = Some(evidence);
                item.verification = None;
            }
            EvidenceStage::Verification => item.verification = Some(evidence),
        }
        self.verified_finish = None;
        Ok(())
    }

    pub fn coverage(&self, key: &ScopeKey) -> Result<CoverageStage, DomainError> {
        Ok(self.items[self.index(key)?].stage(self.scope_revision))
    }

    pub fn set_blocking_threats(&mut self, count: u16, at: Instant) -> Result<(), DomainError> {
        at.ensure_after(self.updated_at)?;
        if count > MAX_BLOCKING_THREATS {
            return Err(DomainError::InvalidScope);
        }
        if count > 0 {
            self.advance(at)?;
        }
        self.blocking_threats = count;
        self.updated_at = at;
        if count > 0 {
            self.verified_finish = None;
        }
        Ok(())
    }

    pub fn eligible_for_confirmation(&self) -> bool {
        self.scope_identified
            && !self.items.is_empty()
            && self.blocking_threats == 0
            && self
                .items
                .iter()
                .all(|item| item.stage(self.scope_revision) == CoverageStage::Verified)
    }

    pub fn confirm(&mut self, at: Instant) -> Result<VerifiedFinish, DomainError> {
        at.ensure_after(self.updated_at)?;
        if !self.eligible_for_confirmation() || self.verified_finish.is_some() {
            return Err(DomainError::NotReady);
        }
        let finish = VerifiedFinish(at);
        self.verified_finish = Some(finish);
        self.updated_at = at;
        Ok(finish)
    }

    /// Explicit reopening requires fresh tests against a new revision.
    pub fn reopen(&mut self, at: Instant) -> Result<(), DomainError> {
        if self.verified_finish.is_none() {
            return Err(DomainError::InvalidTransition);
        }
        self.advance(at)
    }
}
