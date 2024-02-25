use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::Local;

use crate::{domain::models::todo::ToDo, infrastructure::data::repositories::todo::TodoRepository};

pub async fn create_todo(
    Json(mut body): Json<ToDo>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let repository = TodoRepository::new();

    if let Ok(todo) = repository.get_by_title(body.title.clone()).await {
        let json_response = serde_json::json!({
            "status": "error",
            "message": "Todo already exists",
            "data": todo,
        });

        return Err((StatusCode::BAD_REQUEST, Json(json_response)))
    }

    let datetime = Local::now();
    body.id = Some("some-id".to_string());
    body.completed = Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    let todo = body.to_owned();
    let todo = repository
        .create_todo(todo)
        .await
        .unwrap()[0]
        .to_owned();

    let json_reponse = serde_json::json!({
        "status": "success".to_string(),
        "data": todo,
    });

    Ok((StatusCode::CREATED, Json(json_reponse)))
}