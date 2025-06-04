use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::utils;

#[readonly::make]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedView {
    #[readonly]
    pub id: i32,
    #[readonly]
    pub user_can_change: bool,
    pub name: String,
    pub show_on_dashboard: bool,
    pub show_in_sidebar: bool,
    pub sort_field: Option<String>,
    pub sort_reverse: Option<bool>,
    #[serde(with = "utils::saved_view_filter_rules")]
    pub filter_rules: Vec<(super::RuleType, String)>,
    pub page_size: Option<i64>,
    pub display_mode: Option<super::DisplayMode>,
    pub display_fields: Option<serde_json::Value>,
    pub owner: Option<i32>,
}
