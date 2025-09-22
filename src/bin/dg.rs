use clap::Parser;
use dagger::{Cli, Commands, DagRes, DaggerLockfile, DaggerModManager};

fn main() -> DagRes<()> {
    let mut manager = DaggerModManager::new(DaggerLockfile::load()?);
    let args = Cli::parse();

    match args.cmd {
        Commands::Install(iargs) => manager.install(iargs)?,
        Commands::Update(uargs) => manager.update(uargs)?,
    };

    Ok(())
}
