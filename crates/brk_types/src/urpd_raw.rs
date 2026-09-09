use std::{
    collections::BTreeMap,
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

use bitcoin::Amount;
use brk_error::{Error, Result};
use pco::{ChunkConfig, standalone::simple_compress};
use schemars::JsonSchema;
use serde::Serialize;

use crate::{Cents, CentsCompact, CostBasisByPercentile, Date, PERCENTILES, PERCENTILES_LEN, Sats};

mod decode;

/// Raw on-disk URPD: a map of price (cents) to supply (sats).
/// Processed into [`crate::Urpd`] for API responses.
#[derive(Debug, Clone, Default, Serialize, JsonSchema)]
pub struct UrpdRaw {
    pub map: BTreeMap<CentsCompact, Sats>,
}

struct DecodedEntries<'a> {
    entries: Vec<(CentsCompact, Sats)>,
    rest: &'a [u8],
}

impl UrpdRaw {
    /// Resource ceilings for persisted snapshots, not truncation thresholds.
    /// Exceeding either limit is an error. Two million rows leave substantial
    /// headroom above the producer's five-significant-digit dollar buckets.
    pub const MAX_ENTRIES: usize = 2_000_000;
    pub const MAX_ENCODED_BYTES: usize = 64 * 1024 * 1024;

    /// Validate the supply of a raw or weighted subset of Bitcoin's UTXO set.
    pub fn checked_supply(&self) -> Result<Sats> {
        checked_supply(self.map.values().map(|sats| u64::from(*sats)))
    }

    pub fn checked_entry_supply(entries: &[(CentsCompact, Sats)]) -> Result<Sats> {
        checked_supply(entries.iter().map(|(_, sats)| u64::from(*sats)))
    }

    /// Return sat- and acquisition-value-weighted percentiles using shared passes.
    pub fn cost_basis_percentile_prices(&self) -> CostBasisByPercentile {
        Self::cost_basis_percentile_prices_from_entries(
            self.map.iter().map(|(&price, &sats)| (price, sats)),
        )
    }

    fn cost_basis_percentile_prices_from_entries(
        entries: impl Iterator<Item = (CentsCompact, Sats)> + Clone,
    ) -> CostBasisByPercentile {
        let (total_sats, total_value) = entries.clone().fold(
            (0_u128, 0_u128),
            |(total_sats, total_value), (price, sats)| {
                let sats = u128::from(u64::from(sats));
                (total_sats + sats, total_value + price.as_u128() * sats)
            },
        );
        let per_coin_targets = Self::percentile_targets(total_sats);
        let per_dollar_targets = Self::percentile_targets(total_value);
        let mut prices = CostBasisByPercentile::default();
        let mut per_coin_index = if total_sats == 0 { PERCENTILES_LEN } else { 0 };
        let mut per_dollar_index = if total_value == 0 { PERCENTILES_LEN } else { 0 };
        let mut cumulative_sats = 0_u128;
        let mut cumulative_value = 0_u128;

        for (price, sats) in entries {
            let sats = u128::from(u64::from(sats));
            cumulative_sats += sats;
            cumulative_value += price.as_u128() * sats;
            let price = price.into();
            Self::fill_percentile_prices(
                &mut prices.per_coin,
                &per_coin_targets,
                &mut per_coin_index,
                cumulative_sats,
                price,
            );
            Self::fill_percentile_prices(
                &mut prices.per_dollar,
                &per_dollar_targets,
                &mut per_dollar_index,
                cumulative_value,
                price,
            );
            if per_coin_index == PERCENTILES_LEN && per_dollar_index == PERCENTILES_LEN {
                break;
            }
        }

        prices
    }

    fn percentile_targets(total: u128) -> [u128; PERCENTILES_LEN] {
        PERCENTILES.map(|percentile| (total * u128::from(percentile) / 100).saturating_sub(1))
    }

    fn fill_percentile_prices(
        prices: &mut [Cents; PERCENTILES_LEN],
        targets: &[u128; PERCENTILES_LEN],
        target_index: &mut usize,
        cumulative: u128,
        price: Cents,
    ) {
        while *target_index < PERCENTILES_LEN && cumulative > targets[*target_index] {
            prices[*target_index] = price;
            *target_index += 1;
        }
    }

    pub fn dir(states_path: &Path, name: &str) -> PathBuf {
        states_path.join(name).join("urpd")
    }

    pub fn path(states_path: &Path, name: &str, date: Date) -> PathBuf {
        Self::dir(states_path, name).join(date.to_string())
    }

    pub fn read(states_path: &Path, name: &str, date: Date) -> Result<Self> {
        let bytes = Self::read_bytes(states_path, name, date)?;
        Self::deserialize_exact(&bytes)
    }

    /// Decode exactly one snapshot, rejecting trailing data.
    pub fn deserialize_exact(bytes: &[u8]) -> Result<Self> {
        Ok(Self {
            map: Self::deserialize_entries(bytes)?.into_iter().collect(),
        })
    }

    /// Read persisted entries and calculate percentiles without building a map.
    pub fn read_cost_basis_percentile_prices(
        states_path: &Path,
        name: &str,
        date: Date,
    ) -> Result<CostBasisByPercentile> {
        let bytes = Self::read_bytes(states_path, name, date)?;
        let entries = Self::deserialize_entries(&bytes)?;
        Ok(Self::cost_basis_percentile_prices_from_entries(
            entries.iter().copied(),
        ))
    }

    /// Capture an encoded snapshot without decoding it. The producer's publication
    /// guard must protect this read when snapshots can be rewritten concurrently.
    pub fn read_bytes(states_path: &Path, name: &str, date: Date) -> Result<Vec<u8>> {
        let path = Self::path(states_path, name, date);
        Self::read_encoded_file(&path).map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("Cannot read URPD '{}': {error}", path.display()),
            )
            .into()
        })
    }

    /// Shared bounded capture for raw and indexed age-range snapshot files.
    pub fn read_encoded_file(path: &Path) -> io::Result<Vec<u8>> {
        let file = fs::File::open(path)?;
        let metadata = file.metadata()?;
        if !metadata.is_file() || metadata.len() > Self::MAX_ENCODED_BYTES as u64 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "URPD file exceeds snapshot limits",
            ));
        }
        let mut bytes = Vec::with_capacity(metadata.len() as usize + 1);
        file.take(Self::MAX_ENCODED_BYTES as u64 + 1)
            .read_to_end(&mut bytes)?;
        if bytes.len() > Self::MAX_ENCODED_BYTES {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "URPD file grew beyond snapshot limits",
            ));
        }
        Ok(bytes)
    }

    pub fn write(
        states_path: &Path,
        name: &str,
        date: Date,
        entries: impl Iterator<Item = (CentsCompact, Sats)>,
    ) -> Result<()> {
        let dir = Self::dir(states_path, name);
        fs::create_dir_all(&dir)?;
        fs::write(dir.join(date.to_string()), Self::serialize_iter(entries)?)?;
        Ok(())
    }

    /// Apply one scalar weight to every price bucket, flooring to whole sats.
    pub fn apply_weight(mut self, weight: f64) -> Self {
        debug_assert!(weight.is_finite() && weight >= 0.0);

        if weight == 1.0 {
            return self;
        }
        if weight == 0.0 {
            self.map.clear();
            return self;
        }

        self.map.retain(|_, sats| {
            *sats = Sats::from((u64::from(*sats) as f64 * weight).floor() as u64);
            *sats != Sats::ZERO
        });
        self
    }

    /// Deserialize from the pco-compressed format, returning remaining bytes.
    pub fn deserialize_with_rest(data: &[u8]) -> Result<(Self, &[u8])> {
        Self::decode_entries(data).map(|decoded| {
            (
                Self {
                    map: decoded.entries.into_iter().collect(),
                },
                decoded.rest,
            )
        })
    }

    fn decode_entries(data: &[u8]) -> Result<DecodedEntries<'_>> {
        if data.len() < 24 {
            return Err(Error::Deserialization(format!(
                "UrpdRaw: data too short ({} bytes, need >= 24)",
                data.len()
            )));
        }
        let read_length = |bytes: &[u8]| {
            usize::try_from(u64::from_le_bytes(bytes.try_into().unwrap())).map_err(|_| {
                Error::Deserialization("UrpdRaw: length exceeds platform capacity".into())
            })
        };
        let entry_count = read_length(&data[0..8])?;
        let keys_len = read_length(&data[8..16])?;
        let values_len = read_length(&data[16..24])?;

        let keys_start = 24_usize;
        let values_start = keys_start.checked_add(keys_len).ok_or_else(|| {
            Error::Deserialization("UrpdRaw: key section length overflows".into())
        })?;
        let rest_start = values_start.checked_add(values_len).ok_or_else(|| {
            Error::Deserialization("UrpdRaw: value section length overflows".into())
        })?;
        if rest_start > Self::MAX_ENCODED_BYTES {
            return Err(Error::Deserialization(
                "UrpdRaw: encoded section exceeds snapshot limit".into(),
            ));
        }

        if data.len() < rest_start {
            return Err(Error::Deserialization(format!(
                "UrpdRaw: data too short ({} bytes, need >= {})",
                data.len(),
                rest_start
            )));
        }

        // Reject oversized counts before compressed streams request output memory.
        if entry_count > Self::MAX_ENTRIES {
            return Err(Error::Deserialization(
                "UrpdRaw: entry count exceeds snapshot limit".into(),
            ));
        }
        let keys: Vec<u32> = decode::exact(&data[keys_start..values_start], entry_count)?;
        if keys.last() == Some(&u32::MAX) || !keys.windows(2).all(|pair| pair[0] < pair[1]) {
            return Err(Error::Deserialization(
                "UrpdRaw: prices must be finite and strictly sorted".into(),
            ));
        }
        let values: Vec<u64> = decode::exact(&data[values_start..rest_start], entry_count)?;
        checked_supply(values.iter().copied())?;

        let entries = keys
            .into_iter()
            .zip(values)
            .map(|(k, v)| (CentsCompact::new(k), Sats::from(v)))
            .collect::<Vec<_>>();

        Ok(DecodedEntries {
            entries,
            rest: &data[rest_start..],
        })
    }

    /// Deserialize exactly one sorted sequence of on-disk entries.
    pub fn deserialize_entries(data: &[u8]) -> Result<Vec<(CentsCompact, Sats)>> {
        let decoded = Self::decode_entries(data)?;
        if !decoded.rest.is_empty() {
            return Err(Error::Deserialization(format!(
                "UrpdRaw: {} trailing bytes",
                decoded.rest.len()
            )));
        }
        Ok(decoded.entries)
    }

    /// Deserialize from the pco-compressed format.
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        Self::deserialize_with_rest(data).map(|(s, _)| s)
    }

    /// Serialize to the pco-compressed format.
    pub fn serialize(&self) -> Result<Vec<u8>> {
        Self::serialize_iter(self.map.iter().map(|(&k, &v)| (k, v)))
    }

    /// Serialize from a sorted iterator of (price, sats) pairs.
    pub fn serialize_iter(iter: impl Iterator<Item = (CentsCompact, Sats)>) -> Result<Vec<u8>> {
        let mut keys = Vec::new();
        let mut values = Vec::new();
        for (key, value) in iter {
            if keys.len() == Self::MAX_ENTRIES {
                return Err(Error::Internal(
                    "UrpdRaw: entry count exceeds snapshot limit",
                ));
            }
            keys.push(
                key.finite_inner()
                    .ok_or(Error::Internal("UrpdRaw: non-finite price"))?,
            );
            values.push(u64::from(value));
        }
        checked_supply(values.iter().copied())?;

        let config = ChunkConfig::default();
        let compressed_keys = simple_compress(&keys, &config)?;
        let compressed_values = simple_compress(&values, &config)?;

        let mut buffer = Vec::new();
        buffer.extend((keys.len() as u64).to_le_bytes());
        buffer.extend((compressed_keys.len() as u64).to_le_bytes());
        buffer.extend((compressed_values.len() as u64).to_le_bytes());
        buffer.extend(compressed_keys);
        buffer.extend(compressed_values);

        Ok(buffer)
    }
}

fn checked_supply(mut values: impl Iterator<Item = u64>) -> Result<Sats> {
    values
        .try_fold(0_u64, |sum, value| {
            sum.checked_add(value)
                .filter(|sum| *sum <= Amount::MAX_MONEY.to_sat())
                .ok_or_else(|| {
                    Error::Deserialization("UrpdRaw: supply exceeds Bitcoin's maximum".into())
                })
        })
        .map(Sats::from)
}

#[cfg(test)]
#[path = "../tests/unit/urpd_raw.rs"]
mod tests;
