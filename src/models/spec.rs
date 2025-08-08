use std::{cell::RefCell, rc::Rc};

use mlua::{Error, Result, Value};
use serde::{Deserialize, Serialize};

use crate::DaggerSpecManager;

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct DaggerSpecification {
    pub uri: String,
    pub tag: Option<String>,
    pub branch: Option<String>,
}

impl DaggerSpecification {
    pub fn from_value(val: Value, specs: Rc<RefCell<DaggerSpecManager>>) -> Result<()> {
        fn force_str(val: Value) -> Option<String> {
            match val {
                Value::String(s) => Some(s.to_string_lossy()),
                _ => None,
            }
        }

        match val {
            Value::String(s) => {
                if let Ok(mut guard) = specs.try_borrow_mut() {
                    guard.insert(DaggerSpecification {
                        uri: s.to_string_lossy(),
                        tag: None,
                        branch: None,
                    });
                }
            }
            Value::Table(tbl) => {
                if (tbl.contains_key("tag").unwrap_or(false)
                    || tbl.contains_key("branch").unwrap_or(false))
                    && !tbl.contains_key(2).unwrap_or(true)
                {
                    if let Ok(mut guard) = specs.try_borrow_mut() {
                        guard.insert(DaggerSpecification {
                            uri: force_str(tbl.get(1)?)
                                .ok_or(Error::runtime("No URI were supplied in the mod spec."))?,
                            tag: force_str(tbl.get("tag")?),
                            branch: force_str(tbl.get("branch")?),
                        });
                    };

                    if let Ok(val) = tbl.get("require") {
                        Self::from_value(val, specs)?;
                    };
                } else {
                    for item in tbl.sequence_values() {
                        Self::from_value(item?, Rc::clone(&specs))?;
                    }
                }
            }
            Value::Nil => (),
            _ => return Err(Error::runtime("Invalid specification supplied.")),
        };

        Ok(())
    }

    // pub fn get_git_url(&self) -> String {}
}
