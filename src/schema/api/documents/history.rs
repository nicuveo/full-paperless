use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct History {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

#[must_use]
pub fn history() -> History {
    History::new()
}

impl History {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn page(mut self, value: i32) -> Self {
        self.page = Some(value);
        self
    }

    #[must_use]
    pub fn page_size(mut self, value: i32) -> Self {
        self.page_size = Some(value);
        self
    }
}
