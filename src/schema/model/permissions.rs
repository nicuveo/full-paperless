use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Permissions {
    pub view: Option<PermissionsView>,
    pub change: Option<PermissionsView>,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PermissionsView {
    pub users: Option<Vec<i32>>,
    pub groups: Option<Vec<i32>>,
}

impl Permissions {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl PermissionsView {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
