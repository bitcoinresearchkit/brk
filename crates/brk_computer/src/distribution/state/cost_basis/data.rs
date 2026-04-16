use std::{
    collections::{BTreeMap, btree_map::Entry},
    fs,
    path::{Path, PathBuf},
};

use brk_error::{Error, Result};
use brk_types::{
    Cents, CentsCompact, CentsSats, CentsSquaredSats, CostBasisDistribution, Height, Sats,
};
use rustc_hash::FxHashMap;
use vecdb::{Bytes, unlikely};

use super::{Accumulate, CachedUnrealizedState, UnrealizedState};
use crate::distribution::state::pending::{PendingCapDelta, PendingDelta, PendingCapitalizedCapRawDelta};

/// Type alias for the price-to-sats map used in cost basis data.
pub(super) type CostBasisMap = BTreeMap<CentsCompact, Sats>;

const STATE_TO_KEEP: usize = 10;

/// Common interface for cost basis tracking.
///
/// Implemented by `CostBasisRaw` (scalars only) and `CostBasisData` (full map + scalars).
pub trait CostBasisOps: Send + Sync + 'static {
    fn create(path: &Path, name: &str) -> Self;
    fn with_price_rounding(self, digits: i32) -> Self;
    fn import_at_or_before(&mut self, height: Height) -> Result<Height>;
    fn cap_raw(&self) -> CentsSats;
    fn capitalized_cap_raw(&self) -> CentsSquaredSats;
    fn increment(
        &mut self,
        price: Cents,
        sats: Sats,
        price_sats: CentsSats,
        capitalized_cap: CentsSquaredSats,
    );
    fn decrement(
        &mut self,
        price: Cents,
        sats: Sats,
        price_sats: CentsSats,
        capitalized_cap: CentsSquaredSats,
    );
    fn apply_pending(&mut self);
    fn init(&mut self);
    fn clean(&mut self) -> Result<()>;
    fn write(&mut self, height: Height, cleanup: bool) -> Result<()>;
}

// ─── CostBasisRaw ───────────────────────────────────────────────────────────

#[derive(Clone, Default, Debug)]
struct RawState {
    cap_raw: CentsSats,
}

impl RawState {
    fn serialize(&self) -> Vec<u8> {
        self.cap_raw.to_bytes().to_vec()
    }

    fn deserialize(data: &[u8]) -> Result<Self> {
        Ok(Self {
            cap_raw: CentsSats::from_bytes(&data[0..16])?,
        })
    }
}

/// Lightweight cost basis tracking: only cap_raw scalar.
/// No BTreeMap, no unrealized computation, no pending map.
/// Used by cohorts that only need realized cap on restart (amount_range, address).
#[derive(Clone, Debug)]
pub struct CostBasisRaw {
    pathbuf: PathBuf,
    state: Option<RawState>,
    pending_cap: PendingCapDelta,
}

impl CostBasisRaw {
    pub(super) fn path_by_height(&self) -> PathBuf {
        self.pathbuf.join("by_height")
    }

    pub(super) fn path_state(&self, height: Height) -> PathBuf {
        self.path_by_height().join(height.to_string())
    }

    pub(super) fn read_dir(
        &self,
        keep_only_before: Option<Height>,
    ) -> Result<BTreeMap<Height, PathBuf>> {
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
            .collect())
    }

    fn apply_pending_cap(&mut self) {
        if self.pending_cap.is_zero() {
            return;
        }
        let state = self.state.as_mut().unwrap();

        state.cap_raw += self.pending_cap.inc;
        if unlikely(state.cap_raw.inner() < self.pending_cap.dec.inner()) {
            panic!(
                "CostBasis cap_raw underflow!\n\
                Path: {:?}\n\
                Current cap_raw (after increments): {}\n\
                Trying to decrement by: {}",
                self.pathbuf, state.cap_raw, self.pending_cap.dec
            );
        }
        state.cap_raw -= self.pending_cap.dec;

        self.pending_cap = PendingCapDelta::default();
    }

    fn write_and_cleanup(&mut self, height: Height, cleanup: bool) -> Result<()> {
        if cleanup {
            let files = self.read_dir(Some(height))?;
            for (_, path) in files
                .iter()
                .take(files.len().saturating_sub(STATE_TO_KEEP - 1))
            {
                fs::remove_file(path)?;
            }
        }
        Ok(())
    }
}

impl CostBasisOps for CostBasisRaw {
    fn create(path: &Path, name: &str) -> Self {
        Self {
            pathbuf: path.join(format!("{name}_cost_basis")),
            state: None,
            pending_cap: PendingCapDelta::default(),
        }
    }

    fn with_price_rounding(self, _digits: i32) -> Self {
        self
    }

    fn import_at_or_before(&mut self, height: Height) -> Result<Height> {
        let files = self.read_dir(None)?;
        let (&height, path) = files.range(..=height).next_back().ok_or(Error::NotFound(
            "No cost basis state found at or before height".into(),
        ))?;
        let data = fs::read(path)?;
        // Handle both formats: full (map + raw at end) and raw-only (16 bytes).
        self.state = Some(if data.len() == 16 {
            RawState::deserialize(&data)?
        } else {
            let (_, rest) = CostBasisDistribution::deserialize_with_rest(&data)?;
            RawState::deserialize(rest)?
        });
        self.pending_cap = PendingCapDelta::default();
        Ok(height)
    }

    fn cap_raw(&self) -> CentsSats {
        debug_assert!(self.pending_cap.is_zero());
        self.state.as_ref().unwrap().cap_raw
    }

    fn capitalized_cap_raw(&self) -> CentsSquaredSats {
        CentsSquaredSats::ZERO
    }

    #[inline]
    fn increment(
        &mut self,
        _price: Cents,
        _sats: Sats,
        price_sats: CentsSats,
        _capitalized_cap: CentsSquaredSats,
    ) {
        self.pending_cap.inc += price_sats;
    }

    #[inline]
    fn decrement(
        &mut self,
        _price: Cents,
        _sats: Sats,
        price_sats: CentsSats,
        _capitalized_cap: CentsSquaredSats,
    ) {
        self.pending_cap.dec += price_sats;
    }

    fn apply_pending(&mut self) {
        self.apply_pending_cap();
    }

    fn init(&mut self) {
        self.state.replace(RawState::default());
        self.pending_cap = PendingCapDelta::default();
    }

    fn clean(&mut self) -> Result<()> {
        let _ = fs::remove_dir_all(&self.pathbuf);
        fs::create_dir_all(self.path_by_height())?;
        Ok(())
    }

    fn write(&mut self, height: Height, cleanup: bool) -> Result<()> {
        self.apply_pending_cap();
        self.write_and_cleanup(height, cleanup)?;
        fs::write(
            self.path_state(height),
            self.state.as_ref().unwrap().serialize(),
        )?;
        Ok(())
    }
}

// ─── CostBasisData ──────────────────────────────────────────────────────────

/// Full cost basis tracking: BTreeMap distribution + raw scalars.
/// Composes `CostBasisRaw` for scalar tracking, adds map, pending, and cache.
///
/// Generic over the accumulator `S`:
/// - `WithCapital`: tracks all fields including invested capital + capitalized cap (128 bytes)
/// - `WithoutCapital`: tracks only supply + unrealized profit/loss (64 bytes, 1 cache line)
#[derive(Clone, Debug)]
pub struct CostBasisData<S: Accumulate> {
    raw: CostBasisRaw,
    map: Option<CostBasisDistribution>,
    pending: FxHashMap<CentsCompact, PendingDelta>,
    cache: Option<CachedUnrealizedState<S>>,
    rounding_digits: Option<i32>,
    capitalized_cap_raw: CentsSquaredSats,
    pending_capitalized_cap: PendingCapitalizedCapRawDelta,
}

impl<S: Accumulate> CostBasisData<S> {
    #[inline]
    fn round_price(&self, price: Cents) -> Cents {
        match self.rounding_digits {
            Some(digits) => price.round_to_dollar(digits),
            None => price,
        }
    }

    pub(crate) fn map(&self) -> &CostBasisMap {
        debug_assert!(self.pending.is_empty() && self.raw.pending_cap.is_zero());
        &self.map.as_ref().unwrap().map
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.pending.is_empty() && self.map.as_ref().unwrap().map.is_empty()
    }

    pub(crate) fn for_each_pending(&self, mut f: impl FnMut(&CentsCompact, &PendingDelta)) {
        self.pending.iter().for_each(|(k, v)| f(k, v));
    }

    pub(crate) fn compute_unrealized_state(&mut self, height_price: Cents) -> UnrealizedState {
        if self.is_empty() {
            return UnrealizedState::ZERO;
        }

        let map = &self.map.as_ref().unwrap().map;

        if let Some(cache) = self.cache.as_mut() {
            cache.get_at_price(height_price, map)
        } else {
            let cache = CachedUnrealizedState::compute_fresh(height_price, map);
            let state = cache.current_state();
            self.cache = Some(cache);
            state
        }
    }

    fn apply_map_pending(&mut self) {
        if self.pending.is_empty() {
            return;
        }
        let map = &mut self.map.as_mut().unwrap().map;
        for (cents, PendingDelta { inc, dec }) in self.pending.drain() {
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
                            self.raw.pathbuf,
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
                            self.raw.pathbuf,
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
    }
}

impl<S: Accumulate> CostBasisOps for CostBasisData<S> {
    fn create(path: &Path, name: &str) -> Self {
        Self {
            raw: CostBasisRaw::create(path, name),
            map: None,
            pending: FxHashMap::default(),
            cache: None,
            rounding_digits: None,
            capitalized_cap_raw: CentsSquaredSats::ZERO,
            pending_capitalized_cap: PendingCapitalizedCapRawDelta::default(),
        }
    }

    fn with_price_rounding(mut self, digits: i32) -> Self {
        self.rounding_digits = Some(digits);
        self
    }

    fn import_at_or_before(&mut self, height: Height) -> Result<Height> {
        let files = self.raw.read_dir(None)?;
        let (&height, path) = files.range(..=height).next_back().ok_or(Error::NotFound(
            "No cost basis state found at or before height".into(),
        ))?;
        let data = fs::read(path)?;
        let (base, rest) = CostBasisDistribution::deserialize_with_rest(&data)?;
        self.map = Some(base);
        self.raw.state = Some(RawState::deserialize(rest)?);
        debug_assert!(
            rest.len() >= 32,
            "CostBasisData state too short: {} bytes",
            rest.len()
        );
        self.capitalized_cap_raw = CentsSquaredSats::from_bytes(&rest[16..32])?;
        self.pending.clear();
        self.raw.pending_cap = PendingCapDelta::default();
        self.pending_capitalized_cap = PendingCapitalizedCapRawDelta::default();
        self.cache = None;
        Ok(height)
    }

    fn cap_raw(&self) -> CentsSats {
        self.raw.cap_raw()
    }

    fn capitalized_cap_raw(&self) -> CentsSquaredSats {
        self.capitalized_cap_raw
    }

    #[inline]
    fn increment(
        &mut self,
        price: Cents,
        sats: Sats,
        price_sats: CentsSats,
        capitalized_cap: CentsSquaredSats,
    ) {
        let price = self.round_price(price);
        self.pending.entry(price.into()).or_default().inc += sats;
        self.raw.pending_cap.inc += price_sats;
        if capitalized_cap != CentsSquaredSats::ZERO {
            self.pending_capitalized_cap.inc += capitalized_cap;
        }
        if let Some(cache) = self.cache.as_mut() {
            cache.on_receive(price, sats);
        }
    }

    #[inline]
    fn decrement(
        &mut self,
        price: Cents,
        sats: Sats,
        price_sats: CentsSats,
        capitalized_cap: CentsSquaredSats,
    ) {
        let price = self.round_price(price);
        self.pending.entry(price.into()).or_default().dec += sats;
        self.raw.pending_cap.dec += price_sats;
        if capitalized_cap != CentsSquaredSats::ZERO {
            self.pending_capitalized_cap.dec += capitalized_cap;
        }
        if let Some(cache) = self.cache.as_mut() {
            cache.on_send(price, sats);
        }
    }

    fn apply_pending(&mut self) {
        self.apply_map_pending();
        self.raw.apply_pending_cap();
        self.capitalized_cap_raw += self.pending_capitalized_cap.inc;
        debug_assert!(
            self.capitalized_cap_raw >= self.pending_capitalized_cap.dec,
            "CostBasis capitalized_cap_raw underflow!\n\
            Path: {:?}\n\
            Current (after increments): {:?}\n\
            Trying to decrement by: {:?}",
            self.raw.pathbuf,
            self.capitalized_cap_raw,
            self.pending_capitalized_cap.dec
        );
        self.capitalized_cap_raw -= self.pending_capitalized_cap.dec;
        self.pending_capitalized_cap = PendingCapitalizedCapRawDelta::default();
    }

    fn init(&mut self) {
        self.raw.init();
        self.map.replace(CostBasisDistribution::default());
        self.pending.clear();
        self.cache = None;
        self.capitalized_cap_raw = CentsSquaredSats::ZERO;
        self.pending_capitalized_cap = PendingCapitalizedCapRawDelta::default();
    }

    fn clean(&mut self) -> Result<()> {
        self.raw.clean()?;
        self.cache = None;
        Ok(())
    }

    fn write(&mut self, height: Height, cleanup: bool) -> Result<()> {
        self.apply_pending();
        self.raw.write_and_cleanup(height, cleanup)?;

        let raw_state = self.raw.state.as_ref().unwrap();
        let mut buffer = self.map.as_ref().unwrap().serialize()?;
        buffer.extend(raw_state.cap_raw.to_bytes());
        buffer.extend(self.capitalized_cap_raw.to_bytes());
        fs::write(self.raw.path_state(height), buffer)?;

        Ok(())
    }
}
