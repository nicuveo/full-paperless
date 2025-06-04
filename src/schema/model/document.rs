use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use crate::schema::utils;

#[readonly::make]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Document {
    #[readonly]
    pub id: i32,
    pub correspondent: Option<i32>,
    pub document_type: Option<i32>,
    pub storage_path: Option<i32>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub tags: Vec<i32>,
    pub created: Option<String>,
    pub created_date: Option<String>,
    pub modified: String,
    pub added: String,
    pub deleted_at: Option<String>,
    pub archive_serial_number: Option<i64>,
    pub original_file_name: Option<String>,
    pub archived_file_name: Option<String>,
    pub owner: Option<i32>,
    pub permissions: super::Permissions,
    pub user_can_change: bool,
    pub is_shared_by_requester: bool,
    pub notes: Vec<super::Note>,
    #[serde(with = "utils::custom_fields::map")]
    pub custom_fields: HashMap<i32, serde_json::Value>,
    pub page_count: Option<i32>,
    pub mime_type: String,
}
