use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::model;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Create {
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
    pub user_permissions: Option<Vec<model::PermissionClass>>,
}

#[must_use]
pub fn create(username: String) -> Create {
    Create::new(username)
}

impl From<&model::User> for Create {
    fn from(item: &model::User) -> Self {
        Self {
            username: item.username.clone(),
            email: item.email.clone(),
            password: item.password.clone(),
            first_name: item.first_name.clone(),
            last_name: item.last_name.clone(),
            date_joined: item.date_joined.clone(),
            is_staff: item.is_staff,
            is_active: item.is_active,
            is_superuser: item.is_superuser,
            groups: item.groups.clone(),
            user_permissions: Some(item.user_permissions.clone()),
        }
    }
}

impl Create {
    #[must_use]
    pub fn new(username: String) -> Self {
        Self {
            username,
            email: None,
            password: None,
            first_name: None,
            last_name: None,
            date_joined: None,
            is_staff: None,
            is_active: None,
            is_superuser: None,
            groups: None,
            user_permissions: None,
        }
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
