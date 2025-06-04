use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[readonly::make]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowActionEmail {
    #[readonly]
    pub id: Option<i32>,
    pub subject: String,
    pub body: String,
    pub to: String,
    pub include_document: Option<bool>,
}
