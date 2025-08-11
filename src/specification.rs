use std::collections::HashMap;

use mlua::{Error, Result, Table, Value};

use crate::BoxedStr;

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UpdateSource {
    #[default]
    GitHub,
    GitServices(BoxedStr),
    Local,
}

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq)]
pub struct DaggerSpecification {
    pub tag: Option<BoxedStr>,
    pub branch: Option<BoxedStr>,
    pub src: UpdateSource,
}

impl DaggerSpecification {
    pub fn from_value(
        val: Value,
        specs: &mut HashMap<BoxedStr, DaggerSpecification>,
    ) -> Result<()> {
        match val {
            Value::String(s) => {
                specs.insert(
                    BoxedStr::from(&*s.to_str()?),
                    DaggerSpecification {
                        tag: None,
                        branch: None,
                        src: UpdateSource::GitHub,
                    },
                );
            }

            Value::Table(tbl) => {
                if (tbl.contains_key("tag").unwrap_or(false)
                    || tbl.contains_key("branch").unwrap_or(false)
                    || tbl.contains_key("method").unwrap_or(false)
                    || tbl.contains_key("dependencies").unwrap_or(false))
                    && !tbl.contains_key(2).unwrap_or(true)
                {
                    let src = {
                        if let Ok(src) = tbl.get::<Value>("method")
                            && let Some(src) = src.as_string().and_then(|s| s.to_str().ok())
                        {
                            match &*src {
                                "local" | "Local" => UpdateSource::Local,

                                src if src.starts_with("git:") => {
                                    UpdateSource::GitServices(BoxedStr::from(unsafe {
                                        src.strip_prefix("git:").unwrap_unchecked()
                                    }))
                                }

                                src if src.starts_with("Git:") => {
                                    UpdateSource::GitServices(BoxedStr::from(unsafe {
                                        src.strip_prefix("Git:").unwrap_unchecked()
                                    }))
                                }

                                _ => {
                                    return Err(Error::runtime(
                                        "[Lua] Unknown update method were supplied.",
                                    ))
                                }
                            }
                        } else {
                            UpdateSource::GitHub
                        }
                    };

                    specs.insert(
                        tbl.get::<BoxedStr>(1)
                            .map_err(|_| Error::runtime("No URI were supplied in the mod spec."))?,
                        DaggerSpecification {
                            tag: tbl.get("tag")?,
                            branch: tbl.get("branch")?,
                            src,
                        },
                    );

                    if let Ok(deps) = tbl.get::<Table>("dependencies") {
                        Self::from_value(Value::Table(deps), specs)?;
                    };
                } else {
                    for item in tbl.sequence_values().filter_map(|i| i.ok()) {
                        Self::from_value(item, specs)?;
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
