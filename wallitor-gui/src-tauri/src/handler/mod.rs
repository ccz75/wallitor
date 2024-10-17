pub mod apply;
pub mod file;
pub mod wallpaper;

use std::path::{Path, PathBuf};

pub fn get_absolute_path(path: &String) -> PathBuf {
    let binding = std::env::current_exe().unwrap();
    let base_dir = binding.parent().unwrap();
    return base_dir.join(Path::new(path));
}
