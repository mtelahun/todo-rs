use serde::{Deserialize, Serialize};
use surreal_id::NewId;
use surrealdb::opt::RecordId;

use super::models::todo::TBL_TODO;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TodoId(RecordId);

impl NewId for TodoId {
    const TABLE: &'static str = TBL_TODO;

    fn from_inner_id<T: Into<surrealdb::sql::Id>>(inner_id: T) -> Self {
        TodoId(RecordId {
            tb: Self::TABLE.to_string(),
            id: inner_id.into(),
        })
    }

    fn get_inner_string(&self) -> String {
        self.0.id.to_string()
    }
}

impl From<TodoId> for surrealdb::sql::Id {
    fn from(value: TodoId) -> Self {
        surrealdb::sql::Id::String(value.to_string())
    }
}

impl std::fmt::Display for TodoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
