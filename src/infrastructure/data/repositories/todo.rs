use crate::domain::models::todo::ToDo;
use crate::state::AppState;
use surrealdb::err::Error::Thrown;
use surrealdb::Error;

#[derive(Debug, Default)]
pub struct TodoRepository {
    table: String,
}

impl TodoRepository {
    pub fn new() -> Self {
        TodoRepository {
            table: String::from("todo"),
        }
    }

    pub async fn get_all(&self, state: &AppState) -> Result<Vec<ToDo>, Error> {
        let db = state.db.read().await;
        let records = db.select(&self.table).await?;

        Ok(records)
    }

    pub async fn get_by_id(&self, id: String, state: &AppState) -> Result<ToDo, Error> {
        let db = state.db.read().await;
        if let Some(record) = db.select((&self.table, id.clone())).await? {
            return Ok(record);
        }
        let error = Error::Db(Thrown(format!("Todo with id {} not found", id)));

        Err(error)
    }

    pub async fn get_by_title(&self, title: String, state: &AppState) -> Result<ToDo, Error> {
        let db = state.db.read().await;
        if let Some(record) = db
            .query("SELECT * FROM todo WHERE title = $title")
            .bind(("title", title.clone()))
            .await?
            .take(0)?
        {
            return Ok(record);
        }
        let error = Error::Db(Thrown(format!("Todo with title {} not found", title)));

        Err(error)
    }

    pub async fn create_todo(&self, content: ToDo, state: &AppState) -> Result<Vec<ToDo>, Error> {
        let db = state.db.write().await;
        let record = db.create(&self.table).content(content).await?;

        Ok(record)
    }

    pub async fn update_todo(&self, content: ToDo, state: &AppState) -> Result<ToDo, Error> {
        let db = state.db.write().await;
        let record = db
            .update((&self.table, content.id.clone().unwrap()))
            .content(content)
            .await?
            .unwrap();

        Ok(record)
    }

    pub async fn delete_todo(&self, id: String, state: &AppState) -> Result<ToDo, Error> {
        let db = state.db.write().await;
        let result = db.delete((&self.table, id)).await?.unwrap();

        Ok(result)
    }
}
