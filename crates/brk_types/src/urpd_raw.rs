use std::collections::BTreeMap;

use brk_error::Result;
use pco::{
    ChunkConfig,
    standalone::{simple_compress, simple_decompress},
};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::Bytes;

use crate::{CentsCompact, Sats};

/// Raw on-disk URPD: a map of price (cents) to supply (sats).
/// Processed into [`crate::Urpd`] for API responses.
#[derive(Debug, Clone, Default, Serialize, JsonSchema)]
pub struct UrpdRaw {
    pub map: BTreeMap<CentsCompact, Sats>,
}

impl UrpdRaw {
    /// Deserialize from the pco-compressed format, returning remaining bytes.
    pub fn deserialize_with_rest(data: &[u8]) -> Result<(Self, &[u8])> {
        if data.len() < 24 {
            return Err(brk_error::Error::Deserialization(format!(
                "UrpdRaw: data too short ({} bytes, need >= 24)",
                data.len()
            )));
        }
        let entry_count = usize::from_bytes(&data[0..8])?;
        let keys_len = usize::from_bytes(&data[8..16])?;
        let values_len = usize::from_bytes(&data[16..24])?;

        let keys_start = 24;
        let values_start = keys_start + keys_len;
        let rest_start = values_start + values_len;

        if data.len() < rest_start {
            return Err(brk_error::Error::Deserialization(format!(
                "UrpdRaw: data too short ({} bytes, need >= {})",
                data.len(),
                rest_start
            )));
        }

        let keys: Vec<u32> = simple_decompress(&data[keys_start..values_start])?;
        let values: Vec<u64> = simple_decompress(&data[values_start..rest_start])?;

        let map: BTreeMap<CentsCompact, Sats> = keys
            .into_iter()
            .zip(values)
            .map(|(k, v)| (CentsCompact::new(k), Sats::from(v)))
            .collect();

        debug_assert_eq!(map.len(), entry_count);

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
    pub fn serialize_iter(iter: impl Iterator<Item = (CentsCompact, Sats)>) -> Result<Vec<u8>> {
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
}
