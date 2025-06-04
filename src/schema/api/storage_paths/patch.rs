use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::model;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Patch {
    pub name: Option<String>,
    pub path: Option<String>,
    #[serde(rename = "match")]
    pub matches: Option<String>,
    pub matching_algorithm: Option<model::MatchingAlgorithm>,
    pub is_insensitive: Option<bool>,
    pub owner: Option<i32>,
    pub set_permissions: Option<model::Permissions>,
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
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    #[must_use]
    pub fn path(mut self, value: String) -> Self {
        self.path = Some(value);
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
