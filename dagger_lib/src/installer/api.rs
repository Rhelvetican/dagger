use std::borrow::Cow;

use git2::Progress;

pub type Str<'a> = Cow<'a, str>;

/// Shared behaviours of an installable Dagger mod.
pub trait InstallableMod {
    /// Get the URL to install from.
    fn get_url(&self) -> Str<'_>;
    /// Get the ID of the mod.
    fn get_id(&self) -> Str<'_>;
    /// Get the mod branch to install from.
    fn get_branch(&self) -> Option<Str<'_>>;
    /// Get the tag/release of the mod.
    fn get_tag(&self) -> Option<Str<'_>>;
}

/// Shared behaviours of an upgradable Dagger mod.
pub trait UpgradableMod {
    /// Get the ID of the mod.
    fn get_id(&self) -> Str<'_>;
    /// Get the mod branch to install from if it has one.
    fn get_branch(&self) -> Option<Str<'_>>;
    /// Get the tag/release of the mod if it has one.
    fn get_tag(&self) -> Option<Str<'_>>;
}

/// Shared behaviours of an listable Dagger mod.
pub trait ListableMod {
    /// Get the ID of the mod.
    fn get_id(&self) -> Option<Str<'_>>;
    /// Whether to list the tags or not.
    fn list_tags(&self) -> bool;
}

pub trait GitCallback {
    fn callback(&mut self, progress: Progress<'_>) -> bool;
}
