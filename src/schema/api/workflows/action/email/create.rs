use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::model;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Create {
    pub subject: String,
    pub body: String,
    pub to: String,
    pub include_document: Option<bool>,
}

#[must_use]
pub fn create(subject: String, body: String, to: String) -> Create {
    Create::new(subject, body, to)
}

impl From<&model::WorkflowActionEmail> for Create {
    fn from(item: &model::WorkflowActionEmail) -> Self {
        Self {
            subject: item.subject.clone(),
            body: item.body.clone(),
            to: item.to.clone(),
            include_document: item.include_document,
        }
    }
}

impl Create {
    #[must_use]
    pub fn new(subject: String, body: String, to: String) -> Self {
        Self {
            subject,
            body,
            to,
            include_document: None,
        }
    }

    #[must_use]
    pub fn include_document(mut self, value: bool) -> Self {
        self.include_document = Some(value);
        self
    }
}
