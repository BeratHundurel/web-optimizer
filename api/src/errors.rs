use std::{error::Error, fmt};

use axum::extract::multipart::MultipartError;
use tokio::sync::AcquireError;

#[derive(Debug)]
pub enum AppError {
    MissingFileName,
    FailedToReadData,
    InvalidImageFormat,
    ConversionFailed,
    NoImagesProcessed,
    ZipStartError,
    ZipWriteError,
    ZipFinishError,
    ProcessingError,
    InvalidFileName,
    MultipartError(MultipartError),
    AcquireError(AcquireError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AppError {}

impl From<MultipartError> for AppError {
    fn from(error: MultipartError) -> Self {
        AppError::MultipartError(error)
    }
}

impl From<AcquireError> for AppError {
    fn from(error: AcquireError) -> Self {
        AppError::AcquireError(error)
    }
}

impl From<AppError> for Box<dyn Error + Send> {
    fn from(error: AppError) -> Self {
        Box::new(error)
    }
}