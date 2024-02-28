use surreal_id::NewId;

use crate::test_state::TestState;

#[tokio::test]
async fn delete_happy_path() {
    // Arrange
    let state = TestState::new().await;
    let response_json = state
        .create_todo("My first to-do", "The content of my first to-do")
        .await;
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
