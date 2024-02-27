use surrealdb::{engine::remote::ws::Client, Result as SurrealResult, Surreal};
use todo_rs::infrastructure::data::db_context::surreal_context::connect_db;

pub async fn connect_to_db() -> SurrealResult<Surreal<Client>> {
    let ns = format!("test_{}", random_name());
    let db = uuid::Uuid::new_v4().to_string();
    let addr = format!("127.0.0.1:8000");
    let client = connect_db(&addr, "root", "root", &ns, &db)
        .await
        .expect("failed to connect to surrealdb");

    Ok(client)
}

pub fn random_name() -> String {
    let mut no = String::from("");
    for _ in 0..6 {
        no.push(rand::random())
    }

    no
}
