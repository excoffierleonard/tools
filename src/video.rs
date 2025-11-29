use std::{
    fs,
    process::{Command, Stdio},
};

use tempfile::{Builder, NamedTempFile};

use crate::error::Error;

pub fn compress_video(input_bytes: &[u8]) -> Result<Vec<u8>, Error> {
    let input_file = NamedTempFile::new()?;
    let output_file = Builder::new().suffix(".mp4").tempfile()?;

    fs::write(input_file.path(), input_bytes)?;

    Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(input_file.path())
        .arg("-c:v")
        .arg("hevc_nvenc") // Migration to AV1 when we will have a 40+ GPU
        .arg(output_file.path())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    let output_bytes = fs::read(output_file.path())?;
    Ok(output_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn compressing_video_does_work() {
        let input_bytes = fs::read("tests/inputs/water-uhd_3840_2160_25fps.mp4").unwrap();
        let compressed_data = compress_video(&input_bytes).unwrap();
        fs::write("tests/outputs/output.mp4", &compressed_data).unwrap();

        assert!(!compressed_data.is_empty(), "Output is empty or corrupted");
    }
}
