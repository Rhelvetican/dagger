use thiserror::Error;

#[derive(Debug, Error)]
pub enum TomlError {
    #[error("Error serializing TOML document: {}", .0)]
    Ser(#[from] toml::ser::Error),
    #[error("Error deserializing TOML document: {}", .0)]
    De(#[from] toml::de::Error),
}

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Dagger: {}", .0)]
    DaggerInternal(#[from] dagger_lib::DaggerError),
    #[error("TOML: {}", .0)]
    Toml(#[from] TomlError),
    #[error("Clap: {}", .0)]
    Clap(#[from] clap::Error),
}

impl CliError {
    #[inline]
    pub fn runtime(msg: &'static str) -> Self {
        Self::DaggerInternal(dagger_lib::DaggerError::runtime(msg))
    }
}

pub type Result<T> = std::result::Result<T, CliError>;
