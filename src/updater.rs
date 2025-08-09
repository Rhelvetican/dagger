use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use git2::{build::RepoBuilder, DescribeOptions, FetchOptions, RemoteCallbacks, Repository};

use crate::{err::DaggerError, DaggerSpecification};

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
    fn get_mod_path(&self, mod_id: &str) -> PathBuf {
        let (_, id) = mod_id.split_once('/').unwrap_or(("", mod_id));
        self.base_path.join(id)
    }

    fn update(&self, id: &str, spec: &DaggerSpecification) -> Result<(), DaggerError> {
        let mut callbacks = RemoteCallbacks::new();
        let mut finished = false;

        callbacks.transfer_progress(|stat| {
            if stat.total_objects() > 0 && !finished {
                println!(
                    "[Git] Receiving objects: {}/{} ({} bytes).",
                    stat.received_objects(),
                    stat.total_objects(),
                    stat.received_bytes()
                );
            }

            if !finished && stat.received_objects() == stat.total_objects() {
                finished = true;
            }

            true
        });

        let mut fetch_opts = FetchOptions::new();
        fetch_opts.remote_callbacks(callbacks);

        let path = self.get_mod_path(id);
        println!("[Dagger] Updating {}...", id);

        let repo = match Repository::open(&path) {
            Ok(repo) => {
                let mut origin = repo.find_remote("origin")?;
                origin.fetch::<&str>(&[], Some(&mut fetch_opts), None)?;
                drop(origin);

                repo
            }
            _ => {
                let mut repo_builder = RepoBuilder::new();
                repo_builder.fetch_options(fetch_opts);

                if let Some(branch) = &spec.branch {
                    repo_builder.branch(branch);
                }

                repo_builder.clone(&spec.get_git_url(id), &path)?
            }
        };

        if let Some(tag) = &spec.tag {
            let tag = if &**tag != "*" {
                match repo.revparse_single(&format!("refs/tags/v{}", tag)) {
                    Ok(tag) => tag,
                    _ => repo.revparse_single(&format!("refs/tags/{}", tag))?,
                }
            } else {
                let mut desc = DescribeOptions::new();
                desc.max_candidates_tags(1).describe_tags();
                let tag = repo.describe(&desc)?.format(None)?;
                repo.revparse_single(&format!("refs/tags/{}", tag))?
            };

            repo.checkout_tree(&tag, None)?;
            return Ok(());
        }

        Ok(())
    }

    pub fn update_all(&self) -> Result<(), DaggerError> {
        for (mod_entry, spec) in self.specs.iter() {
            self.update(mod_entry, spec)?;
        }

        Ok(())
    }

    pub fn update_single(&self, mod_entry: &str) -> Result<(), DaggerError> {
        let spec = self
            .specs
            .get(mod_entry)
            .ok_or(DaggerError::custom_error("Unknown mod supplied."))?;

        self.update(mod_entry, spec)
    }
}
