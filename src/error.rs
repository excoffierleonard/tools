use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Image processing error: {0}")]
    Image(#[from] image::ImageError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("FFmpeg error: {0}")]
    Ffmpeg(String),

    #[error("Oxipng error: {0}")]
    Oxipng(#[from] oxipng::PngError),
}
