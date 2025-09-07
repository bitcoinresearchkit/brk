use std::{collections::BTreeMap, path::Path};

use allocative::Allocative;
use brk_error::Result;
use brk_indexer::Indexer;
use brk_store::AnyStore;
use brk_structs::{AddressBytes, Height, OutputIndex, OutputType};
use rayon::prelude::*;
use vecdb::{
    AnyCollectableVec, AnyIterableVec, AnyStoredVec, AnyVec, Database, Exit, GenericStoredVec,
    PAGE_SIZE, RawVec, StoredIndex, VecIterator, Version,
};

mod id;
mod pool;
#[allow(clippy::module_inception)]
mod pools;
mod vecs;

pub use id::*;
pub use pool::*;
pub use pools::*;

use crate::{
    chain,
    indexes::{self, Indexes},
    price,
};

#[derive(Clone, Allocative)]
pub struct Vecs {
    db: Database,
    pools: &'static Pools,
    height_to_pool: RawVec<Height, PoolId>,

    vecs: BTreeMap<PoolId, vecs::Vecs>,
}

impl Vecs {
    pub fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let db = Database::open(&parent_path.join("pools"))?;
        db.set_min_len(PAGE_SIZE * 1_000_000)?;
        let pools = pools();

        let version = parent_version + Version::new(3) + Version::new(pools.len() as u64);

        let this = Self {
            height_to_pool: RawVec::forced_import(&db, "pool", version + Version::ZERO)?,
            vecs: pools
                .iter()
                .map(|pool| {
                    vecs::Vecs::forced_import(
                        &db,
                        pool.id,
                        pools,
                        version + Version::ZERO,
                        indexes,
                        price,
                    )
                    .map(|vecs| (pool.id, vecs))
                })
                .collect::<Result<BTreeMap<_, _>>>()?,
            pools,
            db,
        };

        this.db.retain_regions(
            this.vecs()
                .into_iter()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;

        Ok(this)
    }

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        chain: &chain::Vecs,
        price: Option<&price::Vecs>,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_(indexer, indexes, starting_indexes, chain, price, exit)?;
        self.db.flush_then_punch()?;
        Ok(())
    }

    fn compute_(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        chain: &chain::Vecs,
        price: Option<&price::Vecs>,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_height_to_pool(indexer, indexes, starting_indexes, exit)?;

        self.vecs.par_iter_mut().try_for_each(|(_, vecs)| {
            vecs.compute(
                indexer,
                indexes,
                starting_indexes,
                &self.height_to_pool,
                chain,
                price,
                exit,
            )
        })?;

        Ok(())
    }

    fn compute_height_to_pool(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.height_to_pool.validate_computed_version_or_reset(
            self.height_to_pool.version() + indexer.stores.height_to_coinbase_tag.version(),
        )?;

        let mut height_to_first_txindex_iter = indexer.vecs.height_to_first_txindex.iter();
        let mut txindex_to_first_outputindex_iter =
            indexer.vecs.txindex_to_first_outputindex.iter();
        let mut txindex_to_output_count_iter = indexes.txindex_to_output_count.iter();
        let mut outputindex_to_outputtype_iter = indexer.vecs.outputindex_to_outputtype.iter();
        let mut outputindex_to_typeindex_iter = indexer.vecs.outputindex_to_typeindex.iter();
        let mut p2pk65addressindex_to_p2pk65bytes_iter =
            indexer.vecs.p2pk65addressindex_to_p2pk65bytes.iter();
        let mut p2pk33addressindex_to_p2pk33bytes_iter =
            indexer.vecs.p2pk33addressindex_to_p2pk33bytes.iter();
        let mut p2pkhaddressindex_to_p2pkhbytes_iter =
            indexer.vecs.p2pkhaddressindex_to_p2pkhbytes.iter();
        let mut p2shaddressindex_to_p2shbytes_iter =
            indexer.vecs.p2shaddressindex_to_p2shbytes.iter();
        let mut p2wpkhaddressindex_to_p2wpkhbytes_iter =
            indexer.vecs.p2wpkhaddressindex_to_p2wpkhbytes.iter();
        let mut p2wshaddressindex_to_p2wshbytes_iter =
            indexer.vecs.p2wshaddressindex_to_p2wshbytes.iter();
        let mut p2traddressindex_to_p2trbytes_iter =
            indexer.vecs.p2traddressindex_to_p2trbytes.iter();
        let mut p2aaddressindex_to_p2abytes_iter = indexer.vecs.p2aaddressindex_to_p2abytes.iter();

        let unknown = self.pools.get_unknown();

        let min = starting_indexes
            .height
            .unwrap_to_usize()
            .min(self.height_to_pool.len());

        indexer
            .stores
            .height_to_coinbase_tag
            .iter()
            .skip(min)
            .try_for_each(|(height, coinbase_tag)| -> Result<()> {
                let txindex = height_to_first_txindex_iter.unwrap_get_inner(height);
                let outputindex = txindex_to_first_outputindex_iter.unwrap_get_inner(txindex);
                let outputcount = txindex_to_output_count_iter.unwrap_get_inner(txindex);

                let pool = (*outputindex..(*outputindex + *outputcount))
                    .map(OutputIndex::from)
                    .find_map(|outputindex| {
                        let outputtype =
                            outputindex_to_outputtype_iter.unwrap_get_inner(outputindex);
                        let typeindex = outputindex_to_typeindex_iter.unwrap_get_inner(outputindex);

                        let address = match outputtype {
                            OutputType::P2PK65 => Some(AddressBytes::from(
                                p2pk65addressindex_to_p2pk65bytes_iter
                                    .unwrap_get_inner(typeindex.into()),
                            )),
                            OutputType::P2PK33 => Some(AddressBytes::from(
                                p2pk33addressindex_to_p2pk33bytes_iter
                                    .unwrap_get_inner(typeindex.into()),
                            )),
                            OutputType::P2PKH => Some(AddressBytes::from(
                                p2pkhaddressindex_to_p2pkhbytes_iter
                                    .unwrap_get_inner(typeindex.into()),
                            )),
                            OutputType::P2SH => Some(AddressBytes::from(
                                p2shaddressindex_to_p2shbytes_iter
                                    .unwrap_get_inner(typeindex.into()),
                            )),
                            OutputType::P2WPKH => Some(AddressBytes::from(
                                p2wpkhaddressindex_to_p2wpkhbytes_iter
                                    .unwrap_get_inner(typeindex.into()),
                            )),
                            OutputType::P2WSH => Some(AddressBytes::from(
                                p2wshaddressindex_to_p2wshbytes_iter
                                    .unwrap_get_inner(typeindex.into()),
                            )),
                            OutputType::P2TR => Some(AddressBytes::from(
                                p2traddressindex_to_p2trbytes_iter
                                    .unwrap_get_inner(typeindex.into()),
                            )),
                            OutputType::P2A => Some(AddressBytes::from(
                                p2aaddressindex_to_p2abytes_iter.unwrap_get_inner(typeindex.into()),
                            )),
                            _ => None,
                        };

                        address
                            .and_then(|address| self.pools.find_from_address(&address.to_string()))
                    })
                    .or_else(|| self.pools.find_from_coinbase_tag(&coinbase_tag))
                    .unwrap_or(unknown);

                self.height_to_pool.push_if_needed(height, pool.id)?;
                Ok(())
            })?;

        self.height_to_pool.safe_flush(exit)?;
        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            self.vecs
                .iter()
                .flat_map(|(_, vecs)| vecs.vecs())
                .collect::<Vec<_>>(),
            vec![&self.height_to_pool],
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
