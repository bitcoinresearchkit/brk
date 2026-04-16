//! Exposed address tracking (quantum / pubkey-exposure sense).
//!
//! An address is "exposed" once its public key is in the blockchain. Once
//! exposed, any funds at that address are at cryptographic risk (e.g. from
//! a quantum attacker capable of recovering the private key from the pubkey).
//!
//! When the pubkey gets exposed depends on the address type:
//!
//! - **P2PK33, P2PK65, P2TR**: the pubkey (or P2TR's tweaked output key) is
//!   directly in the locking script of the funding output. These addresses are
//!   exposed the moment they receive any funds.
//! - **P2PKH, P2SH, P2WPKH, P2WSH**: the locking script contains a hash of
//!   the pubkey/script. The pubkey is only revealed when spending. Note that
//!   even the spending tx itself exposes the pubkey while the address still
//!   holds funds — during the mempool window between broadcast and confirmation,
//!   the pubkey is visible while the UTXO being spent is still unspent on-chain.
//!   So every spent address of these types has had at least one moment with
//!   funds at quantum risk.
//! - **P2A**: anyone-can-spend, no pubkey at all. Excluded from both counters.
//!
//! Formally, with `is_funding_exposed` = `output_type.pubkey_exposed_at_funding()`:
//! - `funded` (count): `(utxo_count > 0) AND (is_funding_exposed OR spent_txo_count >= 1)`
//! - `total` (count): `(is_funding_exposed AND ever received) OR spent_txo_count >= 1`
//! - `supply` (sats): sum of balances of addresses currently in the funded set
//!
//! For P2PK/P2TR types this means `total ≡ total_addr_count` and
//! `funded ≡ funded_addr_count` (every address of those types is exposed by
//! virtue of existing). For P2PKH/P2SH/P2WPKH/P2WSH it's the strict subset of
//! addresses that have been spent from. The aggregate `all` exposed counter
//! sums these, giving "Bitcoin addresses currently with funds at quantum risk".
//!
//! All metrics are tracked as running counters and require no extra fields
//! on the address data — they're maintained via delta detection in
//! `process_received` and `process_sent`.

mod count;
mod supply;

pub use count::{AddrTypeToExposedAddrCount, ExposedAddrCountsVecs};
pub use supply::{AddrTypeToExposedSupply, ExposedAddrSupplyVecs, ExposedSupplyShareVecs};

use brk_cohort::ByAddrType;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Indexes, Sats, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{indexes, internal::RatioSatsBp16, prices};

/// Top-level container for all exposed address tracking: counts (funded +
/// total), the funded supply, and share of supply.
#[derive(Traversable)]
pub struct ExposedAddrVecs<M: StorageMode = Rw> {
    pub count: ExposedAddrCountsVecs<M>,
    pub supply: ExposedAddrSupplyVecs<M>,
    #[traversable(wrap = "supply", rename = "share")]
    pub supply_share: ExposedSupplyShareVecs<M>,
}

impl ExposedAddrVecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            count: ExposedAddrCountsVecs::forced_import(db, version, indexes)?,
            supply: ExposedAddrSupplyVecs::forced_import(db, version, indexes)?,
            supply_share: ExposedSupplyShareVecs::forced_import(db, version, indexes)?,
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.count
            .min_stateful_len()
            .min(self.supply.min_stateful_len())
    }

    pub(crate) fn par_iter_height_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        self.count
            .par_iter_height_mut()
            .chain(self.supply.par_iter_height_mut())
    }

    pub(crate) fn reset_height(&mut self) -> Result<()> {
        self.count.reset_height()?;
        self.supply.reset_height()?;
        self.supply_share.reset_height()?;
        Ok(())
    }

    pub(crate) fn compute_rest(
        &mut self,
        starting_indexes: &Indexes,
        prices: &prices::Vecs,
        all_supply_sats: &impl ReadableVec<Height, Sats>,
        type_supply_sats: &ByAddrType<&impl ReadableVec<Height, Sats>>,
        exit: &Exit,
    ) -> Result<()> {
        self.count.compute_rest(starting_indexes, exit)?;
        self.supply
            .compute_rest(starting_indexes.height, prices, exit)?;

        let max_from = starting_indexes.height;

        self.supply_share
            .all
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from,
                &self.supply.all.sats.height,
                all_supply_sats,
                exit,
            )?;

        for ((_, share), ((_, exposed), (_, denom))) in self
            .supply_share
            .by_addr_type
            .iter_mut()
            .zip(self.supply.by_addr_type.iter().zip(type_supply_sats.iter()))
        {
            share.compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from,
                &exposed.sats.height,
                *denom,
                exit,
            )?;
        }

        Ok(())
    }
}
