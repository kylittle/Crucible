use std::process::{Command, Stdio};

/// Looks for images in <fname>/artifacts and loads
/// all the ppms. Then uses ffmpeg to build an mp4 video
/// TODO: use the ffmpeg crate if we need more power
pub fn make_mp4(_res: (usize, usize), frame_rate: usize, padding: usize, fname: &str) {
    let output_path = fname.to_owned() + "/movie.mp4";
    let image_pattern = fname.to_owned() + &format!("/artifacts/image%0{padding}d.ppm");
    let frame_rate = frame_rate.to_string();

    Command::new("ffmpeg")
        .args([
            "-framerate",
            &frame_rate,
            "-i",
            &image_pattern,
            "-vf",
            "scale=trunc(iw/2)*2:trunc(ih/2)*2",
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            "-crf",
            "25",
            &output_path,
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("FFMPEG failed");

    eprintln!("Successfully created movie: {output_path}");
}
