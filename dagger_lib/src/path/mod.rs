use std::{env::var_os, path::PathBuf, sync::LazyLock};

use dirs::config_dir;

static DIRS: LazyLock<PathBuf> = LazyLock::new(|| {
    if let Some(path) = var_os("LOVELY_MOD_DIR") {
        PathBuf::from(path)
    } else {
        config_dir().unwrap_or_default().join("Balatro")
    }
});

#[derive(Debug, Clone, Copy)]
pub struct PathImpl;

pub trait DaggerPathApi {
    fn balatro_dir() -> &'static PathBuf;
    fn config_dir() -> PathBuf;

    #[inline]
    fn balatro_mod_dir() -> PathBuf {
        Self::balatro_dir().join("Mods")
    }
}

impl DaggerPathApi for PathImpl {
    fn balatro_dir() -> &'static PathBuf {
        &DIRS
    }

    fn config_dir() -> PathBuf {
        DIRS.join("Dagger")
    }
}
