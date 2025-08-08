use std::path::PathBuf;

use crate::Directories;

impl Directories {
    /// Get the Dagger configuration directory.
    pub fn config_dir() -> PathBuf {
        let mut dir = Self::config();
        dir.push("Dagger\\");
        dir
    }

    /// Get the Balatro mod directory.
    pub fn mod_dir() -> PathBuf {
        let mut dir = Self::data();
        dir.push("Balatro\\Mods\\");
        dir
    }
}
