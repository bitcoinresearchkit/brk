use std::path::Path;

use brk_core::{Bitcoin, Dollars, Sats, TxIndex};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyStoredVec, Compressed, Result, StoredVec, Version};

use crate::storage::{
    EagerVec, marketprice,
    vecs::{Indexes, indexes},
};

use super::{ComputedVecsFromTxindex, StorableVecGeneatorOptions};

#[derive(Clone)]
pub struct ComputedValueVecsFromTxindex {
    pub sats: ComputedVecsFromTxindex<Sats>,
    pub bitcoin: ComputedVecsFromTxindex<Bitcoin>,
    pub dollars: Option<ComputedVecsFromTxindex<Dollars>>,
}

const VERSION: Version = Version::ONE;

impl ComputedValueVecsFromTxindex {
    pub fn forced_import(
        path: &Path,
        name: &str,
        compute_source: bool,
        version: Version,
        compressed: Compressed,
        options: StorableVecGeneatorOptions,
        compute_dollars: bool,
    ) -> color_eyre::Result<Self> {
        Ok(Self {
            sats: ComputedVecsFromTxindex::forced_import(
                path,
                name,
                compute_source,
                VERSION + version,
                compressed,
                options,
            )?,
            bitcoin: ComputedVecsFromTxindex::forced_import(
                path,
                &format!("{name}_in_btc"),
                true,
                VERSION + version,
                compressed,
                options,
            )?,
            dollars: compute_dollars.then(|| {
                ComputedVecsFromTxindex::forced_import(
                    path,
                    &format!("{name}_in_usd"),
                    true,
                    VERSION + version,
                    compressed,
                    options,
                )
                .unwrap()
            }),
        })
    }

    pub fn compute_all<F>(
        &mut self,
        indexer: &mut Indexer,
        indexes: &mut indexes::Vecs,
        marketprices: &mut Option<&mut marketprice::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
        mut compute: F,
    ) -> color_eyre::Result<()>
    where
        F: FnMut(
            &mut EagerVec<TxIndex, Sats>,
            &mut Indexer,
            &mut indexes::Vecs,
            &Indexes,
            &Exit,
        ) -> Result<()>,
    {
        compute(
            self.sats.txindex.as_mut().unwrap(),
            indexer,
            indexes,
            starting_indexes,
            exit,
        )?;

        self.compute_rest(indexer, indexes, marketprices, starting_indexes, exit, None)?;

        Ok(())
    }

    pub fn compute_rest(
        &mut self,
        indexer: &mut Indexer,
        indexes: &mut indexes::Vecs,
        marketprices: &mut Option<&mut marketprice::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
        mut txindex: Option<&mut StoredVec<TxIndex, Sats>>,
    ) -> color_eyre::Result<()> {
        if let Some(txindex) = txindex.as_mut() {
            self.sats
                .compute_rest(indexer, indexes, starting_indexes, exit, Some(txindex))?;
        } else {
            self.sats
                .compute_rest(indexer, indexes, starting_indexes, exit, None)?;
        }

        let txindex = txindex.unwrap_or_else(|| self.sats.txindex.as_mut().unwrap().mut_vec());

        self.bitcoin.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_from_sats(starting_indexes.txindex, txindex, exit)
            },
        )?;

        let txindex = self.bitcoin.txindex.as_mut().unwrap().mut_vec();
        let price = marketprices
            .as_mut()
            .unwrap()
            .chainindexes_to_close
            .height
            .mut_vec();

        if let Some(dollars) = self.dollars.as_mut() {
            dollars.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |v, _, indexes, starting_indexes, exit| {
                    v.compute_from_bitcoin(
                        starting_indexes.txindex,
                        txindex,
                        indexes.txindex_to_height.mut_vec(),
                        price,
                        exit,
                    )
                },
            )?;
        }

        Ok(())
    }

    pub fn any_vecs(&self) -> Vec<&dyn AnyStoredVec> {
        [
            self.sats.any_vecs(),
            self.bitcoin.any_vecs(),
            self.dollars.as_ref().map_or(vec![], |v| v.any_vecs()),
        ]
        .concat()
    }
}
