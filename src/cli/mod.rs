mod commands;
pub use commands::Commands;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CliArgs {
    #[command(subcommand)]
    pub cmd: Commands,
}
