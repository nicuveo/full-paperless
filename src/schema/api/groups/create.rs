use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::model;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Create {
    pub name: String,
    pub permissions: Vec<model::PermissionClass>,
}

#[must_use]
pub fn create(name: String, permissions: Vec<model::PermissionClass>) -> Create {
    Create::new(name, permissions)
}

impl From<&model::Group> for Create {
    fn from(item: &model::Group) -> Self {
        Self {
            name: item.name.clone(),
            permissions: item.permissions.clone(),
        }
    }
}

impl Create {
    #[must_use]
    pub fn new(name: String, permissions: Vec<model::PermissionClass>) -> Self {
        Self { name, permissions }
    }
}
