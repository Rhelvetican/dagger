use crate::{CowStr, Result};

#[derive(Debug, Clone, PartialEq)]
pub struct Metadata<'a> {
    branch: CowStr<'a>,
    commit: CowStr<'a>,
}

pub trait DaggerModManagerApi<'a> {
    fn install<I: InstallArgs>(&mut self, args: I) -> Result<Metadata<'a>>;
    fn update<U: UpgradeArgs>(&mut self, args: U) -> Result<Metadata<'a>>;
    fn uninstall<U: UninstallArgs>(&mut self, args: U) -> Result<()>;
    fn list<L: ListArgs>(&self, args: L) -> Result<()>;
}

pub trait RepoDetails {
    fn tag(&self) -> Option<CowStr<'_>>;
    fn branch(&self) -> Option<CowStr<'_>>;
}

pub trait InstallArgs: RepoDetails {
    fn url(&self) -> CowStr<'_>;
    fn id(&self) -> CowStr<'_>;
}

pub trait UpgradeArgs: RepoDetails {
    fn id(&self) -> CowStr<'_>;
}

pub trait ListArgs {
    fn id(&self) -> CowStr<'_>;
}

pub trait UninstallArgs {
    fn id(&self) -> CowStr<'_>;
}
