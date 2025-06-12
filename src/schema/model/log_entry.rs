use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[readonly::make]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogEntry {
    #[readonly]
    pub id: i32,
    pub timestamp: String,
    pub action: String,
    pub changes: HashMap<String, serde_json::Value>,
    pub actor: super::Actor,
}
