use std::collections::HashMap;

use axum::{Json, extract::{Path, Query}, response::IntoResponse};
use serde_json::{Value, json};

use crate::module::{enums::ApiError, structs::User};



pub async fn health_handler()-> impl IntoResponse{
    Json(json!({
        "status": "ok",
        "msg": "Server is running",
        "StatusCode": 200,
        "data": [
            {"name": "gobind", "age": 18},
            {"name": "gobind", "age": 18},
            {"name": "gobind", "age": 18},
            {"name": "gobind", "age": 18},
            {"name": "gobind", "age": 18},
        ]
    }))
}

pub async fn list_users()-> Result<Json<Value>, ApiError>{
    let users = vec![
        User {
            name: "gobind".to_string(),
            country: "india".to_string(),
        },
        User {
            name: "john".to_string(),
            country: "usa".to_string(),
        },
    ];
    Ok(Json(json!(users)))
    // Err(ApiError::InternalError)
}

pub async fn dynamic_route(Path(username): Path<String>) -> String {
    let result = format!("hello {username}");
    result
}

pub async fn query(Query(params): Query<HashMap<String, String>>) -> String {
    let result = format!("{:?}", params);
    result
}