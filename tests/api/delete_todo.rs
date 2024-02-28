use serde::{Deserialize, Serialize};
use surreal_id::NewId;
use todo_rs::domain::models::todo::ToDo;

use crate::test_state::TestState;

#[derive(Clone, Debug, Serialize)]
struct CreatePost {
    title: String,
    content: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateResponse {
    #[allow(dead_code)]
    status: String,
    data: ToDo,
}

#[tokio::test]
async fn delete_happy_path() {
    // Arrange
    let state = TestState::new().await;
    let json_data = serde_json::json!(CreatePost {
        title: "My first to-do".to_string(),
        content: "The content of my first to-do".to_string(),
    });
    let response = state
        .api_client
        .post(format!("http://{}/api/todos", state.app_address))
        .json(&json_data)
        .send()
        .await
        .expect("failed to send request to api end-point");
    assert_eq!(response.status(), 201, "successfull POST response");
    let response_json = response
        .json::<CreateResponse>()
        .await
        .expect("failed to deserialize response");
    let todo_id = response_json.data.id.unwrap();

    // Act
    let response = state
        .api_client
        .delete(format!(
            "http://{}/api/todos/{}",
            state.app_address,
            todo_id.id_without_brackets()
        ))
        .send()
        .await
        .expect("failed to delete todo");
    let response_status = response.status();
    let unavailable = state
        .api_client
        .get(format!(
            "http://{}/api/todos/{}",
            state.app_address, todo_id
        ))
        .send()
        .await
        .expect("failed to delete todo");
    let unavailable_status = unavailable.status();

    // Assert
    assert_eq!(response_status, 204, "successfull DELETE response");
    assert_eq!(unavailable_status, 503, "deleted todo is no longer there");
}
