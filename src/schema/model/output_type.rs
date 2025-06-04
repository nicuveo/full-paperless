use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OutputType {
    #[serde(rename = "pdf")]
    Pdf,
    #[serde(rename = "pdfa")]
    Pdfa,
    #[serde(rename = "pdfa-1")]
    Pdfa1,
    #[serde(rename = "pdfa-2")]
    Pdfa2,
    #[serde(rename = "pdfa-3")]
    Pdfa3,
}
