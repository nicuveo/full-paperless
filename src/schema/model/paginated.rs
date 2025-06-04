use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Paginated<T> {
    pub count: i32,
    pub results: Vec<T>,
    #[serde(default)]
    pub all: Vec<i32>,
    pub(crate) next: Option<Url>,
    pub(crate) previous: Option<Url>,
}

impl<T> Paginated<T> {
    #[must_use]
    pub fn raw_previous_url(&self) -> Option<&Url> {
        self.previous.as_ref()
    }

    #[must_use]
    pub fn raw_next_url(&self) -> Option<&Url> {
        self.next.as_ref()
    }
}
