use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{infrastructure::data::repositories::todo::TodoRepository, state::AppState};

pub async fn get_todo_by_id(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let repository = TodoRepository::new();
    let id = id.to_string();

    if let Ok(todo) = repository.get_by_id(id.clone(), &state).await {
        return Ok(Json(todo));
    }
    let error_response = serde_json::json!({
        "status": "error",
        "message": format!("failed to get Todo with ID: {}", id),
    });

    Err((StatusCode::SERVICE_UNAVAILABLE, Json(error_response)))
}
