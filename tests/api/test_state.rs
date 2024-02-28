use std::{net::SocketAddr, time::Duration};

use surrealdb::{engine::remote::ws::Client, Surreal};
use todo_rs::{
    infrastructure::data::db_context::surreal_context::connect_db,
    startup::{build, serve},
    state::AppState,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct TestState {
    pub db: Surreal<Client>,
    pub app_address: SocketAddr,
    pub api_client: reqwest::Client,
}

impl TestState {
    pub async fn new() -> Self {
        let dbname = format!("{}", Uuid::new_v4());
        let db = connect_db("127.0.0.1:8000", "root", "root", "test", &dbname)
            .await
            .expect("failed to connect to database");
        let state = AppState::new(db.clone());
        let (app, listener) = build(Some("127.0.0.1:0".into())).await;
        let app_address = listener.local_addr().unwrap();
        let app = app.with_state(state);
        tokio::task::spawn(async {
            serve(listener, app)
                .await
                .expect("failed to launch the application");
        });

        // Give the app time to launch and listen for traffic
        tokio::time::sleep(Duration::from_millis(500)).await;

        let api_client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();

        Self {
            app_address,
            api_client,
            db,
        }
    }
}
