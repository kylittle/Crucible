use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use crate::camera::Camera;

// starts up progress bars for the camera
pub fn init_pb() -> (MultiProgress, ProgressStyle) {
    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    (m, sty)
}

impl Camera {
    pub fn add_pb(&mut self, work: u64) -> ProgressBar {
        let pb = self.mp.add(ProgressBar::new(work));
        pb.set_style(self.sty.clone());

        pb
    }
}
