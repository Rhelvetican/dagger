use clap::Subcommand;

#[derive(Debug, Clone, Copy, Subcommand)]
pub enum Commands {
    Update {
        #[arg(long, default_value_t = false)]
        all: bool,
    },
}
