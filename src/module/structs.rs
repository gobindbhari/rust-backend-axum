use sea_orm::Database;
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub name: String,
    pub country: String,
}

#[derive(clone)]
pub struct AppStore {
    pub db: DatabaseConnection
}