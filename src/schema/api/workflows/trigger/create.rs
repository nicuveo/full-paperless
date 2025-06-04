use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::model;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Create {
    #[serde(rename = "type")]
    pub trigger_type: model::WorkflowTriggerType,
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
pub fn create(trigger_type: model::WorkflowTriggerType) -> Create {
    Create::new(trigger_type)
}

impl From<&model::WorkflowTrigger> for Create {
    fn from(item: &model::WorkflowTrigger) -> Self {
        Self {
            trigger_type: item.trigger_type,
            sources: item.sources.clone(),
            filter_path: item.filter_path.clone(),
            filter_filename: item.filter_filename.clone(),
            filter_mailrule: item.filter_mailrule,
            matching_algorithm: item.matching_algorithm,
            matches: item.matches.clone(),
            is_insensitive: item.is_insensitive,
            filter_has_tags: item.filter_has_tags.clone(),
            filter_has_correspondent: item.filter_has_correspondent,
            filter_has_document_type: item.filter_has_document_type,
            schedule_offset_days: item.schedule_offset_days,
            schedule_is_recurring: item.schedule_is_recurring,
            schedule_recurring_interval_days: item.schedule_recurring_interval_days,
            schedule_date_field: item.schedule_date_field,
            schedule_date_custom_field: item.schedule_date_custom_field,
        }
    }
}

impl Create {
    #[must_use]
    pub fn new(trigger_type: model::WorkflowTriggerType) -> Self {
        Self {
            trigger_type,
            sources: None,
            filter_path: None,
            filter_filename: None,
            filter_mailrule: None,
            matching_algorithm: None,
            matches: None,
            is_insensitive: None,
            filter_has_tags: None,
            filter_has_correspondent: None,
            filter_has_document_type: None,
            schedule_offset_days: None,
            schedule_is_recurring: None,
            schedule_recurring_interval_days: None,
            schedule_date_field: None,
            schedule_date_custom_field: None,
        }
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
