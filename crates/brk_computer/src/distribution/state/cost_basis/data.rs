use std::{
    collections::{btree_map::Entry, BTreeMap},
    fs,
    path::{Path, PathBuf},
};

use brk_error::{Error, Result};
use brk_types::{
    Cents, CentsCompact, CentsSats, CentsSquaredSats, CostBasisDistribution, Height, Sats,
};
use rustc_hash::FxHashMap;
use vecdb::{Bytes, unlikely};

use super::{CachedUnrealizedState, UnrealizedState};

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
    rounding_digits: Option<i32>,
    /// Monotonically increasing counter, bumped on each apply_pending with actual changes.
    generation: u64,
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
            rounding_digits: None,
            generation: 0,
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
        Ok(height)
    }

    fn assert_pending_empty(&self) {
        debug_assert!(
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

    pub(crate) fn map(&self) -> &CostBasisMap {
        self.assert_pending_empty();
        &self.state.as_ref().unwrap().base.map
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.pending.is_empty() && self.state.as_ref().unwrap().base.map.is_empty()
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
        if self.pending.is_empty() {
            return;
        }
        self.generation = self.generation.wrapping_add(1);
        let map = &mut self.state.as_mut().unwrap().base.map;
        for (cents, (inc, dec)) in self.pending.drain() {
            match map.entry(cents) {
                Entry::Occupied(mut e) => {
                    *e.get_mut() += inc;
                    if unlikely(*e.get() < dec) {
                        panic!(
                            "CostBasisData::apply_pending underflow!\n\
                            Path: {:?}\n\
                            Price: {}\n\
                            Current + increments: {}\n\
                            Trying to decrement by: {}",
                            self.pathbuf,
                            cents.to_dollars(),
                            e.get(),
                            dec
                        );
                    }
                    *e.get_mut() -= dec;
                    if *e.get() == Sats::ZERO {
                        e.remove();
                    }
                }
                Entry::Vacant(e) => {
                    if unlikely(inc < dec) {
                        panic!(
                            "CostBasisData::apply_pending underflow (new entry)!\n\
                            Path: {:?}\n\
                            Price: {}\n\
                            Increment: {}\n\
                            Trying to decrement by: {}",
                            self.pathbuf,
                            cents.to_dollars(),
                            inc,
                            dec
                        );
                    }
                    let val = inc - dec;
                    if val != Sats::ZERO {
                        e.insert(val);
                    }
                }
            }
        }

        // Apply raw values
        let state = self.state.as_mut().unwrap();
        state.cap_raw += self.pending_raw.cap_inc;

        // Check for underflow before subtracting
        if unlikely(state.cap_raw.inner() < self.pending_raw.cap_dec.inner()) {
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

            if unlikely(state.investor_cap_raw.inner() < self.pending_raw.investor_cap_dec.inner()) {
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
    }

    pub(crate) fn compute_unrealized_state(&mut self, height_price: Cents) -> UnrealizedState {
        if self.is_empty() {
            return UnrealizedState::ZERO;
        }

        let map = &self.state.as_ref().unwrap().base.map;

        if let Some(cache) = self.cache.as_mut() {
            cache.get_at_price(height_price, map)
        } else {
            let cache = CachedUnrealizedState::compute_fresh(height_price, map);
            let state = cache.current_state();
            self.cache = Some(cache);
            state
        }
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

        fs::write(
            self.path_state(height),
            self.state.as_ref().unwrap().serialize()?,
        )?;

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
