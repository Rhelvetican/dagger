use std::fs::create_dir_all;

use clap::Parser;
use dagger::{Cli, Commands, DagRes, DaggerLockfile, DaggerModManager, DaggerPathApi, PathImpl};

fn main() -> DagRes<()> {
    let mut manager = DaggerModManager::new(DaggerLockfile::load().unwrap_or_default());

    let cfg_dir = PathImpl::config_dir();
    if !cfg_dir.exists() {
        create_dir_all(&cfg_dir)?;
    }

    let args = Cli::parse();

    match args.cmd {
        Commands::Install(iargs) => manager.install(iargs)?,
        Commands::Update(uargs) => manager.update(uargs)?,
        Commands::List => manager.list()?,
    };

    Ok(())
}
