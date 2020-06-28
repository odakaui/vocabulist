use indicatif::{ProgressBar, ProgressStyle};

pub fn new(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_message("Excluding");
    pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.black} [{bar:40.black/black}] [{pos:>7}/{len:7}] {msg}")
            .progress_chars("##-"));

    pb
}
