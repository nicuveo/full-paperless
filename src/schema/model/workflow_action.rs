use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[readonly::make]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowAction {
    #[readonly]
    pub id: i32,
    #[serde(rename = "type")]
    pub action_type: Option<super::WorkflowActionType>,
    pub assign_title: Option<String>,
    pub assign_tags: Option<Vec<i32>>,
    pub assign_correspondent: Option<i32>,
    pub assign_document_type: Option<i32>,
    pub assign_storage_path: Option<i32>,
    pub assign_owner: Option<i32>,
    pub assign_view_users: Option<Vec<i32>>,
    pub assign_view_groups: Option<Vec<i32>>,
    pub assign_change_users: Option<Vec<i32>>,
    pub assign_change_groups: Option<Vec<i32>>,
    pub assign_custom_fields: Option<Vec<i32>>,
    pub assign_custom_fields_values: Option<serde_json::Value>,
    pub remove_all_tags: Option<bool>,
    pub remove_tags: Option<Vec<i32>>,
    pub remove_all_correspondents: Option<bool>,
    pub remove_correspondents: Option<Vec<i32>>,
    pub remove_all_document_types: Option<bool>,
    pub remove_document_types: Option<Vec<i32>>,
    pub remove_all_storage_paths: Option<bool>,
    pub remove_storage_paths: Option<Vec<i32>>,
    pub remove_custom_fields: Option<Vec<i32>>,
    pub remove_all_custom_fields: Option<bool>,
    pub remove_all_owners: Option<bool>,
    pub remove_owners: Option<Vec<i32>>,
    pub remove_all_permissions: Option<bool>,
    pub remove_view_users: Option<Vec<i32>>,
    pub remove_view_groups: Option<Vec<i32>>,
    pub remove_change_users: Option<Vec<i32>>,
    pub remove_change_groups: Option<Vec<i32>>,
    pub email: Option<super::WorkflowActionEmail>,
    pub webhook: Option<super::WorkflowActionWebhook>,
}
