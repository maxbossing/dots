use std::fs::{remove_file, symlink_metadata};
use std::path::PathBuf;
use dirs::home_dir;
use crate::config::Dot;
use crate::platform;

pub fn deploy_dots(dots: Vec<Dot>, dots_dir: PathBuf) {
    let prepended_dots = dots.iter().map(|m|
        Dot {
            source: dots_dir.join(&m.source),
            destination: if m.destination.is_absolute() { m.destination.clone() } else { prepend_user_dir(&m.destination) }
        }
    );

    for dot in prepended_dots {
        println!("linking from {} to {}", dot.source.display(), dot.destination.display());
        let _ = platform::platform::symlink(&dot.source, &dot.destination).map_err(|err|
            eprintln!("failed to symlink: {}", err.to_string())
        );
    }
}

pub fn unlink_dots(dots: Vec<Dot>, dots_dir: PathBuf) {
    let prepended_dots = dots.iter().map(|m|
        Dot {
            source: dots_dir.join(&m.source),
            destination: prepend_user_dir(&m.destination)
        }
    );

    for dot in prepended_dots {
        println!("unlinking  {}", dot.destination.display());
        let metadata = symlink_metadata(dot.destination.clone());
        if metadata.is_err() {
            eprintln!("failed to query metadata for {}: {}", dot.destination.display(), metadata.err().unwrap());
            continue
        }
        if metadata.ok().unwrap().is_symlink() {
            let _ = remove_file(dot.destination).map_err(|err|
                eprintln!("failed to remove symlink: {}", err)
            );
        }
    }
}

fn prepend_user_dir(path: &PathBuf) -> PathBuf {
    home_dir().unwrap().join(path)
}
