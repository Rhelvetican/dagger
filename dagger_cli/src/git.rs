use std::{fs::remove_dir_all, path::Path, thread::sleep, time::Duration};

use dagger_lib::{
    git2::{
        build::{CheckoutBuilder, RepoBuilder},
        *,
    },
    *,
};

pub trait CleanUpPath {
    fn clean(&self) -> Result<&Self>;
}

impl CleanUpPath for Path {
    fn clean(&self) -> Result<&Path> {
        remove_dir_all(self)?;
        Ok(self)
    }
}

pub struct Repo {
    pub repo: Repository,
}

impl Repo {
    #[inline]
    pub fn new(repo: Repository) -> Self {
        Self { repo }
    }

    pub fn get_latest_tag(&self) -> Result<Reference<'_>> {
        self.repo
            .tag_names(None)
            .iter()
            .flatten()
            .filter_map(|s| {
                s.and_then(|s| self.repo.find_reference(&format!("refs/tags/{}", s)).ok())
            })
            .fold(None, |latest, this| {
                if latest.as_ref().is_some_and(|latest_ref: &Reference<'_>| {
                    this.peel_to_commit()
                        .map(|c| c.time().seconds())
                        .unwrap_or(i64::MIN)
                        < latest_ref
                            .peel_to_commit()
                            .map(|c| c.time().seconds())
                            .unwrap_or(i64::MIN)
                }) {
                    latest
                } else {
                    Some(this)
                }
            })
            .ok_or(DaggerError::runtime("Failed to get latest tag."))
    }

    pub fn list_tags(&self) -> Result<Vec<String>> {
        let mut tags = self
            .repo
            .tag_names(None)
            .iter()
            .flatten()
            .flatten()
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        tags.sort_by_key(|tag| {
            self.repo
                .find_reference(&format!("refs/tags/{}", tag))
                .and_then(|refer| refer.peel_to_commit().map(|c| c.time().seconds()))
                .unwrap_or(i64::MIN)
        });

        Ok(tags)
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct GitManager;

impl GitManager {
    pub fn install<I, Cb>(&self, args: I, cb: Option<&mut Cb>) -> Result<Metadata>
    where
        I: InstallArgs,
        Cb: GitCallback,
    {
        let id = args.id();
        let mut remote = RemoteCallbacks::new();

        if let Some(cb) = cb {
            remote.transfer_progress(|p| cb.callback(p));
        }

        let mut fetch_opts = FetchOptions::new();
        fetch_opts.depth(1).remote_callbacks(remote);

        let mut repo = RepoBuilder::new();

        if let Some(br) = args.branch().as_deref() {
            println!("{}'s branch was set to {}", args.id(), br);
            repo.branch(br);
        }

        if args.tag().is_some() {
            fetch_opts.download_tags(AutotagOption::All);
        } else {
            fetch_opts.download_tags(AutotagOption::Auto);
        }

        repo.fetch_options(fetch_opts);
        let repo = Repo::new(repo.clone(
            &args.url(),
            DaggerPaths::balatro_mod_dir().join(id.as_str()).clean()?,
        )?);

        if let Some(tag) = args.tag().as_deref() {
            let git_ref = if tag == "*" {
                repo.get_latest_tag()?
            } else {
                repo.repo
                    .find_reference(&format!("refs/tags/v{}", tag))
                    .or_else(|_| repo.repo.find_reference(&format!("refs/tags/{}", tag)))?
            };

            let commit = git_ref.peel_to_commit()?;

            repo.repo.checkout_tree(
                commit.as_object(),
                Some(
                    CheckoutBuilder::new()
                        .force()
                        .allow_conflicts(true)
                        .recreate_missing(true)
                        .remove_untracked(true)
                        .use_theirs(true),
                ),
            )?;
        }

        let head = repo.repo.head()?;

        Ok(Metadata::new(
            head.is_branch()
                .then(|| head.shorthand())
                .flatten()
                .unwrap_or_default()
                .to_string(),
            head.peel_to_commit()?.id().to_string(),
            match args.tag().as_ref().map(CowStr::as_str) {
                None => None,
                Some("*") => Some(
                    repo.get_latest_tag()?
                        .name()
                        .unwrap()
                        .trim_start_matches("refs/tags/")
                        .to_string(),
                ),
                Some(s) => Some(s.to_string()),
            },
        ))
    }

    pub fn update<U, Cb>(&self, args: U, cb: Option<&mut Cb>) -> Result<Metadata>
    where
        U: UpgradeArgs,
        Cb: GitCallback,
    {
        let mut remote_callbacks = RemoteCallbacks::new();
        if let Some(cb) = cb {
            remote_callbacks.transfer_progress(|pg| cb.callback(pg));
        }

        let mut fetch_opts = FetchOptions::new();
        fetch_opts.depth(1).remote_callbacks(remote_callbacks);

        let install_path = DaggerPaths::balatro_mod_dir().join(args.id().as_str());
        let repo = Repo::new(Repository::open(&install_path)?);

        if args.tag().is_some() {
            fetch_opts.download_tags(AutotagOption::All);
        }

        let mut remote = repo.repo.find_remote("origin")?;

        if let Some(branch) = args.branch().as_ref() {
            remote.fetch(&[branch.as_str()], Some(&mut fetch_opts), None)?;
        } else {
            remote.fetch::<&str>(&[], Some(&mut fetch_opts), None)?;
        }

        if let Some(tag) = args.tag().as_ref() {
            let refer = if (tag) == "*" {
                repo.get_latest_tag()?
            } else {
                repo.repo
                    .find_reference(&format!("refs/tags/v{}", tag))
                    .or_else(|_| repo.repo.find_reference(&format!("refs/tags/{}", tag)))?
            };

            let commit_obj = refer.peel_to_commit()?;

            repo.repo.checkout_tree(
                commit_obj.as_object(),
                Some(
                    CheckoutBuilder::new()
                        .force()
                        .allow_conflicts(true)
                        .recreate_missing(true)
                        .remove_untracked(true)
                        .use_theirs(true),
                ),
            )?;
        } else {
            repo.repo.checkout_head(Some(
                CheckoutBuilder::new()
                    .force()
                    .allow_conflicts(true)
                    .recreate_missing(true)
                    .remove_untracked(true)
                    .use_theirs(true),
            ))?;
        }

        let head = repo.repo.head()?;

        Ok(Metadata::new(
            head.shorthand().unwrap_or_default().to_string(),
            head.peel_to_commit()?.id().to_string(),
            match args.tag().as_ref().map(CowStr::as_str) {
                None => None,
                Some("*") => Some(
                    repo.get_latest_tag()?
                        .name()
                        .unwrap()
                        .trim_start_matches("refs/tags/")
                        .to_string(),
                ),
                Some(s) => Some(s.to_string()),
            },
        ))
    }

    pub fn uninstall<U: UninstallArgs>(&self, args: U) -> Result<()> {
        let install_path = DaggerPaths::balatro_mod_dir().join(args.id().as_str());
        remove_dir_all(&install_path)?;
        Ok(())
    }

    pub fn get_tags<L: ListArgs>(&self, args: L) -> Result<Vec<String>> {
        let install_path = DaggerPaths::balatro_mod_dir().join(args.id().as_str());
        let repo = Repo::new(Repository::open(&install_path)?);
        repo.list_tags()
    }
}
