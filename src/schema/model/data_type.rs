use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DataType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "monetary")]
    Monetary,
    #[serde(rename = "documentlink")]
    Documentlink,
    #[serde(rename = "select")]
    Select,
}
