use std::env;
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use std::process::exit;
use dirs::{home_dir};
use crate::config::{Config, ConfigLoadError, Dot};

mod config;

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();

    if args.len() > 1 {
        eprintln!("usage: {} [config]", args[0]);
        exit(1);
    }

    if args.len() == 1 && (args[0] == "-h" || args[0] == "--help") {
        println!("Usage: {} [config]", args[0]);
        println!("options:");
        println!("  -h, --help        Print help information");
        println!("By default, dots will look for a bummsdots.toml file in the current directory");
        println!("This can be changed by passing the filename");
        exit(0);
    }

    let config = Config::load(args.get(0).map(|o| PathBuf::from(o))).map_err(|err|
        match err {
            ConfigLoadError::IOError(err) => {
                eprintln!("failed to load config file: {}", err);
                exit(1)
            }
            ConfigLoadError::DeserializationError(err) => {
                eprintln!("failed to deserialize config file: {}", err);
                exit(1)
            }
        }
    ).unwrap();

    let prepended_mappings = config.dots.iter().map(|m|
        Dot {
            source: env::current_dir().expect("failed to get current dir").join(&m.source),
            destination: prepend_user_dir(&m.destination)
        }
    );

    for dot in prepended_mappings {
        println!("linking from {} to {}", dot.source.display(), dot.destination.display());
        let _ = symlink(&dot.source, &dot.destination).map_err(|err|
            eprintln!("failed to symlink: {}", err.to_string())
        );
    }
}

fn prepend_user_dir(path: &PathBuf) -> PathBuf {
    home_dir().unwrap().join(path)
}