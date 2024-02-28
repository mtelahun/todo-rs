use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::Local;
use surreal_id::NewId;
use uuid::Uuid;

use crate::{
    domain::{models::todo::ToDo, todo_id::TodoId},
    infrastructure::data::repositories::todo::TodoRepository,
    state::AppState,
};

pub async fn create_todo(
    State(state): State<AppState>,
    Json(mut body): Json<ToDo>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let repository = TodoRepository::new();

    if let Ok(todo) = repository.get_by_title(body.title.clone(), &state).await {
        let json_response = serde_json::json!({
            "status": "error",
            "message": "Todo already exists",
            "data": todo,
        });

        return Err((StatusCode::BAD_REQUEST, Json(json_response)));
    }

    let datetime = Local::now();
    body.id = Some(TodoId::new(Uuid::new_v4().to_string()).unwrap());
    body.completed = Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    let todo = body.to_owned();
    let todo = repository.create_todo(todo, &state).await.unwrap()[0].to_owned();

    let json_reponse = serde_json::json!({
        "status": "success".to_string(),
        "data": todo,
    });

    Ok((StatusCode::CREATED, Json(json_reponse)))
}
