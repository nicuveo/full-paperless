use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
pub enum PdfLayout {
    SystemDefault = 0,
    TextThenHtml = 1,
    HtmlThenText = 2,
    HtmlOnly = 3,
    TextOnly = 4,
}
