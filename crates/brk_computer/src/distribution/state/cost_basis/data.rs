use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use brk_error::{Error, Result};
use brk_types::{
    CentsCompact, CentsSats, CentsSquaredSats, Cents, CostBasisDistribution, Height, Sats,
};
use rustc_hash::FxHashMap;
use vecdb::Bytes;

use super::{CachedUnrealizedState, Percentiles, UnrealizedState};

/// Type alias for the price-to-sats map used in cost basis data.
pub(super) type CostBasisMap = BTreeMap<CentsCompact, Sats>;

#[derive(Clone, Debug, Default)]
struct PendingRaw {
    cap_inc: CentsSats,
    cap_dec: CentsSats,
    investor_cap_inc: CentsSquaredSats,
    investor_cap_dec: CentsSquaredSats,
}

#[derive(Clone, Debug)]
pub struct CostBasisData {
    pathbuf: PathBuf,
    state: Option<State>,
    pending: FxHashMap<CentsCompact, (Sats, Sats)>,
    pending_raw: PendingRaw,
    cache: Option<CachedUnrealizedState>,
    percentiles_dirty: bool,
    cached_percentiles: Option<Percentiles>,
    rounding_digits: Option<i32>,
}

const STATE_TO_KEEP: usize = 10;

impl CostBasisData {
    pub(crate) fn create(path: &Path, name: &str) -> Self {
        Self {
            pathbuf: path.join(format!("{name}_cost_basis")),
            state: None,
            pending: FxHashMap::default(),
            pending_raw: PendingRaw::default(),
            cache: None,
            percentiles_dirty: true,
            cached_percentiles: None,
            rounding_digits: None,
        }
    }

    pub(crate) fn with_price_rounding(mut self, digits: i32) -> Self {
        self.rounding_digits = Some(digits);
        self
    }

    #[inline]
    fn round_price(&self, price: Cents) -> Cents {
        match self.rounding_digits {
            Some(digits) => price.round_to_dollar(digits),
            None => price,
        }
    }

    pub(crate) fn import_at_or_before(&mut self, height: Height) -> Result<Height> {
        let files = self.read_dir(None)?;
        let (&height, path) = files.range(..=height).next_back().ok_or(Error::NotFound(
            "No cost basis state found at or before height".into(),
        ))?;
        self.state = Some(State::deserialize(&fs::read(path)?)?);
        self.pending.clear();
        self.pending_raw = PendingRaw::default();
        self.cache = None;
        self.percentiles_dirty = true;
        self.cached_percentiles = None;
        Ok(height)
    }

    fn assert_pending_empty(&self) {
        assert!(
            self.pending.is_empty() && self.pending_raw_is_zero(),
            "CostBasisData: pending not empty, call apply_pending first"
        );
    }

    fn pending_raw_is_zero(&self) -> bool {
        self.pending_raw.cap_inc == CentsSats::ZERO
            && self.pending_raw.cap_dec == CentsSats::ZERO
            && self.pending_raw.investor_cap_inc == CentsSquaredSats::ZERO
            && self.pending_raw.investor_cap_dec == CentsSquaredSats::ZERO
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (CentsCompact, &Sats)> {
        self.assert_pending_empty();
        self.state.as_ref().unwrap().base.map.iter().map(|(&k, v)| (k, v))
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.pending.is_empty() && self.state.as_ref().unwrap().base.map.is_empty()
    }

    pub(crate) fn first_key_value(&self) -> Option<(CentsCompact, &Sats)> {
        self.assert_pending_empty();
        self.state
            .as_ref().unwrap()
            .base
            .map
            .first_key_value()
            .map(|(&k, v)| (k, v))
    }

    pub(crate) fn last_key_value(&self) -> Option<(CentsCompact, &Sats)> {
        self.assert_pending_empty();
        self.state
            .as_ref().unwrap()
            .base
            .map
            .last_key_value()
            .map(|(&k, v)| (k, v))
    }

    /// Get the exact cap_raw value (not recomputed from map).
    pub(crate) fn cap_raw(&self) -> CentsSats {
        self.assert_pending_empty();
        self.state.as_ref().unwrap().cap_raw
    }

    /// Get the exact investor_cap_raw value (not recomputed from map).
    pub(crate) fn investor_cap_raw(&self) -> CentsSquaredSats {
        self.assert_pending_empty();
        self.state.as_ref().unwrap().investor_cap_raw
    }

    /// Increment with pre-computed typed values.
    /// Handles rounding and cache update.
    pub(crate) fn increment(
        &mut self,
        price: Cents,
        sats: Sats,
        price_sats: CentsSats,
        investor_cap: CentsSquaredSats,
    ) {
        let price = self.round_price(price);
        self.pending.entry(price.into()).or_default().0 += sats;
        self.pending_raw.cap_inc += price_sats;
        if investor_cap != CentsSquaredSats::ZERO {
            self.pending_raw.investor_cap_inc += investor_cap;
        }
        if let Some(cache) = self.cache.as_mut() {
            cache.on_receive(price, sats);
        }
    }

    /// Decrement with pre-computed typed values.
    /// Handles rounding and cache update.
    pub(crate) fn decrement(
        &mut self,
        price: Cents,
        sats: Sats,
        price_sats: CentsSats,
        investor_cap: CentsSquaredSats,
    ) {
        let price = self.round_price(price);
        self.pending.entry(price.into()).or_default().1 += sats;
        self.pending_raw.cap_dec += price_sats;
        if investor_cap != CentsSquaredSats::ZERO {
            self.pending_raw.investor_cap_dec += investor_cap;
        }
        if let Some(cache) = self.cache.as_mut() {
            cache.on_send(price, sats);
        }
    }

    pub(crate) fn apply_pending(&mut self) {
        if !self.pending.is_empty() {
            self.percentiles_dirty = true;
        }
        for (cents, (inc, dec)) in self.pending.drain() {
            let entry = self.state.as_mut().unwrap().base.map.entry(cents).or_default();
            *entry += inc;
            if *entry < dec {
                panic!(
                    "CostBasisData::apply_pending underflow!\n\
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
                self.state.as_mut().unwrap().base.map.remove(&cents);
            }
        }

        // Apply raw values
        let state = self.state.as_mut().unwrap();
        state.cap_raw += self.pending_raw.cap_inc;

        // Check for underflow before subtracting
        if state.cap_raw.inner() < self.pending_raw.cap_dec.inner() {
            panic!(
                "CostBasisData::apply_pending cap_raw underflow!\n\
                Path: {:?}\n\
                Current cap_raw (after increments): {}\n\
                Trying to decrement by: {}",
                self.pathbuf, state.cap_raw, self.pending_raw.cap_dec
            );
        }
        state.cap_raw -= self.pending_raw.cap_dec;

        // Only process investor_cap if there are non-zero values
        let has_investor_cap = self.pending_raw.investor_cap_inc != CentsSquaredSats::ZERO
            || self.pending_raw.investor_cap_dec != CentsSquaredSats::ZERO;

        if has_investor_cap {
            state.investor_cap_raw += self.pending_raw.investor_cap_inc;

            if state.investor_cap_raw.inner() < self.pending_raw.investor_cap_dec.inner() {
                panic!(
                    "CostBasisData::apply_pending investor_cap_raw underflow!\n\
                    Path: {:?}\n\
                    Current investor_cap_raw (after increments): {}\n\
                    Trying to decrement by: {}",
                    self.pathbuf, state.investor_cap_raw, self.pending_raw.investor_cap_dec
                );
            }
            state.investor_cap_raw -= self.pending_raw.investor_cap_dec;
        }

        self.pending_raw = PendingRaw::default();
    }

    pub(crate) fn init(&mut self) {
        self.state.replace(State::default());
        self.pending.clear();
        self.pending_raw = PendingRaw::default();
        self.cache = None;
        self.percentiles_dirty = true;
        self.cached_percentiles = None;
    }

    pub(crate) fn compute_percentiles(&mut self) -> Option<Percentiles> {
        self.assert_pending_empty();
        if !self.percentiles_dirty {
            return self.cached_percentiles;
        }
        self.cached_percentiles = Percentiles::compute(self.iter().map(|(k, &v)| (k, v)));
        self.percentiles_dirty = false;
        self.cached_percentiles
    }

    pub(crate) fn compute_unrealized_states(
        &mut self,
        height_price: Cents,
        date_price: Option<Cents>,
    ) -> (UnrealizedState, Option<UnrealizedState>) {
        if self.is_empty() {
            return (
                UnrealizedState::ZERO,
                date_price.map(|_| UnrealizedState::ZERO),
            );
        }

        let map = &self.state.as_ref().unwrap().base.map;

        let date_state =
            date_price.map(|p| CachedUnrealizedState::compute_full_standalone(p.into(), map));

        let height_state = if let Some(cache) = self.cache.as_mut() {
            cache.get_at_price(height_price, map)
        } else {
            let cache = CachedUnrealizedState::compute_fresh(height_price, map);
            let state = cache.current_state();
            self.cache = Some(cache);
            state
        };

        (height_state, date_state)
    }

    pub(crate) fn clean(&mut self) -> Result<()> {
        let _ = fs::remove_dir_all(&self.pathbuf);
        fs::create_dir_all(self.path_by_height())?;
        self.cache = None;
        Ok(())
    }

    fn path_by_height(&self) -> PathBuf {
        self.pathbuf.join("by_height")
    }

    fn read_dir(&self, keep_only_before: Option<Height>) -> Result<BTreeMap<Height, PathBuf>> {
        let by_height = self.path_by_height();
        if !by_height.exists() {
            return Ok(BTreeMap::new());
        }
        Ok(fs::read_dir(&by_height)?
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                let name = path.file_name()?.to_str()?;
                if let Ok(h) = name.parse::<u32>().map(Height::from) {
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

    pub(crate) fn write(&mut self, height: Height, cleanup: bool) -> Result<()> {
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

        fs::write(self.path_state(height), self.state.as_ref().unwrap().serialize()?)?;

        Ok(())
    }

    fn path_state(&self, height: Height) -> PathBuf {
        self.path_by_height().join(height.to_string())
    }
}

#[derive(Clone, Default, Debug)]
struct State {
    base: CostBasisDistribution,
    /// Exact realized cap: Σ(price × sats)
    cap_raw: CentsSats,
    /// Exact investor cap: Σ(price² × sats)
    investor_cap_raw: CentsSquaredSats,
}

impl State {
    fn serialize(&self) -> Result<Vec<u8>> {
        let mut buffer = self.base.serialize()?;
        buffer.extend(self.cap_raw.to_bytes());
        buffer.extend(self.investor_cap_raw.to_bytes());
        Ok(buffer)
    }

    fn deserialize(data: &[u8]) -> Result<Self> {
        let (base, rest) = CostBasisDistribution::deserialize_with_rest(data)?;
        let cap_raw = CentsSats::from_bytes(&rest[0..16])?;
        let investor_cap_raw = CentsSquaredSats::from_bytes(&rest[16..32])?;

        Ok(Self {
            base,
            cap_raw,
            investor_cap_raw,
        })
    }
}
