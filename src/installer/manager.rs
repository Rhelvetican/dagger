use crate::{
    DagRes, DaggerLockfile, DaggerLockfileEntry, InstallCommandArgs, UpdateCommandArgs,
    installer::git::GitManager,
};

#[derive(Debug, Default)]
pub struct DaggerModManager {
    lock_files: DaggerLockfile,
    git: GitManager,
}

impl DaggerModManager {
    #[inline]
    pub fn new(lock_files: DaggerLockfile) -> Self {
        Self {
            lock_files,
            git: GitManager,
        }
    }

    pub fn install(&mut self, mut args: InstallCommandArgs) -> DagRes<()> {
        let (branch, commit) = self.git.install(&mut args)?;

        let entr = self.lock_files.insert(
            args.get_id().to_string(),
            DaggerLockfileEntry::new(branch, commit),
        );

        Ok(())
    }

    pub fn update(&mut self, args: UpdateCommandArgs) -> DagRes<()> {
        todo!()
    }
}
