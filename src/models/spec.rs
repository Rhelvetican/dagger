use mlua::{Error, FromLua, Lua, Result, Table, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DaggerSpecification {
    repo: String,
    tag: Option<String>,
    branch: Option<String>,
}

impl DaggerSpecification {
    #[inline]
    pub fn new<S: ToString>(repo: S) -> Self {
        Self {
            repo: repo.to_string(),
            tag: None,
            branch: None,
        }
    }

    pub fn from_tbl(tbl: &Table) -> Result<Self> {
        todo!()
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
