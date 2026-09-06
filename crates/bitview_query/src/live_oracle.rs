use brk_error::Result;
use brk_oracle::Oracle;
use brk_types::BlockHash;
use parking_lot::Mutex;

/// One warmed confirmed window. No response bodies or mempool-derived state.
#[derive(Default)]
pub struct LiveOracle {
    current: Mutex<Option<Entry>>,
}

struct Entry {
    tip: BlockHash,
    publications: [u64; 2],
    oracle: Oracle,
}

impl LiveOracle {
    /// Call under both source publication guards. Holding the cache lock during
    /// the rebuild prevents concurrent requests replaying the same window.
    pub fn get_or_try_init(
        &self,
        tip: BlockHash,
        publications: [u64; 2],
        rebuild: impl FnOnce() -> Result<Oracle>,
    ) -> Result<Oracle> {
        let mut current = self.current.lock();
        if let Some(entry) = current.as_ref()
            && entry.tip == tip
            && entry.publications == publications
        {
            return Ok(entry.oracle.clone());
        }
        let oracle = rebuild()?;
        *current = Some(Entry {
            tip,
            publications,
            oracle: oracle.clone(),
        });
        Ok(oracle)
    }
}

#[cfg(test)]
#[path = "../tests/unit/live_oracle.rs"]
mod tests;
