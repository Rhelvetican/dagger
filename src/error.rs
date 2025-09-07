use thiserror::Error;

#[derive(Error, Debug)]
pub enum TomlError {
    #[error("{}", .0)]
    Serializing(#[from] toml::ser::Error),
    #[error("{}", .0)]
    Deserializing(#[from] toml::de::Error),
}

#[derive(Error, Debug)]
pub enum DaggerError {
    #[error("[[Dagger/TOML] - Error]: {}", .0)]
    Toml(#[from] TomlError),
    #[error("[[Dagger/JSON] - Error]: {}", .0)]
    Json(#[from] serde_json::Error),
    #[error("[[Dagger/IO] - Error]: {}", .0)]
    Io(#[from] std::io::Error),
}
