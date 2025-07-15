use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[arg(default_value = "dots.toml")]
    pub config: Option<PathBuf>,
    
    #[command(subcommand)]
    pub command: CliCommand 
}

#[derive(Subcommand)]
pub enum CliCommand {
    Deploy,
    Undeploy
}