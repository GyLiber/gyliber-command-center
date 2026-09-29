use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResourceKind {
    Repository,
    Profile,
    Service,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResourceStatus {
    Active,
    Planned,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct ResourceDescriptor {
    pub id: &'static str,
    pub name: &'static str,
    pub kind: ResourceKind,
    pub status: ResourceStatus,
    pub visibility: &'static str,
    pub url: Option<&'static str>,
}

pub fn catalog() -> &'static [ResourceDescriptor] {
    &[
        ResourceDescriptor {
            id: "command-center-repository",
            name: "GyLiber Command Center Repository",
            kind: ResourceKind::Repository,
            status: ResourceStatus::Active,
            visibility: "PUBLIC",
            url: Some("https://github.com/GyLiber/gyliber-command-center"),
        },
        ResourceDescriptor {
            id: "gyliber-github-profile",
            name: "GyLiber GitHub",
            kind: ResourceKind::Profile,
            status: ResourceStatus::Active,
            visibility: "PUBLIC",
            url: Some("https://github.com/GyLiber"),
        },
        ResourceDescriptor {
            id: "gyliber-upwork-profile",
            name: "GyLiber Upwork",
            kind: ResourceKind::Profile,
            status: ResourceStatus::Planned,
            visibility: "PUBLIC",
            url: None,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::{ResourceKind, ResourceStatus, catalog};

    #[test]
    fn resource_catalog_has_a_safe_initial_set() {
        let resources = catalog();
        assert_eq!(resources.len(), 3);
        assert_eq!(resources[0].kind, ResourceKind::Repository);
        assert_eq!(resources[2].status, ResourceStatus::Planned);
        assert!(resources.iter().all(|resource| resource.visibility == "PUBLIC"));
    }
}
