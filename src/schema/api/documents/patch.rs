use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use crate::schema::model;
use crate::schema::utils;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Patch {
    pub correspondent: Option<i32>,
    pub document_type: Option<i32>,
    pub storage_path: Option<i32>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub tags: Option<Vec<i32>>,
    pub created: Option<String>,
    pub deleted_at: Option<String>,
    pub archive_serial_number: Option<i64>,
    pub owner: Option<i32>,
    pub set_permissions: Option<model::Permissions>,
    #[serde(with = "utils::custom_fields::map_option")]
    pub custom_fields: Option<HashMap<i32, serde_json::Value>>,
    pub remove_inbox_tags: Option<bool>,
}

#[must_use]
pub fn patch() -> Patch {
    Patch::new()
}

impl Patch {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn correspondent(mut self, value: i32) -> Self {
        self.correspondent = Some(value);
        self
    }

    #[must_use]
    pub fn document_type(mut self, value: i32) -> Self {
        self.document_type = Some(value);
        self
    }

    #[must_use]
    pub fn storage_path(mut self, value: i32) -> Self {
        self.storage_path = Some(value);
        self
    }

    #[must_use]
    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    #[must_use]
    pub fn content(mut self, value: String) -> Self {
        self.content = Some(value);
        self
    }

    #[must_use]
    pub fn tags(mut self, value: Vec<i32>) -> Self {
        self.tags = Some(value);
        self
    }

    #[must_use]
    pub fn created(mut self, value: String) -> Self {
        self.created = Some(value);
        self
    }

    #[must_use]
    pub fn deleted_at(mut self, value: String) -> Self {
        self.deleted_at = Some(value);
        self
    }

    #[must_use]
    pub fn archive_serial_number(mut self, value: i64) -> Self {
        self.archive_serial_number = Some(value);
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
    pub fn custom_fields(mut self, value: HashMap<i32, serde_json::Value>) -> Self {
        self.custom_fields = Some(value);
        self
    }

    #[must_use]
    pub fn remove_inbox_tags(mut self, value: bool) -> Self {
        self.remove_inbox_tags = Some(value);
        self
    }
}
