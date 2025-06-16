use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[readonly::make]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomField {
    #[readonly]
    pub id: i32,
    #[serde(default = 0)]
    #[readonly]
    pub document_count: i32,
    pub name: String,
    pub data_type: super::DataType,
    pub extra_data: Option<serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldInstance {
    pub value: serde_json::Value,
    pub field: i32,
}
