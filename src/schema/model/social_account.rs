use serde::{Deserialize, Serialize};

#[readonly::make]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialAccount {
    #[readonly]
    pub id: i32,
    pub provider: String,
    pub name: String,
}
