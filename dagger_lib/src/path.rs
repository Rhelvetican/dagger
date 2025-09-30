use std::{
    env::var_os,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use dirs::config_dir;

static BALATRO_MOD_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    var_os("LOVELY_MOD_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            config_dir()
                .map(|s| s.join("Balatro/Mods"))
                .unwrap_or_default()
        })
});

static DAGGER_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    var_os("DAGGER_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| config_dir().unwrap_or_default().join("Dagger"))
});

#[derive(Debug, Clone, Copy)]
pub struct DaggerPaths;

impl DaggerPaths {
    #[inline]
    pub fn config_dir() -> &'static Path {
        &DAGGER_DIR
    }

    #[inline]
    pub fn balatro_mod_dir() -> &'static Path {
        &BALATRO_MOD_DIR
    }
}
