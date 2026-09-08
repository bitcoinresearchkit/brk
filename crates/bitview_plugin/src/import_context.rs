use std::path::Path;

use vecdb::CacheBudget;

/// Shared resources available while importing a plugin composition.
#[derive(Clone, Copy, Debug)]
pub struct ImportContext<'a> {
    data_path: &'a Path,
    cache_budget: &'static CacheBudget,
}

impl<'a> ImportContext<'a> {
    pub const fn new(data_path: &'a Path, cache_budget: &'static CacheBudget) -> Self {
        Self {
            data_path,
            cache_budget,
        }
    }

    pub const fn cache_budget(self) -> &'static CacheBudget {
        self.cache_budget
    }

    pub const fn data_path(self) -> &'a Path {
        self.data_path
    }
}
