use std::net::SocketAddr;

use surrealdb::{engine::remote::ws::Client, Surreal};
use todo_rs::{
    infrastructure::data::db_context::surreal_context::connect_db,
    startup::{build, serve},
    state::AppState,
};

use crate::utils::connect_to_db;

#[derive(Debug)]
pub struct TestState {
    pub db: Surreal<Client>,
    pub app_address: SocketAddr,
    pub api_client: reqwest::Client,
}

impl TestState {
    pub async fn new() -> Self {
        let (app, listener) = build(None).await;
        let app_address = listener.local_addr().unwrap();
        let db = connect_db("http://127.0.0.1:8000/", "root", "root", "test", "todo")
            .await
            .expect("failed to connect to database");
        let state = AppState::new(db);
        let app = app.with_state(state);
        std::thread::spawn(|| async move {
            serve(listener, app)
                .await
                .expect("failed to launch the application");
        });

        let api_client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();

        Self {
            app_address,
            api_client,
            db: connect_to_db()
                .await
                .expect("failed to get db for test state"),
        }
    }
}
