use std::path::PathBuf;

use mlua::{Error, FromLua, Lua, Result, Table, Value};
use serde::{Deserialize, Serialize};

use crate::utils::lua_optional;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DaggerSpecification {
    uri: String,
    tag: Option<String>,
    branch: Option<String>,
    depends: Option<Vec<DaggerSpecification>>,
}

impl DaggerSpecification {
    #[inline]
    pub fn new<S: ToString>(uri: S) -> Self {
        Self {
            uri: uri.to_string(),
            tag: None,
            branch: None,
            depends: None,
        }
    }

    pub fn from_lua_tbl(tbl: &Table, lua: &Lua) -> Result<Self> {
        let uri = tbl.get::<String>(1)?;
        let mut inst = Self::new(uri);

        if let Ok(tag) = tbl.get::<Value>("tag") {
            inst.tag = lua_optional(tag, lua);
        }

        if let Ok(branch) = tbl.get::<Value>("branch") {
            inst.branch = lua_optional(branch, lua);
        }

        if let Ok(depends) = tbl.get::<Value>("depends") {
            inst.depends = lua_optional(depends, lua);
        }

        Ok(inst)
    }
}

impl FromLua for DaggerSpecification {
    fn from_lua(value: Value, _: &Lua) -> Result<Self> {
        match value {
            Value::String(s) => {
                let s = s.to_str()?;
                Ok(Self::new(s))
            }

            Value::Table(tbl) => {
                todo!()
            }

            _ => Err(Error::runtime("Incorrect specification supplied.")),
        }
    }
}
