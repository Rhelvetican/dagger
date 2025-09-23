/// Shared behaviours of an installable Dagger mod.
pub trait InstallableMod {
    /// Get the URL to install from.
    fn get_url(&self) -> &str;
    /// Get the ID of the mod if it has one.
    fn get_id(&self) -> Option<&str>;
    /// Get the mod branch to install from.
    fn get_branch(&self) -> Option<&str>;
    /// Get the tag/release of the mod.
    fn get_tag(&self) -> Option<&str>;
}

/// Shared behaviours of an upgradable Dagger mod.
pub trait UpgradableMod {
    /// Get the ID of the mod.
    fn get_id(&self) -> &str;
    /// Get the mod branch to install from if it has one.
    fn get_branch(&self) -> Option<&str>;
    /// Get the tag/release of the mod if it has one.
    fn get_tag(&self) -> Option<&str>;
}

/// Shared behaviours of an listable Dagger mod.
pub trait ListableMod {
    /// Get the ID of the mod.
    fn get_id(&self) -> &str;
    /// Whether to list the tags or not.
    fn list_tags(&self) -> bool;
}

pub trait GitCallback {
    fn 
}
