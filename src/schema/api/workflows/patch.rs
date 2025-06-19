use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Patch {
    pub name: Option<String>,
    pub order: Option<i32>,
    pub enabled: Option<bool>,
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
    pub fn order(mut self, value: i32) -> Self {
        self.order = Some(value);
        self
    }

    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
}
