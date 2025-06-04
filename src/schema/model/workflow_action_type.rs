use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
pub enum WorkflowActionType {
    Assignment = 1,
    Removal = 2,
    Email = 3,
    Webhook = 4,
}
