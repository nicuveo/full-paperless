use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FileVersion {
    #[serde(rename = "archive")]
    Archive,
    #[serde(rename = "original")]
    Original,
}
