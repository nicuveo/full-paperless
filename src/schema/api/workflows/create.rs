use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::action;
use super::trigger;
use crate::schema::model;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Create {
    pub name: String,
    pub triggers: Vec<trigger::Create>,
    pub actions: Vec<action::Create>,
    pub order: Option<i32>,
    pub enabled: Option<bool>,
}

#[must_use]
pub fn create(name: String) -> Create {
    Create::new(name)
}

impl From<&model::Workflow> for Create {
    fn from(item: &model::Workflow) -> Self {
        Self {
            name: item.name.clone(),
            triggers: item.triggers.iter().map(trigger::Create::from).collect(),
            actions: item.actions.iter().map(action::Create::from).collect(),
            order: item.order,
            enabled: item.enabled,
        }
    }
}

impl Create {
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            order: None,
            enabled: None,
            triggers: Vec::new(),
            actions: Vec::new(),
        }
    }

    #[must_use]
    pub fn trigger(mut self, trigger: trigger::Create) -> Self {
        self.triggers.push(trigger);
        self
    }

    #[must_use]
    pub fn triggers(mut self, triggers: Vec<trigger::Create>) -> Self {
        self.triggers = triggers;
        self
    }

    #[must_use]
    pub fn action(mut self, action: action::Create) -> Self {
        self.actions.push(action);
        self
    }

    #[must_use]
    pub fn actions(mut self, actions: Vec<action::Create>) -> Self {
        self.actions = actions;
        self
    }

    #[must_use]
    pub fn order(mut self, value: i32) -> Self {
        self.order = Some(value);
        self
    }

    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
}
