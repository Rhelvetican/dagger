use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub struct PathImpl;

pub trait DaggerPathApi {
    fn balatro_dir() -> PathBuf;
    fn config_dir() -> PathBuf;

    #[inline]
    fn balatro_mod_dir() -> PathBuf {
        Self::balatro_dir().join("Mods")
    }
}

#[cfg(target_os = "linux")]
pub mod lnx;
#[cfg(target_os = "macos")]
pub mod mac;
#[cfg(target_os = "windows")]
pub mod win;
