use std::path::PathBuf;

use dirs::{config_dir, data_dir};

#[derive(Default, Clone, Copy)]
pub struct Directories;

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
impl Directories {
    fn config() -> PathBuf {
        unsafe { config_dir().unwrap_unchecked() }
    }

    fn data() -> PathBuf {
        unsafe { data_dir().unwrap_unchecked() }
    }
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
impl Directories {
    fn config() -> PathBuf {
        compile_error!("Unsupported platform. Only Windows, Linux or MacOs are supported.")
    }

    fn data() -> PathBuf {
        compile_error!("Unsupported platform. Only Windows, Linux or MacOs are supported.")
    }
}

impl Directories {
    /// Get the Dagger configuration directory.
    pub fn config_dir() -> PathBuf {
        let mut dir = Self::config();
        dir.push("/Dagger/");
        dir
    }

    /// Get the Balatro mod directory.
    pub fn mod_dir() -> PathBuf {
        let mut dir = Self::data();
        dir.push("/Balatro/Mods/");
        dir
    }
}
