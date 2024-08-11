use axum::response::IntoResponse;
use axum::extract::Multipart;
use axum::http::{StatusCode, HeaderValue};
use axum::response::Response;
use crate::services::image_service;

pub async fn convert_images(mut multipart: Multipart) -> impl IntoResponse {
    match image_service::process_images(&mut multipart).await {
        Ok(zip_data) => {
            Response::builder()
                .header(axum::http::header::CONTENT_TYPE, HeaderValue::from_static("application/zip"))
                .header(axum::http::header::CONTENT_DISPOSITION, HeaderValue::from_static("attachment; filename=\"images.zip\""))
                .body(axum::body::Body::from(zip_data))
                .unwrap()
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}