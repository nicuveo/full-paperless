use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::model;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Patch {
    pub name: Option<String>,
    pub data_type: Option<model::DataType>,
    pub extra_data: Option<serde_json::Value>,
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
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    #[must_use]
    pub fn data_type(mut self, value: model::DataType) -> Self {
        self.data_type = Some(value);
        self
    }

    #[must_use]
    pub fn extra_data(mut self, value: serde_json::Value) -> Self {
        self.extra_data = Some(value);
        self
    }
}
