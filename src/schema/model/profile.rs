use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::model;

#[readonly::make]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub email: Option<String>,
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[readonly]
    pub auth_token: String,
    #[readonly]
    pub social_accounts: Vec<model::SocialAccount>,
    #[readonly]
    pub has_usable_password: bool,
    #[readonly]
    pub is_mfa_enabled: bool,
}
