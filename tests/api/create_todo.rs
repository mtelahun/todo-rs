use serde::{Deserialize, Serialize};
use todo_rs::domain::models::todo::ToDo;

use crate::test_state::TestState;

#[derive(Clone, Debug, Serialize)]
struct CreatePost {
    title: String,
    content: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateResponse {
    status: String,
    data: ToDo,
}

#[tokio::test]
async fn create_happy_path() {
    // Arrange
    let state = TestState::new().await;
    let json_data = serde_json::json!(CreatePost {
        title: "My first to-do".to_string(),
        content: "The content of my first to-do".to_string(),
    });

    // Act
    let response = state
        .api_client
        .post(format!("http://{}/api/todos", state.app_address))
        .json(&json_data)
        .send()
        .await
        .expect("failed to send request to api end-point");
    let response_status = response.status();
    assert_eq!(response_status, 201, "successfull POST response");
    let response_json = response
        .json::<CreateResponse>()
        .await
        .expect("failed to deserialize response");

    // Assert
    assert_eq!(
        response_json.status, "success",
        "response status is 'success'"
    );
    assert_eq!(
        response_json.data.title, "My first to-do",
        "todo title is same as input"
    );
    assert_eq!(
        response_json.data.content, "The content of my first to-do",
        "todo content is same as input"
    );
    assert!(!response_json.data.completed.unwrap());
}
