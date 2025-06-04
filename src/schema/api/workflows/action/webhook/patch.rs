use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Patch {
    pub url: String,
    pub use_params: Option<bool>,
    pub as_json: Option<bool>,
    pub params: Option<serde_json::Value>,
    pub body: Option<String>,
    pub headers: Option<serde_json::Value>,
    pub include_document: Option<bool>,
}

#[must_use]
pub fn patch(url: String) -> Patch {
    Patch::new(url)
}

impl Patch {
    #[must_use]
    pub fn new(url: String) -> Self {
        Self {
            url,
            use_params: None,
            as_json: None,
            params: None,
            body: None,
            headers: None,
            include_document: None,
        }
    }

    #[must_use]
    pub fn use_params(mut self, value: bool) -> Self {
        self.use_params = Some(value);
        self
    }

    #[must_use]
    pub fn as_json(mut self, value: bool) -> Self {
        self.as_json = Some(value);
        self
    }

    #[must_use]
    pub fn params(mut self, value: serde_json::Value) -> Self {
        self.params = Some(value);
        self
    }

    #[must_use]
    pub fn body(mut self, value: String) -> Self {
        self.body = Some(value);
        self
    }

    #[must_use]
    pub fn headers(mut self, value: serde_json::Value) -> Self {
        self.headers = Some(value);
        self
    }

    #[must_use]
    pub fn include_document(mut self, value: bool) -> Self {
        self.include_document = Some(value);
        self
    }
}
