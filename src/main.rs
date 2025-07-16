use std::env;
use std::fs::{remove_file, symlink_metadata};
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use std::process::exit;
use clap::Parser;
use dirs::{home_dir};
use crate::cli::{Cli, CliCommand};
use crate::config::{Config, ConfigLoadError, Dot};
use crate::dots::{deploy_dots, unlink_dots};

mod config;
mod cli;
mod dots;

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
        CliCommand::Unlink => unlink_dots(config.dots, config.dots_dir),
    }
}