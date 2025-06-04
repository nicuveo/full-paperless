use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[readonly::make]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MailRule {
    #[readonly]
    pub id: i32,
    #[readonly]
    pub user_can_change: bool,
    pub name: String,
    pub account: i32,
    pub enabled: Option<bool>,
    pub folder: Option<String>,
    pub filter_from: Option<String>,
    pub filter_to: Option<String>,
    pub filter_subject: Option<String>,
    pub filter_body: Option<String>,
    pub filter_attachment_filename_include: Option<String>,
    pub filter_attachment_filename_exclude: Option<String>,
    pub maximum_age: Option<i64>,
    pub action: Option<super::MailRuleAction>,
    pub action_parameter: Option<String>,
    pub assign_title_from: Option<super::AssignTitleFrom>,
    pub assign_tags: Option<Vec<i32>>,
    pub assign_correspondent_from: Option<super::AssignCorrespondentFrom>,
    pub assign_correspondent: Option<i32>,
    pub assign_document_type: Option<i32>,
    pub assign_owner_from_rule: Option<bool>,
    pub order: Option<i32>,
    pub attachment_type: Option<super::AttachmentType>,
    pub consumption_scope: Option<super::ConsumptionScope>,
    pub pdf_layout: Option<super::PdfLayout>,
    pub owner: Option<i32>,
    pub permissions: super::Permissions,
}
