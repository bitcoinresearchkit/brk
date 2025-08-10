use brk_error::Result;
use brk_indexer::Indexer;
use brk_structs::{Bitcoin, DateIndex, Dollars, Sats, Version};
use vecdb::{
    AnyCollectableVec, CollectableVec, Computation, Database, EagerVec, Exit, Format, StoredVec,
};

use crate::{
    Indexes,
    grouped::ComputedVecsFromDateIndex,
    indexes, price,
    traits::{ComputeFromBitcoin, ComputeFromSats},
};

use super::{Source, VecBuilderOptions};

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
        db: &Database,
        name: &str,
        source: Source<DateIndex, Sats>,
        version: Version,
        format: Format,
        computation: Computation,
        options: VecBuilderOptions,
        compute_dollars: bool,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            sats: ComputedVecsFromDateIndex::forced_import(
                db,
                name,
                source,
                version + VERSION,
                format,
                computation,
                indexes,
                options,
            )?,
            bitcoin: ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_in_btc"),
                Source::Compute,
                version + VERSION,
                format,
                computation,
                indexes,
                options,
            )?,
            dollars: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
                    db,
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
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
        mut compute: F,
    ) -> Result<()>
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
        self.compute_rest(indexer, indexes, price, starting_indexes, exit, dateindex)?;

        Ok(())
    }

    pub fn compute_rest(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
        dateindex: Option<&impl CollectableVec<DateIndex, Sats>>,
    ) -> Result<()> {
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
        let dateindex_to_close = price
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
