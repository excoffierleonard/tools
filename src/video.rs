use std::{path::Path, process::Command, sync::OnceLock};

static USE_NVENC: OnceLock<bool> = OnceLock::new();

fn nvenc_works() -> bool {
    Command::new("ffmpeg")
        .args([
            "-f",
            "lavfi",
            "-i",
            "nullsrc=s=64x64:d=1",
            "-c:v",
            "hevc_nvenc",
            "-frames:v",
            "1",
            "-f",
            "null",
            "-",
        ])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn use_nvenc() -> bool {
    *USE_NVENC.get_or_init(nvenc_works)
}

pub fn compress_video(input: &Path, output: &Path) {
    let codec = if use_nvenc() { "hevc_nvenc" } else { "libx265" };

    Command::new("ffmpeg")
        .args([
            "-y", // Overwrite output files without asking
            "-i",
            input.to_str().unwrap_or_default(),
            "-c:v",
            codec,
            output.to_str().unwrap_or_default(),
        ])
        .status()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn compressing_video_does_work() {
        let input = Path::new("tests/inputs/water-uhd_3840_2160_25fps.mp4");
        let output = Path::new("tests/outputs/output.mp4");
        compress_video(input, output);

        assert!(output.exists(), "Output file was not created");
        assert!(
            output.metadata().unwrap().len() > 0,
            "Output file is empty or corrupted"
        );

        // Clean up
        fs::remove_file(output).unwrap();
    }

    #[test]
    fn nvenc_detection_works() {
        let nvenc_available = nvenc_works();
        let use_nvenc_flag = use_nvenc();
        assert_eq!(nvenc_available, use_nvenc_flag, "NVENC detection mismatch");
    }
}
