use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    path::Path,
    str::FromStr,
};

use git2::Repository;
use mlua::{Error, Value};

use crate::err::DaggerError;

#[derive(Debug, Clone, Default)]
pub enum ModSourceType {
    #[default]
    Github,
    Local,
    GitService {
        fmt: String,
    },
}

#[derive(Debug, Clone, Default)]
pub struct ModSpec {
    src_ty: ModSourceType,
    branch: Option<String>,
    commit: Option<String>,
    directory: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ModSpecifications {
    inner: HashMap<String, ModSpec>,
}

impl ModSpecifications {
    pub fn new() -> Self {
        Self::default()
    }
}

impl DerefMut for ModSpecifications {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Deref for ModSpecifications {
    type Target = HashMap<String, ModSpec>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl FromStr for ModSpec {
    type Err = DaggerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mod_path = AsRef::<Path>::as_ref(s);
        if mod_path.is_dir()
            && let Some(mod_name) = mod_path.components().last()
            && mod_path
                .join(format!("{}.json", mod_name.as_os_str().display()))
                .is_file()
        {
            return Ok(Self {
                src_ty: ModSourceType::Local,
                directory: mod_name.as_os_str().to_str().map(|s| s.to_owned()),
                ..Default::default()
            });
        }

        if s.split('/').count() == 2 {
            return Ok(Self {
                src_ty: ModSourceType::Github,
                ..Default::default()
            });
        }
    }
}

impl ModSpec {
    pub fn from_lua_single(lua_value: Value) -> Result<Self, DaggerError> {
        match lua_value {
            Value::String(s) => s.to_string_lossy().parse(),
            Value::Table(tbl) => {
                let src_ty = tbl
                    .get::<String>("source")
                    .ok()
                    .and_then(|s| match &*(s.to_lowercase()) {
                        "local" => Some(ModSourceType::Local),
                        "github" => Some(ModSourceType::Github),
                        _ => Some(ModSourceType::GitService { fmt: s }),
                    })
                    .unwrap_or(ModSourceType::Github);

                let branch = tbl.get("branch");
            }

            _ => Err(DaggerError::Lua(Error::runtime(
                "Invalid config were supplied.",
            ))),
        }
    }
}
