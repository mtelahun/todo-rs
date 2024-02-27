use axum::{response::IntoResponse, Json};

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Working fine, thanks!";
    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE,
    });

    Json(json_response)
}
