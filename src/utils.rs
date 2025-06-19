use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Method {
    GET,
    PUT,
    POST,
    PATCH,
    DELETE,
}

#[must_use]
pub fn extract_params(url: &Url) -> Vec<(String, String)> {
    url.query_pairs()
        .into_iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect::<Vec<_>>()
}

pub mod body {
    pub const NONE: Option<&String> = None;
}

pub mod params {
    pub const NONE: &Vec<(String, String)> = &Vec::new();
}
