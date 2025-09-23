use std::{
    env::var,
    path::{Path, PathBuf},
};

use crate::path::{DaggerPathApi, PathImpl};

impl DaggerPathApi for PathImpl {
    #[inline]
    fn config_dir() -> PathBuf {
        var("AppData")
            .as_deref()
            .map(Path::new)
            .map(|p| p.join("Dagger"))
            .unwrap()
    }

    #[inline]
    fn balatro_dir() -> PathBuf {
        var("AppData")
            .as_deref()
            .map(Path::new)
            .map(|p| p.join("Balatro"))
            .unwrap()
    }
}
