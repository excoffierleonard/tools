use std::io::Cursor;

use image::{
    ImageReader,
    codecs::png::{CompressionType, FilterType, PngEncoder},
};

pub fn compress_image_to_png(input_bytes: &[u8]) -> Vec<u8> {
    let img = ImageReader::new(Cursor::new(input_bytes))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    let mut buffer = Vec::new();
    let encoder = PngEncoder::new_with_quality(
        Cursor::new(&mut buffer),
        CompressionType::Best,
        FilterType::Adaptive,
    );
    img.write_with_encoder(encoder).unwrap();

    buffer
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_compress_to_png() {
        let input_bytes = fs::read("tests/inputs/lenna.png").unwrap();
        let compressed_data = compress_image_to_png(&input_bytes);
        fs::write("tests/outputs/lenna_compressed.png", &compressed_data).unwrap();
        assert!(compressed_data.len() < input_bytes.len());
    }
}
