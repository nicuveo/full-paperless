use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DisplayMode {
    #[serde(rename = "table")]
    Table,
    #[serde(rename = "smallCards")]
    SmallCards,
    #[serde(rename = "largeCards")]
    LargeCards,
}
