use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
pub enum Source {
    ConsumeFolder = 1,
    ApiUpload = 2,
    MailFetch = 3,
    WebUi = 4,
}
