use std::path::PathBuf;

use crate::path::{DaggerPathApi, PathImpl};

impl DaggerPathApi for PathImpl {
    #[inline]
    fn balatro_dir() -> PathBuf {
        PathBuf::from("~/Library/Application Support/Balatro")
    }

    #[inline]
    fn config_dir() -> PathBuf {
        PathBuf::from("~/Library/Dagger")
    }
}
