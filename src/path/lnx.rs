use std::{
    env::var,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::path::{DaggerPathApi, PathImpl};

fn is_dir<P: AsRef<OsStr>>(path: P) -> bool {
    Path::new(&path).is_dir()
}

const BALATRO_DIRS: &[&str] = &[
    "/.local/share/Steam/steamapps/compatdata/2379780/pfx/drive_c/users/steamuser/AppData/Roaming/Balatro",
    "/.var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/compatdata/2379780/pfx/drive_c/users/steamuser/AppData/Roaming/Balatro",
    "/.steam/steam/steamapps/compatdata/2379780/pfx/drive_c/users/steamuser/AppData/Roaming/Balatro",
    "/.steam/debian-installation/steamapps/compatdata/2379780/pfx/drive_c/users/steamuser/AppData/Roaming/Balatro",
];

impl DaggerPathApi for PathImpl {
    #[inline]
    fn balatro_dir() -> PathBuf {
        let home = PathBuf::from(var("HOME").unwrap_or_default());

        BALATRO_DIRS
            .into_iter()
            .copied()
            .map(|s| home.join(s))
            .find(|s| is_dir(s))
            .unwrap_or(home.join("Balatro"))
    }

    fn config_dir() -> PathBuf {
        PathBuf::from("~/.config/dagger")
    }
}
