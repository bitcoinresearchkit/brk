use std::path::Path;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{AddressBytes, AddressHash, Height, OutputType, TypeIndex, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Database, PAGE_SIZE, Reader, Stamp};

use crate::parallel_import;

mod address;
mod blocks;
mod macros;
mod output;
mod tx;
mod txin;
mod txout;

pub use address::*;
pub use blocks::*;
pub use output::*;
pub use tx::*;
pub use txin::*;
pub use txout::*;

use crate::Indexes;

#[derive(Clone, Traversable)]
pub struct Vecs {
    db: Database,
    pub block: BlockVecs,
    pub tx: TxVecs,
    pub txin: TxinVecs,
    pub txout: TxoutVecs,
    pub address: AddressVecs,
    pub output: OutputVecs,
}

impl Vecs {
    pub fn forced_import(parent: &Path, version: Version) -> Result<Self> {
        log::debug!("Opening vecs database...");
        let db = Database::open(&parent.join("vecs"))?;
        log::debug!("Setting min len...");
        db.set_min_len(PAGE_SIZE * 50_000_000)?;

        log::debug!("Importing sub-vecs in parallel...");
        let (block, tx, txin, txout, address, output) = parallel_import! {
            block = {
                log::debug!("Importing BlockVecs...");
                let r = BlockVecs::forced_import(&db, version);
                log::debug!("BlockVecs imported.");
                r
            },
            tx = {
                log::debug!("Importing TxVecs...");
                let r = TxVecs::forced_import(&db, version);
                log::debug!("TxVecs imported.");
                r
            },
            txin = {
                log::debug!("Importing TxinVecs...");
                let r = TxinVecs::forced_import(&db, version);
                log::debug!("TxinVecs imported.");
                r
            },
            txout = {
                log::debug!("Importing TxoutVecs...");
                let r = TxoutVecs::forced_import(&db, version);
                log::debug!("TxoutVecs imported.");
                r
            },
            address = {
                log::debug!("Importing AddressVecs...");
                let r = AddressVecs::forced_import(&db, version);
                log::debug!("AddressVecs imported.");
                r
            },
            output = {
                log::debug!("Importing OutputVecs...");
                let r = OutputVecs::forced_import(&db, version);
                log::debug!("OutputVecs imported.");
                r
            },
        };
        log::debug!("Sub-vecs imported.");

        let this = Self {
            db,
            block,
            tx,
            txin,
            txout,
            address,
            output,
        };

        this.db.retain_regions(
            this.iter_any_exportable()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;
        this.db.compact()?;

        Ok(this)
    }

    pub fn rollback_if_needed(&mut self, starting_indexes: &Indexes) -> Result<()> {
        let saved_height = starting_indexes.height.decremented().unwrap_or_default();
        let stamp = Stamp::from(u64::from(saved_height));

        self.block.truncate(starting_indexes.height, stamp)?;

        self.tx
            .truncate(starting_indexes.height, starting_indexes.txindex, stamp)?;

        self.txin
            .truncate(starting_indexes.height, starting_indexes.txinindex, stamp)?;

        self.txout
            .truncate(starting_indexes.height, starting_indexes.txoutindex, stamp)?;

        self.address.truncate(
            starting_indexes.height,
            starting_indexes.p2pk65addressindex,
            starting_indexes.p2pk33addressindex,
            starting_indexes.p2pkhaddressindex,
            starting_indexes.p2shaddressindex,
            starting_indexes.p2wpkhaddressindex,
            starting_indexes.p2wshaddressindex,
            starting_indexes.p2traddressindex,
            starting_indexes.p2aaddressindex,
            stamp,
        )?;

        self.output.truncate(
            starting_indexes.height,
            starting_indexes.emptyoutputindex,
            starting_indexes.opreturnindex,
            starting_indexes.p2msoutputindex,
            starting_indexes.unknownoutputindex,
            stamp,
        )?;

        Ok(())
    }

    pub fn get_addressbytes_by_type(
        &self,
        addresstype: OutputType,
        typeindex: TypeIndex,
        reader: &Reader,
    ) -> Result<Option<AddressBytes>> {
        self.address
            .get_bytes_by_type(addresstype, typeindex, reader)
    }

    pub fn push_bytes_if_needed(&mut self, index: TypeIndex, bytes: AddressBytes) -> Result<()> {
        self.address.push_bytes_if_needed(index, bytes)
    }

    pub fn flush(&mut self, height: Height) -> Result<()> {
        self.par_iter_mut_any_stored_vec()
            .try_for_each(|vec| vec.stamped_write(Stamp::from(height)))?;
        self.db.flush()?;
        Ok(())
    }

    pub fn starting_height(&mut self) -> Height {
        self.par_iter_mut_any_stored_vec()
            .map(|vec| {
                let h = Height::from(vec.stamp());
                if h > Height::ZERO { h.incremented() } else { h }
            })
            .min()
            .unwrap()
    }

    pub fn compact(&self) -> Result<()> {
        self.db.compact()?;
        Ok(())
    }

    pub fn iter_address_hashes_from(
        &self,
        address_type: OutputType,
        height: Height,
    ) -> Result<Box<dyn Iterator<Item = AddressHash> + '_>> {
        self.address.iter_hashes_from(address_type, height)
    }

    fn par_iter_mut_any_stored_vec(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        self.block
            .par_iter_mut_any()
            .chain(self.tx.par_iter_mut_any())
            .chain(self.txin.par_iter_mut_any())
            .chain(self.txout.par_iter_mut_any())
            .chain(self.address.par_iter_mut_any())
            .chain(self.output.par_iter_mut_any())
    }

    pub fn db(&self) -> &Database {
        &self.db
    }
}
