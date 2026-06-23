use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub name: String,
    pub country: String,
}
