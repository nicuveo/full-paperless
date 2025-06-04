use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
pub enum ConsumptionScope {
    OnlyProcessAttachments = 1,
    ProcessFullMail = 2,
    ProcessFullMailAndAttachmentsAsSeparateDocuments = 3,
}
