use axum::{extract::Multipart, http::StatusCode, response::IntoResponse, routing::post, Router};
use image::DynamicImage;
use tokio::{fs::File, io::AsyncWriteExt};
use webp::Encoder;

// Handles image conversion from multipart form-data to WebP format
async fn convert_images(mut multipart: Multipart) -> impl IntoResponse {
    // Process each field in the multipart form-data
    while let Some(field) = multipart.next_field().await.unwrap() {
        // Retrieve the file name, if present
        let filename = field.file_name().map(|name| name.to_string());

        // Read the entire image data from the field
        let data = field.bytes().await.unwrap();

        // Attempt to load the image from the provided data
        let img = match image::load_from_memory(&data) {
            Ok(image) => image,
            Err(_) => {
                // If loading the image fails, return a 400 Bad Request response
                return (StatusCode::BAD_REQUEST, "Invalid image format").into_response();
            }
        };

        // Proceed only if a filename was provided
        if let Some(filename) = filename {
            // Convert the loaded image to WebP format
            let webp_image = convert_to_webp(img);

            // Construct the path for the output WebP file
            let output_file = format!("{}.webp", filename);

            // Create a file and write the WebP data to it
            let mut file = File::create(&output_file).await.unwrap();
            file.write_all(&webp_image).await.unwrap();
        }
    }

    // Return a 200 OK response once all images have been processed
    (StatusCode::OK, "Images converted successfully").into_response()
}

// Converts a DynamicImage to WebP format
fn convert_to_webp(img: DynamicImage) -> Vec<u8> {
    // Convert the image to an RGB8 image for WebP encoding
    let rgb_img = img.to_rgb8();

    // Retrieve the dimensions of the image
    let (width, height) = rgb_img.dimensions();

    // Create an encoder for the RGB image with the specified width and height
    let encoder = Encoder::from_rgb(rgb_img.as_ref(), width, height);

    // Encode the image to WebP format with a quality setting of 92.0
    encoder.encode(92.0).to_vec()
}

#[tokio::main]
async fn main() {
    // Define the Axum router with a route for converting images
    let app = Router::new().route("/convert", post(convert_images));

    // Bind the server to port 3000 and start serving requests
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}