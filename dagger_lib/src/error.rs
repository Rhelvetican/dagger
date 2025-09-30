use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TomlError {
    #[error("[Dagger/Toml] Error: {}", .0)]
    Deserialize(#[from] toml::de::Error),
    #[error("[Dagger/Toml] Error: {}", .0)]
    Serialize(#[from] toml::ser::Error),
}

/// An enum encompassing all possible point of failure in Dagger.
#[derive(Debug, Error)]
pub enum DaggerError {
    /// An error with `git`.
    #[error("[Dagger/Git] Error: {}", .0)]
    Git(#[from] git2::Error),
    /// An error deserializing/serializing TOML documents.
    #[error("[Dagger/Toml] Error: {}", .0)]
    Toml(#[from] TomlError),
    /// An error in IO operation.
    #[error("[Dagger/IO] Error: {}", .0)]
    Io(#[from] io::Error),
    /// An error that occurred during runtime.
    #[error("[Dagger/Runtime] Error: {}", .0)]
    Runtime(&'static str),
}

impl DaggerError {
    #[inline]
    pub fn runtime(msg: &'static str) -> Self {
        Self::Runtime(msg)
    }
}

pub type Result<T> = ::std::result::Result<T, DaggerError>;
