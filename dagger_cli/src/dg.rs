use clap::Parser;
use dagger_lib::DaggerModManagerApi;

use crate::{cli::*, error::*, manager::DaggerModManager, spinner::*};

mod cli;
mod error;
mod git;
mod manager;
mod spinner;

fn main() -> Result<()> {
    let mut manager = DaggerModManager::new();
    manager.refresh();

    let cli = Cli::parse();

    match cli.cmd {
        Commands::Install(mut i) => {
            let mut cb = TransferProgress::new(&format!("Installing {}...", i.get_id()));

            manager.install(i, &mut cb)?
        }

        Commands::Update(up) => match up.cmd {
            UpdateCommands::All => {
                for id in manager.get_mod_ids() {
                    let mut cb = TransferProgress::new(&format!("Updating {}...", &id));
                    manager.update(&*id, &mut cb)?;
                }
            }

            UpdateCommands::Item(item) => {
                let mut cb = TransferProgress::new(&format!("Updating {}...", &item.id));
                manager.update(item, &mut cb)?;
            }
        },

        Commands::Uninstall(uni) => manager.uninstall(uni)?,

        Commands::List(list) => match list.cmd {
            ListCommands::All => manager
                .get_mod_ids()
                .iter()
                .try_for_each(|s| manager.list(s.as_str(), list.list_tags))?,
            ListCommands::Item(item) => manager.list(item, list.list_tags)?,
        },
    };

    Ok(())
}
