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

        self.lock_files.insert(
            args.get_id().to_string(),
            DaggerLockfileEntry::new(branch, commit),
        );

        self.lock_files.save()?;

        Ok(())
    }

    pub fn list(&self) -> DagRes<()> {
        for (id, entr) in self.lock_files.iter() {
            println!(
                "{} => Current branch: {}, Current commit: {}",
                id,
                entr.branch(),
                entr.commit()
            );
        }

        Ok(())
    }

    pub fn update(&mut self, args: UpdateCommandArgs) -> DagRes<()> {
        if args.cmd.is_all() {
            for (id, entr) in self.lock_files.iter_mut() {
                let (branch, commit) = self.git.update_with_id(id)?;
                entr.set_branch(branch);
                entr.set_commit(commit);
            }

            println!("All mods are up-to-date!");
        } else if let Some(item) = args.cmd.as_item() {
            let (branch, commit) = self.git.update(item)?;

            self.lock_files.entry(item.id.clone()).and_modify(|entr| {
                entr.set_branch(branch);
                entr.set_commit(commit);
            });
        }

        Ok(())
    }
}
