use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UnpaperClean {
    #[serde(rename = "clean")]
    Clean,
    #[serde(rename = "clean-final")]
    CleanFinal,
    #[serde(rename = "none")]
    None,
}
