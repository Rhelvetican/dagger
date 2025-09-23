use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FmtRes},
    fs::File,
    io::{BufWriter, Read, Write},
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use toml::ser::Buffer;

use crate::{
    DaggerPathApi, PathImpl,
    error::{DagRes, TomlError},
};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct DaggerLockfile {
    #[serde(flatten)]
    inner: HashMap<String, DaggerLockfileEntry>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct DaggerLockfileEntry {
    branch: String,
    commit: String,
}

impl Display for DaggerLockfileEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtRes {
        write!(
            f,
            "Current branch: {}, Current commit: {}",
            self.branch(),
            self.commit()
        )
    }
}

impl DaggerLockfileEntry {
    #[inline]
    pub fn new(branch: String, commit: String) -> Self {
        Self { branch, commit }
    }

    #[inline]
    pub fn set_branch(&mut self, branch: String) {
        self.branch = branch;
    }

    #[inline]
    pub fn set_commit(&mut self, commit: String) {
        self.commit = commit;
    }

    #[inline]
    pub fn branch(&self) -> &str {
        &self.branch
    }

    #[inline]
    pub fn commit(&self) -> &str {
        &self.commit
    }
}

impl DaggerLockfile {
    #[inline]
    pub fn get_lock_path() -> PathBuf {
        PathImpl::config_dir().as_path().join("daggerLock.toml")
    }

    pub fn load() -> DagRes<Self> {
        let lock_path = Self::get_lock_path();
        let mut reader = File::open(&lock_path)?;
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        Ok(toml::from_str(&buf).map_err(TomlError::from)?)
    }

    pub fn save(&self) -> DagRes<()> {
        let lock_path = Self::get_lock_path();
        let mut buf = BufWriter::new(File::create(&lock_path)?);

        let mut tbuf = Buffer::new();
        let ser = toml::Serializer::pretty(&mut tbuf);
        self.serialize(ser).map_err(TomlError::from)?;
        buf.write_all(tbuf.to_string().as_bytes())?;

        Ok(())
    }
}

impl Deref for DaggerLockfile {
    type Target = HashMap<String, DaggerLockfileEntry>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for DaggerLockfile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
