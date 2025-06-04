use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct List {
    full_perms: bool,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

#[must_use]
pub fn list() -> List {
    List::new()
}

impl Default for List {
    fn default() -> Self {
        Self {
            full_perms: true,
            page: None,
            page_size: None,
        }
    }
}

impl List {
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
