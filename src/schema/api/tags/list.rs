use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::utils;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct List {
    full_perms: bool,
    pub id: Option<i32>,
    #[serde(rename = "id__in", with = "utils::comma_list")]
    pub id_in: Option<Vec<i32>>,
    #[serde(rename = "name__icontains")]
    pub name_icontains: Option<String>,
    #[serde(rename = "name__iendswith")]
    pub name_iendswith: Option<String>,
    #[serde(rename = "name__iexact")]
    pub name_iexact: Option<String>,
    #[serde(rename = "name__istartswith")]
    pub name_istartswith: Option<String>,
    #[serde(rename = "path__icontains")]
    pub path_icontains: Option<String>,
    #[serde(rename = "path__iendswith")]
    pub path_iendswith: Option<String>,
    #[serde(rename = "path__iexact")]
    pub path_iexact: Option<String>,
    #[serde(rename = "path__istartswith")]
    pub path_istartswith: Option<String>,
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
        Self {
            full_perms: true,
            id: None,
            id_in: None,
            name_icontains: None,
            name_iendswith: None,
            name_iexact: None,
            name_istartswith: None,
            path_icontains: None,
            path_iendswith: None,
            path_iexact: None,
            path_istartswith: None,
            ordering: None,
            page: None,
            page_size: None,
        }
    }

    #[must_use]
    pub fn id(mut self, value: i32) -> Self {
        self.id = Some(value);
        self
    }

    #[must_use]
    pub fn id_in(mut self, value: Vec<i32>) -> Self {
        self.id_in = Some(value);
        self
    }

    #[must_use]
    pub fn name_icontains(mut self, value: String) -> Self {
        self.name_icontains = Some(value);
        self
    }

    #[must_use]
    pub fn name_iendswith(mut self, value: String) -> Self {
        self.name_iendswith = Some(value);
        self
    }

    #[must_use]
    pub fn name_iexact(mut self, value: String) -> Self {
        self.name_iexact = Some(value);
        self
    }

    #[must_use]
    pub fn name_istartswith(mut self, value: String) -> Self {
        self.name_istartswith = Some(value);
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
