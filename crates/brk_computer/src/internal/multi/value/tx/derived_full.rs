//! Value type for Full pattern from TxIndex.

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Sats, TxIndex, Version};
use vecdb::{CollectableVec, Database, Exit, IterableCloneableVec};

use crate::{
    ComputeIndexes, indexes,
    internal::{DerivedTxFull, ValueDollarsTxFull, LazyDerivedTxFull, SatsToBitcoin},
    price,
};

#[derive(Clone, Traversable)]
pub struct ValueDerivedTxFull {
    pub sats: DerivedTxFull<Sats>,
    pub bitcoin: LazyDerivedTxFull<Bitcoin, Sats>,
    pub dollars: Option<ValueDollarsTxFull>,
}

const VERSION: Version = Version::ZERO;

impl ValueDerivedTxFull {
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

        let sats = DerivedTxFull::forced_import(db, name, v, indexes)?;

        let bitcoin =
            LazyDerivedTxFull::from_computed::<SatsToBitcoin>(&format!("{name}_btc"), v, &sats);

        let dollars = price
            .map(|price| {
                ValueDollarsTxFull::forced_import(
                    db,
                    &format!("{name}_usd"),
                    v,
                    indexes,
                    sats_txindex.boxed_clone(),
                    indexer.vecs.transactions.height.boxed_clone(),
                    price.usd.split.close.height.boxed_clone(),
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
        self.sats
            .derive_from(indexer, indexes, starting_indexes, txindex_source, exit)?;

        if let Some(dollars) = self.dollars.as_mut() {
            dollars.derive_from(indexer, indexes, starting_indexes, exit)?;
        }

        Ok(())
    }
}
