use thiserror::Error;

#[derive(Error, Debug)]
pub enum DaggerError {
    #[error("[Dagger/IO]: Error: {}", .0)]
    Io(#[from] std::io::Error),
    #[error("[Dagger/JSON]: Error: {}", .0)]
    Json(#[from] serde_json::Error),
    #[error("[Dagger/Lua]: Error: {}", .0)]
    Lua(#[from] mlua::Error),
    #[error("[Dagger/Runtime]: Error: {}",.0)]
    Runtime(String),
}

impl DaggerError {
    #[inline]
    pub fn runtime<S: ToString>(s: S) -> Self {
        Self::Runtime(s.to_string())
    }
}
