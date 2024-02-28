use crate::test_state::TestState;

#[tokio::test]
async fn create_happy_path() {
    // Arrange
    let state = TestState::new().await;

    // Act
    let response_json = state
        .create_todo("My first to-do", "The content of my first to-do")
        .await;

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
