use serde::{Deserialize, Serialize};

#[readonly::make]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Group {
    #[readonly]
    pub id: i32,
    pub name: String,
    pub permissions: Vec<super::PermissionClass>,
}
