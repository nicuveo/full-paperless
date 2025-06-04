use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::model;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Patch {
    #[serde(rename = "type")]
    pub trigger_type: Option<model::WorkflowTriggerType>,
    pub sources: Option<Vec<model::Source>>,
    pub filter_path: Option<String>,
    pub filter_filename: Option<String>,
    pub filter_mailrule: Option<i32>,
    pub matching_algorithm: Option<model::WorkflowTriggerMatchingAlgorithm>,
    #[serde(rename = "match")]
    pub matches: Option<String>,
    pub is_insensitive: Option<bool>,
    pub filter_has_tags: Option<Vec<i32>>,
    pub filter_has_correspondent: Option<i32>,
    pub filter_has_document_type: Option<i32>,
    pub schedule_offset_days: Option<i64>,
    pub schedule_is_recurring: Option<bool>,
    pub schedule_recurring_interval_days: Option<i64>,
    pub schedule_date_field: Option<model::ScheduleDateField>,
    pub schedule_date_custom_field: Option<i32>,
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
    pub fn trigger_type(mut self, value: model::WorkflowTriggerType) -> Self {
        self.trigger_type = Some(value);
        self
    }

    #[must_use]
    pub fn sources(mut self, value: Vec<model::Source>) -> Self {
        self.sources = Some(value);
        self
    }

    #[must_use]
    pub fn filter_path(mut self, value: String) -> Self {
        self.filter_path = Some(value);
        self
    }

    #[must_use]
    pub fn filter_filename(mut self, value: String) -> Self {
        self.filter_filename = Some(value);
        self
    }

    #[must_use]
    pub fn filter_mailrule(mut self, value: i32) -> Self {
        self.filter_mailrule = Some(value);
        self
    }

    #[must_use]
    pub fn matching_algorithm(mut self, value: model::WorkflowTriggerMatchingAlgorithm) -> Self {
        self.matching_algorithm = Some(value);
        self
    }

    #[must_use]
    pub fn matches(mut self, value: String) -> Self {
        self.matches = Some(value);
        self
    }

    #[must_use]
    pub fn is_insensitive(mut self, value: bool) -> Self {
        self.is_insensitive = Some(value);
        self
    }

    #[must_use]
    pub fn filter_has_tags(mut self, value: Vec<i32>) -> Self {
        self.filter_has_tags = Some(value);
        self
    }

    #[must_use]
    pub fn filter_has_correspondent(mut self, value: i32) -> Self {
        self.filter_has_correspondent = Some(value);
        self
    }

    #[must_use]
    pub fn filter_has_document_type(mut self, value: i32) -> Self {
        self.filter_has_document_type = Some(value);
        self
    }

    #[must_use]
    pub fn schedule_offset_days(mut self, value: i64) -> Self {
        self.schedule_offset_days = Some(value);
        self
    }

    #[must_use]
    pub fn schedule_is_recurring(mut self, value: bool) -> Self {
        self.schedule_is_recurring = Some(value);
        self
    }

    #[must_use]
    pub fn schedule_recurring_interval_days(mut self, value: i64) -> Self {
        self.schedule_recurring_interval_days = Some(value);
        self
    }

    #[must_use]
    pub fn schedule_date_field(mut self, value: model::ScheduleDateField) -> Self {
        self.schedule_date_field = Some(value);
        self
    }

    #[must_use]
    pub fn schedule_date_custom_field(mut self, value: i32) -> Self {
        self.schedule_date_custom_field = Some(value);
        self
    }
}
