use bitview_plugin_distribution::EncodedAgeRangeUrpds;
use brk_error::{Error, Result};
use brk_types::{CentsCompact, Sats, Urpd, UrpdRaw};

#[allow(clippy::large_enum_variant)] // One captured request input; keep aggregate sections inline.
pub enum UrpdInput {
    Raw(Vec<u8>),
    Aggregate(EncodedAgeRangeUrpds),
}

impl UrpdInput {
    pub fn decode(&self) -> Result<UrpdRaw> {
        Ok(UrpdRaw {
            map: self.decode_entries()?.into_iter().collect(),
        })
    }

    fn decode_entries(&self) -> Result<Vec<(CentsCompact, Sats)>> {
        match self {
            Self::Raw(bytes) => UrpdRaw::deserialize_entries(bytes),
            Self::Aggregate(input) => input.decode_entries(),
        }
    }
}

use super::ResolvedUrpd;

impl ResolvedUrpd {
    /// Visit encoded sections in decoding order without allocating an iterator wrapper.
    pub fn for_each_section(&self, mut visit: impl FnMut(&[u8])) {
        match &self.input {
            UrpdInput::Raw(bytes) => visit(bytes),
            UrpdInput::Aggregate(input) => input.sections().for_each(visit),
        }
    }

    pub fn build(self) -> Result<Urpd> {
        let raw = self.validated_raw()?;
        drop(self.input);
        Ok(Urpd::build(
            self.cohort,
            self.date,
            self.weight,
            self.close,
            &raw,
            self.aggregation,
        ))
    }

    /// Check captured inputs without constructing response buckets or JSON.
    pub fn validate(self) -> Result<()> {
        self.validate_metadata()?;
        let entries = self.input.decode_entries()?;
        // Decoder validation bounds the source total by MAX_MONEY and the
        // scalar is in [0, 1], so the weighted sum cannot overflow u64.
        let supply = entries
            .iter()
            .map(|(_, sats)| (u64::from(*sats) as f64 * self.scalar).floor() as u64)
            .sum();
        self.validate_market_value(supply)
    }

    fn validated_raw(&self) -> Result<UrpdRaw> {
        self.validate_metadata()?;
        let raw = self.input.decode()?.apply_weight(self.scalar);
        self.validate_market_value(u64::from(raw.checked_supply()?))?;
        Ok(raw)
    }

    fn validate_metadata(&self) -> Result<()> {
        if !self.scalar.is_finite() || !(0.0..=1.0).contains(&self.scalar) || self.close.is_nan() {
            return Err(Error::Internal("Invalid URPD weight or close price"));
        }
        Ok(())
    }

    fn validate_market_value(&self, supply: u64) -> Result<()> {
        if u128::from(self.close.inner()) * u128::from(supply) / Sats::ONE_BTC_U128
            > i64::MAX as u128
        {
            return Err(Error::Internal(
                "URPD market value exceeds signed price range",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "../../../tests/unit/impl/urpd/resolved.rs"]
mod tests;
