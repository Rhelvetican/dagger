use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy)]
pub struct PathImpl;

pub trait DaggerPathApi {
    fn balatro_dir() -> &'static str;
    fn config_dir() -> &'static str;

    #[inline]
    fn balatro_mod_dir() -> PathBuf {
        Path::new(Self::balatro_dir()).join("/Mods")
    }
}

pub trait AsPath {
    fn as_path(&self) -> &Path;
}

impl AsPath for str {
    #[inline]
    fn as_path(&self) -> &Path {
        self.as_ref()
    }
}

#[cfg(target_os = "linux")]
pub mod lnx;
#[cfg(target_os = "macos")]
pub mod mac;
#[cfg(target_os = "windows")]
pub mod win;
