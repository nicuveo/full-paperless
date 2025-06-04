use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::model;
use crate::schema::utils;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Patch {
    pub name: Option<String>,
    pub show_on_dashboard: Option<bool>,
    pub show_in_sidebar: Option<bool>,
    pub sort_field: Option<String>,
    pub sort_reverse: Option<bool>,
    #[serde(
        with = "utils::saved_view_filter_rules",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub filter_rules: Vec<(model::RuleType, String)>,
    pub page_size: Option<i64>,
    pub display_mode: Option<model::DisplayMode>,
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
    pub fn show_on_dashboard(mut self, value: bool) -> Self {
        self.show_on_dashboard = Some(value);
        self
    }

    #[must_use]
    pub fn show_in_sidebar(mut self, value: bool) -> Self {
        self.show_in_sidebar = Some(value);
        self
    }

    #[must_use]
    pub fn sort_field(mut self, value: String) -> Self {
        self.sort_field = Some(value);
        self
    }

    #[must_use]
    pub fn sort_reverse(mut self, value: bool) -> Self {
        self.sort_reverse = Some(value);
        self
    }

    #[must_use]
    pub fn filter_rules(mut self, value: Vec<(model::RuleType, String)>) -> Self {
        self.filter_rules = value;
        self
    }

    #[must_use]
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    #[must_use]
    pub fn display_mode(mut self, value: model::DisplayMode) -> Self {
        self.display_mode = Some(value);
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
