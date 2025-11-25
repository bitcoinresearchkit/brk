use serde::Serialize;
use vecdb::{Bytes, Formattable};

use crate::{LoadedAddressData, Sats};

/// Data of an empty address
#[derive(Debug, Default, Clone, Serialize)]
#[repr(C)]
pub struct EmptyAddressData {
    /// Total transaction count
    pub tx_count: u32,
    /// Total funded/spent transaction output count (equal since address is empty)
    pub funded_txo_count: u32,
    /// Total satoshis transferred
    pub transfered: Sats,
}

impl From<LoadedAddressData> for EmptyAddressData {
    #[inline]
    fn from(value: LoadedAddressData) -> Self {
        Self::from(&value)
    }
}

impl From<&LoadedAddressData> for EmptyAddressData {
    #[inline]
    fn from(value: &LoadedAddressData) -> Self {
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

impl std::fmt::Display for EmptyAddressData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "tx_count: {}, funded_txo_count: {}, transfered: {}",
            self.tx_count, self.funded_txo_count, self.transfered
        )
    }
}

impl Formattable for EmptyAddressData {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        true
    }
}

impl Bytes for EmptyAddressData {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.tx_count.to_bytes());
        bytes.extend_from_slice(&self.funded_txo_count.to_bytes());
        bytes.extend_from_slice(&self.transfered.to_bytes());
        bytes
    }

    fn from_bytes(bytes: &[u8]) -> vecdb::Result<Self> {
        let mut offset = 0;
        let tx_count = u32::from_bytes(&bytes[offset..])?;
        offset += tx_count.to_bytes().len();
        let funded_txo_count = u32::from_bytes(&bytes[offset..])?;
        offset += funded_txo_count.to_bytes().len();
        let transfered = Sats::from_bytes(&bytes[offset..])?;
        Ok(Self {
            tx_count,
            funded_txo_count,
            transfered,
        })
    }
}
