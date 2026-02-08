use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Image processing error: {0}")]
    Image(#[from] image::ImageError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Base64 decode error: {0}")]
    Base64(#[from] base64::DecodeError),

    #[error("Missing environment variable: {0}")]
    MissingEnv(String),

    #[error("Gemini API error: {0}")]
    Api(String),

    #[error("FFmpeg error: {0}")]
    Ffmpeg(String),

    #[error("Oxipng error: {0}")]
    Oxipng(#[from] oxipng::PngError),
}
