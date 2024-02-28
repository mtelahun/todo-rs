use todo_rs::{
    startup::{build, serve},
    state::AppState,
};

use todo_rs::infrastructure::data::db_context::surreal_context::connect_db;

#[tokio::main]
async fn main() {
    let db = connect_db("127.0.0.1:8000", "root", "root", "test", "todo")
        .await
        .expect("failed to connect to database");
    let state = AppState::new(db);

    let (app, listener) = build(None).await;
    let app = app.with_state(state);

    serve(listener, app)
        .await
        .expect("failed to start serving the application")
}
