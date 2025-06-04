use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::model;
use crate::schema::utils;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Create {
    pub name: String,
    pub show_on_dashboard: bool,
    pub show_in_sidebar: bool,
    pub sort_field: Option<String>,
    pub sort_reverse: Option<bool>,
    #[serde(with = "utils::saved_view_filter_rules")]
    pub filter_rules: Vec<(model::RuleType, String)>,
    pub display_fields: Option<serde_json::Value>,
    pub page_size: Option<i64>,
    pub display_mode: Option<model::DisplayMode>,
    pub owner: Option<i32>,
}

#[must_use]
pub fn create(
    name: String,
    show_on_dashboard: bool,
    show_in_sidebar: bool,
    filter_rules: Vec<(model::RuleType, String)>,
) -> Create {
    Create::new(name, show_on_dashboard, show_in_sidebar, filter_rules)
}

impl From<&model::SavedView> for Create {
    fn from(item: &model::SavedView) -> Self {
        Self {
            name: item.name.clone(),
            show_on_dashboard: item.show_on_dashboard,
            show_in_sidebar: item.show_in_sidebar,
            sort_field: item.sort_field.clone(),
            sort_reverse: item.sort_reverse,
            filter_rules: item.filter_rules.clone(),
            display_fields: item.display_fields.clone(),
            page_size: item.page_size,
            display_mode: item.display_mode,
            owner: item.owner,
        }
    }
}

impl Create {
    #[must_use]
    pub fn new(
        name: String,
        show_on_dashboard: bool,
        show_in_sidebar: bool,
        filter_rules: Vec<(model::RuleType, String)>,
    ) -> Self {
        Self {
            name,
            show_on_dashboard,
            show_in_sidebar,
            sort_field: None,
            sort_reverse: None,
            filter_rules,
            page_size: None,
            display_mode: None,
            display_fields: None,
            owner: None,
        }
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
    pub fn display_fields(mut self, value: serde_json::Value) -> Self {
        self.display_fields = Some(value);
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
}
