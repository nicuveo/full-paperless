use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::model;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Create {
    pub expiration: Option<String>,
    pub document: Option<i32>,
    pub file_version: Option<model::FileVersion>,
}

#[must_use]
pub fn create() -> Create {
    Create::new()
}

impl From<&model::ShareLink> for Create {
    fn from(item: &model::ShareLink) -> Self {
        Self {
            expiration: item.expiration.clone(),
            document: item.document,
            file_version: item.file_version,
        }
    }
}

impl Create {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn expiration(mut self, value: String) -> Self {
        self.expiration = Some(value);
        self
    }

    #[must_use]
    pub fn document(mut self, value: i32) -> Self {
        self.document = Some(value);
        self
    }

    #[must_use]
    pub fn set_permissions(mut self, value: model::FileVersion) -> Self {
        self.file_version = Some(value);
        self
    }
}
