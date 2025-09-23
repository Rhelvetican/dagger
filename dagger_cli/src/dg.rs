use std::error::Error;

use clap::Parser;
use dagger_lib::{DaggerLockfile, DaggerModManager};

use crate::{
    cli::{Cli, Commands, InstallCommandArgs, ListCommandArgs, UpdateCommandArgs},
    spinner::TransferProgress,
};

mod cli;
mod spinner;

fn main() -> Result<(), Box<dyn Error>> {
    let mut manager = DaggerModManager::new(DaggerLockfile::load().unwrap_or_default());
    let cli = Cli::parse();

    match cli.cmd {
        Commands::Install(i) => install(&mut manager, i)?,
        Commands::Update(u) => update(&mut manager, u)?,
        Commands::List(l) => list(&mut manager, l)?,
    };

    Ok(())
}

fn install(
    manager: &mut DaggerModManager,
    mut args: InstallCommandArgs,
) -> Result<(), Box<dyn Error>> {
    let mut cb = TransferProgress::new(TransferProgress::construct_spinner(&format!(
        "Installing {}...",
        args.get_id()
    )));

    Ok(manager.install(args, Some(&mut cb))?)
}

fn update(manager: &mut DaggerModManager, args: UpdateCommandArgs) -> Result<(), Box<dyn Error>> {
    if args.cmd.is_all() {
        fn callback(id: &str) -> TransferProgress<'_> {
            TransferProgress::new(TransferProgress::construct_spinner(&format!(
                "Updating {}...",
                id
            )))
        }

        manager.update_all(Some(callback))?;
    } else if let Some(item) = args.cmd.as_item() {
        let mut cb = TransferProgress::new(TransferProgress::construct_spinner(&format!(
            "Updating {}...",
            &item.id
        )));

        manager.update_item(item, Some(&mut cb))?;
    };

    Ok(())
}

fn list(manager: &mut DaggerModManager, args: ListCommandArgs) -> Result<(), Box<dyn Error>> {
    if args.cmd.is_all() {
        manager.list_all(args.list_tags)?;
    } else {
        manager.list_item(args)?;
    }

    Ok(())
}
