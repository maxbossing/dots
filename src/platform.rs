use std::path::PathBuf;

#[cfg(target_os = "linux")]
pub mod platform {
    use super::*;
    pub fn symlink(target: &PathBuf, destination: &PathBuf) -> std::io::Result<()> {
        std::os::unix::fs::symlink(target, destination)
    }
}

#[cfg(target_os = "windows")]
pub mod platform {
    use super::*;
    pub fn symlink(target: &PathBuf, destination: &PathBuf) -> std::io::Result<()> {
        if target.is_dir() {
            std::os::windows::fs::symlink_dir(target, destination)
        } else {
            std::os::windows::fs::symlink_file(target, destination)
        }
    }
}

#[cfg (not (any (target_os = "linux", target_os = "windows")))]
pub mod platform {
    use super::*;
    pub fn symlink(target: &PathBuf, destination: &PathBuf) -> std::io::Result<()> {
        unimplemented!();
    }
}
