use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TaskViewType {
    #[serde(rename = "auto_task")]
    AutoTask,
    #[serde(rename = "scheduled_task")]
    ScheduledTask,
    #[serde(rename = "manual_task")]
    ManualTask,
}
