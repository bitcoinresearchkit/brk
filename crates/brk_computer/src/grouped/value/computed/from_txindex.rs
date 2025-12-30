use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Close, Dollars, Height, Sats, TxIndex, Version};
use vecdb::{
    CollectableVec, Database, Exit, IterableCloneableVec, LazyVecFrom1, LazyVecFrom3, PcoVec,
    VecIndex,
};

use crate::{Indexes, grouped::Source, indexes, price, utils::OptionExt};

use crate::grouped::{ComputedVecsFromTxindex, VecBuilderOptions};

#[derive(Clone, Traversable)]
pub struct ComputedValueVecsFromTxindex {
    pub sats: ComputedVecsFromTxindex<Sats>,
    pub bitcoin_txindex: LazyVecFrom1<TxIndex, Bitcoin, TxIndex, Sats>,
    pub bitcoin: ComputedVecsFromTxindex<Bitcoin>,
    #[allow(clippy::type_complexity)]
    pub dollars_txindex: Option<
        LazyVecFrom3<TxIndex, Dollars, TxIndex, Bitcoin, TxIndex, Height, Height, Close<Dollars>>,
    >,
    pub dollars: Option<ComputedVecsFromTxindex<Dollars>>,
}

const VERSION: Version = Version::ZERO;

impl ComputedValueVecsFromTxindex {
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        db: &Database,
        name: &str,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        source: Source<TxIndex, Sats>,
        version: Version,
        price: Option<&price::Vecs>,
        options: VecBuilderOptions,
    ) -> Result<Self> {
        let name_btc = format!("{name}_btc");
        let name_usd = format!("{name}_usd");

        let sats = ComputedVecsFromTxindex::forced_import(
            db,
            name,
            source.clone(),
            version + VERSION,
            indexes,
            options,
        )?;

        let source_vec = source.vec();

        let bitcoin_txindex = LazyVecFrom1::init(
            &name_btc,
            version + VERSION,
            source_vec.unwrap_or_else(|| sats.txindex.u().boxed_clone()),
            |txindex: TxIndex, iter| iter.get_at(txindex.to_usize()).map(Bitcoin::from),
        );

        let bitcoin = ComputedVecsFromTxindex::forced_import(
            db,
            &name_btc,
            Source::Vec(bitcoin_txindex.boxed_clone()),
            version + VERSION,
            indexes,
            options,
        )?;

        let dollars_txindex = price.map(|price| {
            LazyVecFrom3::init(
                &name_usd,
                version + VERSION,
                bitcoin_txindex.boxed_clone(),
                indexer.vecs.tx.txindex_to_height.boxed_clone(),
                price.chainindexes_to_price_close.height.boxed_clone(),
                |txindex: TxIndex,
                 txindex_to_btc_iter,
                 txindex_to_height_iter,
                 height_to_price_close_iter| {
                    let txindex = txindex.to_usize();
                    txindex_to_btc_iter.get_at(txindex).and_then(|btc| {
                        txindex_to_height_iter.get_at(txindex).and_then(|height| {
                            height_to_price_close_iter
                                .get_at(height.to_usize())
                                .map(|close| *close * btc)
                        })
                    })
                },
            )
        });

        Ok(Self {
            sats,
            bitcoin_txindex,
            bitcoin,
            dollars: dollars_txindex.as_ref().map(|dtx| {
                ComputedVecsFromTxindex::forced_import(
                    db,
                    &name_usd,
                    Source::Vec(dtx.boxed_clone()),
                    version + VERSION,
                    indexes,
                    options,
                )
                .unwrap()
            }),
            dollars_txindex,
        })
    }

    pub fn compute_rest(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        txindex: Option<&impl CollectableVec<TxIndex, Sats>>,
        price: Option<&price::Vecs>,
    ) -> Result<()> {
        if let Some(txindex) = txindex {
            self.sats
                .compute_rest(indexer, indexes, starting_indexes, exit, Some(txindex))?;
        } else {
            let txindex: Option<&PcoVec<TxIndex, Sats>> = None;
            self.sats
                .compute_rest(indexer, indexes, starting_indexes, exit, txindex)?;
        }

        self.bitcoin.compute_rest_from_sats(
            indexer,
            indexes,
            starting_indexes,
            exit,
            &self.sats,
            Some(&self.bitcoin_txindex),
        )?;

        if let Some(dollars) = self.dollars.as_mut() {
            let dollars_txindex = self.dollars_txindex.um();

            dollars.compute_rest_from_bitcoin(
                indexer,
                indexes,
                starting_indexes,
                exit,
                &self.bitcoin,
                Some(dollars_txindex),
                price.u(),
            )?;
        }

        Ok(())
    }
}
