use axum::{
    routing::{get, post},
    Router,
};

use crate::application::{
    commands::{create_todo::create_todo, delete_todo::delete_todo, update_todo::update_todo},
    queries::{get_all_todos::get_all_todos, get_todo_by_id::get_todo_by_id},
};

use self::health_check::health_check_handler;

pub mod health_check;

pub fn create_router() -> Router {
    Router::new()
        .route("/api/health_check", get(health_check_handler))
        .route("/api/todos", post(create_todo).get(get_all_todos))
        .route(
            "/api/todos/:id",
            get(get_todo_by_id).put(update_todo).delete(delete_todo),
        )
}
