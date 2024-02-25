use surrealdb::Error;
use surrealdb::err::Error::Thrown;
use crate::domain::models::todo::ToDo;
use crate::infrastructure::data::db_context::surreal_context::DB;

pub struct TodoRepository {
    table: String,
}

impl TodoRepository {
    pub fn new() -> Self {
        TodoRepository {
            table: String::from("todo"),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<ToDo>, Error> {
        let records = DB.select(&self.table).await?;

        Ok(records)
    }

    pub async fn get_by_id(&self, id: String) -> Result<ToDo, Error> {
        if let Some(record) = DB.select((&self.table, id.clone())).await? {
            return Ok(record)
        }
        let error = Error::Db(
            Thrown(
                format!("Todo with id {} not found", id)
            )
        );

        Err(error)
    }

    pub async fn get_by_title(&self, title: String) -> Result<ToDo, Error> {
        if let Some(record) = DB
            .query("SELECT * FROM todo WHERE title = $title")
            .bind(("title", title.clone()))
            .await?
            .take(0)?
        {
            return Ok(record)
        }
        let error = Error::Db(Thrown(format!("Todo with title {} not found", title)));

        Err(error)
    }

    pub async fn create_todo(&self, content: ToDo) -> Result<Vec<ToDo>, Error> {
        let record = DB
            .create(&self.table)
            .content(content)
            .await?;

        Ok(record)
    }

    pub async fn update_todo(&self, content: ToDo) -> Result<ToDo, Error> {
        let record = DB
            .update((&self.table, content.id.clone().unwrap()))
            .content(content)
            .await?
            .unwrap();

        Ok(record)
    }

    pub async fn delete_todo(&self, id: String) -> Result<ToDo, Error> {
        let result = DB
            .delete((&self.table, id))
            .await?
            .unwrap();

        Ok(result)
    }
}