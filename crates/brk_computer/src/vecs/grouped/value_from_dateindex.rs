use std::path::Path;

use brk_core::{Bitcoin, DateIndex, Dollars, Result, Sats, Version};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyCollectableVec, CollectableVec, Computation, EagerVec, Format, StoredVec};

use crate::vecs::{Indexes, fetched, grouped::ComputedVecsFromDateIndex, indexes};

use super::{EagerVecBuilderOptions, Source};

#[derive(Clone)]
pub struct ComputedValueVecsFromDateIndex {
    pub sats: ComputedVecsFromDateIndex<Sats>,
    pub bitcoin: ComputedVecsFromDateIndex<Bitcoin>,
    pub dollars: Option<ComputedVecsFromDateIndex<Dollars>>,
}

const VERSION: Version = Version::ZERO;

impl ComputedValueVecsFromDateIndex {
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        path: &Path,
        name: &str,
        source: Source<DateIndex, Sats>,
        version: Version,
        format: Format,
        computation: Computation,
        options: EagerVecBuilderOptions,
        compute_dollars: bool,
    ) -> color_eyre::Result<Self> {
        Ok(Self {
            sats: ComputedVecsFromDateIndex::forced_import(
                path,
                name,
                source,
                version + VERSION,
                format,
                computation,
                options,
            )?,
            bitcoin: ComputedVecsFromDateIndex::forced_import(
                path,
                &format!("{name}_in_btc"),
                Source::Compute,
                version + VERSION,
                format,
                computation,
                options,
            )?,
            dollars: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    path,
                    &format!("{name}_in_usd"),
                    Source::Compute,
                    version + VERSION,
                    format,
                    computation,
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
            &mut EagerVec<DateIndex, Sats>,
            &Indexer,
            &indexes::Vecs,
            &Indexes,
            &Exit,
        ) -> Result<()>,
    {
        compute(
            self.sats.dateindex.as_mut().unwrap(),
            indexer,
            indexes,
            starting_indexes,
            exit,
        )?;

        let dateindex: Option<&StoredVec<DateIndex, Sats>> = None;
        self.compute_rest(indexer, indexes, fetched, starting_indexes, exit, dateindex)?;

        Ok(())
    }

    pub fn compute_rest(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        fetched: Option<&fetched::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
        dateindex: Option<&impl CollectableVec<DateIndex, Sats>>,
    ) -> color_eyre::Result<()> {
        if let Some(dateindex) = dateindex {
            self.sats
                .compute_rest(indexes, starting_indexes, exit, Some(dateindex))?;

            self.bitcoin.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |v, _, _, starting_indexes, exit| {
                    v.compute_from_sats(starting_indexes.dateindex, dateindex, exit)
                },
            )?;
        } else {
            let dateindex: Option<&StoredVec<DateIndex, Sats>> = None;

            self.sats
                .compute_rest(indexes, starting_indexes, exit, dateindex)?;

            self.bitcoin.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |v, _, _, starting_indexes, exit| {
                    v.compute_from_sats(
                        starting_indexes.dateindex,
                        self.sats.dateindex.as_ref().unwrap(),
                        exit,
                    )
                },
            )?;
        }

        let dateindex_to_bitcoin = self.bitcoin.dateindex.as_ref().unwrap();
        let dateindex_to_close = fetched
            .as_ref()
            .unwrap()
            .timeindexes_to_close
            .dateindex
            .as_ref()
            .unwrap();

        if let Some(dollars) = self.dollars.as_mut() {
            dollars.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |v, _, _, starting_indexes, exit| {
                    v.compute_from_bitcoin(
                        starting_indexes.dateindex,
                        dateindex_to_bitcoin,
                        dateindex_to_close,
                        exit,
                    )
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
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
