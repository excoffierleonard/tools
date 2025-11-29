use std::{
    fs,
    process::{Command, Stdio},
};

use tempfile::{Builder, NamedTempFile};

use crate::error::Error;

pub fn compress_video_lossy_to_mp4(input_bytes: &[u8]) -> Result<Vec<u8>, Error> {
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
    use std::{fs, path::Path};

    #[ignore = "Requires ffmpeg with nvenc support and a compatible GPU"]
    #[test]
    fn compressing_video_does_work() {
        let input_bytes = fs::read(Path::new("tests").join("inputs").join("water.mp4")).unwrap();
        let compressed_data = compress_video_lossy_to_mp4(&input_bytes).unwrap();
        fs::write(
            Path::new("tests")
                .join("outputs")
                .join("water_compressed.mp4"),
            &compressed_data,
        )
        .unwrap();

        assert!(!compressed_data.is_empty(), "Output is empty or corrupted");
        assert!(compressed_data.len() < input_bytes.len());
    }
}
