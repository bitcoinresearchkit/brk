use brk_error::Result;
use brk_indexer::Indexer;
use brk_structs::{Bitcoin, Close, Dollars, Height, Sats, TxIndex, Version};
use vecdb::{
    AnyCloneableIterableVec, AnyCollectableVec, CollectableVec, Database, Exit, Format,
    LazyVecFrom1, LazyVecFrom3, StoredIndex, StoredVec,
};

use crate::{Indexes, grouped::Source, indexes, price};

use super::{ComputedVecsFromTxindex, VecBuilderOptions};

#[derive(Clone)]
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
        indexes: &indexes::Vecs,
        source: Source<TxIndex, Sats>,
        version: Version,
        format: Format,
        price: Option<&price::Vecs>,
        options: VecBuilderOptions,
    ) -> Result<Self> {
        let compute_dollars = price.is_some();

        let name_in_btc = format!("{name}_in_btc");
        let name_in_usd = format!("{name}_in_usd");

        let sats = ComputedVecsFromTxindex::forced_import(
            db,
            name,
            source.clone(),
            version + VERSION,
            format,
            indexes,
            options,
        )?;

        let source_vec = source.vec();

        let bitcoin_txindex = LazyVecFrom1::init(
            &name_in_btc,
            version + VERSION,
            source_vec.map_or_else(|| sats.txindex.as_ref().unwrap().boxed_clone(), |s| s),
            |txindex: TxIndex, iter| {
                iter.next_at(txindex.unwrap_to_usize()).map(|(_, value)| {
                    let sats = value.into_owned();
                    Bitcoin::from(sats)
                })
            },
        );

        let bitcoin = ComputedVecsFromTxindex::forced_import(
            db,
            &name_in_btc,
            Source::None,
            version + VERSION,
            format,
            indexes,
            options,
        )?;

        let dollars_txindex = price.map(|price| {
            LazyVecFrom3::init(
                &name_in_usd,
                version + VERSION,
                bitcoin_txindex.boxed_clone(),
                indexes.txindex_to_height.boxed_clone(),
                price.chainindexes_to_close.height.boxed_clone(),
                |txindex: TxIndex,
                 txindex_to_btc_iter,
                 txindex_to_height_iter,
                 height_to_close_iter| {
                    let txindex = txindex.unwrap_to_usize();
                    txindex_to_btc_iter.next_at(txindex).and_then(|(_, value)| {
                        let btc = value.into_owned();
                        txindex_to_height_iter
                            .next_at(txindex)
                            .and_then(|(_, value)| {
                                let height = value.into_owned();
                                height_to_close_iter
                                    .next_at(height.unwrap_to_usize())
                                    .map(|(_, close)| *close.into_owned() * btc)
                            })
                    })
                },
            )
        });

        Ok(Self {
            sats,
            bitcoin_txindex,
            bitcoin,
            dollars_txindex,
            dollars: compute_dollars.then(|| {
                ComputedVecsFromTxindex::forced_import(
                    db,
                    &name_in_usd,
                    Source::None,
                    version + VERSION,
                    format,
                    indexes,
                    options,
                )
                .unwrap()
            }),
        })
    }

    // pub fn compute_all<F>(
    //     &mut self,
    //     indexer: &Indexer,
    //     indexes: &indexes::Vecs,
    //     price: Option<&marketprice::Vecs>,
    //     starting_indexes: &Indexes,
    //     exit: &Exit,
    //     mut compute: F,
    // ) -> Result<()>
    // where
    //     F: FnMut(
    //         &mut EagerVec<TxIndex, Sats>,
    //         &Indexer,
    //         &indexes::Vecs,
    //         &Indexes,
    //         &Exit,
    //     ) -> Result<()>,
    // {
    //     compute(
    //         self.sats.txindex.as_mut().unwrap(),
    //         indexer,
    //         indexes,
    //         starting_indexes,
    //         exit,
    //     )?;

    //     let txindex: Option<&StoredVec<TxIndex, Sats>> = None;
    //     self.compute_rest(
    //         indexer,
    //         indexes,
    //         fetched,
    //         starting_indexes,
    //         exit,
    //         txindex,
    //     )?;

    //     Ok(())
    // }

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
            let txindex: Option<&StoredVec<TxIndex, Sats>> = None;
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
            let dollars_txindex = self.dollars_txindex.as_mut().unwrap();

            dollars.compute_rest_from_bitcoin(
                indexer,
                indexes,
                starting_indexes,
                exit,
                &self.bitcoin,
                Some(dollars_txindex),
                price.as_ref().unwrap(),
            )?;
        }

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            self.sats.vecs(),
            vec![&self.bitcoin_txindex as &dyn AnyCollectableVec],
            self.bitcoin.vecs(),
            self.dollars_txindex
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.dollars.as_ref().map_or(vec![], |v| v.vecs()),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
