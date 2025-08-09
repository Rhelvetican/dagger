use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use git2::{build::RepoBuilder, Error, Repository};

use crate::DaggerSpecification;

#[derive(Debug, Clone)]
pub struct DaggerModManager<'dag> {
    base_path: &'dag Path,
    specs: HashMap<Box<str>, DaggerSpecification>,
}

impl<'dag> DaggerModManager<'dag> {
    #[inline]
    pub fn new<P: AsRef<Path>>(
        base_path: &'dag P,
        specs: HashMap<Box<str>, DaggerSpecification>,
    ) -> Self {
        Self {
            base_path: base_path.as_ref(),
            specs,
        }
    }
}

impl DaggerModManager<'_> {
    #[inline]
    pub fn get_mod_path(&self, mod_id: &str) -> PathBuf {
        let (_, id) = mod_id.split_once('/').unwrap_or(("", mod_id));
        self.base_path.join(id)
    }

    pub fn update_all(&self) -> Result<(), Error> {
        Ok(())
    }

    pub fn update(&self, mod_entry: &str) -> Result<(), Error> {
        Ok(())
    }
}
