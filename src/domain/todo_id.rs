use core::ops::Deref;
use serde::{Deserialize, Serialize};
use surreal_id::NewId;
use uuid::Uuid;

use super::models::todo::TBL_TODO;

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct RecordId(Uuid);

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct TodoId(RecordId);

impl RecordId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn parse(input: &str) -> Self {
        Self(Uuid::parse_str(input).unwrap_or_default())
    }
}

impl From<RecordId> for surrealdb::sql::Id {
    fn from(value: RecordId) -> Self {
        surrealdb::sql::Id::String(value.to_string())
    }
}

impl From<surrealdb::sql::Id> for RecordId {
    fn from(value: surrealdb::sql::Id) -> Self {
        let input = match value {
            surrealdb::sql::Id::String(s) => s,
            _ => Uuid::default().to_string(),
        };

        Self::parse(&input)
    }
}

impl Deref for RecordId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl NewId for TodoId {
    const TABLE: &'static str = TBL_TODO;

    fn from_inner_id<T: Into<surrealdb::sql::Id>>(inner_id: T) -> Self {
        TodoId(RecordId::from(inner_id.into()))
    }

    fn get_inner_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<TodoId> for surrealdb::sql::Id {
    fn from(value: TodoId) -> Self {
        surrealdb::sql::Id::String(value.to_string())
    }
}

impl std::fmt::Display for RecordId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for TodoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
