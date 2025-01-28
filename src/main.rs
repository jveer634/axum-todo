use axum::{routing::get, Json, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { Json("Hello World") }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on port 3000");
    axum::serve(listener, app).await.unwrap();
}
