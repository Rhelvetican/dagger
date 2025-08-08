use std::path::PathBuf;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use dirs::{config_local_dir, data_dir};

#[derive(Default, Clone, Copy)]
pub struct Directories;

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
impl Directories {
    fn config() -> PathBuf {
        unsafe { config_local_dir().unwrap_unchecked() }
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
