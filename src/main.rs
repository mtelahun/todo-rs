use todo_rs::{
    startup::{build, serve},
    state::AppState,
};

use todo_rs::infrastructure::data::db_context::surreal_context::connect_db;

#[tokio::main]
async fn main() {
    let db_host = std::env::var("DATABASE_HOST").unwrap_or("127.0.0.1".into());
    let db_port = std::env::var("DATABASE_PORT").unwrap_or("8000".into());
    let db_url = format!("{}:{}", db_host, db_port);
    let db = connect_db(&db_url, "root", "root", "test", "todo")
        .await
        .expect("failed to connect to database");
    let state = AppState::new(db);

    let (app, listener) = build(None).await;
    let app = app.with_state(state);

    serve(listener, app)
        .await
        .expect("failed to start serving the application")
}
