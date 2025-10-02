use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    sync::LazyLock,
};

use dagger_lib::{
    DaggerError, DaggerPaths, GitCallback, InstallArgs, ListArgs, Metadata, UninstallArgs,
    UpgradeArgs, git2::Repository,
};
use serde::{Deserialize, Serialize};
use toml::{Deserializer, Serializer, ser::Buffer};

use crate::{
    error::{CliError, Result, TomlError},
    git::{GitManager, Repo},
};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DaggerModManager {
    #[serde(flatten)]
    status: HashMap<String, Metadata>,
    #[serde(skip, default)]
    internal: GitManager,
}

static DAGGER_LOCK_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| DaggerPaths::config_dir().join("dagger_lock.toml"));

impl DaggerModManager {
    #[inline]
    pub fn new() -> Self {
        let mut buf = String::new();

        File::open(&*DAGGER_LOCK_PATH)
            .map_err(DaggerError::from)
            .map_err(CliError::from)
            .and_then(|mut f| {
                f.read_to_string(&mut buf)
                    .map_err(DaggerError::from)
                    .map_err(CliError::from)
            })
            .and_then(|_| {
                Deserializer::parse(&buf)
                    .map_err(TomlError::from)
                    .map_err(CliError::from)
            })
            .and_then(|de| {
                Self::deserialize(de)
                    .map_err(TomlError::from)
                    .map_err(CliError::from)
            })
            .unwrap_or_default()
    }

    #[inline]
    pub fn get_mod_ids(&self) -> Vec<String> {
        self.status.keys().cloned().collect()
    }

    #[inline]
    pub fn install<I, Cb>(&mut self, args: I, cb: Option<&mut Cb>) -> Result<()>
    where
        I: InstallArgs,
        Cb: GitCallback,
    {
        let (id, metadata) = (args.id().to_string(), self.internal.install(args, cb)?);
        self.status.insert(id, metadata.clone());
        Ok(())
    }

    #[inline]
    pub fn uninstall<U>(&mut self, args: U) -> Result<()>
    where
        U: UninstallArgs,
    {
        self.status.remove(&args.id().to_string());
        self.internal.uninstall(args)?;
        Ok(())
    }

    #[inline]
    pub fn update<U, Cb>(&mut self, args: U, cb: Option<&mut Cb>) -> Result<()>
    where
        U: UpgradeArgs,
        Cb: GitCallback,
    {
        let (id, metadata) = (args.id().to_string(), self.internal.update(args, cb)?);
        self.status.insert(id, metadata.clone());
        Ok(())
    }

    pub fn list<L>(&self, args: L, show_tags: bool) -> Result<()>
    where
        L: ListArgs,
    {
        let Some(item) = self.status.get(args.id().as_str()) else {
            return Err(CliError::runtime("No such mod found."));
        };

        print!("  Mod: {}", args.id());

        if show_tags {
            print!(
                r#"
    │ │
    │ │ ╭─ Current branch: {} 
    │ ╰─┴─ Current commit: {} 
    │
    ╰─── Tags:"#,
                item.branch(),
                item.commit()
            );

            for tag in self.internal.get_tags(args)? {
                println!("      + {}", tag)
            }
        } else {
            print!(
                r#"
    │
    │ ╭─ Current branch: {}
    ╰─┴─ Current commit: {}
                "#,
                item.branch(),
                item.commit()
            );
        }

        println!("\n\n\n");
        Ok(())
    }

    pub fn refresh(&mut self) {
        self.status
            .retain(|entr, _| DaggerPaths::balatro_mod_dir().join(entr).is_dir());
    }

    pub fn save(&self) -> Result<()> {
        let mut buf = Buffer::new();
        let ser = Serializer::pretty(&mut buf);
        self.serialize(ser).map_err(TomlError::from)?;

        let mut file = File::create(&*DAGGER_LOCK_PATH).map_err(DaggerError::from)?;
        file.write_all(buf.to_string().as_bytes())
            .map_err(DaggerError::from)?;

        Ok(())
    }
}

impl Drop for DaggerModManager {
    fn drop(&mut self) {
        self.refresh();
        self.save().unwrap()
    }
}
