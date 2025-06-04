use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[readonly::make]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MailAccount {
    #[readonly]
    pub id: i32,
    #[readonly]
    pub user_can_change: bool,
    pub name: String,
    pub imap_server: String,
    pub imap_port: Option<i64>,
    pub imap_security: Option<super::ImapSecurity>,
    pub username: String,
    pub password: String,
    pub character_set: Option<String>,
    pub is_token: Option<bool>,
    pub owner: Option<i32>,
    pub account_type: Option<super::AccountType>,
    pub expiration: Option<String>,
    pub permissions: super::Permissions,
}
