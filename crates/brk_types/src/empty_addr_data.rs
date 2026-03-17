use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{Bytes, Formattable};

use crate::{FundedAddrData, Sats};

/// Data of an empty address
#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
#[repr(C)]
pub struct EmptyAddrData {
    /// Total transaction count
    pub tx_count: u32,
    /// Total funded/spent transaction output count (equal since address is empty)
    pub funded_txo_count: u32,
    /// Total satoshis transferred
    pub transfered: Sats,
}

impl From<FundedAddrData> for EmptyAddrData {
    #[inline]
    fn from(value: FundedAddrData) -> Self {
        Self::from(&value)
    }
}

impl From<&FundedAddrData> for EmptyAddrData {
    #[inline]
    fn from(value: &FundedAddrData) -> Self {
        if value.sent != value.received {
            dbg!(&value);
            panic!("Trying to convert not empty wallet to empty !");
        }
        Self {
            tx_count: value.tx_count,
            funded_txo_count: value.funded_txo_count,
            transfered: value.sent,
        }
    }
}

impl std::fmt::Display for EmptyAddrData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "tx_count: {}, funded_txo_count: {}, transfered: {}",
            self.tx_count, self.funded_txo_count, self.transfered
        )
    }
}

impl Formattable for EmptyAddrData {
    fn write_to(&self, buf: &mut Vec<u8>) {
        use std::fmt::Write;
        let mut s = String::new();
        write!(s, "{}", self).unwrap();
        buf.extend_from_slice(s.as_bytes());
    }

    fn fmt_csv(&self, f: &mut String) -> std::fmt::Result {
        let start = f.len();
        self.fmt_into(f);
        if f.as_bytes()[start..].contains(&b',') {
            f.insert(start, '"');
            f.push('"');
        }
        Ok(())
    }

    fn fmt_json(&self, buf: &mut Vec<u8>) {
        buf.push(b'"');
        self.write_to(buf);
        buf.push(b'"');
    }
}

impl Bytes for EmptyAddrData {
    type Array = [u8; size_of::<Self>()];

    fn to_bytes(&self) -> Self::Array {
        let mut arr = [0u8; size_of::<Self>()];
        arr[0..4].copy_from_slice(self.tx_count.to_bytes().as_ref());
        arr[4..8].copy_from_slice(self.funded_txo_count.to_bytes().as_ref());
        arr[8..16].copy_from_slice(self.transfered.to_bytes().as_ref());
        arr
    }

    fn from_bytes(bytes: &[u8]) -> vecdb::Result<Self> {
        Ok(Self {
            tx_count: u32::from_bytes(&bytes[0..4])?,
            funded_txo_count: u32::from_bytes(&bytes[4..8])?,
            transfered: Sats::from_bytes(&bytes[8..16])?,
        })
    }
}
