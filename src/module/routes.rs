use axum::{Router, routing::get};

use crate::module::{handlers::{dynamic_route, health_handler, list_users, query}, structs::AppStore};

pub fn create_app() -> Router<AppStore> {
    let routes = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/user/{username}", get(dynamic_route))
        .route("/query", get(query))

        .route("/users", get(list_users))
        .route("/health", get(health_handler));

    routes
}
