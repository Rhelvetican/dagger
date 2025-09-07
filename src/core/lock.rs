use serde::{Deserialize, Serialize};
use serde_json::{from_str, ser::PrettyFormatter, Serializer};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufWriter, Read},
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

use crate::{
    err::DaggerError,
    path::{DaggerPaths, DaggerPathsImpl},
};

#[inline]
fn _default_lockfile_location() -> PathBuf {
    DaggerPathsImpl::config_dir().join("dagger-lock.json")
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Lock {
    #[serde(skip, default = "_default_lockfile_location")]
    src: PathBuf,
    #[serde(flatten)]
    pub entries: HashMap<String, LockEntry>,
}

impl DerefMut for Lock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entries
    }
}

impl Deref for Lock {
    type Target = HashMap<String, LockEntry>;

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl Default for Lock {
    fn default() -> Self {
        let src = _default_lockfile_location();
        if let Ok(ref mut file) = File::open(&src) {
            let mut buf = String::new();
            if file.read_to_string(&mut buf).is_ok_and(|n| n > 0)
                && let Ok(entries) = from_str(&buf)
            {
                return Self { src, entries };
            }
        }

        Self {
            src,
            entries: HashMap::new(),
        }
    }
}

impl Lock {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn path(&self) -> &Path {
        &self.src
    }

    pub fn save(&self) -> Result<(), DaggerError> {
        let mut file = BufWriter::new(
            OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(self.path())?,
        );

        let mut ser = Serializer::with_formatter(&mut file, PrettyFormatter::with_indent(b"    "));
        self.serialize(&mut ser)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LockEntry {
    pub branch: String,
    pub commit: String,
}

impl LockEntry {
    #[inline]
    pub fn new(branch: String, commit: String) -> Self {
        Self { branch, commit }
    }
}
