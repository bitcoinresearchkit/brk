use std::{
    collections::BTreeMap,
    fs,
    ops::Bound,
    path::{Path, PathBuf},
};

use brk_error::{Error, Result};
use brk_types::{CentsCompact, Dollars, Height, Sats, SupplyState};
use derive_more::{Deref, DerefMut};
use pco::{standalone::{simple_compress, simple_decompress}, ChunkConfig};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use vecdb::Bytes;

use crate::{
    internal::{PERCENTILES, PERCENTILES_LEN},
    utils::OptionExt,
};

#[derive(Clone, Debug)]
pub struct PriceToAmount {
    pathbuf: PathBuf,
    state: Option<State>,
    /// Pending deltas: (total_increment, total_decrement) per price.
    /// Flushed to BTreeMap before reads and at end of block.
    pending: FxHashMap<CentsCompact, (Sats, Sats)>,
}

const STATE_AT_: &str = "state_at_";
const STATE_TO_KEEP: usize = 10;

impl PriceToAmount {
    pub fn create(path: &Path, name: &str) -> Self {
        Self {
            pathbuf: path.join(format!("{name}_price_to_amount")),
            state: None,
            pending: FxHashMap::default(),
        }
    }

    pub fn import_at_or_before(&mut self, height: Height) -> Result<Height> {
        let files = self.read_dir(None)?;
        let (&height, path) = files.range(..=height).next_back().ok_or(Error::NotFound(
            "No price state found at or before height".into(),
        ))?;
        self.state = Some(State::deserialize(&fs::read(path)?)?);
        self.pending.clear();
        Ok(height)
    }

    fn assert_pending_empty(&self) {
        assert!(
            self.pending.is_empty(),
            "PriceToAmount: pending not empty, call apply_pending first"
        );
    }

    pub fn iter(&self) -> impl Iterator<Item = (Dollars, &Sats)> {
        self.assert_pending_empty();
        self.state.u().iter().map(|(k, v)| (k.to_dollars(), v))
    }

    /// Iterate over entries in a price range with explicit bounds.
    pub fn range(
        &self,
        bounds: (Bound<Dollars>, Bound<Dollars>),
    ) -> impl Iterator<Item = (Dollars, &Sats)> {
        self.assert_pending_empty();

        let start = match bounds.0 {
            Bound::Included(d) => Bound::Included(CentsCompact::from(d)),
            Bound::Excluded(d) => Bound::Excluded(CentsCompact::from(d)),
            Bound::Unbounded => Bound::Unbounded,
        };

        let end = match bounds.1 {
            Bound::Included(d) => Bound::Included(CentsCompact::from(d)),
            Bound::Excluded(d) => Bound::Excluded(CentsCompact::from(d)),
            Bound::Unbounded => Bound::Unbounded,
        };

        self.state
            .u()
            .range((start, end))
            .map(|(k, v)| (k.to_dollars(), v))
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty() && self.state.u().is_empty()
    }

    pub fn first_key_value(&self) -> Option<(Dollars, &Sats)> {
        self.assert_pending_empty();
        self.state
            .u()
            .first_key_value()
            .map(|(k, v)| (k.to_dollars(), v))
    }

    pub fn last_key_value(&self) -> Option<(Dollars, &Sats)> {
        self.assert_pending_empty();
        self.state
            .u()
            .last_key_value()
            .map(|(k, v)| (k.to_dollars(), v))
    }

    /// Accumulate increment in pending batch. O(1).
    pub fn increment(&mut self, price: Dollars, supply_state: &SupplyState) {
        self.pending.entry(CentsCompact::from(price)).or_default().0 += supply_state.value;
    }

    /// Accumulate decrement in pending batch. O(1).
    pub fn decrement(&mut self, price: Dollars, supply_state: &SupplyState) {
        self.pending.entry(CentsCompact::from(price)).or_default().1 += supply_state.value;
    }

    /// Apply pending deltas to BTreeMap. O(k log n) where k = unique prices in pending.
    /// Must be called before any read operations.
    pub fn apply_pending(&mut self) {
        for (cents, (inc, dec)) in self.pending.drain() {
            let entry = self.state.um().entry(cents).or_default();
            *entry += inc;
            if *entry < dec {
                panic!(
                    "PriceToAmount::apply_pending underflow!\n\
                    Path: {:?}\n\
                    Price: {}\n\
                    Current + increments: {}\n\
                    Trying to decrement by: {}",
                    self.pathbuf,
                    cents.to_dollars(),
                    entry,
                    dec
                );
            }
            *entry -= dec;
            if *entry == Sats::ZERO {
                self.state.um().remove(&cents);
            }
        }
    }

    pub fn init(&mut self) {
        self.state.replace(State::default());
        self.pending.clear();
    }

    /// Compute percentile prices by iterating the BTreeMap directly.
    /// O(n) where n = number of unique prices.
    pub fn compute_percentiles(&self) -> [Dollars; PERCENTILES_LEN] {
        self.assert_pending_empty();

        let state = match self.state.as_ref() {
            Some(s) if !s.is_empty() => s,
            _ => return [Dollars::NAN; PERCENTILES_LEN],
        };

        let total: u64 = state.values().map(|&s| u64::from(s)).sum();
        if total == 0 {
            return [Dollars::NAN; PERCENTILES_LEN];
        }

        let mut result = [Dollars::NAN; PERCENTILES_LEN];
        let mut cumsum = 0u64;
        let mut idx = 0;

        for (&cents, &amount) in state.iter() {
            cumsum += u64::from(amount);
            while idx < PERCENTILES_LEN && cumsum >= total * u64::from(PERCENTILES[idx]) / 100 {
                result[idx] = cents.to_dollars();
                idx += 1;
            }
        }

        result
    }

    pub fn clean(&mut self) -> Result<()> {
        let _ = fs::remove_dir_all(&self.pathbuf);
        fs::create_dir_all(&self.pathbuf)?;
        Ok(())
    }

    fn read_dir(&self, keep_only_before: Option<Height>) -> Result<BTreeMap<Height, PathBuf>> {
        Ok(fs::read_dir(&self.pathbuf)?
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                let name = path.file_name()?.to_str()?;
                let height_str = name.strip_prefix(STATE_AT_).unwrap_or(name);
                if let Ok(h) = height_str.parse::<u32>().map(Height::from) {
                    if keep_only_before.is_none_or(|height| h < height) {
                        Some((h, path))
                    } else {
                        let _ = fs::remove_file(path);
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<BTreeMap<Height, PathBuf>>())
    }

    /// Flush state to disk, optionally cleaning up old state files.
    pub fn write(&mut self, height: Height, cleanup: bool) -> Result<()> {
        self.apply_pending();

        if cleanup {
            let files = self.read_dir(Some(height))?;

            for (_, path) in files
                .iter()
                .take(files.len().saturating_sub(STATE_TO_KEEP - 1))
            {
                fs::remove_file(path)?;
            }
        }

        fs::write(self.path_state(height), self.state.u().serialize()?)?;

        Ok(())
    }

    fn path_state(&self, height: Height) -> PathBuf {
        Self::path_state_(&self.pathbuf, height)
    }
    fn path_state_(path: &Path, height: Height) -> PathBuf {
        path.join(u32::from(height).to_string())
    }
}

#[derive(Clone, Default, Debug, Deref, DerefMut, Serialize, Deserialize)]
struct State(BTreeMap<CentsCompact, Sats>);

impl State {
    fn serialize(&self) -> vecdb::Result<Vec<u8>> {
        let keys: Vec<i32> = self.keys().map(|k| i32::from(*k)).collect();
        let values: Vec<u64> = self.values().map(|v| u64::from(*v)).collect();

        let config = ChunkConfig::default();
        let compressed_keys = simple_compress(&keys, &config)?;
        let compressed_values = simple_compress(&values, &config)?;

        let mut buffer = Vec::new();
        buffer.extend(keys.len().to_bytes());
        buffer.extend(compressed_keys.len().to_bytes());
        buffer.extend(compressed_keys);
        buffer.extend(compressed_values);

        Ok(buffer)
    }

    fn deserialize(data: &[u8]) -> vecdb::Result<Self> {
        let entry_count = usize::from_bytes(&data[0..8])?;
        let keys_len = usize::from_bytes(&data[8..16])?;

        let keys: Vec<i32> = simple_decompress(&data[16..16 + keys_len])?;
        let values: Vec<u64> = simple_decompress(&data[16 + keys_len..])?;

        let map: BTreeMap<CentsCompact, Sats> = keys
            .into_iter()
            .zip(values)
            .map(|(k, v)| (CentsCompact::from(k), Sats::from(v)))
            .collect();

        assert_eq!(map.len(), entry_count);

        Ok(Self(map))
    }
}
