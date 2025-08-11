use thiserror::Error;
pub use DaggerError::*;

#[derive(Debug, Error, Clone)]
pub enum DaggerError {
    #[error("[Git] No git installation were detected in $PATH.\r\nPlease install git and/or ensure it's detectable from $PATH.")]
    GitNotInstalled,
    #[error("[Error/Lua] {}", .0)]
    LuaError(#[from] mlua::Error),
}
