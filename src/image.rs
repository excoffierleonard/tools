use std::{io::Cursor, num::NonZeroU64, time::Duration};

use base64::{Engine, engine::general_purpose::STANDARD};
use image::{ImageFormat, ImageReader, codecs::jpeg::JpegEncoder};
use oxipng::{Deflater, Options, ZopfliOptions, optimize_from_memory};
use serde_json::Value;

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

pub fn upscale_image(input_bytes: &[u8], api_key: &str) -> Result<Vec<u8>, Error> {
    let img = ImageReader::new(Cursor::new(input_bytes))
        .with_guessed_format()?
        .decode()?;

    let mut png_buffer = Vec::new();
    img.write_to(&mut Cursor::new(&mut png_buffer), ImageFormat::Png)?;
    let encoded = STANDARD.encode(png_buffer);

    let body = serde_json::json!({
        "contents": [
            {
                "parts": [
                    { "text": "Upscale the image." },
                    {
                        "inline_data": {
                            "mime_type": "image/png",
                            "data": encoded
                        }
                    }
                ]
            }
        ],
        "generationConfig": {
            "imageConfig": {
                "imageSize": "4K"
            }
        }
    });

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(120))
        .build()?;
    let response = client
        .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-3-pro-image-preview:generateContent")
        .header("x-goog-api-key", api_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()?;
    let value: Value = response.error_for_status()?.json()?;
    let image_data = extract_inline_image_data(&value, "<non-text response>")?;
    let decoded = STANDARD.decode(image_data)?;
    Ok(decoded)
}

fn extract_inline_image_data<'a>(value: &'a Value, raw_text: &str) -> Result<&'a str, Error> {
    let candidates = value
        .get("candidates")
        .and_then(Value::as_array)
        .ok_or_else(|| Error::Api(format!("missing candidates in response: {}", raw_text)))?;
    let content = candidates.first()
        .and_then(|candidate| candidate.get("content"))
        .ok_or_else(|| Error::Api(format!("missing content in response: {}", raw_text)))?;
    let parts = content
        .get("parts")
        .and_then(Value::as_array)
        .ok_or_else(|| Error::Api(format!("missing parts in response: {}", raw_text)))?;
    let data = parts
        .iter()
        .find_map(|part| {
            part.get("inline_data")
                .or_else(|| part.get("inlineData"))
                .and_then(|inline| inline.get("data"))
                .and_then(|d| d.as_str())
        })
        .ok_or_else(|| Error::Api(format!("no inline image data in response: {}", raw_text)))?;
    Ok(data)
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

    #[ignore = "Requires GEMINI_API_KEY and network access"]
    #[test]
    fn test_image_upscale() {
        dotenvy::dotenv().ok();
        let api_key = std::env::var("GEMINI_API_KEY").unwrap();
        let input_bytes = fs::read(Path::new("tests").join("inputs").join("cat.jpg")).unwrap();
        let upscaled_data = upscale_image(&input_bytes, &api_key).unwrap();
        fs::write(
            Path::new("tests").join("outputs").join("cat_upscaled.jpg"),
            &upscaled_data,
        )
        .unwrap();

        assert!(!upscaled_data.is_empty(), "Output is empty or corrupted");
    }
}
