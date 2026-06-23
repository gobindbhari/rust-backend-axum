
use dotenvy::dotenv;

use crate::module::{db, routes::create_app, structs::AppStore};

mod module;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db = db::connect_db().await;

    println!("\n Database connected! \n");

    let state = AppStore{db};

    let app = create_app().with_state(state);

    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("failed to bind tcp listener");

    println!("\n Server is running on \n\n \t\t http://localhost:3000 \n");

    axum::serve(listener, app).await.expect("failed to start server");
}


