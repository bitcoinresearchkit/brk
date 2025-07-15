use std::path::Path;

use brk_core::{Bitcoin, Dollars, Height, Result, Sats, Version};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyCollectableVec, CollectableVec, EagerVec, Format, StoredVec};

use crate::vecs::{Indexes, fetched, grouped::Source, indexes};

#[derive(Clone)]
pub struct ComputedHeightValueVecs {
    pub sats: Option<EagerVec<Height, Sats>>,
    pub bitcoin: EagerVec<Height, Bitcoin>,
    pub dollars: Option<EagerVec<Height, Dollars>>,
}

const VERSION: Version = Version::ZERO;

impl ComputedHeightValueVecs {
    pub fn forced_import(
        path: &Path,
        name: &str,
        source: Source<Height, Sats>,
        version: Version,
        format: Format,
        compute_dollars: bool,
    ) -> color_eyre::Result<Self> {
        Ok(Self {
            sats: source.is_compute().then(|| {
                EagerVec::forced_import(path, name, version + VERSION + Version::ZERO, format)
                    .unwrap()
            }),
            bitcoin: EagerVec::forced_import(
                path,
                &format!("{name}_in_btc"),
                version + VERSION + Version::ZERO,
                format,
            )?,
            dollars: compute_dollars.then(|| {
                EagerVec::forced_import(
                    path,
                    &format!("{name}_in_usd"),
                    version + VERSION + Version::ZERO,
                    format,
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
            self.sats.as_mut().unwrap(),
            indexer,
            indexes,
            starting_indexes,
            exit,
        )?;

        let height: Option<&StoredVec<Height, Sats>> = None;
        self.compute_rest(fetched, starting_indexes, exit, height)?;

        Ok(())
    }

    pub fn compute_rest(
        &mut self,
        fetched: Option<&fetched::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
        height: Option<&impl CollectableVec<Height, Sats>>,
    ) -> color_eyre::Result<()> {
        if let Some(height) = height {
            self.bitcoin
                .compute_from_sats(starting_indexes.height, height, exit)?;
        } else {
            self.bitcoin.compute_from_sats(
                starting_indexes.height,
                self.sats.as_ref().unwrap(),
                exit,
            )?;
        }

        let height_to_bitcoin = &self.bitcoin;
        let height_to_close = &fetched.as_ref().unwrap().chainindexes_to_close.height;

        if let Some(dollars) = self.dollars.as_mut() {
            dollars.compute_from_bitcoin(
                starting_indexes.height,
                height_to_bitcoin,
                height_to_close,
                exit,
            )?;
        }

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            vec![&self.bitcoin as &dyn AnyCollectableVec],
            self.sats.as_ref().map_or(vec![], |v| vec![v]),
            self.dollars.as_ref().map_or(vec![], |v| vec![v]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
