use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Sats, StoredU64, Version};
use vecdb::{AnyStoredVec, Database, Exit, ReadableVec, Rw, StorageMode, WritableVec};

use crate::{indexes, internal::ValuePerBlock, prices};

/// Average amount held per UTXO and per funded address.
///
/// `utxo = supply / utxo_count`, `addr = supply / funded_addr_count`.
#[derive(Traversable)]
pub struct AvgAmountMetrics<M: StorageMode = Rw> {
    pub utxo: ValuePerBlock<M>,
    pub addr: ValuePerBlock<M>,
}

impl AvgAmountMetrics {
    pub(crate) fn forced_import(
        db: &Database,
        prefix: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let name = |suffix: &str| {
            if prefix.is_empty() {
                suffix.to_string()
            } else {
                format!("{prefix}_{suffix}")
            }
        };
        Ok(Self {
            utxo: ValuePerBlock::forced_import(db, &name("avg_utxo_amount"), version, indexes)?,
            addr: ValuePerBlock::forced_import(db, &name("avg_addr_amount"), version, indexes)?,
        })
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.utxo.sats.height as &mut dyn AnyStoredVec,
            &mut self.utxo.cents.height,
            &mut self.addr.sats.height,
            &mut self.addr.cents.height,
        ]
    }

    pub(crate) fn reset_height(&mut self) -> Result<()> {
        self.utxo.sats.height.reset()?;
        self.utxo.cents.height.reset()?;
        self.addr.sats.height.reset()?;
        self.addr.cents.height.reset()?;
        Ok(())
    }

    pub(crate) fn compute(
        &mut self,
        prices: &prices::Vecs,
        supply_sats: &impl ReadableVec<Height, Sats>,
        utxo_count: &impl ReadableVec<Height, StoredU64>,
        funded_addr_count: &impl ReadableVec<Height, StoredU64>,
        max_from: Height,
        exit: &Exit,
    ) -> Result<()> {
        self.utxo
            .sats
            .height
            .compute_divide(max_from, supply_sats, utxo_count, exit)?;
        self.utxo.compute(prices, max_from, exit)?;

        self.addr
            .sats
            .height
            .compute_divide(max_from, supply_sats, funded_addr_count, exit)?;
        self.addr.compute(prices, max_from, exit)?;

        Ok(())
    }
}
