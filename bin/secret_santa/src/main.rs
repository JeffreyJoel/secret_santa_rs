use axum;
use server::app;

#[tokio::main]
async fn main() {
    let app = app();

    // Bind to address and serve
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Server running at {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
