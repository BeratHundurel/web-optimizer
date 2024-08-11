use axum::{extract::DefaultBodyLimit, routing::post, Router};
use crate::handlers::image_handler;

pub fn router() -> Router {
    Router::new()
        .route("/convert", post(image_handler::convert_images).layer(DefaultBodyLimit::disable()))
}