use std::env;
use std::fs::{remove_file, symlink_metadata};
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use std::process::exit;
use clap::Parser;
use dirs::{home_dir};
use crate::cli::{Cli, CliCommand};
use crate::config::{Config, ConfigLoadError, Dot};

mod config;
mod cli;

fn main() {
    let cli = Cli::parse();
    
    let config = Config::load(cli.config).map_err(|err|
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
    
    match cli.command {
        CliCommand::Deploy => deploy_dots(config.dots, config.dots_dir),
        CliCommand::Undeploy => {}
    }
}

fn deploy_dots(dots: Vec<Dot>, dots_dir: PathBuf) {
    let prepended_dots = dots.iter().map(|m|
        Dot {
            source: dots_dir.join(&m.source),
            destination: prepend_user_dir(&m.destination)
        }
    );
    
    for dot in prepended_dots {
        println!("linking from {} to {}", dot.source.display(), dot.destination.display());
        let _ = symlink(&dot.source, &dot.destination).map_err(|err|
            eprintln!("failed to symlink: {}", err.to_string())
        );
    }
}

fn undeploy_dots(dots: Vec<Dot>, dots_dir: PathBuf) {
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