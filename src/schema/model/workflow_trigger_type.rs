use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
pub enum WorkflowTriggerType {
    ConsumptionStarted = 1,
    DocumentAdded = 2,
    DocumentUpdated = 3,
    Scheduled = 4,
}
