use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::model;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Create {
    pub name: String,
    pub data_type: model::DataType,
    pub extra_data: Option<serde_json::Value>,
}

#[must_use]
pub fn create(name: String, data_type: model::DataType) -> Create {
    Create::new(name, data_type)
}

impl From<&model::CustomField> for Create {
    fn from(item: &model::CustomField) -> Self {
        Self {
            name: item.name.clone(),
            data_type: item.data_type,
            extra_data: item.extra_data.clone(),
        }
    }
}

impl Create {
    #[must_use]
    pub fn new(name: String, data_type: model::DataType) -> Self {
        Self {
            name,
            data_type,
            extra_data: None,
        }
    }

    #[must_use]
    pub fn name(mut self, value: String) -> Self {
        self.name = value;
        self
    }

    #[must_use]
    pub fn data_type(mut self, value: model::DataType) -> Self {
        self.data_type = value;
        self
    }

    #[must_use]
    pub fn extra_data(mut self, value: serde_json::Value) -> Self {
        self.extra_data = Some(value);
        self
    }
}
