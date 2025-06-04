use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[readonly::make]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Note {
    #[readonly]
    pub id: i32,
    pub note: Option<String>,
    pub created: Option<String>,
    pub user: super::BasicUser,
}
