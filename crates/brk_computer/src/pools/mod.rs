use std::{collections::BTreeMap, path::Path};

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{Addr, AddrBytes, Height, Indexes, OutputType, PoolSlug, Pools, TxOutIndex, pools};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, AnyVec, BytesVec, Database, Exit, ImportableVec, ReadableVec, Rw, StorageMode,
    VecIndex, Version, WritableVec,
};

pub mod major;
pub mod minor;

use crate::{
    blocks, indexes,
    internal::{
        CachedWindowStarts,
        db_utils::{finalize_db, open_db},
    },
    mining, prices,
};

pub const DB_NAME: &str = "pools";

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    db: Database,
    pools: &'static Pools,

    pub pool: M::Stored<BytesVec<Height, PoolSlug>>,
    pub major: BTreeMap<PoolSlug, major::Vecs<M>>,
    pub minor: BTreeMap<PoolSlug, minor::Vecs<M>>,
}

impl Vecs {
    pub(crate) fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let db = open_db(parent_path, DB_NAME, 100_000)?;
        let pools = pools();

        let version = parent_version + Version::new(3) + Version::new(pools.len() as u32);

        let mut major_map = BTreeMap::new();
        let mut minor_map = BTreeMap::new();

        for pool in pools.iter() {
            if pool.slug.is_major() {
                major_map.insert(
                    pool.slug,
                    major::Vecs::forced_import(&db, pool.slug, version, indexes, cached_starts)?,
                );
            } else {
                minor_map.insert(
                    pool.slug,
                    minor::Vecs::forced_import(&db, pool.slug, version, indexes, cached_starts)?,
                );
            }
        }

        let this = Self {
            pool: BytesVec::forced_import(&db, "pool", version)?,
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
        self.db.sync_bg_tasks()?;

        self.compute_pool(indexer, indexes, starting_indexes, exit)?;

        self.major.par_iter_mut().try_for_each(|(_, vecs)| {
            vecs.compute(starting_indexes, &self.pool, blocks, prices, mining, exit)
        })?;

        self.minor
            .par_iter_mut()
            .try_for_each(|(_, vecs)| vecs.compute(starting_indexes, &self.pool, blocks, exit))?;

        let exit = exit.clone();
        self.db.run_bg(move |db| {
            let _lock = exit.lock();
            db.compact_deferred_default()
        });
        Ok(())
    }

    fn compute_pool(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let dep_version = indexer.vecs.blocks.coinbase_tag.version();
        let pool_vec_version = self.pool.header().vec_version();
        let pool_computed = self.pool.header().computed_version();
        let expected = pool_vec_version + dep_version;
        if expected != pool_computed {
            tracing::warn!(
                "Pool version mismatch: vec_version={pool_vec_version:?} + dep={dep_version:?} = {expected:?}, stored computed={pool_computed:?}, len={}",
                self.pool.len()
            );
        }
        self.pool
            .validate_computed_version_or_reset(dep_version)?;

        let first_txout_index = indexer.vecs.transactions.first_txout_index.reader();
        let output_type = indexer.vecs.outputs.output_type.reader();
        let type_index = indexer.vecs.outputs.type_index.reader();
        let p2pk65 = indexer.vecs.addrs.p2pk65.bytes.reader();
        let p2pk33 = indexer.vecs.addrs.p2pk33.bytes.reader();
        let p2pkh = indexer.vecs.addrs.p2pkh.bytes.reader();
        let p2sh = indexer.vecs.addrs.p2sh.bytes.reader();
        let p2wpkh = indexer.vecs.addrs.p2wpkh.bytes.reader();
        let p2wsh = indexer.vecs.addrs.p2wsh.bytes.reader();
        let p2tr = indexer.vecs.addrs.p2tr.bytes.reader();
        let p2a = indexer.vecs.addrs.p2a.bytes.reader();

        let unknown = self.pools.get_unknown();

        let min = starting_indexes.height.to_usize().min(self.pool.len());

        // Cursors avoid per-height PcoVec page decompression.
        // Heights are sequential, tx_index values derived from them are monotonically
        // increasing, so both cursors only advance forward.
        let mut first_tx_index_cursor = indexer.vecs.transactions.first_tx_index.cursor();
        first_tx_index_cursor.advance(min);
        let mut output_count_cursor = indexes.tx_index.output_count.cursor();

        self.pool.truncate_if_needed_at(min)?;

        let len = indexer.vecs.blocks.coinbase_tag.len();

        indexer.vecs.blocks.coinbase_tag.try_for_each_range_at(
            min,
            len,
            |coinbase_tag| -> Result<()> {
                let tx_index = first_tx_index_cursor.next().unwrap();
                let out_start = first_txout_index.get(tx_index.to_usize());

                let ti = tx_index.to_usize();
                output_count_cursor.advance(ti - output_count_cursor.position());
                let output_count_val = output_count_cursor.next().unwrap();

                let pool = (*out_start..(*out_start + *output_count_val))
                    .map(TxOutIndex::from)
                    .find_map(|txout_index| {
                        let ot = output_type.get(txout_index.to_usize());
                        let ti = usize::from(type_index.get(txout_index.to_usize()));
                        match ot {
                            OutputType::P2PK65 => Some(AddrBytes::from(p2pk65.get(ti))),
                            OutputType::P2PK33 => Some(AddrBytes::from(p2pk33.get(ti))),
                            OutputType::P2PKH => Some(AddrBytes::from(p2pkh.get(ti))),
                            OutputType::P2SH => Some(AddrBytes::from(p2sh.get(ti))),
                            OutputType::P2WPKH => Some(AddrBytes::from(p2wpkh.get(ti))),
                            OutputType::P2WSH => Some(AddrBytes::from(p2wsh.get(ti))),
                            OutputType::P2TR => Some(AddrBytes::from(p2tr.get(ti))),
                            OutputType::P2A => Some(AddrBytes::from(p2a.get(ti))),
                            _ => None,
                        }
                        .map(|bytes| Addr::try_from(&bytes).unwrap())
                        .and_then(|addr| self.pools.find_from_addr(&addr))
                    })
                    .or_else(|| self.pools.find_from_coinbase_tag(&coinbase_tag.as_str()))
                    .unwrap_or(unknown);

                self.pool.push(pool.slug);
                Ok(())
            },
        )?;

        let _lock = exit.lock();
        self.pool.write()?;
        Ok(())
    }
}
