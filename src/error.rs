use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Image processing error: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Oxipng error: {0}")]
    OxipngError(#[from] oxipng::PngError),
}
