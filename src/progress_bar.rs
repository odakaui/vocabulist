use indicatif::{ProgressBar, ProgressStyle};

pub fn new(len: u64, message: &str) -> ProgressBar {
    let pb = ProgressBar::new(len).with_style(ProgressStyle::default_bar()
            .template("{spinner:.black} [{bar:40.black/black}] [{pos:>7}/{len:7}] {msg}")
            .progress_chars("##-"));
    pb.set_message(message);

    pb
}
