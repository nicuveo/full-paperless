use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Suggestions {
    pub correspondents: Vec<i32>,
    pub tags: Vec<i32>,
    pub document_types: Vec<i32>,
    pub storage_paths: Vec<i32>,
    pub dates: Vec<String>,
}
