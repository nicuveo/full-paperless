use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
pub enum MailRuleAction {
    Delete = 1,
    MoveToFolder = 2,
    MarkAsRead = 3,
    FlagTheMail = 4,
    TagTheMail = 5,
}
