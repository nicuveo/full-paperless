use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[readonly::make]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskView {
    #[readonly]
    pub id: i32,
    #[readonly]
    pub related_document: Option<String>,
    pub task_id: String,
    pub task_name: Option<super::TaskName>,
    pub task_file_name: Option<String>,
    pub date_created: Option<String>,
    pub date_done: Option<String>,
    #[serde(rename = "type")]
    pub view_type: Option<super::TaskViewType>,
    pub status: Option<super::TaskStatus>,
    pub result: Option<String>,
    pub acknowledged: Option<bool>,
    pub owner: Option<i32>,
}
