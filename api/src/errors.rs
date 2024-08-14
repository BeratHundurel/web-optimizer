use axum::extract::multipart::MultipartError;
use thiserror::Error;
use tokio::sync::AcquireError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Missing file name in multipart form-data.")]
    MissingFileName,
    #[error("Failed to read data from the field.")]
    FailedToReadData,
    #[error("Invalid image format encountered.")]
    InvalidImageFormat,
    #[error("Image conversion to WebP failed.")]
    ConversionFailed,
    #[error("No images were processed.")]
    NoImagesProcessed,
    #[error("Failed to start a new file in the ZIP archive.")]
    ZipStartError,
    #[error("Failed to write data to the ZIP archive.")]
    ZipWriteError,
    #[error("Failed to finish the ZIP archive.")]
    ZipFinishError,
    #[error("An error occurred while processing the multipart form-data.")]
    ProcessingError,
    #[error("Failed to load image.")]
    LoadError,
    #[error("Failed to minify the CSS or JS.")]
    MinifyError,
    #[error("Multipart error: {0}")]
    MultipartError(#[from] MultipartError),
    #[error("Acquire error: {0}")]
    AcquireError(#[from] AcquireError),
}
