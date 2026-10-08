use super::store::{Backup, RecoveryMetadata};
use gyliber_command_center::resolution_control::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Meta {
    pub operation_id: Identifier,
    pub generation: Option<Identifier>,
    pub expected_revision: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Mutation {
    pub meta: Meta,
    pub command: Command,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Restore {
    pub meta: Meta,
    pub confirm_recovery_metadata: bool,
    pub backup: Backup,
    pub recovery_metadata: RecoveryMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Purge {
    pub meta: Meta,
    pub confirmation: String,
}

#[derive(Debug, Clone, Serialize)]
pub(super) enum Operation {
    Mutation {
        meta: Meta,
        command: Command,
    },
    Restore {
        meta: Meta,
        backup: Backup,
        recovery_metadata: RecoveryMetadata,
        confirm_recovery_metadata: bool,
    },
    Purge {
        meta: Meta,
        confirmation: String,
    },
}

impl Operation {
    pub fn meta(&self) -> &Meta {
        match self {
            Self::Mutation { meta, .. } | Self::Restore { meta, .. } | Self::Purge { meta, .. } => {
                meta
            }
        }
    }
}
