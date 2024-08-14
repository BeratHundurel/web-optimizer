use minifier::css::minify as minify_css;
use minifier::js::minify as minify_js;

use crate::errors::AppError;

pub async fn handle_minify_js(js: String) -> String {
    let minified_js = minify_js(&js).to_string();
    minified_js
}

pub async fn handle_minify_css(css: String) -> String {
    let minified_css = minify_css(&css)
        .map_err(|_| AppError::MinifyError)
        .unwrap()
        .to_string();
    minified_css
}
