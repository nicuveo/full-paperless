use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[readonly::make]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoragePath {
    #[readonly]
    pub id: i32,
    #[readonly]
    pub slug: String,
    #[readonly]
    pub document_count: i32,
    #[readonly]
    pub user_can_change: bool,
    pub name: String,
    pub path: String,
    #[serde(rename = "match")]
    pub matches: Option<String>,
    pub matching_algorithm: Option<super::MatchingAlgorithm>,
    pub is_insensitive: Option<bool>,
    pub owner: Option<i32>,
    pub permissions: super::Permissions,
}
