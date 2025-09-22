use clap::{Args, Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    cmd: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    Install(CliArgs),
    Update(CliArgs),
}

#[derive(Debug, Clone, Args)]
#[group(required = true, multiple = false)]
pub struct CliArgs {
    #[arg(short, long, default_value_t = false)]
    all: bool,
    #[arg(short, long)]
    item: Option<String>,
}
