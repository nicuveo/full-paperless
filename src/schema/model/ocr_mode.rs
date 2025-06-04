use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OcrMode {
    #[serde(rename = "skip")]
    Skip,
    #[serde(rename = "redo")]
    Redo,
    #[serde(rename = "force")]
    Force,
    #[serde(rename = "skip_noarchive")]
    SkipNoarchive,
}
