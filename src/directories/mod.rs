#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use crate::cfg_if;

#[derive(Default, Clone, Copy)]
pub struct Directories;

cfg_if! {
    if #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))] {
        use std::path::PathBuf;

        use dirs::{config_local_dir, data_dir};
        impl Directories {
            fn config() -> PathBuf {
                unsafe { config_local_dir().unwrap_unchecked() }
            }

            fn data() -> PathBuf {
                unsafe { data_dir().unwrap_unchecked() }
            }
        }
    } else {
        use std::path::PathBuf;

        use dirs::{config_local_dir, data_dir};
        impl Directories {
            fn config() -> PathBuf {
                compile_error!("Unsupported platform. Only Windows, Linux or MacOs are supported.")
            }

            fn data() -> PathBuf {
                compile_error!("Unsupported platform. Only Windows, Linux or MacOs are supported.")
            }
        }
    }
}
