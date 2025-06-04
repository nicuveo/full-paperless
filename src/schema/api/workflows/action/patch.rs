use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::email;
use super::webhook;
use crate::schema::model;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Patch {
    #[serde(rename = "type")]
    pub action_type: Option<model::WorkflowActionType>,
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
    pub email: Option<email::Patch>,
    pub webhook: Option<webhook::Patch>,
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
    pub fn action_type(mut self, value: model::WorkflowActionType) -> Self {
        self.action_type = Some(value);
        self
    }

    #[must_use]
    pub fn assign_title(mut self, value: String) -> Self {
        self.assign_title = Some(value);
        self
    }

    #[must_use]
    pub fn assign_tags(mut self, value: Vec<i32>) -> Self {
        self.assign_tags = Some(value);
        self
    }

    #[must_use]
    pub fn assign_correspondent(mut self, value: i32) -> Self {
        self.assign_correspondent = Some(value);
        self
    }

    #[must_use]
    pub fn assign_document_type(mut self, value: i32) -> Self {
        self.assign_document_type = Some(value);
        self
    }

    #[must_use]
    pub fn assign_storage_path(mut self, value: i32) -> Self {
        self.assign_storage_path = Some(value);
        self
    }

    #[must_use]
    pub fn assign_owner(mut self, value: i32) -> Self {
        self.assign_owner = Some(value);
        self
    }

    #[must_use]
    pub fn assign_view_users(mut self, value: Vec<i32>) -> Self {
        self.assign_view_users = Some(value);
        self
    }

    #[must_use]
    pub fn assign_view_groups(mut self, value: Vec<i32>) -> Self {
        self.assign_view_groups = Some(value);
        self
    }

    #[must_use]
    pub fn assign_change_users(mut self, value: Vec<i32>) -> Self {
        self.assign_change_users = Some(value);
        self
    }

    #[must_use]
    pub fn assign_change_groups(mut self, value: Vec<i32>) -> Self {
        self.assign_change_groups = Some(value);
        self
    }

    #[must_use]
    pub fn assign_custom_fields(mut self, value: Vec<i32>) -> Self {
        self.assign_custom_fields = Some(value);
        self
    }

    #[must_use]
    pub fn assign_custom_fields_values(mut self, value: serde_json::Value) -> Self {
        self.assign_custom_fields_values = Some(value);
        self
    }

    #[must_use]
    pub fn remove_all_tags(mut self, value: bool) -> Self {
        self.remove_all_tags = Some(value);
        self
    }

    #[must_use]
    pub fn remove_tags(mut self, value: Vec<i32>) -> Self {
        self.remove_tags = Some(value);
        self
    }

    #[must_use]
    pub fn remove_all_correspondents(mut self, value: bool) -> Self {
        self.remove_all_correspondents = Some(value);
        self
    }

    #[must_use]
    pub fn remove_correspondents(mut self, value: Vec<i32>) -> Self {
        self.remove_correspondents = Some(value);
        self
    }

    #[must_use]
    pub fn remove_all_document_types(mut self, value: bool) -> Self {
        self.remove_all_document_types = Some(value);
        self
    }

    #[must_use]
    pub fn remove_document_types(mut self, value: Vec<i32>) -> Self {
        self.remove_document_types = Some(value);
        self
    }

    #[must_use]
    pub fn remove_all_storage_paths(mut self, value: bool) -> Self {
        self.remove_all_storage_paths = Some(value);
        self
    }

    #[must_use]
    pub fn remove_storage_paths(mut self, value: Vec<i32>) -> Self {
        self.remove_storage_paths = Some(value);
        self
    }

    #[must_use]
    pub fn remove_custom_fields(mut self, value: Vec<i32>) -> Self {
        self.remove_custom_fields = Some(value);
        self
    }

    #[must_use]
    pub fn remove_all_custom_fields(mut self, value: bool) -> Self {
        self.remove_all_custom_fields = Some(value);
        self
    }

    #[must_use]
    pub fn remove_all_owners(mut self, value: bool) -> Self {
        self.remove_all_owners = Some(value);
        self
    }

    #[must_use]
    pub fn remove_owners(mut self, value: Vec<i32>) -> Self {
        self.remove_owners = Some(value);
        self
    }

    #[must_use]
    pub fn remove_all_permissions(mut self, value: bool) -> Self {
        self.remove_all_permissions = Some(value);
        self
    }

    #[must_use]
    pub fn remove_view_users(mut self, value: Vec<i32>) -> Self {
        self.remove_view_users = Some(value);
        self
    }

    #[must_use]
    pub fn remove_view_groups(mut self, value: Vec<i32>) -> Self {
        self.remove_view_groups = Some(value);
        self
    }

    #[must_use]
    pub fn remove_change_users(mut self, value: Vec<i32>) -> Self {
        self.remove_change_users = Some(value);
        self
    }

    #[must_use]
    pub fn remove_change_groups(mut self, value: Vec<i32>) -> Self {
        self.remove_change_groups = Some(value);
        self
    }

    #[must_use]
    pub fn email(mut self, value: email::Patch) -> Self {
        self.email = Some(value);
        self
    }

    #[must_use]
    pub fn webhook(mut self, value: webhook::Patch) -> Self {
        self.webhook = Some(value);
        self
    }
}
