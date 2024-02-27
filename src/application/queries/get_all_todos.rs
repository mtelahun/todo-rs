use axum::{extract::State, response::IntoResponse, Json};

use crate::infrastructure::data::repositories::todo::TodoRepository;
use crate::state::AppState;

pub async fn get_all_todos(State(state): State<AppState>) -> impl IntoResponse {
    let repository = TodoRepository::new();

    let mut todos = Vec::new();
    if let Ok(result) = repository.get_all(&state).await {
        todos = result;
    }
    let json_response = serde_json::json!({
        "status": "success",
        "results": todos.len(),
        "todos": todos,
    });

    Json(json_response)
}
