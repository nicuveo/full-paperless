use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct List {
    #[serde(rename = "name__icontains")]
    pub username_icontains: Option<String>,
    #[serde(rename = "name__iendswith")]
    pub username_iendswith: Option<String>,
    #[serde(rename = "name__iexact")]
    pub username_iexact: Option<String>,
    #[serde(rename = "name__istartswith")]
    pub username_istartswith: Option<String>,
    pub ordering: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

#[must_use]
pub fn list() -> List {
    List::new()
}

impl List {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn username_icontains(mut self, value: String) -> Self {
        self.username_icontains = Some(value);
        self
    }

    #[must_use]
    pub fn username_iendswith(mut self, value: String) -> Self {
        self.username_iendswith = Some(value);
        self
    }

    #[must_use]
    pub fn username_iexact(mut self, value: String) -> Self {
        self.username_iexact = Some(value);
        self
    }

    #[must_use]
    pub fn username_istartswith(mut self, value: String) -> Self {
        self.username_istartswith = Some(value);
        self
    }

    #[must_use]
    pub fn ordering(mut self, value: String) -> Self {
        self.ordering = Some(value);
        self
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
