use std::thread::{self};

use allocative::Allocative;

mod _database;
mod _trait;
mod address_index_to_address_data;
mod address_index_to_empty_address_data;
mod address_to_address_index;
mod metadata;
mod txid_to_tx_data;
mod txout_index_to_address_index;
mod txout_index_to_amount;

pub use _database::*;
use _trait::*;
pub use address_index_to_address_data::*;
pub use address_index_to_empty_address_data::*;
pub use address_to_address_index::*;
use itertools::Itertools;
use metadata::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
pub use txid_to_tx_data::*;
pub use txout_index_to_address_index::*;
pub use txout_index_to_amount::*;

use crate::{
    log,
    structs::{Date, Height},
    utils::time,
    Exit,
};

#[derive(Allocative)]
pub struct Databases {
    pub address_index_to_address_data: AddressIndexToAddressData,
    pub address_index_to_empty_address_data: AddressIndexToEmptyAddressData,
    pub address_to_address_index: AddressToAddressIndex,
    pub txid_to_tx_data: TxidToTxData,
    pub txout_index_to_address_index: TxoutIndexToAddressIndex,
    pub txout_index_to_amount: TxoutIndexToAmount,
}

impl Databases {
    pub fn import() -> Self {
        let address_index_to_address_data = AddressIndexToAddressData::init();

        let address_index_to_empty_address_data = AddressIndexToEmptyAddressData::init();

        let address_to_address_index = AddressToAddressIndex::init();

        let txid_to_tx_data = TxidToTxData::init();

        let txout_index_to_address_index = TxoutIndexToAddressIndex::init();

        let txout_index_to_amount = TxoutIndexToAmount::init();

        Self {
            address_index_to_address_data,
            address_index_to_empty_address_data,
            address_to_address_index,
            txid_to_tx_data,
            txout_index_to_address_index,
            txout_index_to_amount,
        }
    }

    pub fn drain_to_vec(&mut self) -> Vec<Box<dyn AnyDatabase + Send>> {
        self.txid_to_tx_data
            .drain_to_vec()
            .into_iter()
            .chain(self.txout_index_to_amount.drain_to_vec())
            .chain(self.address_to_address_index.drain_to_vec())
            .chain(self.address_index_to_address_data.drain_to_vec())
            .chain(self.address_index_to_empty_address_data.drain_to_vec())
            .chain(self.txout_index_to_address_index.drain_to_vec())
            .collect_vec()
    }

    fn export_metadata(&mut self, height: Height, date: Date) -> color_eyre::Result<()> {
        self.txid_to_tx_data.export_metadata(height, date)?;
        self.txout_index_to_amount.export_metadata(height, date)?;
        self.address_index_to_address_data
            .export_metadata(height, date)?;
        self.address_index_to_empty_address_data
            .export_metadata(height, date)?;
        self.address_to_address_index
            .export_metadata(height, date)?;
        self.txout_index_to_address_index
            .export_metadata(height, date)?;
        Ok(())
    }

    pub fn export(&mut self, height: Height, date: Date) -> color_eyre::Result<()> {
        self.export_metadata(height, date)?;

        self.drain_to_vec()
            .into_par_iter()
            .try_for_each(AnyDatabase::boxed_export)?;

        Ok(())
    }

    fn open_all(&mut self) {
        thread::scope(|s| {
            s.spawn(|| {
                self.address_index_to_address_data.open_all();
            });

            s.spawn(|| {
                self.address_index_to_empty_address_data.open_all();
            });

            s.spawn(|| {
                self.address_to_address_index.open_all();
            });

            s.spawn(|| {
                self.txid_to_tx_data.open_all();
            });

            s.spawn(|| {
                self.txout_index_to_address_index.open_all();
            });

            s.spawn(|| {
                self.txout_index_to_amount.open_all();
            });
        });
    }

    pub fn defragment(&mut self, exit: &Exit) {
        exit.block();

        log("Databases defragmentation");

        time("Defragmenting databases", || {
            time("Opened all databases", || self.open_all());

            log("Defragmenting...");

            self.drain_to_vec()
                .into_par_iter()
                .for_each(AnyDatabase::boxed_defragment);
        });

        exit.unblock();
    }

    pub fn reset(&mut self, include_addresses: bool) {
        if include_addresses {
            let _ = self.address_index_to_address_data.reset();
            let _ = self.address_index_to_empty_address_data.reset();
            let _ = self.address_to_address_index.reset();
            let _ = self.txout_index_to_address_index.reset();
        }

        let _ = self.txid_to_tx_data.reset();
        let _ = self.txout_index_to_amount.reset();
    }

    pub fn check_if_needs_to_compute_addresses(&self, height: Height, date: Date) -> bool {
        let check_height = |last_height: Option<Height>| {
            last_height.map_or(true, |last_height| last_height < height)
        };

        let check_date =
            |last_date: Option<Date>| last_date.map_or(true, |last_date| last_date < date);

        let check_metadata = |metadata: &Metadata| {
            check_height(metadata.last_height) || check_date(metadata.last_date)
        };

        // We only need to check one as we previously checked that they're all in sync
        check_metadata(&self.address_to_address_index.metadata)
    }

    pub fn check_if_usable(
        &self,
        min_initial_last_address_height: Option<Height>,
        min_initial_last_address_date: Option<Date>,
    ) -> bool {
        let are_tx_databases_in_sync = self
            .txout_index_to_amount
            .metadata
            .check_if_in_sync(&self.txid_to_tx_data.metadata);

        if !are_tx_databases_in_sync {
            return false;
        }

        let are_address_databases_in_sync = self
            .address_to_address_index
            .metadata
            .check_if_in_sync(&self.address_index_to_empty_address_data.metadata)
            && self
                .address_to_address_index
                .metadata
                .check_if_in_sync(&self.address_index_to_address_data.metadata)
            && self
                .address_to_address_index
                .metadata
                .check_if_in_sync(&self.txout_index_to_address_index.metadata);

        if !are_address_databases_in_sync {
            return false;
        }

        let are_address_databases_farer_or_in_sync_with_tx_database = self
            .address_to_address_index
            .metadata
            .check_farer_or_in_sync(&self.txid_to_tx_data.metadata);

        if !are_address_databases_farer_or_in_sync_with_tx_database {
            return false;
        }

        // let are_address_datasets_farer_or_in_sync_with_address_databases =
        min_initial_last_address_height >= self.address_to_address_index.metadata.last_height
            && min_initial_last_address_date >= self.address_to_address_index.metadata.last_date
    }
}
