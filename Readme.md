# Rust Minification & Image Conversion API

This Rust backend provides three endpoints for optimizing web assets:

1. Minify CSS files.
2. Minify JavaScript files.
3. Convert PNG/JPG images to WebP format.

## API Endpoints

### Minify CSS

- **Endpoint:** `/minify-css`
- **Method:** `POST`
- **Request Body:** Raw CSS content.
- **Response:** Minified CSS content.

### Minify JavaScript

- **Endpoint:** `/minify-js`
- **Method:** `POST`
- **Request Body:** Raw JavaScript content.
- **Response:** Minified JavaScript content.

### Convert Images to WebP

- **Endpoint:** `/convert`
- **Method:** `POST`
- **Request Body:** Multipart form-data with a PNG or JPG image.
- **Response:** WebP image.