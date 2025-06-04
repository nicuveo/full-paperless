use serde::{Deserialize, Serialize};

#[readonly::make]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Actor {
    #[readonly]
    pub id: i32,
    pub username: String,
}
