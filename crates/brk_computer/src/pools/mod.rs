use std::{collections::BTreeMap, path::Path};

use brk_error::Result;
use brk_indexer::Indexer;
use brk_store::AnyStore;
use brk_traversable::Traversable;
use brk_types::{Address, AddressBytes, Height, OutputType, PoolSlug, Pools, TxOutIndex, pools};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, AnyVec, BytesVec, Database, Exit, GenericStoredVec, ImportableVec, IterableVec,
    PAGE_SIZE, TypedVecIterator, VecIndex, Version,
};

mod vecs;

use crate::{
    chain,
    indexes::{self, Indexes},
    price,
};

#[derive(Clone, Traversable)]
pub struct Vecs {
    db: Database,
    pools: &'static Pools,

    pub height_to_pool: BytesVec<Height, PoolSlug>,
    pub vecs: BTreeMap<PoolSlug, vecs::Vecs>,
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
            height_to_pool: BytesVec::forced_import(&db, "pool", version + Version::ZERO)?,
            vecs: pools
                .iter()
                .map(|pool| {
                    vecs::Vecs::forced_import(
                        &db,
                        pool.slug,
                        pools,
                        version + Version::ZERO,
                        indexes,
                        price,
                    )
                    .map(|vecs| (pool.slug, vecs))
                })
                .collect::<Result<BTreeMap<_, _>>>()?,
            pools,
            db,
        };

        this.db.retain_regions(
            this.iter_any_exportable()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;

        this.db.compact()?;

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
        self.db.compact()?;
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

        let mut height_to_first_txindex_iter = indexer.vecs.tx.height_to_first_txindex.iter()?;
        let mut txindex_to_first_txoutindex_iter =
            indexer.vecs.tx.txindex_to_first_txoutindex.iter()?;
        let mut txindex_to_output_count_iter = indexes.txindex_to_output_count.iter();
        let mut txoutindex_to_outputtype_iter =
            indexer.vecs.txout.txoutindex_to_outputtype.iter()?;
        let mut txoutindex_to_typeindex_iter =
            indexer.vecs.txout.txoutindex_to_typeindex.iter()?;
        let mut p2pk65addressindex_to_p2pk65bytes_iter = indexer
            .vecs
            .address
            .p2pk65addressindex_to_p2pk65bytes
            .iter()?;
        let mut p2pk33addressindex_to_p2pk33bytes_iter = indexer
            .vecs
            .address
            .p2pk33addressindex_to_p2pk33bytes
            .iter()?;
        let mut p2pkhaddressindex_to_p2pkhbytes_iter = indexer
            .vecs
            .address
            .p2pkhaddressindex_to_p2pkhbytes
            .iter()?;
        let mut p2shaddressindex_to_p2shbytes_iter =
            indexer.vecs.address.p2shaddressindex_to_p2shbytes.iter()?;
        let mut p2wpkhaddressindex_to_p2wpkhbytes_iter = indexer
            .vecs
            .address
            .p2wpkhaddressindex_to_p2wpkhbytes
            .iter()?;
        let mut p2wshaddressindex_to_p2wshbytes_iter = indexer
            .vecs
            .address
            .p2wshaddressindex_to_p2wshbytes
            .iter()?;
        let mut p2traddressindex_to_p2trbytes_iter =
            indexer.vecs.address.p2traddressindex_to_p2trbytes.iter()?;
        let mut p2aaddressindex_to_p2abytes_iter =
            indexer.vecs.address.p2aaddressindex_to_p2abytes.iter()?;

        let unknown = self.pools.get_unknown();

        let min = starting_indexes
            .height
            .to_usize()
            .min(self.height_to_pool.len());

        indexer
            .stores
            .height_to_coinbase_tag
            .iter()
            .skip(min)
            .try_for_each(|(height, coinbase_tag)| -> Result<()> {
                let txindex = height_to_first_txindex_iter.get_unwrap(height);
                let txoutindex = txindex_to_first_txoutindex_iter.get_unwrap(txindex);
                let outputcount = txindex_to_output_count_iter.get_unwrap(txindex);

                let pool = (*txoutindex..(*txoutindex + *outputcount))
                    .map(TxOutIndex::from)
                    .find_map(|txoutindex| {
                        let outputtype = txoutindex_to_outputtype_iter.get_unwrap(txoutindex);
                        let typeindex = txoutindex_to_typeindex_iter.get_unwrap(txoutindex);

                        match outputtype {
                            OutputType::P2PK65 => Some(AddressBytes::from(
                                p2pk65addressindex_to_p2pk65bytes_iter.get_unwrap(typeindex.into()),
                            )),
                            OutputType::P2PK33 => Some(AddressBytes::from(
                                p2pk33addressindex_to_p2pk33bytes_iter.get_unwrap(typeindex.into()),
                            )),
                            OutputType::P2PKH => Some(AddressBytes::from(
                                p2pkhaddressindex_to_p2pkhbytes_iter.get_unwrap(typeindex.into()),
                            )),
                            OutputType::P2SH => Some(AddressBytes::from(
                                p2shaddressindex_to_p2shbytes_iter.get_unwrap(typeindex.into()),
                            )),
                            OutputType::P2WPKH => Some(AddressBytes::from(
                                p2wpkhaddressindex_to_p2wpkhbytes_iter.get_unwrap(typeindex.into()),
                            )),
                            OutputType::P2WSH => Some(AddressBytes::from(
                                p2wshaddressindex_to_p2wshbytes_iter.get_unwrap(typeindex.into()),
                            )),
                            OutputType::P2TR => Some(AddressBytes::from(
                                p2traddressindex_to_p2trbytes_iter.get_unwrap(typeindex.into()),
                            )),
                            OutputType::P2A => Some(AddressBytes::from(
                                p2aaddressindex_to_p2abytes_iter.get_unwrap(typeindex.into()),
                            )),
                            _ => None,
                        }
                        .map(|bytes| Address::try_from(&bytes).unwrap())
                        .and_then(|address| self.pools.find_from_address(&address))
                    })
                    .or_else(|| self.pools.find_from_coinbase_tag(&coinbase_tag))
                    .unwrap_or(unknown);

                self.height_to_pool.truncate_push(height, pool.slug)?;
                Ok(())
            })?;

        self.height_to_pool.safe_write(exit)?;
        Ok(())
    }
}
