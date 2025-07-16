use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author = "Max Bossing", version, about = "System-agnostic dotfile deployer", long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value = "dots.toml")]
    pub config: Option<PathBuf>,
    
    #[command(subcommand)]
    pub command: CliCommand 
}

#[derive(Subcommand)]
pub enum CliCommand {
    #[clap(about = "Deploys a dots set")]
    Deploy,
    #[clap(about = "Unlinks (tries to remove) a dots deployment")]
    Unlink
}