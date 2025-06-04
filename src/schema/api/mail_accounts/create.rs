use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::model;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Create {
    pub name: String,
    pub imap_server: String,
    pub imap_port: Option<i64>,
    pub imap_security: Option<model::ImapSecurity>,
    pub username: String,
    pub password: String,
    pub character_set: Option<String>,
    pub is_token: Option<bool>,
    pub owner: Option<i32>,
    pub set_permissions: Option<model::Permissions>,
    pub account_type: Option<model::AccountType>,
    pub expiration: Option<String>,
}

#[must_use]
pub fn create(name: String, imap_server: String, username: String, password: String) -> Create {
    Create::new(name, imap_server, username, password)
}

impl From<&model::MailAccount> for Create {
    fn from(item: &model::MailAccount) -> Self {
        Self {
            name: item.name.clone(),
            imap_server: item.imap_server.clone(),
            imap_port: item.imap_port,
            imap_security: item.imap_security,
            username: item.username.clone(),
            password: item.password.clone(),
            character_set: item.character_set.clone(),
            is_token: item.is_token,
            owner: item.owner,
            set_permissions: Some(item.permissions.clone()),
            account_type: item.account_type,
            expiration: item.expiration.clone(),
        }
    }
}

impl Create {
    #[must_use]
    pub fn new(name: String, imap_server: String, username: String, password: String) -> Self {
        Self {
            name,
            imap_server,
            imap_port: None,
            imap_security: None,
            username,
            password,
            character_set: None,
            is_token: None,
            owner: None,
            set_permissions: None,
            account_type: None,
            expiration: None,
        }
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
