use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[readonly::make]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowActionWebhook {
    #[readonly]
    pub id: Option<i32>,
    pub url: String,
    pub use_params: Option<bool>,
    pub as_json: Option<bool>,
    pub params: Option<serde_json::Value>,
    pub body: Option<String>,
    pub headers: Option<serde_json::Value>,
    pub include_document: Option<bool>,
}
