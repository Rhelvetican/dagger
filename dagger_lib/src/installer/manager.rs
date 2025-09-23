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
            args.get_id().to_string(),
            DaggerLockfileEntry::new(branch, commit),
        );

        self.lock_files.save()?;

        Ok(())
    }

    pub fn list_item<L>(&self, args: L) -> DagRes<()>
    where
        L: ListableMod,
    {
        let Some(item) = args.get_id().and_then(|s| self.lock_files.get(&*s)) else {
            return Err(DaggerError::runtime("No such mod found."));
        };

        println!("{} => {}", args.get_id().unwrap(), item);

        if args.list_tags() {
            self.git
                .list_tags(&args.get_id().unwrap())?
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
            .get_mut(&*args.get_id())
            .ok_or(DaggerError::runtime("No such mod is found."))?;

        entr.set_branch(branch);
        entr.set_commit(commit);

        Ok(())
    }

    pub fn update_all<'a, CbCon, Cb>(&'a mut self, mut cb: Option<CbCon>) -> DagRes<()>
    where
        Cb: GitCallback,
        CbCon: Fn(&'a str) -> Cb,
    {
        for (id, entr) in self.lock_files.iter_mut() {
            let mut cbref = cb.as_mut().map(|s| s(id));
            let (branch, commit) = self.git.update_with_id(id, cbref.as_mut())?;

            entr.set_branch(branch);
            entr.set_commit(commit);
        }

        Ok(())
    }
}
