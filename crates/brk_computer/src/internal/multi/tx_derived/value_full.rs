//! Value type for Full pattern from TxIndex.

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Sats, TxIndex, Version};
use vecdb::{Database, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode};

use crate::{
    ComputeIndexes, indexes,
    internal::{LazyTxDerivedFull, SatsToBitcoin, TxDerivedFull, ValueDollarsFromTxFull},
    prices,
};

#[derive(Traversable)]
pub struct ValueTxDerivedFull<M: StorageMode = Rw> {
    pub sats: TxDerivedFull<Sats, M>,
    pub btc: LazyTxDerivedFull<Bitcoin, Sats>,
    pub usd: ValueDollarsFromTxFull<M>,
}

const VERSION: Version = Version::ZERO;

impl ValueTxDerivedFull {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        indexer: &Indexer,
        prices: &prices::Vecs,
        sats_txindex: &impl ReadableCloneableVec<TxIndex, Sats>,
    ) -> Result<Self> {
        let v = version + VERSION;

        let sats = TxDerivedFull::forced_import(db, name, v, indexes)?;

        let btc =
            LazyTxDerivedFull::from_computed::<SatsToBitcoin>(&format!("{name}_btc"), v, &sats);

        let usd = ValueDollarsFromTxFull::forced_import(
            db,
            &format!("{name}_usd"),
            v,
            indexes,
            &sats.height,
            prices.usd.price.read_only_boxed_clone(),
            sats_txindex.read_only_boxed_clone(),
            indexer.vecs.transactions.height.read_only_boxed_clone(),
        )?;

        Ok(Self {
            sats,
            btc,
            usd,
        })
    }

    /// Derive from source, skipping first N transactions per block from all calculations.
    ///
    /// Use `skip_count: 1` to exclude coinbase transactions from fee/feerate stats.
    pub(crate) fn derive_from_with_skip(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        txindex_source: &impl ReadableVec<TxIndex, Sats>,
        exit: &Exit,
        skip_count: usize,
    ) -> Result<()> {
        self.sats.derive_from_with_skip(
            indexer,
            indexes,
            starting_indexes,
            txindex_source,
            exit,
            skip_count,
        )?;

        self.usd.derive_from(indexes, starting_indexes, exit)?;

        Ok(())
    }
}
