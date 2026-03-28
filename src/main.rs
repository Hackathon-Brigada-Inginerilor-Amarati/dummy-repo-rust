use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // Definirea rutei principale
    let app = Router::new().route("/", get(|| async { "Hello World\n" }));

    // Pornirea serverului
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
