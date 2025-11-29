use std::{
    path::Path,
    process::{Command, Stdio},
};

use crate::error::Error;

pub fn compress_video(input: &Path, output: &Path) -> Result<(), Error> {
    Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(input)
        .arg("-c:v")
        .arg("hevc_nvenc")
        .arg(output)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compressing_video_does_work() {
        let input = Path::new("tests/inputs/water-uhd_3840_2160_25fps.mp4");
        let output = Path::new("tests/outputs/output.mp4");
        compress_video(input, output).unwrap();

        assert!(output.exists(), "Output file was not created");
        assert!(
            output.metadata().unwrap().len() > 0,
            "Output file is empty or corrupted"
        );
    }
}
