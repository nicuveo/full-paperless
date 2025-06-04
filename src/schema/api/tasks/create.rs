use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::model;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Create {
    pub task_id: String,
    pub task_name: Option<model::TaskName>,
    pub task_file_name: Option<String>,
    pub date_created: Option<String>,
    pub date_done: Option<String>,
    #[serde(rename = "type")]
    pub view_type: Option<model::TaskViewType>,
    pub status: Option<model::TaskStatus>,
    pub result: Option<String>,
    pub acknowledged: Option<bool>,
    pub owner: Option<i32>,
}

#[must_use]
pub fn create(task_id: String) -> Create {
    Create::new(task_id)
}

impl From<&model::TaskView> for Create {
    fn from(item: &model::TaskView) -> Self {
        Self {
            task_id: item.task_id.clone(),
            task_name: item.task_name,
            task_file_name: item.task_file_name.clone(),
            date_created: item.date_created.clone(),
            date_done: item.date_done.clone(),
            view_type: item.view_type,
            status: item.status,
            result: item.result.clone(),
            acknowledged: item.acknowledged,
            owner: item.owner,
        }
    }
}

impl Create {
    #[must_use]
    pub fn new(task_id: String) -> Self {
        Self {
            task_id,
            task_name: None,
            task_file_name: None,
            date_created: None,
            date_done: None,
            view_type: None,
            status: None,
            result: None,
            acknowledged: None,
            owner: None,
        }
    }

    #[must_use]
    pub fn task_name(mut self, value: model::TaskName) -> Self {
        self.task_name = Some(value);
        self
    }

    #[must_use]
    pub fn task_file_name(mut self, value: String) -> Self {
        self.task_file_name = Some(value);
        self
    }

    #[must_use]
    pub fn date_created(mut self, value: String) -> Self {
        self.date_created = Some(value);
        self
    }

    #[must_use]
    pub fn date_done(mut self, value: String) -> Self {
        self.date_done = Some(value);
        self
    }

    #[must_use]
    pub fn view_type(mut self, value: model::TaskViewType) -> Self {
        self.view_type = Some(value);
        self
    }

    #[must_use]
    pub fn status(mut self, value: model::TaskStatus) -> Self {
        self.status = Some(value);
        self
    }

    #[must_use]
    pub fn result(mut self, value: String) -> Self {
        self.result = Some(value);
        self
    }

    #[must_use]
    pub fn acknowledged(mut self, value: bool) -> Self {
        self.acknowledged = Some(value);
        self
    }

    #[must_use]
    pub fn owner(mut self, value: i32) -> Self {
        self.owner = Some(value);
        self
    }
}
