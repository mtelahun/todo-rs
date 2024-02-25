use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::{domain::models::todo::ToDo, infrastructure::data::repositories::todo::TodoRepository};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateTodoRequest {
    pub title: String,
    pub content: String,
    pub completed: Option<bool>,
}

pub async fn update_todo(
    Path(id): Path<String>,
    Json(body): Json<UpdateTodoRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let repository = TodoRepository::new();

    match repository.get_by_id(id.clone()).await {
        Ok(todo) => {
            let datetime = Local::now();
            let title = body.title.to_owned();
            let content = body.content.to_owned();
            let completed = body.completed.unwrap_or(todo.completed.unwrap());
            let payload = ToDo {
                id: todo.id.to_owned(),
                title: if !title.is_empty() {
                    title
                } else {
                    todo.title.to_owned()
                },
                content: if !content.is_empty() {
                    content
                } else {
                    todo.content.to_owned()
                },
                completed: Some(completed),
                createdAt: todo.createdAt,
                updatedAt: Some(datetime),
            };

            let todo_response = repository
                .update_todo(payload)
                .await
                .unwrap();
            let json_response = serde_json::json!({
                "status": "success",
                "data": todo_response,
            });

            Ok((StatusCode::OK, Json(json_response)))
        },
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Todo with ID: {} not found", id),
            });

            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}