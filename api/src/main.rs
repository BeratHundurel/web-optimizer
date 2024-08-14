use crate::routes::image_routes;
use axum::Router;
use routes::minifier_routes;
mod errors;
mod handlers;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(image_routes::router())
        .merge(minifier_routes::router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
