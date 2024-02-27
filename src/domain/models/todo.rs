use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::domain::todo_id::TodoId;

pub const TBL_TODO: &str = "todo";

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ToDo {
    pub id: Option<TodoId>,
    pub title: String,
    pub content: String,
    pub completed: Option<bool>,
    pub createdAt: Option<DateTime<Local>>,
    pub updatedAt: Option<DateTime<Local>>,
}
