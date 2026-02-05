use std::{
    collections::BTreeMap,
    fs,
    ops::Bound,
    path::{Path, PathBuf},
};

use brk_error::{Error, Result};
use brk_types::{
    CentsSats, CentsSquaredSats, CentsUnsigned, CentsUnsignedCompact, CostBasisDistribution,
    Height, Sats,
};
use rustc_hash::FxHashMap;
use vecdb::Bytes;

use crate::utils::OptionExt;

use super::Percentiles;

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
    pending: FxHashMap<CentsUnsignedCompact, (Sats, Sats)>,
    pending_raw: PendingRaw,
}

const STATE_TO_KEEP: usize = 10;

impl CostBasisData {
    pub fn create(path: &Path, name: &str) -> Self {
        Self {
            pathbuf: path.join(format!("{name}_cost_basis")),
            state: None,
            pending: FxHashMap::default(),
            pending_raw: PendingRaw::default(),
        }
    }

    pub fn import_at_or_before(&mut self, height: Height) -> Result<Height> {
        let files = self.read_dir(None)?;
        let (&height, path) = files.range(..=height).next_back().ok_or(Error::NotFound(
            "No cost basis state found at or before height".into(),
        ))?;
        self.state = Some(State::deserialize(&fs::read(path)?)?);
        self.pending.clear();
        self.pending_raw = PendingRaw::default();
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

    pub fn iter(&self) -> impl Iterator<Item = (CentsUnsignedCompact, &Sats)> {
        self.assert_pending_empty();
        self.state.u().base.map.iter().map(|(&k, v)| (k, v))
    }

    pub fn range(
        &self,
        bounds: (Bound<CentsUnsignedCompact>, Bound<CentsUnsignedCompact>),
    ) -> impl Iterator<Item = (CentsUnsignedCompact, &Sats)> {
        self.assert_pending_empty();
        self.state.u().base.map.range(bounds).map(|(&k, v)| (k, v))
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty() && self.state.u().base.map.is_empty()
    }

    pub fn first_key_value(&self) -> Option<(CentsUnsignedCompact, &Sats)> {
        self.assert_pending_empty();
        self.state
            .u()
            .base
            .map
            .first_key_value()
            .map(|(&k, v)| (k, v))
    }

    pub fn last_key_value(&self) -> Option<(CentsUnsignedCompact, &Sats)> {
        self.assert_pending_empty();
        self.state
            .u()
            .base
            .map
            .last_key_value()
            .map(|(&k, v)| (k, v))
    }

    /// Get the exact cap_raw value (not recomputed from map).
    pub fn cap_raw(&self) -> CentsSats {
        self.assert_pending_empty();
        self.state.u().cap_raw
    }

    /// Get the exact investor_cap_raw value (not recomputed from map).
    pub fn investor_cap_raw(&self) -> CentsSquaredSats {
        self.assert_pending_empty();
        self.state.u().investor_cap_raw
    }

    /// Increment with pre-computed typed values
    pub fn increment(
        &mut self,
        price: CentsUnsigned,
        sats: Sats,
        price_sats: CentsSats,
        investor_cap: CentsSquaredSats,
    ) {
        self.pending.entry(price.into()).or_default().0 += sats;
        self.pending_raw.cap_inc += price_sats;
        if investor_cap != CentsSquaredSats::ZERO {
            self.pending_raw.investor_cap_inc += investor_cap;
        }
    }

    /// Decrement with pre-computed typed values
    pub fn decrement(
        &mut self,
        price: CentsUnsigned,
        sats: Sats,
        price_sats: CentsSats,
        investor_cap: CentsSquaredSats,
    ) {
        self.pending.entry(price.into()).or_default().1 += sats;
        self.pending_raw.cap_dec += price_sats;
        if investor_cap != CentsSquaredSats::ZERO {
            self.pending_raw.investor_cap_dec += investor_cap;
        }
    }

    pub fn apply_pending(&mut self) {
        for (cents, (inc, dec)) in self.pending.drain() {
            let entry = self.state.um().base.map.entry(cents).or_default();
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
                self.state.um().base.map.remove(&cents);
            }
        }

        // Apply raw values
        let state = self.state.um();
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

    pub fn init(&mut self) {
        self.state.replace(State::default());
        self.pending.clear();
        self.pending_raw = PendingRaw::default();
    }

    pub fn compute_percentiles(&self) -> Option<Percentiles> {
        self.assert_pending_empty();
        Percentiles::compute(self.iter().map(|(k, &v)| (k, v)))
    }

    pub fn clean(&mut self) -> Result<()> {
        let _ = fs::remove_dir_all(&self.pathbuf);
        fs::create_dir_all(self.path_by_height())?;
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
