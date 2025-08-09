use thiserror::Error;

#[derive(Debug, Error)]
pub enum DaggerError {
    #[error("[Lua] {}", .0)]
    LuaError(#[from] mlua::Error),
    #[error("[Git] {}", .0)]
    GitError(#[from] git2::Error),
    #[error("[IO] {}", .0)]
    IoError(#[from] std::io::Error),
    #[error("[Other] {}", .0)]
    Other(String),
}

impl DaggerError {
    #[inline]
    pub fn custom_error<S: ToString>(msg: S) -> Self {
        Self::Other(msg.to_string())
    }
}
