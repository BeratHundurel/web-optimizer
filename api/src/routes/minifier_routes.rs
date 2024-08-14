use axum::{routing::post, Router};

use crate::handlers::minifier_handler;

pub fn router() -> Router {
    Router::new()
        .route("/minify-css", post(minifier_handler::handle_minify_css))
        .route("/minify-js", post(minifier_handler::handle_minify_js))
}
