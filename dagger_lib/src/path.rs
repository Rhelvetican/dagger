use std::{
    env::var_os,
    fs::create_dir_all,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use dirs::config_dir;

trait CreateDirIfNotExists {
    fn create_dir_if_not_exists(self) -> Self;
}

impl CreateDirIfNotExists for &Path {
    fn create_dir_if_not_exists(self) -> Self {
        if !self.exists() {
            create_dir_all(&self).unwrap();
        }

        self
    }
}

impl CreateDirIfNotExists for PathBuf {
    fn create_dir_if_not_exists(self) -> Self {
        if !self.exists() {
            create_dir_all(&self).unwrap();
        }

        self
    }
}

static BALATRO_MOD_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    var_os("LOVELY_MOD_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            config_dir()
                .map(|s| s.join("Balatro/Mods"))
                .unwrap_or_default()
        })
        .create_dir_if_not_exists()
});

static DAGGER_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    var_os("DAGGER_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| config_dir().unwrap_or_default().join("Dagger"))
        .create_dir_if_not_exists()
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
