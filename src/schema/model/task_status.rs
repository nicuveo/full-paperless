use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TaskStatus {
    #[serde(rename = "FAILURE")]
    Failure,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "RECEIVED")]
    Received,
    #[serde(rename = "RETRY")]
    Retry,
    #[serde(rename = "REVOKED")]
    Revoked,
    #[serde(rename = "STARTED")]
    Started,
    #[serde(rename = "SUCCESS")]
    Success,
}
