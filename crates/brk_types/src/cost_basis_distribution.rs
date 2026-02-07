use std::collections::BTreeMap;

use rustc_hash::FxHashMap;

use brk_error::Result;
use pco::{ChunkConfig, standalone::{simple_compress, simple_decompress}};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::Bytes;

use crate::{Bitcoin, CentsUnsigned, CentsUnsignedCompact, CostBasisBucket, CostBasisValue, Dollars, Sats};

/// Cost basis distribution: a map of price (cents) to sats.
#[derive(Debug, Clone, Default, Serialize, JsonSchema)]
pub struct CostBasisDistribution {
    pub map: BTreeMap<CentsUnsignedCompact, Sats>,
}

/// Formatted cost basis output.
/// Key: price floor in USD (dollars).
/// Value: BTC (for supply) or USD (for realized/unrealized).
pub type CostBasisFormatted = BTreeMap<Dollars, f64>;

impl CostBasisDistribution {
    /// Deserialize from the pco-compressed format, returning remaining bytes.
    pub fn deserialize_with_rest(data: &[u8]) -> Result<(Self, &[u8])> {
        let entry_count = usize::from_bytes(&data[0..8])?;
        let keys_len = usize::from_bytes(&data[8..16])?;
        let values_len = usize::from_bytes(&data[16..24])?;

        let keys_start = 24;
        let values_start = keys_start + keys_len;
        let rest_start = values_start + values_len;

        let keys: Vec<u32> = simple_decompress(&data[keys_start..values_start])?;
        let values: Vec<u64> = simple_decompress(&data[values_start..rest_start])?;

        let map: BTreeMap<CentsUnsignedCompact, Sats> = keys
            .into_iter()
            .zip(values)
            .map(|(k, v)| (CentsUnsignedCompact::new(k), Sats::from(v)))
            .collect();

        assert_eq!(map.len(), entry_count);

        Ok((Self { map }, &data[rest_start..]))
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
    pub fn serialize_iter(iter: impl Iterator<Item = (CentsUnsignedCompact, Sats)>) -> Result<Vec<u8>> {
        let entries: Vec<_> = iter.collect();
        let keys: Vec<u32> = entries.iter().map(|(k, _)| k.inner()).collect();
        let values: Vec<u64> = entries.iter().map(|(_, v)| u64::from(*v)).collect();

        let config = ChunkConfig::default();
        let compressed_keys = simple_compress(&keys, &config)?;
        let compressed_values = simple_compress(&values, &config)?;

        let mut buffer = Vec::new();
        buffer.extend(keys.len().to_bytes());
        buffer.extend(compressed_keys.len().to_bytes());
        buffer.extend(compressed_values.len().to_bytes());
        buffer.extend(compressed_keys);
        buffer.extend(compressed_values);

        Ok(buffer)
    }

    /// Format the distribution with optional bucketing and value transformation.
    ///
    /// - `bucket`: How to aggregate prices (raw, linear, or logarithmic)
    /// - `value`: What value to compute (supply, realized, or unrealized)
    /// - `spot_cents`: Current spot price in cents (required for unrealized)
    pub fn format(
        &self,
        bucket: CostBasisBucket,
        value: CostBasisValue,
        spot_cents: CentsUnsigned,
    ) -> CostBasisFormatted {
        let spot = Dollars::from(spot_cents);
        let needs_realized = value == CostBasisValue::Realized;
        let mut result: FxHashMap<CentsUnsigned, (Sats, Dollars)> =
            FxHashMap::with_capacity_and_hasher(self.map.len(), Default::default());

        // Aggregate into buckets
        for (&price_cents, &sats) in &self.map {
            let price_cents_u = CentsUnsigned::from(price_cents);

            let bucket_key = match bucket {
                CostBasisBucket::Raw => price_cents_u,
                _ => bucket.bucket_floor(price_cents_u).unwrap_or(price_cents_u),
            };

            let entry = result.entry(bucket_key).or_insert((Sats::ZERO, Dollars::ZERO));
            entry.0 += sats;
            // Only compute realized value if needed
            if needs_realized {
                entry.1 += Dollars::from(price_cents_u) * sats;
            }
        }

        // Convert to final output based on value type
        result
            .into_iter()
            .map(|(cents, (sats, realized))| {
                let k = Dollars::from(cents);
                let v = match value {
                    CostBasisValue::Supply => f64::from(Bitcoin::from(sats)),
                    CostBasisValue::Realized => f64::from(realized),
                    CostBasisValue::Unrealized => f64::from((spot - k) * sats),
                };
                (k, v)
            })
            .collect()
    }
}
