use std::sync::Arc;

use brk_core::{Bitcoin, Close, Dollars, Height, Sats, TxIndex, Version};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vecs::{
    AnyCollectableVec, CloneableAnyIterableVec, CollectableVec, Computation, ComputedVecFrom3,
    File, Format, LazyVecFrom1, StoredIndex, StoredVec,
};

use crate::{Indexes, fetched, grouped::Source, indexes};

use super::{ComputedVecsFromTxindex, VecBuilderOptions};

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

const VERSION: Version = Version::ZERO;

impl ComputedValueVecsFromTxindex {
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        file: &Arc<File>,
        name: &str,
        indexes: &indexes::Vecs,
        source: Source<TxIndex, Sats>,
        version: Version,
        computation: Computation,
        format: Format,
        fetched: Option<&fetched::Vecs>,
        options: VecBuilderOptions,
    ) -> color_eyre::Result<Self> {
        let compute_dollars = fetched.is_some();

        let name_in_btc = format!("{name}_in_btc");
        let name_in_usd = format!("{name}_in_usd");

        let sats = ComputedVecsFromTxindex::forced_import(
            file,
            name,
            source.clone(),
            version + VERSION,
            format,
            computation,
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
            file,
            &name_in_btc,
            Source::None,
            version + VERSION,
            format,
            computation,
            indexes,
            options,
        )?;

        let dollars_txindex = fetched.map(|fetched| {
            ComputedVecFrom3::forced_import_or_init_from_3(
                computation,
                file,
                &name_in_usd,
                version + VERSION,
                format,
                bitcoin_txindex.boxed_clone(),
                indexes.txindex_to_height.boxed_clone(),
                fetched.chainindexes_to_close.height.boxed_clone(),
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
            .unwrap()
        });

        Ok(Self {
            sats,
            bitcoin_txindex,
            bitcoin,
            dollars_txindex,
            dollars: compute_dollars.then(|| {
                ComputedVecsFromTxindex::forced_import(
                    file,
                    &name_in_usd,
                    Source::None,
                    version + VERSION,
                    format,
                    computation,
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
        fetched: Option<&fetched::Vecs>,
    ) -> color_eyre::Result<()> {
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

            dollars_txindex.compute_if_necessary(
                starting_indexes.txindex,
                &indexer.vecs.txindex_to_txid,
                exit,
            )?;

            dollars.compute_rest_from_bitcoin(
                indexer,
                indexes,
                starting_indexes,
                exit,
                &self.bitcoin,
                Some(dollars_txindex),
                fetched.as_ref().unwrap(),
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
