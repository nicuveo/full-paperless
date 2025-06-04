use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[readonly::make]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowTrigger {
    #[readonly]
    pub id: i32,
    pub sources: Option<Vec<super::Source>>,
    #[serde(rename = "type")]
    pub trigger_type: super::WorkflowTriggerType,
    pub filter_path: Option<String>,
    pub filter_filename: Option<String>,
    pub filter_mailrule: Option<i32>,
    pub matching_algorithm: Option<super::WorkflowTriggerMatchingAlgorithm>,
    #[serde(rename = "match")]
    pub matches: Option<String>,
    pub is_insensitive: Option<bool>,
    pub filter_has_tags: Option<Vec<i32>>,
    pub filter_has_correspondent: Option<i32>,
    pub filter_has_document_type: Option<i32>,
    pub schedule_offset_days: Option<i64>,
    pub schedule_is_recurring: Option<bool>,
    pub schedule_recurring_interval_days: Option<i64>,
    pub schedule_date_field: Option<super::ScheduleDateField>,
    pub schedule_date_custom_field: Option<i32>,
}
