use clap::{Args, Subcommand};

#[derive(Debug, Clone, Subcommand)]
#[non_exhaustive]
pub enum Commands {
    Update(UpdateCommand),
}

#[derive(Debug, Clone, Args)]
#[group(required = true, multiple = false)]
pub struct UpdateCommand {
    /// Update every mods defined in the manifest.
    #[arg(long)]
    all: bool,
    /// Update only this mod.
    /// Must be identical to the mod declaration.
    /// EG:
    /// `"<maintainer>/<mod>"`,
    /// `"SpectralPack/Cryptid"`
    #[arg(short, long)]
    mod_name: Option<String>,
}

impl UpdateCommand {
    #[inline]
    pub fn all(&self) -> bool {
        self.all
    }

    #[inline]
    pub fn get_mod(&self) -> Option<&str> {
        self.mod_name.as_deref()
    }
}
