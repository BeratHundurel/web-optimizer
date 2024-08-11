use crate::errors::AppError;
use axum::extract::Multipart;
use load_image::{load_data, Image, ImageData};
use std::io::{Cursor, Write};
use webp::Encoder;
use zip::{write::SimpleFileOptions, ZipWriter};

pub async fn process_images(multipart: &mut Multipart) -> Result<Vec<u8>, AppError> {
    let mut webp_images = Vec::new();
    
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
            .map_err(|_| AppError::FailedToReadData)?;
        
        let img = load_image(&data).map_err(|_| AppError::InvalidImageFormat)?;
        let webp_image = convert_to_webp(img).ok_or(AppError::ConversionFailed)?;

        webp_images.push((filename, webp_image));
    }

    if webp_images.is_empty() {
        return Err(AppError::NoImagesProcessed);
    }

    let mut zip_data = Vec::new();
    {
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

        zip.finish()
            .map_err(|_| AppError::ZipFinishError)?;
    }

    Ok(zip_data)
}

pub fn load_image(data: &[u8]) -> Result<Image, ()> {
    load_data(data).map_err(|_| ())
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
    Some(encoder.encode(80.0).to_vec())
}
