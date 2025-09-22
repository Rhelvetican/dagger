use std::fs::{remove_dir_all, remove_file};

use git2::{
    AutotagOption, Error as GitError, FetchOptions, ObjectType, RemoteCallbacks,
    build::{CheckoutBuilder, RepoBuilder},
};

use crate::{
    DagRes, DaggerPathApi, InstallCommandArgs, PathImpl, utils::spinner::TransferProgress,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct GitManager;

impl GitManager {
    pub fn install(&self, args: &mut InstallCommandArgs) -> DagRes<(String, String)> {
        let id = &*args.get_id().to_string().into_boxed_str();

        let mut spinner = TransferProgress::new(TransferProgress::construct_spinner(&format!(
            "Installing {}...\r\n",
            id
        )));

        let mut remote_callbacks = RemoteCallbacks::new();
        remote_callbacks.transfer_progress(|prog| spinner.update(prog));

        let mut fetch_opts = FetchOptions::new();
        fetch_opts.depth(1).remote_callbacks(remote_callbacks);

        let mut repo = RepoBuilder::new();

        if let Some(branch) = args.branch.as_deref() {
            repo.branch(branch);
        }

        if args.tag.is_some() {
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
        let repo = repo.clone(&args.url, &install_path)?;

        if let Some(tag) = args.tag.as_deref() {
            let refer;
            if let Ok(reference) = repo.find_reference(&format!("refs/tags/{}", tag)) {
                refer = reference;
            } else {
                refer = repo.find_reference(&format!("refs/tags/v{}", tag))?
            };

            let commit_obj = refer.peel(ObjectType::Commit)?;

            let commit_tag = commit_obj
                .as_commit()
                .ok_or(GitError::from_str("Tag does not points to a commit."))?;

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
}
