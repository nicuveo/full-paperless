use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[readonly::make]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShareLink {
    #[readonly]
    pub id: i32,
    #[readonly]
    pub slug: String,
    #[readonly]
    pub created: String,
    pub expiration: Option<String>,
    pub document: Option<i32>,
    pub file_version: Option<super::FileVersion>,
}
