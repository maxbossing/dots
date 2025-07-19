use std::path::PathBuf;

#[cfg(target_os = "linux")]
pub mod platform {
    use super::*;
    pub fn symlink(target: &PathBuf, destination: &PathBuf) -> std::io::Result<()> {
        std::os::unix::fs::symlink(target, destination)
    }
}

#[cfg (not (any (target_os = "linux")))]
pub mod platform {
    use super::*;
    pub fn symlink(target: &PathBuf, destination: &PathBuf) -> std::io::Result<()> {
        unimplemented!();
    }
}
