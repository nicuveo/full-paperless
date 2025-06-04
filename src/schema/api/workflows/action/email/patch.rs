use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Patch {
    pub subject: String,
    pub body: String,
    pub to: String,
    pub include_document: Option<bool>,
}

#[must_use]
pub fn patch(subject: String, body: String, to: String) -> Patch {
    Patch::new(subject, body, to)
}

impl Patch {
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
