use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::model;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Patch {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_joined: Option<String>,
    pub is_staff: Option<bool>,
    pub is_active: Option<bool>,
    pub is_superuser: Option<bool>,
    pub groups: Option<Vec<i32>>,
    pub user_permissions: Option<Vec<model::PermissionClass>>,
}

#[must_use]
pub fn patch() -> Patch {
    Patch::new()
}

impl Patch {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn username(mut self, value: String) -> Self {
        self.username = Some(value);
        self
    }

    #[must_use]
    pub fn email(mut self, value: String) -> Self {
        self.email = Some(value);
        self
    }

    #[must_use]
    pub fn password(mut self, value: String) -> Self {
        self.password = Some(value);
        self
    }

    #[must_use]
    pub fn first_name(mut self, value: String) -> Self {
        self.first_name = Some(value);
        self
    }

    #[must_use]
    pub fn last_name(mut self, value: String) -> Self {
        self.last_name = Some(value);
        self
    }

    #[must_use]
    pub fn date_joined(mut self, value: String) -> Self {
        self.date_joined = Some(value);
        self
    }

    #[must_use]
    pub fn is_staff(mut self, value: bool) -> Self {
        self.is_staff = Some(value);
        self
    }

    #[must_use]
    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    #[must_use]
    pub fn is_superuser(mut self, value: bool) -> Self {
        self.is_superuser = Some(value);
        self
    }

    #[must_use]
    pub fn groups(mut self, value: Vec<i32>) -> Self {
        self.groups = Some(value);
        self
    }

    #[must_use]
    pub fn user_permissions(mut self, value: Vec<model::PermissionClass>) -> Self {
        self.user_permissions = Some(value);
        self
    }
}
