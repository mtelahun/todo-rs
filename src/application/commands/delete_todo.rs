use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{infrastructure::data::repositories::todo::TodoRepository, state::AppState};

pub async fn delete_todo(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let repository = TodoRepository::new();
    let id = id.to_string();
    if repository.get_by_id(id.clone(), &state).await.is_ok() {
        let _ = repository.delete_todo(id.clone(), &state).await.unwrap();

        return Ok(StatusCode::NO_CONTENT);
    }
    let error_response = serde_json::json!({
        "status": "error",
        "message": format!("Todo with ID: {} not found", id)
    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
}
