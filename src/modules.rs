use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
    Critical,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModuleStatus {
    Active,
    Reserved,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct ModuleDescriptor {
    pub id: &'static str,
    pub name: &'static str,
    pub ui_path: Option<&'static str>,
    pub status: ModuleStatus,
    pub classification: DataClassification,
    pub live: bool,
}

pub fn catalog() -> &'static [ModuleDescriptor] {
    &[
        ModuleDescriptor {
            id: "live-system-state",
            name: "Live System State",
            ui_path: Some("/command/state"),
            status: ModuleStatus::Active,
            classification: DataClassification::Internal,
            live: true,
        },
        ModuleDescriptor {
            id: "finance",
            name: "Finance",
            ui_path: None,
            status: ModuleStatus::Reserved,
            classification: DataClassification::Restricted,
            live: true,
        },
        ModuleDescriptor {
            id: "intellectual-property",
            name: "Intellectual Property",
            path: "/command",
            status: ModuleStatus::Reserved,
            classification: DataClassification::Restricted,
            live: false,
        },
        ModuleDescriptor {
            id: "people",
            name: "People",
            path: "/command",
            status: ModuleStatus::Reserved,
            classification: DataClassification::Restricted,
            live: false,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::{DataClassification, ModuleStatus, catalog};

    #[test]
    fn catalog_contains_only_declared_modules() {
        let modules = catalog();
        assert_eq!(modules.len(), 4);
        assert_eq!(modules[0].status, ModuleStatus::Active);
        assert_eq!(modules[1].classification, DataClassification::Restricted);
    }
}
