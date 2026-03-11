use std::{collections::BTreeMap, path::Path};

use brk_error::Result;
use brk_indexer::Indexer;
use brk_store::AnyStore;
use brk_traversable::Traversable;
use brk_types::{
    Address, AddressBytes, Height, Indexes, OutputType, PoolSlug, Pools, TxOutIndex, pools,
};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, AnyVec, BytesVec, Database, Exit, ImportableVec, ReadableVec, Rw, StorageMode,
    VecIndex, Version, WritableVec,
};

pub mod major;
pub mod minor;

use crate::{
    blocks, indexes,
    internal::{finalize_db, open_db},
    mining, prices,
};

pub const DB_NAME: &str = "pools";

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    db: Database,
    pools: &'static Pools,

    pub height_to_pool: M::Stored<BytesVec<Height, PoolSlug>>,
    pub major: BTreeMap<PoolSlug, major::Vecs<M>>,
    pub minor: BTreeMap<PoolSlug, minor::Vecs<M>>,
}

impl Vecs {
    pub(crate) fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let db = open_db(parent_path, DB_NAME, 1_000_000)?;
        let pools = pools();

        let version = parent_version + Version::new(3) + Version::new(pools.len() as u32);

        let mut major_map = BTreeMap::new();
        let mut minor_map = BTreeMap::new();

        for pool in pools.iter() {
            if pool.slug.is_major() {
                major_map.insert(
                    pool.slug,
                    major::Vecs::forced_import(&db, pool.slug, version, indexes)?,
                );
            } else {
                minor_map.insert(
                    pool.slug,
                    minor::Vecs::forced_import(&db, pool.slug, version, indexes)?,
                );
            }
        }

        let this = Self {
            height_to_pool: BytesVec::forced_import(&db, "pool", version)?,
            major: major_map,
            minor: minor_map,
            pools,
            db,
        };

        finalize_db(&this.db, &this)?;
        Ok(this)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        mining: &mining::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_height_to_pool(indexer, indexes, starting_indexes, exit)?;

        self.major.par_iter_mut().try_for_each(|(_, vecs)| {
            vecs.compute(
                starting_indexes,
                &self.height_to_pool,
                blocks,
                prices,
                mining,
                exit,
            )
        })?;

        self.minor.par_iter_mut().try_for_each(|(_, vecs)| {
            vecs.compute(starting_indexes, &self.height_to_pool, blocks, exit)
        })?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }

    fn compute_height_to_pool(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.height_to_pool
            .validate_computed_version_or_reset(indexer.stores.height_to_coinbase_tag.version())?;

        let first_txoutindex = indexer.vecs.transactions.first_txoutindex.reader();
        let outputtype = indexer.vecs.outputs.outputtype.reader();
        let typeindex = indexer.vecs.outputs.typeindex.reader();
        let p2pk65 = indexer.vecs.addresses.p2pk65.bytes.reader();
        let p2pk33 = indexer.vecs.addresses.p2pk33.bytes.reader();
        let p2pkh = indexer.vecs.addresses.p2pkh.bytes.reader();
        let p2sh = indexer.vecs.addresses.p2sh.bytes.reader();
        let p2wpkh = indexer.vecs.addresses.p2wpkh.bytes.reader();
        let p2wsh = indexer.vecs.addresses.p2wsh.bytes.reader();
        let p2tr = indexer.vecs.addresses.p2tr.bytes.reader();
        let p2a = indexer.vecs.addresses.p2a.bytes.reader();

        let unknown = self.pools.get_unknown();

        let min = starting_indexes
            .height
            .to_usize()
            .min(self.height_to_pool.len());

        // Cursors avoid per-height PcoVec page decompression.
        // Heights are sequential, txindex values derived from them are monotonically
        // increasing, so both cursors only advance forward.
        let mut first_txindex_cursor = indexer.vecs.transactions.first_txindex.cursor();
        first_txindex_cursor.advance(min);
        let mut output_count_cursor = indexes.txindex.output_count.cursor();

        indexer
            .stores
            .height_to_coinbase_tag
            .iter()
            .skip(min)
            .try_for_each(|(height, coinbase_tag)| -> Result<()> {
                let txindex = first_txindex_cursor.next().unwrap();
                let out_start = first_txoutindex.get(txindex.to_usize());

                let ti = txindex.to_usize();
                output_count_cursor.advance(ti - output_count_cursor.position());
                let outputcount = output_count_cursor.next().unwrap();

                let pool = (*out_start..(*out_start + *outputcount))
                    .map(TxOutIndex::from)
                    .find_map(|txoutindex| {
                        let ot = outputtype.get(txoutindex.to_usize());
                        let ti = usize::from(typeindex.get(txoutindex.to_usize()));
                        match ot {
                            OutputType::P2PK65 => Some(AddressBytes::from(p2pk65.get(ti))),
                            OutputType::P2PK33 => Some(AddressBytes::from(p2pk33.get(ti))),
                            OutputType::P2PKH => Some(AddressBytes::from(p2pkh.get(ti))),
                            OutputType::P2SH => Some(AddressBytes::from(p2sh.get(ti))),
                            OutputType::P2WPKH => Some(AddressBytes::from(p2wpkh.get(ti))),
                            OutputType::P2WSH => Some(AddressBytes::from(p2wsh.get(ti))),
                            OutputType::P2TR => Some(AddressBytes::from(p2tr.get(ti))),
                            OutputType::P2A => Some(AddressBytes::from(p2a.get(ti))),
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

        let _lock = exit.lock();
        self.height_to_pool.write()?;
        Ok(())
    }
}
