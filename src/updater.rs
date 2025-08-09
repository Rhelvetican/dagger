use std::{collections::HashMap, path::Path};

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
    pub fn update_all(&self) -> Result<(), Error> {
        for (url, spec) in self.specs.iter() {
            let mut repo = RepoBuilder::new();
            let (git_url, (_, to)) = (
                spec.get_git_url(url),
                url.split_once('/').unwrap_or((url, "")),
            );

            let dir = self.base_path.join(to);

            if let Some(branch) = &spec.branch {
                repo.branch(branch);
            }

            repo.clone(&git_url, dir.as_path())?;
        }

        Ok(())
    }

    pub fn update(&self, mod_entry: &str) -> Result<Repository, Error> {
        if let Some(spec_ref) = self.specs.get(mod_entry) {
            let mut repo = RepoBuilder::new();
            let (git_url, (_, to)) = (
                spec_ref.get_git_url(mod_entry),
                mod_entry.split_once('/').unwrap_or((mod_entry, "")),
            );

            let dir = self.base_path.join(to);

            if let Some(branch) = &spec_ref.branch {
                repo.branch(branch);
            }

            repo.clone(&git_url, dir.as_path())
        } else {
            Err(Error::from_str(
                "No such mod were found in the Dagger manifest.",
            ))
        }
    }
}
