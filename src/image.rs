use std::{io::Cursor, num::NonZeroU8};

use image::{ImageFormat, ImageReader, codecs::jpeg::JpegEncoder};
use oxipng::{Deflaters, Options, optimize_from_memory};

use crate::error::Error;

pub fn compress_image_lossy_to_jpeg(input_bytes: &[u8]) -> Result<Vec<u8>, Error> {
    let img = ImageReader::new(Cursor::new(input_bytes))
        .with_guessed_format()?
        .decode()?;

    let mut buffer = Vec::new();
    let encoder = JpegEncoder::new_with_quality(&mut buffer, 99);
    img.write_with_encoder(encoder)?;

    Ok(buffer)
}

pub fn compress_image_lossless_to_png(input_bytes: &[u8]) -> Result<Vec<u8>, Error> {
    let img = ImageReader::new(Cursor::new(input_bytes))
        .with_guessed_format()?
        .decode()?;

    // First encode to PNG
    let mut buffer = Vec::new();
    img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;

    // Then optimize with oxipng
    let mut options = Options::max_compression();
    options.deflate = Deflaters::Zopfli {
        iterations: NonZeroU8::new(255).unwrap(),
    };
    let optimized = optimize_from_memory(&buffer, &options)?;
    Ok(optimized)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_lossy_image_compression() {
        let input_bytes = fs::read("tests/inputs/lenna.png").unwrap();
        let compressed_data = compress_image_lossy_to_jpeg(&input_bytes).unwrap();
        fs::write("tests/outputs/lenna_compressed.jpeg", &compressed_data).unwrap();

        assert!(compressed_data.len() < input_bytes.len());
    }

    #[ignore = "This test is very slow due to lossless compression"]
    #[test]
    fn test_lossless_image_compression() {
        let input_bytes = fs::read("tests/inputs/lenna.png").unwrap();
        let compressed_data = compress_image_lossless_to_png(&input_bytes).unwrap();
        fs::write("tests/outputs/lenna_compressed.png", &compressed_data).unwrap();

        assert!(compressed_data.len() < input_bytes.len());
    }
}
