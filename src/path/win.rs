use crate::path::{DaggerPathApi, PathImpl};
use constcat::concat as constcat;

const APP_DATA_ROAMING: &str = env!("AppData");

impl DaggerPathApi for PathImpl {
    #[inline]
    fn config_dir() -> &'static str {
        constcat!(APP_DATA_ROAMING, "/Dagger")
    }

    #[inline]
    fn balatro_dir() -> &'static str {
        constcat!(APP_DATA_ROAMING, "/Balatro")
    }
}
