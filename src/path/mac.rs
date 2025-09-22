use crate::path::{DaggerPathApi, PathImpl};

impl DaggerPathApi for PathImpl {
    #[inline]
    fn balatro_dir() -> &'static str {
        "~/Library/Application Support/Balatro"
    }

    #[inline]
    fn config_dir() -> &'static str {
        "~/Library/Dagger"
    }
}
