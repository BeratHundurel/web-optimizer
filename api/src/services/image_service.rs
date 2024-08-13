use crate::errors::AppError;
use axum::extract::Multipart;
use load_image::{load_data, Image, ImageData};
use rayon::prelude::*;
use std::{
    io::{Cursor, Write},
    path::Path,
};
use webp::Encoder;
use zip::{write::SimpleFileOptions, ZipWriter};

// Process images from multipart form-data
pub async fn convert_images_to_webp(multipart: &mut Multipart) -> Result<Vec<u8>, AppError> {
    let webp_images = collect_images(multipart).await?;

    if webp_images.is_empty() {
        return Err(AppError::NoImagesProcessed);
    }

    let zip_data = create_zip(webp_images)?;

    Ok(zip_data)
}

async fn collect_images(multipart: &mut Multipart) -> Result<Vec<(String, Vec<u8>)>, AppError> {
    let mut image_data = Vec::new();

    // Collect the data and filename synchronously
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| AppError::ProcessingError)?
    {
        let filename = field
            .file_name()
            .ok_or(AppError::MissingFileName)?
            .to_string();
        let data = field
            .bytes()
            .await
            .map_err(|_| AppError::FailedToReadData)?
            .to_vec();

        image_data.push((data, filename));
    }

    // Process images in parallel using Rayon
    let webp_images: Vec<_> = image_data
        .into_par_iter()
        .map(|(data, filename)| process_image(data, filename))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(webp_images)
}

fn process_image(data: Vec<u8>, filename: String) -> Result<(String, Vec<u8>), AppError> {
    let img = load_image(&data).map_err(|_| AppError::InvalidImageFormat)?;
    let webp_image = convert_to_webp(img).ok_or(AppError::ConversionFailed)?;
    let webp_filename = format!("{}.webp", filename_without_extension(&filename));
    Ok((webp_filename, webp_image))
}

// Create a ZIP file from WebP images
fn create_zip(webp_images: Vec<(String, Vec<u8>)>) -> Result<Vec<u8>, AppError> {
    let mut zip_data = Vec::new();
    let cursor = Cursor::new(&mut zip_data);
    let mut zip = ZipWriter::new(cursor);

    for (filename, image_data) in webp_images {
        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);

        zip.start_file(filename, options)
            .map_err(|_| AppError::ZipStartError)?;
        zip.write_all(&image_data)
            .map_err(|_| AppError::ZipWriteError)?;
    }

    zip.finish().map_err(|_| AppError::ZipFinishError)?;
    Ok(zip_data)
}

fn load_image(data: &[u8]) -> Result<Image, AppError> {
    load_data(data).map_err(|_| AppError::LoadError)
}

fn convert_to_webp(img: Image) -> Option<Vec<u8>> {
    let width = img.width as u32;
    let height = img.height as u32;
    let bitmap: Vec<u8> = match img.bitmap {
        ImageData::RGB8(data) => data
            .into_iter()
            .flat_map(|pixel| vec![pixel.r, pixel.g, pixel.b])
            .collect(),
        ImageData::RGBA8(data) => data
            .into_iter()
            .flat_map(|pixel| vec![pixel.r, pixel.g, pixel.b])
            .collect(),
        ImageData::RGB16(data) => data
            .into_iter()
            .flat_map(|pixel| {
                vec![
                    (pixel.r >> 8) as u8,
                    (pixel.g >> 8) as u8,
                    (pixel.b >> 8) as u8,
                ]
            })
            .collect(),
        _ => return None,
    };

    if bitmap.is_empty() {
        return None;
    }

    let encoder = Encoder::from_rgb(&bitmap, width, height);
 
    Some(encoder.encode(75.0).to_vec())
}

fn filename_without_extension(filename: &str) -> String {
    Path::new(filename)
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string()
}
