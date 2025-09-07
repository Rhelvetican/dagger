use crate::path::{DaggerPaths, DaggerPathsImpl};
use std::{fs::create_dir_all, path::Path};

impl DaggerPaths for DaggerPathsImpl {
    fn balatro_mod_dir() -> &'static Path {
        let path = Path::new("~/AppData/Roaming/Balatro/Mods");
        let _ = create_dir_all(path);
        path
    }

    fn config_dir() -> &'static Path {
        let path = Path::new("~/.config/dagger");
        let _ = create_dir_all(path);
        path
    }
}
