use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a -single--route- two routes
    let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/anotherOne", get(|| async { "another route boom! you looking for this" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8001").await.unwrap();
    println!("server listening on http://0.0.0.0:8001");
    axum::serve(listener, app).await.unwrap();
}