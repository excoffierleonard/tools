use std::{io::Cursor, num::NonZeroU64};

use image::{ImageFormat, ImageReader, codecs::jpeg::JpegEncoder};
use oxipng::{Deflater, Options, ZopfliOptions, optimize_from_memory};

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
    options.deflater = Deflater::Zopfli(ZopfliOptions {
        iteration_count: NonZeroU64::new(255).unwrap(),
        ..Default::default()
    });
    let optimized = optimize_from_memory(&buffer, &options)?;
    Ok(optimized)
}

pub fn upscale_image(_input_bytes: &[u8]) -> Result<Vec<u8>, Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::Path};

    #[test]
    fn test_lossy_image_compression() {
        let input_bytes = fs::read(Path::new("tests").join("inputs").join("lenna.png")).unwrap();
        let compressed_data = compress_image_lossy_to_jpeg(&input_bytes).unwrap();
        fs::write(
            Path::new("tests")
                .join("outputs")
                .join("lenna_compressed.jpeg"),
            &compressed_data,
        )
        .unwrap();

        assert!(!compressed_data.is_empty(), "Output is empty or corrupted");
        assert!(compressed_data.len() < input_bytes.len());
    }

    #[ignore = "This test is very slow due to lossless compression"]
    #[test]
    fn test_lossless_image_compression() {
        let input_bytes = fs::read(Path::new("tests").join("inputs").join("lenna.png")).unwrap();
        let compressed_data = compress_image_lossless_to_png(&input_bytes).unwrap();
        fs::write(
            Path::new("tests")
                .join("outputs")
                .join("lenna_compressed.png"),
            &compressed_data,
        )
        .unwrap();

        assert!(!compressed_data.is_empty(), "Output is empty or corrupted");
        assert!(compressed_data.len() < input_bytes.len());
    }

    #[test]
    fn test_image_upscale() {
        let input_bytes = fs::read(Path::new("tests").join("inputs").join("lenna.png")).unwrap();
        let upscaled_data = upscale_image(&input_bytes).unwrap();
        fs::write(
            Path::new("tests")
                .join("outputs")
                .join("lenna_upscaled.png"),
            &upscaled_data,
        )
        .unwrap();

        assert!(!upscaled_data.is_empty(), "Output is empty or corrupted");
    }
}
