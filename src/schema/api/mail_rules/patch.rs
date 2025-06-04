use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::model;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Patch {
    pub name: Option<String>,
    pub account: Option<i32>,
    pub enabled: Option<bool>,
    pub folder: Option<String>,
    pub filter_from: Option<String>,
    pub filter_to: Option<String>,
    pub filter_subject: Option<String>,
    pub filter_body: Option<String>,
    pub filter_attachment_filename_include: Option<String>,
    pub filter_attachment_filename_exclude: Option<String>,
    pub maximum_age: Option<i64>,
    pub action: Option<model::MailRuleAction>,
    pub action_parameter: Option<String>,
    pub assign_title_from: Option<model::AssignTitleFrom>,
    pub assign_tags: Option<Vec<i32>>,
    pub assign_correspondent_from: Option<model::AssignCorrespondentFrom>,
    pub assign_correspondent: Option<i32>,
    pub assign_document_type: Option<i32>,
    pub assign_owner_from_rule: Option<bool>,
    pub order: Option<i32>,
    pub attachment_type: Option<model::AttachmentType>,
    pub consumption_scope: Option<model::ConsumptionScope>,
    pub pdf_layout: Option<model::PdfLayout>,
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
    pub fn account(mut self, value: i32) -> Self {
        self.account = Some(value);
        self
    }

    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    #[must_use]
    pub fn folder(mut self, value: String) -> Self {
        self.folder = Some(value);
        self
    }

    #[must_use]
    pub fn filter_from(mut self, value: String) -> Self {
        self.filter_from = Some(value);
        self
    }

    #[must_use]
    pub fn filter_to(mut self, value: String) -> Self {
        self.filter_to = Some(value);
        self
    }

    #[must_use]
    pub fn filter_subject(mut self, value: String) -> Self {
        self.filter_subject = Some(value);
        self
    }

    #[must_use]
    pub fn filter_body(mut self, value: String) -> Self {
        self.filter_body = Some(value);
        self
    }

    #[must_use]
    pub fn filter_attachment_filename_include(mut self, value: String) -> Self {
        self.filter_attachment_filename_include = Some(value);
        self
    }

    #[must_use]
    pub fn filter_attachment_filename_exclude(mut self, value: String) -> Self {
        self.filter_attachment_filename_exclude = Some(value);
        self
    }

    #[must_use]
    pub fn maximum_age(mut self, value: i64) -> Self {
        self.maximum_age = Some(value);
        self
    }

    #[must_use]
    pub fn action(mut self, value: model::MailRuleAction) -> Self {
        self.action = Some(value);
        self
    }

    #[must_use]
    pub fn action_parameter(mut self, value: String) -> Self {
        self.action_parameter = Some(value);
        self
    }

    #[must_use]
    pub fn assign_title_from(mut self, value: model::AssignTitleFrom) -> Self {
        self.assign_title_from = Some(value);
        self
    }

    #[must_use]
    pub fn assign_tags(mut self, value: Vec<i32>) -> Self {
        self.assign_tags = Some(value);
        self
    }

    #[must_use]
    pub fn assign_correspondent_from(mut self, value: model::AssignCorrespondentFrom) -> Self {
        self.assign_correspondent_from = Some(value);
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
    pub fn assign_owner_from_rule(mut self, value: bool) -> Self {
        self.assign_owner_from_rule = Some(value);
        self
    }

    #[must_use]
    pub fn order(mut self, value: i32) -> Self {
        self.order = Some(value);
        self
    }

    #[must_use]
    pub fn attachment_type(mut self, value: model::AttachmentType) -> Self {
        self.attachment_type = Some(value);
        self
    }

    #[must_use]
    pub fn consumption_scope(mut self, value: model::ConsumptionScope) -> Self {
        self.consumption_scope = Some(value);
        self
    }

    #[must_use]
    pub fn pdf_layout(mut self, value: model::PdfLayout) -> Self {
        self.pdf_layout = Some(value);
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
