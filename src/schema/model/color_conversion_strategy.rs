use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ColorConversionStrategy {
    LeaveColorUnchanged,
    #[serde(rename = "RGB")]
    Rgb,
    UseDeviceIndependentColor,
    Gray,
    #[serde(rename = "CMYK")]
    Cmyk,
}
