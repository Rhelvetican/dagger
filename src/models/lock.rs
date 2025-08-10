use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::from_reader;

use crate::{DaggerError, Directories};

fn __default_lock_path() -> PathBuf {
    let mut path = Directories::config_dir();
    path.push("dagger_lock.json");
    path
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LockObject {}

#[derive(Debug, Deserialize, Serialize)]
pub struct DaggerLock {
    #[serde(skip, default = "__default_lock_path")]
    path: PathBuf,
    #[serde(flatten)]
    locker: HashMap<Box<str>, LockObject>,
}

impl DaggerLock {
    #[inline]
    fn load_from_storage() -> Result<Self, DaggerError> {
        let mut reader = BufReader::new(File::open(__default_lock_path())?);
        Ok(from_reader(&mut reader)?)
    }

    pub fn init() -> Self {
        if let Ok(this) = Self::load_from_storage() {
            return this;
        }

        Self {
            path: __default_lock_path(),
            locker: HashMap::new(),
        }
    }

    pub fn update(&mut self, key: &str) {}
}
