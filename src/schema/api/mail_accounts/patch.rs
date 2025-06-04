use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::model;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Patch {
    pub name: Option<String>,
    pub imap_server: Option<String>,
    pub imap_port: Option<i64>,
    pub imap_security: Option<model::ImapSecurity>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub character_set: Option<String>,
    pub is_token: Option<bool>,
    pub owner: Option<i32>,
    pub set_permissions: Option<model::Permissions>,
    pub account_type: Option<model::AccountType>,
    pub expiration: Option<String>,
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
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    #[must_use]
    pub fn imap_server(mut self, value: String) -> Self {
        self.imap_server = Some(value);
        self
    }

    #[must_use]
    pub fn imap_port(mut self, value: i64) -> Self {
        self.imap_port = Some(value);
        self
    }

    #[must_use]
    pub fn imap_security(mut self, value: model::ImapSecurity) -> Self {
        self.imap_security = Some(value);
        self
    }

    #[must_use]
    pub fn username(mut self, value: String) -> Self {
        self.username = Some(value);
        self
    }

    #[must_use]
    pub fn password(mut self, value: String) -> Self {
        self.password = Some(value);
        self
    }

    #[must_use]
    pub fn character_set(mut self, value: String) -> Self {
        self.character_set = Some(value);
        self
    }

    #[must_use]
    pub fn is_token(mut self, value: bool) -> Self {
        self.is_token = Some(value);
        self
    }

    #[must_use]
    pub fn owner(mut self, value: i32) -> Self {
        self.owner = Some(value);
        self
    }

    #[must_use]
    pub fn set_permissions(mut self, value: model::Permissions) -> Self {
        self.set_permissions = Some(value);
        self
    }

    #[must_use]
    pub fn account_type(mut self, value: model::AccountType) -> Self {
        self.account_type = Some(value);
        self
    }

    #[must_use]
    pub fn expiration(mut self, value: String) -> Self {
        self.expiration = Some(value);
        self
    }
}
