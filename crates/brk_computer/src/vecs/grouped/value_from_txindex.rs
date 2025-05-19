use std::path::Path;

use brk_core::{Bitcoin, Close, Dollars, Height, Sats, TxIndex};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{
    AnyCollectableVec, BoxedAnyIterableVec, CloneableAnyIterableVec, CollectableVec, Compressed,
    Computation, ComputedVecFrom3, LazyVecFrom1, StoredIndex, StoredVec, Version,
};

use crate::vecs::{Indexes, fetched, indexes};

use super::{ComputedVecsFromTxindex, StorableVecGeneatorOptions};

#[derive(Clone)]
pub struct ComputedValueVecsFromTxindex {
    pub sats: ComputedVecsFromTxindex<Sats>,
    pub bitcoin_txindex: LazyVecFrom1<TxIndex, Bitcoin, TxIndex, Sats>,
    pub bitcoin: ComputedVecsFromTxindex<Bitcoin>,
    #[allow(clippy::type_complexity)]
    pub dollars_txindex: Option<
        ComputedVecFrom3<
            TxIndex,
            Dollars,
            TxIndex,
            Bitcoin,
            TxIndex,
            Height,
            Height,
            Close<Dollars>,
        >,
    >,
    pub dollars: Option<ComputedVecsFromTxindex<Dollars>>,
}

const VERSION: Version = Version::ONE;

impl ComputedValueVecsFromTxindex {
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        path: &Path,
        name: &str,
        indexes: &indexes::Vecs,
        source: Option<BoxedAnyIterableVec<TxIndex, Sats>>,
        version: Version,
        computation: Computation,
        compressed: Compressed,
        fetched: Option<&fetched::Vecs>,
        options: StorableVecGeneatorOptions,
    ) -> color_eyre::Result<Self> {
        let compute_source = source.is_none();
        let compute_dollars = fetched.is_some();

        let name_in_btc = format!("{name}_in_btc");
        let name_in_usd = format!("{name}_in_usd");

        let sats = ComputedVecsFromTxindex::forced_import(
            path,
            name,
            compute_source,
            VERSION + version,
            compressed,
            options,
        )?;

        let bitcoin_txindex = LazyVecFrom1::init(
            &name_in_btc,
            VERSION + version,
            source.map_or_else(|| sats.txindex.as_ref().unwrap().boxed_clone(), |s| s),
            |txindex: TxIndex, iter| {
                iter.next_at(txindex.unwrap_to_usize()).map(|(_, value)| {
                    let sats = value.into_inner();
                    Bitcoin::from(sats)
                })
            },
        );

        let bitcoin = ComputedVecsFromTxindex::forced_import(
            path,
            &name_in_btc,
            false,
            VERSION + version,
            compressed,
            options,
        )?;

        let dollars_txindex = fetched.map(|fetched| {
            ComputedVecFrom3::forced_import_or_init_from_3(
                computation,
                path,
                &name_in_usd,
                VERSION + version,
                compressed,
                bitcoin_txindex.boxed_clone(),
                indexes.txindex_to_height.boxed_clone(),
                fetched.chainindexes_to_close.height.boxed_clone(),
                |txindex: TxIndex,
                 txindex_to_btc_iter,
                 txindex_to_height_iter,
                 height_to_close_iter| {
                    let txindex = txindex.unwrap_to_usize();
                    txindex_to_btc_iter.next_at(txindex).and_then(|(_, value)| {
                        let btc = value.into_inner();
                        txindex_to_height_iter
                            .next_at(txindex)
                            .and_then(|(_, value)| {
                                let height = value.into_inner();
                                height_to_close_iter
                                    .next_at(height.unwrap_to_usize())
                                    .map(|(_, close)| *close.into_inner() * btc)
                            })
                    })
                },
            )
            .unwrap()
        });

        Ok(Self {
            sats,
            bitcoin_txindex,
            bitcoin,
            dollars_txindex,
            dollars: compute_dollars.then(|| {
                ComputedVecsFromTxindex::forced_import(
                    path,
                    &name_in_usd,
                    false,
                    VERSION + version,
                    compressed,
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
    //     fetched: Option<&marketprice::Vecs>,
    //     starting_indexes: &Indexes,
    //     exit: &Exit,
    //     mut compute: F,
    // ) -> color_eyre::Result<()>
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
    ) -> color_eyre::Result<()> {
        if let Some(txindex) = txindex {
            self.sats
                .compute_rest(indexer, indexes, starting_indexes, exit, Some(txindex))?;
        } else {
            let txindex: Option<&StoredVec<TxIndex, Sats>> = None;
            self.sats
                .compute_rest(indexer, indexes, starting_indexes, exit, txindex)?;
        }

        self.bitcoin.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&self.bitcoin_txindex),
        )?;

        if let Some(dollars) = self.dollars.as_mut() {
            let dollars_txindex = self.dollars_txindex.as_mut().unwrap();

            dollars_txindex.compute_if_necessary(starting_indexes.txindex, exit)?;

            dollars.compute_rest(
                indexer,
                indexes,
                starting_indexes,
                exit,
                Some(dollars_txindex),
            )?;
        }

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            self.sats.vecs(),
            self.bitcoin.vecs(),
            self.dollars.as_ref().map_or(vec![], |v| v.vecs()),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
