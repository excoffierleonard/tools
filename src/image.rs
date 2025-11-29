use std::{io::Cursor, num::NonZeroU8};

use image::{ImageReader, codecs::jpeg::JpegEncoder};
use oxipng::{Deflaters, Options, optimize_from_memory};

pub fn compress_image_lossy_jpeg(input_bytes: &[u8]) -> Vec<u8> {
    let img = ImageReader::new(Cursor::new(input_bytes))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    let mut buffer = Vec::new();
    let encoder = JpegEncoder::new_with_quality(&mut buffer, 99);
    img.write_with_encoder(encoder).unwrap();

    buffer
}

pub fn compress_image_lossless_png(input_bytes: &[u8]) -> Vec<u8> {
    let img = image::ImageReader::new(Cursor::new(input_bytes))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    // First encode to PNG
    let mut buffer = Vec::new();
    img.write_to(&mut Cursor::new(&mut buffer), image::ImageFormat::Png)
        .unwrap();

    // Then optimize with oxipng
    let mut options = Options::max_compression();
    options.deflate = Deflaters::Zopfli {
        iterations: NonZeroU8::new(255).unwrap(),
    };
    optimize_from_memory(&buffer, &options).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_lossy_image_compression() {
        let input_bytes = fs::read("tests/inputs/lenna.png").unwrap();
        let compressed_data = compress_image_lossy_jpeg(&input_bytes);
        fs::write("tests/outputs/lenna_compressed.jpeg", &compressed_data).unwrap();

        assert!(compressed_data.len() < input_bytes.len());
    }

    #[ignore = "This test is very slow due to lossless compression"]
    #[test]
    fn test_lossless_image_compression() {
        let input_bytes = fs::read("tests/inputs/lenna.png").unwrap();
        let compressed_data = compress_image_lossless_png(&input_bytes);
        fs::write("tests/outputs/lenna_compressed.png", &compressed_data).unwrap();

        assert!(compressed_data.len() < input_bytes.len());
    }
}
