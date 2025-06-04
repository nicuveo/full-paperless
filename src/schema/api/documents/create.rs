use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use crate::schema::model;
use crate::schema::utils;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Create {
    #[serialize_always]
    pub correspondent: Option<i32>,
    #[serialize_always]
    pub document_type: Option<i32>,
    #[serialize_always]
    pub storage_path: Option<i32>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub tags: Vec<i32>,
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
pub fn create(tags: Vec<i32>) -> Create {
    Create::new(tags)
}

impl From<&model::Document> for Create {
    fn from(item: &model::Document) -> Self {
        Self {
            correspondent: item.correspondent,
            document_type: item.document_type,
            storage_path: item.storage_path,
            title: item.title.clone(),
            content: item.content.clone(),
            tags: item.tags.clone(),
            created: item.created.clone(),
            deleted_at: item.deleted_at.clone(),
            archive_serial_number: item.archive_serial_number,
            owner: item.owner,
            set_permissions: Some(item.permissions.clone()),
            custom_fields: Some(item.custom_fields.clone()),
            remove_inbox_tags: None,
        }
    }
}

impl Create {
    #[must_use]
    pub fn new(tags: Vec<i32>) -> Self {
        Self {
            correspondent: None,
            document_type: None,
            storage_path: None,
            title: None,
            content: None,
            tags,
            created: None,
            deleted_at: None,
            archive_serial_number: None,
            owner: None,
            set_permissions: None,
            custom_fields: None,
            remove_inbox_tags: None,
        }
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
        self.tags = value;
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
