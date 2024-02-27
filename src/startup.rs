use axum::{
    http::{header, HeaderValue, Method},
    Router,
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use crate::{api::create_router, state::AppState};

pub async fn serve(listener: TcpListener, app: Router) -> Result<(), std::io::Error> {
    println!("Server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await
}

pub async fn build(bind_address: Option<String>) -> (Router<AppState>, TcpListener) {
    let router = get_app().await;

    let mut addr = format!("0.0.0.0:{}", 8080);
    if bind_address.is_some() {
        addr = bind_address.unwrap();
    }
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|e| {
            eprintln!("unable to parse local address: {}", e);
        })
        .unwrap();

    (router, listener)
}

pub async fn get_app() -> Router<AppState> {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE]);

    create_router().layer(cors)
}
