use std::{collections::BTreeMap, path::Path};

use brk_error::Result;
use brk_indexer::Indexer;
use brk_store::AnyStore;
use brk_traversable::Traversable;
use brk_types::{Address, AddressBytes, Height, OutputType, PoolSlug, Pools, TxOutIndex, pools};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, AnyVec, BytesVec, Database, Exit, ImportableVec, PAGE_SIZE, ReadableVec, Rw,
    StorageMode, VecIndex, Version, WritableVec,
};

mod vecs;

use crate::{
    blocks,
    indexes::{self, ComputeIndexes},
    mining, prices, transactions,
};

pub const DB_NAME: &str = "pools";

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    db: Database,
    pools: &'static Pools,

    pub height_to_pool: M::Stored<BytesVec<Height, PoolSlug>>,
    pub vecs: BTreeMap<PoolSlug, vecs::Vecs<M>>,
}

impl Vecs {
    pub(crate) fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let db = Database::open(&parent_path.join(DB_NAME))?;
        db.set_min_len(PAGE_SIZE * 1_000_000)?;
        let pools = pools();

        let version = parent_version + Version::new(3) + Version::new(pools.len() as u32);

        let this = Self {
            height_to_pool: BytesVec::forced_import(&db, "pool", version)?,
            vecs: pools
                .iter()
                .map(|pool| {
                    vecs::Vecs::forced_import(&db, pool.slug, version, indexes)
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

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        mining: &mining::Vecs,
        transactions: &transactions::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_(
            indexer,
            indexes,
            blocks,
            prices,
            mining,
            transactions,
            starting_indexes,
            exit,
        )?;
        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn compute_(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        mining: &mining::Vecs,
        transactions: &transactions::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_height_to_pool(indexer, indexes, starting_indexes, exit)?;

        self.vecs.par_iter_mut().try_for_each(|(_, vecs)| {
            vecs.compute(
                starting_indexes,
                &self.height_to_pool,
                blocks,
                prices,
                mining,
                transactions,
                exit,
            )
        })?;

        Ok(())
    }

    fn compute_height_to_pool(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.height_to_pool
            .validate_computed_version_or_reset(indexer.stores.height_to_coinbase_tag.version())?;

        let txindex_to_first_txoutindex_reader =
            indexer.vecs.transactions.first_txoutindex.reader();
        let txoutindex_to_outputtype_reader = indexer.vecs.outputs.outputtype.reader();
        let txoutindex_to_typeindex_reader = indexer.vecs.outputs.typeindex.reader();
        let p2pk65addressindex_to_p2pk65bytes_reader = indexer.vecs.addresses.p2pk65bytes.reader();
        let p2pk33addressindex_to_p2pk33bytes_reader = indexer.vecs.addresses.p2pk33bytes.reader();
        let p2pkhaddressindex_to_p2pkhbytes_reader = indexer.vecs.addresses.p2pkhbytes.reader();
        let p2shaddressindex_to_p2shbytes_reader = indexer.vecs.addresses.p2shbytes.reader();
        let p2wpkhaddressindex_to_p2wpkhbytes_reader = indexer.vecs.addresses.p2wpkhbytes.reader();
        let p2wshaddressindex_to_p2wshbytes_reader = indexer.vecs.addresses.p2wshbytes.reader();
        let p2traddressindex_to_p2trbytes_reader = indexer.vecs.addresses.p2trbytes.reader();
        let p2aaddressindex_to_p2abytes_reader = indexer.vecs.addresses.p2abytes.reader();

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
                let txoutindex = txindex_to_first_txoutindex_reader.get(txindex.to_usize());

                let ti = txindex.to_usize();
                output_count_cursor.advance(ti - output_count_cursor.position());
                let outputcount = output_count_cursor.next().unwrap();

                let pool = (*txoutindex..(*txoutindex + *outputcount))
                    .map(TxOutIndex::from)
                    .find_map(|txoutindex| {
                        let outputtype = txoutindex_to_outputtype_reader.get(txoutindex.to_usize());
                        let typeindex = txoutindex_to_typeindex_reader.get(txoutindex.to_usize());

                        let ti = usize::from(typeindex);
                        match outputtype {
                            OutputType::P2PK65 => Some(AddressBytes::from(
                                p2pk65addressindex_to_p2pk65bytes_reader.get(ti),
                            )),
                            OutputType::P2PK33 => Some(AddressBytes::from(
                                p2pk33addressindex_to_p2pk33bytes_reader.get(ti),
                            )),
                            OutputType::P2PKH => Some(AddressBytes::from(
                                p2pkhaddressindex_to_p2pkhbytes_reader.get(ti),
                            )),
                            OutputType::P2SH => Some(AddressBytes::from(
                                p2shaddressindex_to_p2shbytes_reader.get(ti),
                            )),
                            OutputType::P2WPKH => Some(AddressBytes::from(
                                p2wpkhaddressindex_to_p2wpkhbytes_reader.get(ti),
                            )),
                            OutputType::P2WSH => Some(AddressBytes::from(
                                p2wshaddressindex_to_p2wshbytes_reader.get(ti),
                            )),
                            OutputType::P2TR => Some(AddressBytes::from(
                                p2traddressindex_to_p2trbytes_reader.get(ti),
                            )),
                            OutputType::P2A => Some(AddressBytes::from(
                                p2aaddressindex_to_p2abytes_reader.get(ti),
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

        let _lock = exit.lock();
        self.height_to_pool.write()?;
        Ok(())
    }
}
