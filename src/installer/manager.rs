use crate::{
    DagRes, DaggerError, DaggerLockfile, DaggerLockfileEntry, InstallCommandArgs,
    UpdateCommandArgs, cli::ListCommandArgs, installer::git::GitManager,
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

    pub fn list(&self, args: ListCommandArgs) -> DagRes<()> {
        if let Some(item) = args.cmd.as_item() {
            let entr = self.lock_files.get(item.id()).ok_or(DaggerError::runtime(
                "No such mod were installed with dagger.",
            ))?;

            println!("{} => {}", item.id(), entr);

            if args.list_tags {
                self.git
                    .list_tags(item.id())?
                    .iter()
                    .for_each(|tag| println!("\t{}", tag));
            }
        } else if args.cmd.is_all() {
            for (id, entr) in self.lock_files.iter() {
                println!("{} => {}", id, entr);

                if args.list_tags {
                    self.git
                        .list_tags(id)?
                        .iter()
                        .for_each(|tag| println!("\t{}", tag));
                }
            }
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
