use serde::{Deserialize, Serialize};

use crate::{CowStr, GitCallback};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    branch: String,
    commit: String,
    tag: Option<String>,
}

impl Metadata {
    #[inline]
    pub fn new(branch: String, commit: String, tag: Option<String>) -> Self {
        Self {
            branch,
            commit,
            tag,
        }
    }

    #[inline]
    pub fn branch(&self) -> &str {
        self.branch.as_str()
    }

    #[inline]
    pub fn commit(&self) -> &str {
        self.commit.as_str()
    }

    pub fn tag(&self) -> Option<&str> {
        self.tag.as_deref()
    }
}

pub trait DaggerModManagerApi {
    type Metadata;
    type Result<T>;

    fn install<I, Cb>(&mut self, args: I, cb: &mut Cb) -> Self::Result<Self::Metadata>
    where
        I: InstallArgs,
        Cb: GitCallback;

    fn update<U, Cb>(&mut self, args: U, cb: &mut Cb) -> Self::Result<Self::Metadata>
    where
        U: UpgradeArgs,
        Cb: GitCallback;

    fn uninstall<U: UninstallArgs>(&mut self, args: U) -> Self::Result<()>;

    fn list<L: ListArgs>(&self, args: L, show_tags: bool) -> Self::Result<()>;
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
