use std::sync::{Mutex, OnceLock};

use vecdb::{Budgeted, CacheBudget};

pub static TEST_LOCK: Mutex<()> = Mutex::new(());

pub fn init_cache() -> &'static CacheBudget {
    static CACHE: OnceLock<&'static CacheBudget> = OnceLock::new();
    CACHE.get_or_init(|| Budgeted::init_global(512 * 1024).unwrap())
}
