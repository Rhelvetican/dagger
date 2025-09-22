use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::path::{DaggerPathApi, PathImpl};

fn is_dir<P: AsRef<OsStr>>(path: P) -> bool {
    Path::new(&path).is_dir()
}

const SNAP_BALATRO_DIR: &str = "~/.local/share/Steam/steamapps/compatdata/2379780/pfx/drive_c/users/steamuser/AppData/Roaming/Balatro";
const FLATPAK_BALATRO_DIR: &str = "~/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/compatdata/2379780/pfx/drive_c/users/steamuser/AppData/Roaming/Balatro";
const STEAMDECK_BALATRO_DIR: &str = "~/.steam/steam/steamapps/compatdata/2379780/pfx/drive_c/users/steamuser/AppData/Roaming/Balatro";

impl DaggerPathApi for PathImpl {
    #[inline]
    fn balatro_dir() -> PathBuf {
        PathBuf::from(if is_dir(SNAP_BALATRO_DIR) {
            SNAP_BALATRO_DIR
        } else if is_dir(FLATPAK_BALATRO_DIR) {
            FLATPAK_BALATRO_DIR
        } else if is_dir(STEAMDECK_BALATRO_DIR) {
            STEAMDECK_BALATRO_DIR
        } else {
            "~/Balatro/"
        })
    }

    fn config_dir() -> PathBuf {
        PathBuf::from("~/.config/dagger")
    }
}
