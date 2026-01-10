//! Value type for Full pattern from TxIndex.

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Sats, TxIndex, Version};
use vecdb::{CollectableVec, Database, Exit, IterableCloneableVec};

use crate::{
    ComputeIndexes, indexes,
    internal::{TxDerivedFull, ValueDollarsFromTxFull, LazyTxDerivedFull, SatsToBitcoin},
    price,
};

#[derive(Clone, Traversable)]
pub struct ValueTxDerivedFull {
    pub sats: TxDerivedFull<Sats>,
    pub bitcoin: LazyTxDerivedFull<Bitcoin, Sats>,
    pub dollars: Option<ValueDollarsFromTxFull>,
}

const VERSION: Version = Version::ZERO;

impl ValueTxDerivedFull {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        indexer: &Indexer,
        price: Option<&price::Vecs>,
        sats_txindex: &impl IterableCloneableVec<TxIndex, Sats>,
    ) -> Result<Self> {
        let v = version + VERSION;

        let sats = TxDerivedFull::forced_import(db, name, v, indexes)?;

        let bitcoin =
            LazyTxDerivedFull::from_computed::<SatsToBitcoin>(&format!("{name}_btc"), v, &sats);

        let dollars = price
            .map(|price| {
                ValueDollarsFromTxFull::forced_import(
                    db,
                    &format!("{name}_usd"),
                    v,
                    indexes,
                    &sats.height,
                    price.usd.split.close.height.boxed_clone(),
                    sats_txindex.boxed_clone(),
                    indexer.vecs.transactions.height.boxed_clone(),
                )
            })
            .transpose()?;

        Ok(Self {
            sats,
            bitcoin,
            dollars,
        })
    }

    pub fn derive_from(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        txindex_source: &impl CollectableVec<TxIndex, Sats>,
        exit: &Exit,
    ) -> Result<()> {
        self.derive_from_with_skip(indexer, indexes, starting_indexes, txindex_source, exit, 0)
    }

    /// Derive from source, skipping first N transactions per block from all calculations.
    ///
    /// Use `skip_count: 1` to exclude coinbase transactions from fee/feerate stats.
    pub fn derive_from_with_skip(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        txindex_source: &impl CollectableVec<TxIndex, Sats>,
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

        if let Some(dollars) = self.dollars.as_mut() {
            dollars.derive_from(indexer, indexes, starting_indexes, exit)?;
        }

        Ok(())
    }
}
