use crate::{CowStr, InstallArgs, ListArgs, RepoDetails, UninstallArgs, UpgradeArgs};

impl RepoDetails for str {
    #[inline]
    fn tag(&self) -> Option<CowStr<'_>> {
        None
    }

    #[inline]
    fn branch(&self) -> Option<CowStr<'_>> {
        None
    }
}

impl InstallArgs for str {
    fn url(&self) -> CowStr<'_> {
        if self.starts_with("https://") || self.starts_with("http://") {
            return CowStr::new(self);
        }

        CowStr::new(format!(
            "https://www.github.com/{}.git",
            self.trim_end_matches(".git")
        ))
    }

    #[inline]
    fn id(&self) -> CowStr<'_> {
        CowStr::new(self.split('/').next_back().unwrap_or(self))
    }
}

impl UpgradeArgs for str {
    #[inline]
    fn id(&self) -> CowStr<'_> {
        CowStr::new(self)
    }
}

impl ListArgs for str {
    #[inline]
    fn id(&self) -> CowStr<'_> {
        CowStr::new(self)
    }
}

impl UninstallArgs for str {
    #[inline]
    fn id(&self) -> CowStr<'_> {
        CowStr::new(self)
    }
}

impl RepoDetails for &str {
    #[inline]
    fn tag(&self) -> Option<CowStr<'_>> {
        None
    }

    #[inline]
    fn branch(&self) -> Option<CowStr<'_>> {
        None
    }
}

impl InstallArgs for &str {
    fn url(&self) -> CowStr<'_> {
        if self.starts_with("https://") || self.starts_with("http://") {
            return CowStr::new(*self);
        }

        CowStr::new(format!(
            "https://www.github.com/{}.git",
            self.trim_end_matches(".git")
        ))
    }

    #[inline]
    fn id(&self) -> CowStr<'_> {
        CowStr::new(self.split('/').next_back().unwrap_or(self))
    }
}

impl UpgradeArgs for &str {
    #[inline]
    fn id(&self) -> CowStr<'_> {
        CowStr::new(*self)
    }
}

impl ListArgs for &str {
    #[inline]
    fn id(&self) -> CowStr<'_> {
        CowStr::new(*self)
    }
}

impl UninstallArgs for &str {
    #[inline]
    fn id(&self) -> CowStr<'_> {
        CowStr::new(*self)
    }
}
