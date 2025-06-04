use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SkipArchiveFile {
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "with_text")]
    WithText,
    #[serde(rename = "always")]
    Always,
}
