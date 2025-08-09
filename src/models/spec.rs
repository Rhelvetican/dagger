use std::{cell::RefCell, rc::Rc};

use mlua::{Error, Result, Value};

use crate::DaggerSpecManager;

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UpdateSource {
    #[default]
    GitHub,
    GitServices(Box<str>),
    Local,
}

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq)]
pub struct DaggerSpecification {
    pub tag: Option<Box<str>>,
    pub branch: Option<Box<str>>,
    pub src: UpdateSource,
}

impl DaggerSpecification {
    pub fn from_value(val: Value, specs: Rc<RefCell<DaggerSpecManager>>) -> Result<()> {
        match val {
            Value::String(s) => {
                if let Ok(mut guard) = specs.try_borrow_mut() {
                    guard.insert(
                        &s.to_str()?,
                        DaggerSpecification {
                            tag: None,
                            branch: None,
                            src: UpdateSource::GitHub,
                        },
                    );
                }
            }
            Value::Table(tbl) => {
                if (tbl.contains_key("tag").unwrap_or(false)
                    || tbl.contains_key("branch").unwrap_or(false)
                    || tbl.contains_key("method").unwrap_or(false))
                    && !tbl.contains_key(2).unwrap_or(true)
                {
                    if let Ok(mut guard) = specs.try_borrow_mut() {
                        let src = {
                            if let Ok(src) = tbl.get::<Value>("method")
                                && let Some(src) = src.as_string().and_then(|s| s.to_str().ok())
                            {
                                if src == "local" || src == "Local" {
                                    UpdateSource::Local
                                } else {
                                    UpdateSource::GitServices(Box::from(&*src))
                                }
                            } else {
                                UpdateSource::GitHub
                            }
                        };

                        guard.insert_owned(
                            tbl.get::<Box<str>>(1).map_err(|_| {
                                Error::runtime("No URI were supplied in the mod spec.")
                            })?,
                            DaggerSpecification {
                                tag: tbl.get("tag")?,
                                branch: tbl.get("branch")?,
                                src,
                            },
                        );
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

    pub fn get_git_url(&self, url: &str) -> String {
        match &self.src {
            UpdateSource::Local => url.to_string(),
            UpdateSource::GitServices(src) => format!("{}/{}.git", src, url),
            UpdateSource::GitHub => format!("https://www.github.com/{}.git", url),
        }
    }
}
