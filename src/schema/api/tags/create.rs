use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::model;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Create {
    pub name: String,
    pub color: Option<String>,
    #[serde(rename = "match")]
    pub matches: Option<String>,
    pub matching_algorithm: Option<model::MatchingAlgorithm>,
    pub is_insensitive: Option<bool>,
    pub is_inbox_tag: Option<bool>,
    pub owner: Option<i32>,
    pub set_permissions: Option<model::Permissions>,
}

#[must_use]
pub fn create(name: String) -> Create {
    Create::new(name)
}

impl From<&model::Tag> for Create {
    fn from(item: &model::Tag) -> Self {
        Self {
            name: item.name.clone(),
            color: item.color.clone(),
            matches: item.matches.clone(),
            matching_algorithm: item.matching_algorithm,
            is_insensitive: item.is_insensitive,
            is_inbox_tag: item.is_inbox_tag,
            owner: item.owner,
            set_permissions: Some(item.permissions.clone()),
        }
    }
}

impl Create {
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            color: None,
            matches: None,
            matching_algorithm: None,
            is_insensitive: None,
            is_inbox_tag: None,
            owner: None,
            set_permissions: None,
        }
    }

    #[must_use]
    pub fn color(mut self, value: String) -> Self {
        self.color = Some(value);
        self
    }

    #[must_use]
    pub fn matches(mut self, value: String) -> Self {
        self.matches = Some(value);
        self
    }

    #[must_use]
    pub fn matching_algorithm(mut self, value: model::MatchingAlgorithm) -> Self {
        self.matching_algorithm = Some(value);
        self
    }

    #[must_use]
    pub fn is_insensitive(mut self, value: bool) -> Self {
        self.is_insensitive = Some(value);
        self
    }

    #[must_use]
    pub fn is_inbox_tag(mut self, value: bool) -> Self {
        self.is_inbox_tag = Some(value);
        self
    }

    #[must_use]
    pub fn owner(mut self, value: i32) -> Self {
        self.owner = Some(value);
        self
    }

    #[must_use]
    pub fn set_permissions(mut self, value: model::Permissions) -> Self {
        self.set_permissions = Some(value);
        self
    }
}
