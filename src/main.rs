use axum::http::{header, HeaderValue, Method};
use todo_rs::api::create_router;
use tower_http::cors::CorsLayer;

use todo_rs::infrastructure::data::db_context::surreal_context::connect_db;

#[tokio::main]
async fn main() {
    connect_db().await.expect("failed to connect to database");
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE]);

    let app = create_router().layer(cors);

    let addr = "127.0.0.1:8080";
    println!("Server listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind to port: 127.0.0.1:8080");

    axum::serve(listener, app)
        .await
        .expect("failed to start the server");
}
