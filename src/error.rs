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
}

pub type DagRes<T> = Result<T, DaggerError>;
