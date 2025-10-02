use std::io;

use thiserror::Error;

/// An enum encompassing all possible point of failure in Dagger.
#[derive(Debug, Error)]
pub enum DaggerError {
    /// An error with `git`.
    #[error("[Git] Error: {}", .0)]
    Git(#[from] git2::Error),
    /// An error in IO operation.
    #[error("[IO] Error: {}", .0)]
    Io(#[from] io::Error),
    /// An error that occurred during runtime.
    #[error("[Runtime] Error: {}", .0)]
    Runtime(&'static str),
}

impl DaggerError {
    #[inline]
    pub fn runtime(msg: &'static str) -> Self {
        Self::Runtime(msg)
    }
}

pub type Result<T> = ::std::result::Result<T, DaggerError>;
