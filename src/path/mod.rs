use std::path::Path;
pub struct DaggerPathsImpl;

/// Paths that are used by Dagger.
pub trait DaggerPaths {
    fn balatro_mod_dir() -> &'static Path;
    fn config_dir() -> &'static Path;
}

macro_rules! pathdef {
    ($($os:ident = $os_str:literal),+) => {$(
        #[cfg(target_os = $os_str)]
        pub mod $os;
    )+};
}

// Until I figure out how to do this deal with this.
pathdef!(linux = "linux", windows = "windows", macos = "macos");
