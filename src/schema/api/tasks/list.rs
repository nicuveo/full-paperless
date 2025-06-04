use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::model;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct List {
    pub acknowledged: Option<bool>,
    pub ordering: Option<String>,
    pub status: Option<model::TaskStatus>,
    pub task_name: Option<model::TaskName>,
    pub view_type: Option<model::TaskViewType>,
}

#[must_use]
pub fn list() -> List {
    List::new()
}

impl List {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn acknowledged(mut self, value: bool) -> Self {
        self.acknowledged = Some(value);
        self
    }

    #[must_use]
    pub fn ordering(mut self, value: String) -> Self {
        self.ordering = Some(value);
        self
    }

    #[must_use]
    pub fn status(mut self, value: model::TaskStatus) -> Self {
        self.status = Some(value);
        self
    }

    #[must_use]
    pub fn task_name(mut self, value: model::TaskName) -> Self {
        self.task_name = Some(value);
        self
    }

    #[must_use]
    pub fn view_type(mut self, value: model::TaskViewType) -> Self {
        self.view_type = Some(value);
        self
    }
}
