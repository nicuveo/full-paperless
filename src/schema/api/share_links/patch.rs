use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::model;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Patch {
    pub expiration: Option<String>,
    pub document: Option<i32>,
    pub file_version: Option<model::FileVersion>,
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
