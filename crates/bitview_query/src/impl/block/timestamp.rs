use brk_error::{Error, OptionData, Result};
use brk_types::{BlockHash, BlockTimestamp, Height, Timestamp};
use jiff::Timestamp as JiffTimestamp;
use vecdb::ReadableVec;

use crate::Query;

#[cfg(test)]
#[path = "../../../benches/unit/block_timestamp.rs"]
mod bench;

/// An owned timestamp selection from one published chain view.
pub struct ResolvedBlockTimestamp {
    height: Height,
    hash: BlockHash,
    timestamp: JiffTimestamp,
}

impl ResolvedBlockTimestamp {
    pub fn hash(&self) -> BlockHash {
        self.hash
    }

    /// Format the timestamp only when a response body is needed.
    pub fn into_value(self) -> BlockTimestamp {
        BlockTimestamp {
            height: self.height,
            hash: self.hash,
            timestamp: self
                .timestamp
                .strftime("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        }
    }
}

impl Query {
    /// Greatest header timestamp at or before the target, with earliest-height
    /// ties. Future targets need not select the highest block.
    pub fn block_by_timestamp(&self, timestamp: Timestamp) -> Result<BlockTimestamp> {
        self.resolve_block_by_timestamp(timestamp)
            .map(ResolvedBlockTimestamp::into_value)
    }

    pub fn resolve_block_by_timestamp(&self, target: Timestamp) -> Result<ResolvedBlockTimestamp> {
        let indexer = self.indexer();
        let mappings = self.plugins().mappings;
        let _guard = self.read_publication()?;
        let len = usize::from(self.safe_lengths().height);
        let monotonic = &mappings.timestamp.monotonic;
        let timestamps = &indexer.vecs().blocks.timestamp;
        // Reuse existing snapshots without filling or retaining whole histories.
        // Cold cursors retain only their current Pco page.
        let warm_max = monotonic.cached_snapshot();
        let warm_raw = timestamps.cached_snapshot();
        let mut max_cursor = monotonic.inner.cursor();
        let mut raw_cursor = timestamps.inner.cursor();
        let (height, timestamp) = select_timestamp(
            len,
            target,
            |h| {
                warm_max
                    .as_ref()
                    .and_then(|v| v.get(h).copied())
                    .or_else(|| max_cursor.get(h))
                    .data()
            },
            |h| {
                warm_raw
                    .as_ref()
                    .and_then(|v| v.get(h).copied())
                    .or_else(|| raw_cursor.get(h))
                    .data()
            },
        )?;
        let selected = warm_raw
            .as_ref()
            .and_then(|v| v.get(height).copied())
            .or_else(|| raw_cursor.get(height))
            .data()?;
        if selected != timestamp {
            return Err(Error::Internal(
                "Timestamp mapping disagrees with indexed block",
            ));
        }
        let height = Height::from(height);
        let hash = indexer
            .vecs()
            .blocks
            .blockhash
            .inner
            .collect_one(height)
            .data()?;
        let timestamp = JiffTimestamp::from_second(i64::from(*timestamp))
            .map_err(|_| Error::Internal("Invalid indexed timestamp"))?;
        Ok(ResolvedBlockTimestamp {
            height,
            hash,
            timestamp,
        })
    }
}

fn lower_bound(
    mut end: usize,
    target: Timestamp,
    inclusive: bool,
    read: &mut impl FnMut(usize) -> Result<Timestamp>,
) -> Result<usize> {
    let mut begin = 0;
    while begin < end {
        let mid = begin + (end - begin) / 2;
        let value = read(mid)?;
        if value < target || (inclusive && value == target) {
            begin = mid + 1;
        } else {
            end = mid;
        }
    }
    Ok(begin)
}

/// O(log n + k), not universally O(log n): the first future-skewed header can
/// precede the MTP crossing by many blocks. Far-future inputs never scan.
fn select_timestamp(
    len: usize,
    target: Timestamp,
    mut maximum: impl FnMut(usize) -> Result<Timestamp>,
    mut raw: impl FnMut(usize) -> Result<Timestamp>,
) -> Result<(usize, Timestamp)> {
    if len == 0 {
        return Err(Error::StateUpdating);
    }
    let crossing = lower_bound(len, target, true, &mut maximum)?;
    if crossing == 0 {
        return Err(Error::NotFound("No block at or before timestamp".into()));
    }
    let mut best = maximum(crossing - 1)?;
    let mut height = lower_bound(crossing, best, false, &mut maximum)?;
    if best == target {
        return Ok((height, best));
    }
    // Before the first crossing every timestamp is <= target, so the initial
    // window is known without reading ten preceding blocks. Count above-target
    // values instead of loading or sorting a separate median-time source.
    let mut window = [false; 11];
    let mut above = 0usize;
    for h in crossing..len {
        let timestamp = raw(h)?;
        if timestamp <= target && timestamp > best {
            best = timestamp;
            height = h;
        }
        if best == target {
            break;
        }
        // Header consensus requires descendants to exceed previous MTP.
        // BIP113 reused that clock, rather than introducing the header rule.
        let slot = h % 11;
        above -= usize::from(window[slot]);
        window[slot] = timestamp > target;
        above += usize::from(window[slot]);
        let len = (h + 1).min(11);
        if above >= len.div_ceil(2) {
            break;
        }
    }
    Ok((height, best))
}

#[cfg(test)]
#[path = "../../../tests/unit/impl/block/timestamp.rs"]
mod tests;
