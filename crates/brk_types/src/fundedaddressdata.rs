use brk_error::{Error, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{Bytes, Formattable};

use crate::{CentsSats, CentsSquaredSats, Cents, EmptyAddressData, Sats, SupplyState};

/// Snapshot of cost basis related state.
/// Uses CentsSats (u64) for single-UTXO values, CentsSquaredSats (u128) for investor cap.
#[derive(Clone, Debug)]
pub struct CostBasisSnapshot {
    pub realized_price: Cents,
    pub supply_state: SupplyState,
    /// price × sats (fits u64 for individual UTXOs)
    pub price_sats: CentsSats,
    /// price² × sats (needs u128)
    pub investor_cap: CentsSquaredSats,
}

impl CostBasisSnapshot {
    /// Create from a single UTXO (computes caps from price × value)
    #[inline]
    pub fn from_utxo(price: Cents, supply: &SupplyState) -> Self {
        let price_sats = CentsSats::from_price_sats(price, supply.value);
        Self {
            realized_price: price,
            supply_state: supply.clone(),
            price_sats,
            investor_cap: price_sats.to_investor_cap(price),
        }
    }
}

/// Data for a funded (non-empty) address with current balance
#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
#[repr(C)]
pub struct FundedAddressData {
    /// Total transaction count
    pub tx_count: u32,
    /// Number of transaction outputs funded to this address
    pub funded_txo_count: u32,
    /// Number of transaction outputs spent by this address
    pub spent_txo_count: u32,
    #[serde(skip)]
    padding: u32,
    /// Satoshis received by this address
    pub received: Sats,
    /// Satoshis sent by this address
    pub sent: Sats,
    /// The realized capitalization: Σ(price × sats)
    pub realized_cap_raw: CentsSats,
    /// The investor capitalization: Σ(price² × sats)
    pub investor_cap_raw: CentsSquaredSats,
}

impl FundedAddressData {
    pub fn balance(&self) -> Sats {
        (u64::from(self.received) - u64::from(self.sent)).into()
    }

    pub fn realized_price(&self) -> Cents {
        self.realized_cap_raw.realized_price(self.balance())
    }

    pub fn cost_basis_snapshot(&self) -> CostBasisSnapshot {
        let realized_price = self.realized_price();
        CostBasisSnapshot {
            realized_price,
            supply_state: SupplyState {
                utxo_count: self.utxo_count() as u64,
                value: self.balance(),
            },
            // Use exact value to avoid rounding errors from realized_price × balance
            price_sats: CentsSats::new(self.realized_cap_raw.inner()),
            investor_cap: self.investor_cap_raw,
        }
    }

    #[inline]
    pub fn has_0_sats(&self) -> bool {
        self.balance() == Sats::ZERO
    }

    #[inline]
    pub fn utxo_count(&self) -> u32 {
        self.funded_txo_count
            .checked_sub(self.spent_txo_count)
            .unwrap_or_else(|| {
                panic!(
                    "FundedAddressData corruption: spent_txo_count ({}) > funded_txo_count ({}). \
                Address data: {:?}",
                    self.spent_txo_count, self.funded_txo_count, self
                )
            })
    }

    #[inline]
    pub fn has_1_utxos(&self) -> bool {
        self.utxo_count() == 1
    }

    #[inline]
    pub fn has_0_utxos(&self) -> bool {
        self.funded_txo_count == self.spent_txo_count
    }

    pub fn receive(&mut self, amount: Sats, price: Cents) {
        self.receive_outputs(amount, price, 1);
    }

    pub fn receive_outputs(
        &mut self,
        amount: Sats,
        price: Cents,
        output_count: u32,
    ) {
        self.received += amount;
        self.funded_txo_count += output_count;
        let ps = CentsSats::from_price_sats(price, amount);
        self.realized_cap_raw += ps;
        self.investor_cap_raw += ps.to_investor_cap(price);
    }

    pub fn send(&mut self, amount: Sats, previous_price: Cents) -> Result<()> {
        if self.balance() < amount {
            return Err(Error::Internal("Previous amount smaller than sent amount"));
        }
        self.sent += amount;
        self.spent_txo_count += 1;
        let ps = CentsSats::from_price_sats(previous_price, amount);
        self.realized_cap_raw -= ps;
        self.investor_cap_raw -= ps.to_investor_cap(previous_price);
        Ok(())
    }
}

impl From<EmptyAddressData> for FundedAddressData {
    #[inline]
    fn from(value: EmptyAddressData) -> Self {
        Self::from(&value)
    }
}

impl From<&EmptyAddressData> for FundedAddressData {
    #[inline]
    fn from(value: &EmptyAddressData) -> Self {
        Self {
            tx_count: value.tx_count,
            funded_txo_count: value.funded_txo_count,
            spent_txo_count: value.funded_txo_count,
            padding: 0,
            received: value.transfered,
            sent: value.transfered,
            realized_cap_raw: CentsSats::ZERO,
            investor_cap_raw: CentsSquaredSats::ZERO,
        }
    }
}

impl std::fmt::Display for FundedAddressData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "tx_count: {}, funded_txo_count: {}, spent_txo_count: {}, received: {}, sent: {}, realized_cap_raw: {}, investor_cap_raw: {}",
            self.tx_count,
            self.funded_txo_count,
            self.spent_txo_count,
            self.received,
            self.sent,
            self.realized_cap_raw,
            self.investor_cap_raw,
        )
    }
}

impl Formattable for FundedAddressData {
    fn fmt_csv(&self, f: &mut String) -> std::fmt::Result {
        use std::fmt::Write;
        let start = f.len();
        write!(f, "{}", self)?;
        if f.as_bytes()[start..].contains(&b',') {
            f.insert(start, '"');
            f.push('"');
        }
        Ok(())
    }
}

impl Bytes for FundedAddressData {
    type Array = [u8; size_of::<Self>()];

    fn to_bytes(&self) -> Self::Array {
        let mut arr = [0u8; size_of::<Self>()];
        arr[0..4].copy_from_slice(self.tx_count.to_bytes().as_ref());
        arr[4..8].copy_from_slice(self.funded_txo_count.to_bytes().as_ref());
        arr[8..12].copy_from_slice(self.spent_txo_count.to_bytes().as_ref());
        arr[12..16].copy_from_slice(self.padding.to_bytes().as_ref());
        arr[16..24].copy_from_slice(self.received.to_bytes().as_ref());
        arr[24..32].copy_from_slice(self.sent.to_bytes().as_ref());
        arr[32..48].copy_from_slice(self.realized_cap_raw.to_bytes().as_ref());
        arr[48..64].copy_from_slice(self.investor_cap_raw.to_bytes().as_ref());
        arr
    }

    fn from_bytes(bytes: &[u8]) -> vecdb::Result<Self> {
        Ok(Self {
            tx_count: u32::from_bytes(&bytes[0..4])?,
            funded_txo_count: u32::from_bytes(&bytes[4..8])?,
            spent_txo_count: u32::from_bytes(&bytes[8..12])?,
            padding: u32::from_bytes(&bytes[12..16])?,
            received: Sats::from_bytes(&bytes[16..24])?,
            sent: Sats::from_bytes(&bytes[24..32])?,
            realized_cap_raw: CentsSats::from_bytes(&bytes[32..48])?,
            investor_cap_raw: CentsSquaredSats::from_bytes(&bytes[48..64])?,
        })
    }
}
