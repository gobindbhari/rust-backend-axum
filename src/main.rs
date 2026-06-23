
use crate::module::routes::create_app;

mod module;

#[tokio::main]
async fn main() {
    let app = create_app();

    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("failed to bind tcp listener");

    println!("\n Server is running on \n\n \t\t http://localhost:3000 \n");

    axum::serve(listener, app).await.expect("failed to start server");
}


