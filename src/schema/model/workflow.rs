use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[readonly::make]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    #[readonly]
    pub id: i32,
    pub name: String,
    pub(crate) triggers: Vec<super::WorkflowTrigger>,
    pub(crate) actions: Vec<super::WorkflowAction>,
    pub order: Option<i32>,
    pub enabled: Option<bool>,
}

impl Workflow {
    #[must_use]
    pub fn triggers(&self) -> &[super::WorkflowTrigger] {
        &self.triggers
    }

    #[must_use]
    pub fn actions(&self) -> &[super::WorkflowAction] {
        &self.actions
    }
}
