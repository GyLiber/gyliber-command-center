use serde::{Deserialize, Serialize};
use std::fmt;
use url::Url;

/// Errors are stable codes; they never echo private input into logs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainError {
    InvalidText,
    InvalidIdentifier,
    InvalidReference,
    InvalidTime,
    InvalidBuffer,
    InvalidSchedule,
    InvalidTransition,
    IncompleteAction,
    ClockWentBackwards,
    InvalidScope,
    UnknownScopeItem,
    StaleEvidence,
    MissingPrerequisite,
    NotReady,
    RevisionExhausted,
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for DomainError {}

/// UTF-8 byte bounds also apply during JSON deserialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Text<const MAX: usize>(String);

impl<const MAX: usize> TryFrom<String> for Text<MAX> {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let text = value.trim();
        if text.is_empty()
            || text.len() > MAX
            || text
                .chars()
                .any(|c| c.is_control() && c != '\n' && c != '\t')
        {
            return Err(DomainError::InvalidText);
        }
        Ok(Self(text.to_owned()))
    }
}

impl<const MAX: usize> From<Text<MAX>> for String {
    fn from(value: Text<MAX>) -> Self {
        value.0
    }
}

impl<const MAX: usize> Text<MAX> {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub type Title = Text<160>;
pub type Notes = Text<2048>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Identifier(String);

impl TryFrom<String> for Identifier {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty()
            || value.len() > 64
            || !value
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
        {
            return Err(DomainError::InvalidIdentifier);
        }
        Ok(Self(value))
    }
}

impl From<Identifier> for String {
    fn from(value: Identifier) -> Self {
        value.0
    }
}

impl Identifier {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct WebReference(String);

impl TryFrom<String> for WebReference {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = Text::<2048>::try_from(value)?;
        let url = Url::parse(value.as_str()).map_err(|_| DomainError::InvalidReference)?;
        // The initial pilot forbids embedded credentials and query/fragment
        // tokens. Other source locations can be a non-clickable text reference.
        if !matches!(url.scheme(), "http" | "https")
            || url.host_str().is_none()
            || !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
            || value.as_str().chars().any(char::is_whitespace)
        {
            return Err(DomainError::InvalidReference);
        }
        Ok(Self(url.into()))
    }
}

impl From<WebReference> for String {
    fn from(value: WebReference) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum ArtifactReference {
    Web(WebReference),
    Text(Notes),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResolutionSpec {
    pub title: Title,
    pub objective: Option<Notes>,
    pub client_reference: Option<ArtifactReference>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommitmentKind {
    Unclassified,
    Assessment,
    Project,
    Research,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommitmentSpec {
    pub title: Title,
    pub area: Option<Title>,
    pub kind: CommitmentKind,
    pub source: Option<ArtifactReference>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThreatSpec {
    pub description: Notes,
    pub blocking: bool,
    pub resolution_path: Option<Notes>,
}
