use std::fs::{remove_dir_all, remove_file};

use git2::{
    AutotagOption, Error as GitError, FetchOptions, ObjectType, Reference, RemoteCallbacks,
    Repository,
    build::{CheckoutBuilder, RepoBuilder},
};

use crate::{
    DagRes, DaggerPathApi, PathImpl,
    installer::api::{GitCallback, InstallableMod, UpgradableMod},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct GitManager;

impl GitManager {
    fn get_latest_tag<'a>(&self, repo: &'a Repository) -> DagRes<Reference<'a>> {
        let reference = repo
            .tag_names(None)?
            .iter()
            .flatten()
            .filter_map(|s| repo.find_reference(&format!("refs/tags/{}", s)).ok())
            .fold(None::<Reference>, |latest, this| {
                if let Some(latest) = latest {
                    if this
                        .peel_to_commit()
                        .map(|c| c.time().seconds())
                        .unwrap_or(i64::MIN)
                        > latest
                            .peel_to_commit()
                            .map(|c| c.time().seconds())
                            .unwrap_or(i64::MIN)
                    {
                        Some(this)
                    } else {
                        Some(latest)
                    }
                } else {
                    Some(this)
                }
            });

        Ok(reference.ok_or(GitError::from_str("Repository does not have tags."))?)
    }

    pub fn list_tags(&self, id: &str) -> DagRes<Vec<String>> {
        let install_path = PathImpl::balatro_mod_dir().join(id);
        let repo = Repository::open(&install_path)?;

        Ok(repo
            .tag_names(None)?
            .iter()
            .flatten()
            .map(ToString::to_string)
            .collect())
    }

    pub fn install<I, Cb>(&self, args: &I, callback: Option<&mut Cb>) -> DagRes<(String, String)>
    where
        I: InstallableMod,
        Cb: GitCallback,
    {
        let id = &*args.get_id().to_string().into_boxed_str();

        let mut remote_callbacks = RemoteCallbacks::new();

        if let Some(cb) = callback {
            remote_callbacks.transfer_progress(|pg| cb.callback(pg));
        }

        let mut fetch_opts = FetchOptions::new();
        fetch_opts.depth(1).remote_callbacks(remote_callbacks);

        let mut repo = RepoBuilder::new();

        if let Some(ref branch) = args.get_branch() {
            repo.branch(branch);
        }

        if args.get_tag().is_some() {
            fetch_opts.download_tags(AutotagOption::All);
        } else {
            fetch_opts.download_tags(AutotagOption::None);
        }

        let install_path = PathImpl::balatro_mod_dir().join(id);

        if install_path.try_exists().is_ok_and(|b| b) {
            let _ = remove_dir_all(&install_path);
            let _ = remove_file(&install_path);
        }

        repo.fetch_options(fetch_opts);
        let repo = repo.clone(&args.get_url(), &install_path)?;

        if let Some(tag) = args.get_tag() {
            let refer;

            if (tag) == "*" {
                refer = self.get_latest_tag(&repo)?;
            } else if let Ok(reference) = repo.find_reference(&format!("refs/tags/v{}", tag)) {
                refer = reference;
            } else {
                refer = repo.find_reference(&format!("refs/tags/{}", tag))?
            };

            let commit_obj = refer.peel(ObjectType::Commit)?;

            let commit_tag = commit_obj
                .as_commit()
                .ok_or(GitError::from_str("Tag does not point to a commit."))?;

            repo.checkout_tree(
                commit_tag.as_object(),
                Some(CheckoutBuilder::new().force().allow_conflicts(true)),
            )?;
        }

        let head = repo.head()?;

        Ok((
            head.shorthand().unwrap_or_default().to_string(),
            head.peel_to_commit()?.id().to_string(),
        ))
    }

    pub fn update_with_id<Cb>(
        &self,
        id: &str,
        callback: Option<&mut Cb>,
    ) -> DagRes<(String, String)>
    where
        Cb: GitCallback,
    {
        let mut remote_callbacks = RemoteCallbacks::new();

        if let Some(cb) = callback {
            remote_callbacks.transfer_progress(|pg| cb.callback(pg));
        }

        let mut fetch_opts = FetchOptions::new();
        fetch_opts
            .remote_callbacks(remote_callbacks)
            .download_tags(AutotagOption::None);

        let install_path = PathImpl::balatro_mod_dir().join(id);
        let repo = Repository::open(&install_path)?;

        let mut remote = repo.find_remote("origin")?;
        remote.fetch::<&str>(&[], Some(&mut fetch_opts), None)?;

        repo.checkout_head(Some(CheckoutBuilder::new().force().allow_conflicts(true)))?;

        let head = repo.head()?;
        Ok((
            head.shorthand().unwrap_or_default().to_string(),
            head.peel_to_commit()?.id().to_string(),
        ))
    }

    pub fn update<U, Cb>(&self, args: &U, callback: Option<&mut Cb>) -> DagRes<(String, String)>
    where
        U: UpgradableMod,
        Cb: GitCallback,
    {
        let mut remote_callbacks = RemoteCallbacks::new();
        if let Some(cb) = callback {
            remote_callbacks.transfer_progress(|pg| cb.callback(pg));
        }

        let mut fetch_opts = FetchOptions::new();
        fetch_opts.depth(1).remote_callbacks(remote_callbacks);

        let install_path = PathImpl::balatro_mod_dir().join(&*args.get_id());
        let repo = Repository::open(&install_path)?;

        if args.get_tag().is_some() {
            fetch_opts.download_tags(AutotagOption::All);
        }

        let mut remote = repo.find_remote("origin")?;

        if let Some(branch) = args.get_branch() {
            remote.fetch(&[&*branch], Some(&mut fetch_opts), None)?;
        } else {
            remote.fetch::<&str>(&[], Some(&mut fetch_opts), None)?;
        }

        if let Some(tag) = args.get_tag() {
            let refer;

            if (tag) == "*" {
                refer = self.get_latest_tag(&repo)?;
            } else if let Ok(reference) = repo.find_reference(&format!("refs/tags/v{}", tag)) {
                refer = reference;
            } else {
                refer = repo.find_reference(&format!("refs/tags/{}", tag))?
            };

            let commit_obj = refer.peel(ObjectType::Commit)?;

            let commit_tag = commit_obj
                .as_commit()
                .ok_or(GitError::from_str("Tag does not point to a commit."))?;

            repo.checkout_tree(
                commit_tag.as_object(),
                Some(CheckoutBuilder::new().force().allow_conflicts(true)),
            )?;
        } else {
            repo.checkout_head(Some(CheckoutBuilder::new().force().allow_conflicts(true)))?;
        }

        let head = repo.head()?;
        Ok((
            head.shorthand().unwrap_or_default().to_string(),
            head.peel_to_commit()?.id().to_string(),
        ))
    }
}
