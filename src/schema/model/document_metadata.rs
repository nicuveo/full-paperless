use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub original_checksum: String,
    pub original_size: i32,
    pub original_mime_type: String,
    pub media_filename: String,
    pub has_archive_version: bool,
    pub original_metadata: HashMap<String, serde_json::Value>,
    pub archive_checksum: String,
    pub archive_media_filename: String,
    pub original_filename: String,
    pub archive_size: i32,
    pub archive_metadata: HashMap<String, serde_json::Value>,
    pub lang: String,
}
