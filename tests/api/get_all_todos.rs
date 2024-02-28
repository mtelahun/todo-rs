use todo_rs::domain::todo_id::TodoId;

use crate::test_state::TestState;

#[tokio::test]
async fn get_all_happy_path() {
    // Arrange
    let state = TestState::new().await;
    let res = state
        .create_todo("To-do #1", "The content of my to-do")
        .await;
    let id1 = res.data.id.unwrap();
    let res = state
        .create_todo("To-do #2", "The content of my to-do")
        .await;
    let id2 = res.data.id.unwrap();
    let res = state
        .create_todo("To-do #3", "The content of my to-do")
        .await;
    let id3 = res.data.id.unwrap();

    // Act
    let (status, results, todos) = state.get_all_todos().await;

    // Assert
    assert_eq!(status, "success", "successful response from app");
    assert_eq!(results, 3, "3 results in the response");
    assert_eq!(todos.len(), 3, "3 ToDo(s) in the database");
    let ids = todos
        .iter()
        .map(|t| t.id.clone().unwrap())
        .collect::<Vec<TodoId>>();
    let created_ids = vec![id1, id2, id3];
    for id in created_ids.iter() {
        assert!(
            ids.iter().any(|el| *el == *id),
            "the list of todos from the app contains the right ids"
        )
    }
}
