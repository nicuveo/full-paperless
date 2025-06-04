use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedViewFilterRule {
    pub rule_type: super::RuleType,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,
}
