use clap::Parser;

use crate::{cli::*, error::*, manager::DaggerModManager, spinner::*};

mod cli;
mod error;
mod git;
mod manager;
mod spinner;

fn main() -> Result<()> {
    let mut manager = DaggerModManager::new();

    let cli = Cli::parse();

    match cli.cmd {
        Commands::Install(mut i) => {
            let mut cb = TransferProgress::new(TransferProgress::construct_spinner(&format!(
                "Installing {}...",
                i.get_id()
            )));

            manager.install(i, Some(&mut cb))?
        }

        Commands::Update(up) => match up.cmd {
            UpdateCommands::All => {
                let ids = manager.get_mod_ids();

                for id in ids {
                    let mut cb = TransferProgress::new(TransferProgress::construct_spinner(
                        &format!("Installing {}...", &id),
                    ));
                    manager.update(&*id, Some(&mut cb))?;
                }
            }

            UpdateCommands::Item(item) => {
                let mut cb = TransferProgress::new(TransferProgress::construct_spinner(&format!(
                    "Installing {}...",
                    &item.id
                )));
                manager.update(item, Some(&mut cb))?;
            }
        },

        Commands::Uninstall(uni) => manager.uninstall(uni)?,

        Commands::List(list) => match list.cmd {
            ListCommands::All => manager
                .get_mod_ids()
                .iter()
                .map(|s| manager.list(&**s, list.list_tags))
                .collect::<Result<_>>()?,
            ListCommands::Item(item) => manager.list(item, list.list_tags)?,
        },
    };

    Ok(())
}
