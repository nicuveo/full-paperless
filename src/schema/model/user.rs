use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[readonly::make]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[readonly]
    pub id: i32,
    #[readonly]
    pub is_mfa_enabled: bool,
    #[readonly]
    #[serde(default)]
    pub inherited_permissions: Vec<super::PermissionClass>,
    pub username: String,
    pub email: Option<String>,
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_joined: Option<String>,
    pub is_staff: Option<bool>,
    pub is_active: Option<bool>,
    pub is_superuser: Option<bool>,
    pub groups: Option<Vec<i32>>,
    #[serde(default)]
    pub user_permissions: Vec<super::PermissionClass>,
}
