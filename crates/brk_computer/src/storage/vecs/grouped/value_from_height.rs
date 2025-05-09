use std::path::Path;

use brk_core::{Bitcoin, Dollars, Height, Sats};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{
    AnyCollectableVec, CollectableVec, Compressed, EagerVec, Result, StoredVec, Version,
};

use crate::storage::{
    fetched,
    vecs::{Indexes, indexes},
};

use super::{ComputedVecsFromHeight, StorableVecGeneatorOptions};

#[derive(Clone)]
pub struct ComputedValueVecsFromHeight {
    pub sats: ComputedVecsFromHeight<Sats>,
    pub bitcoin: ComputedVecsFromHeight<Bitcoin>,
    pub dollars: Option<ComputedVecsFromHeight<Dollars>>,
}

const VERSION: Version = Version::ONE;

impl ComputedValueVecsFromHeight {
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
            sats: ComputedVecsFromHeight::forced_import(
                path,
                name,
                compute_source,
                VERSION + version,
                compressed,
                options,
            )?,
            bitcoin: ComputedVecsFromHeight::forced_import(
                path,
                &format!("{name}_in_btc"),
                true,
                VERSION + version,
                compressed,
                options,
            )?,
            dollars: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
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
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        fetched: Option<&fetched::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
        mut compute: F,
    ) -> color_eyre::Result<()>
    where
        F: FnMut(
            &mut EagerVec<Height, Sats>,
            &Indexer,
            &indexes::Vecs,
            &Indexes,
            &Exit,
        ) -> Result<()>,
    {
        compute(
            self.sats.height.as_mut().unwrap(),
            indexer,
            indexes,
            starting_indexes,
            exit,
        )?;

        let height: Option<&StoredVec<Height, Sats>> = None;
        self.compute_rest(indexer, indexes, fetched, starting_indexes, exit, height)?;

        Ok(())
    }

    pub fn compute_rest(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        fetched: Option<&fetched::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
        height: Option<&impl CollectableVec<Height, Sats>>,
    ) -> color_eyre::Result<()> {
        if let Some(height) = height {
            self.sats
                .compute_rest(indexes, starting_indexes, exit, Some(height))?;

            self.bitcoin.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |v, _, _, starting_indexes, exit| {
                    v.compute_from_sats(starting_indexes.height, height, exit)
                },
            )?;
        } else {
            let height: Option<&StoredVec<Height, Sats>> = None;

            self.sats
                .compute_rest(indexes, starting_indexes, exit, height)?;

            self.bitcoin.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |v, _, _, starting_indexes, exit| {
                    v.compute_from_sats(
                        starting_indexes.height,
                        self.sats.height.as_ref().unwrap(),
                        exit,
                    )
                },
            )?;
        }

        let txindex = self.bitcoin.height.as_ref().unwrap();
        let price = &fetched.as_ref().unwrap().chainindexes_to_close.height;

        if let Some(dollars) = self.dollars.as_mut() {
            dollars.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |v, _, _, starting_indexes, exit| {
                    v.compute_from_bitcoin(starting_indexes.height, txindex, price, exit)
                },
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
        .concat()
    }
}
