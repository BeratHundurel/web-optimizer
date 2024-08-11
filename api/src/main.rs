use axum::Router;
use crate::routes::image_routes;

mod routes;
mod handlers;
mod services;
mod errors;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(image_routes::router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
