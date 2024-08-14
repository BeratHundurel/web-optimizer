use axum::Json;
use minifier::css::minify as minify_css;
use minifier::js::minify as minify_js;

use crate::errors::AppError;

pub async fn handle_minify_js(js: String) -> Json<String> {
    let minified_js = minify_js(&js).to_string();
    Json(minified_js)
}

pub async fn handle_minify_css(css: String) -> Json<String> {
    let minified_css = minify_css(&css)
        .map_err(|_| AppError::MinifyError)
        .unwrap()
        .to_string();
    Json(minified_css)
}
