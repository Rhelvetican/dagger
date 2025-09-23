use crate::{
    DagRes, DaggerError, DaggerLockfile, DaggerLockfileEntry,
    installer::{
        api::{GitCallback, InstallableMod, ListableMod, UpgradableMod},
        git::GitManager,
    },
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

    pub fn install<I, Cb>(&mut self, args: I, cb: Option<&mut Cb>) -> DagRes<()>
    where
        I: InstallableMod,
        Cb: GitCallback,
    {
        let (branch, commit) = self.git.install(&args, cb)?;

        self.lock_files.insert(
            args.get_id()
                .unwrap_or_else(|| {
                    args.get_url()
                        .split("/")
                        .last()
                        .map(|s| s.trim_end_matches(".git"))
                        .unwrap_or_default()
                })
                .to_string(),
            DaggerLockfileEntry::new(branch, commit),
        );

        self.lock_files.save()?;

        Ok(())
    }

    pub fn list_item<L>(&self, args: L) -> DagRes<()>
    where
        L: ListableMod,
    {
        let Some(item) = self.lock_files.get(args.get_id()) else {
            return Err(DaggerError::runtime("No such mod found."));
        };

        println!("{} => {}", args.get_id(), item);

        if args.list_tags() {
            self.git
                .list_tags(args.get_id())?
                .iter()
                .for_each(|tag| println!("\t{}", tag));
        }

        Ok(())
    }

    pub fn list_all(&self, list_tags: bool) -> DagRes<()> {
        for (id, entr) in self.lock_files.iter() {
            println!("{} => {}", id, entr);

            if list_tags {
                self.git
                    .list_tags(id)?
                    .iter()
                    .for_each(|tag| println!("\t{}", tag));
            }
        }

        Ok(())
    }

    pub fn update_item<U, Cb>(&mut self, args: U, cb: Option<&mut Cb>) -> DagRes<()>
    where
        U: UpgradableMod,
        Cb: GitCallback,
    {
        let (branch, commit) = self.git.update(&args, cb)?;
        let entr = self
            .lock_files
            .get_mut(args.get_id())
            .ok_or(DaggerError::runtime("No such mod is found."))?;

        entr.set_branch(branch);
        entr.set_commit(commit);

        Ok(())
    }

    pub fn update_all<Cb>(&mut self, cb: Option<Cb>) -> DagRes<()>
    where
        Cb: Clone + GitCallback,
    {
        for (id, entr) in self.lock_files.iter_mut() {
            let mut cbref = cb.clone();
            let (branch, commit) = self.git.update_with_id(id, cbref.as_mut())?;

            entr.set_branch(branch);
            entr.set_commit(commit);
        }

        Ok(())
    }
}
