use std::sync::Arc;

use brk_core::{Bitcoin, Dollars, Height, Result, Sats, Version};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vecs::{AnyCollectableVec, CollectableVec, Computation, EagerVec, File, Format, StoredVec};

use crate::{Indexes, fetched, grouped::Source, indexes};

use super::{ComputedVecsFromHeight, VecBuilderOptions};

#[derive(Clone)]
pub struct ComputedValueVecsFromHeight {
    pub sats: ComputedVecsFromHeight<Sats>,
    pub bitcoin: ComputedVecsFromHeight<Bitcoin>,
    pub dollars: Option<ComputedVecsFromHeight<Dollars>>,
}

const VERSION: Version = Version::ZERO;

impl ComputedValueVecsFromHeight {
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        file: &Arc<File>,
        name: &str,
        source: Source<Height, Sats>,
        version: Version,
        format: Format,
        computation: Computation,
        options: VecBuilderOptions,
        compute_dollars: bool,
        indexes: &indexes::Vecs,
    ) -> color_eyre::Result<Self> {
        Ok(Self {
            sats: ComputedVecsFromHeight::forced_import(
                file,
                name,
                source,
                version + VERSION,
                format,
                computation,
                indexes,
                options,
            )?,
            bitcoin: ComputedVecsFromHeight::forced_import(
                file,
                &format!("{name}_in_btc"),
                Source::Compute,
                version + VERSION,
                format,
                computation,
                indexes,
                options,
            )?,
            dollars: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    file,
                    &format!("{name}_in_usd"),
                    Source::Compute,
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

        let height_to_bitcoin = self.bitcoin.height.as_ref().unwrap();
        let height_to_close = &fetched.as_ref().unwrap().chainindexes_to_close.height;

        if let Some(dollars) = self.dollars.as_mut() {
            dollars.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |v, _, _, starting_indexes, exit| {
                    v.compute_from_bitcoin(
                        starting_indexes.height,
                        height_to_bitcoin,
                        height_to_close,
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
