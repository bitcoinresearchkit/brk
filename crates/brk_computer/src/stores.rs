use std::{path::Path, thread};

use brk_core::{
    AddressData, EmptyAddressData, GroupedByAddressType, Height, OutputIndex, OutputType,
    P2AAddressIndex, P2AAddressIndexOutputindex, P2PK33AddressIndex, P2PK33AddressIndexOutputindex,
    P2PK65AddressIndex, P2PK65AddressIndexOutputindex, P2PKHAddressIndex,
    P2PKHAddressIndexOutputindex, P2SHAddressIndex, P2SHAddressIndexOutputindex, P2TRAddressIndex,
    P2TRAddressIndexOutputindex, P2WPKHAddressIndex, P2WPKHAddressIndexOutputindex,
    P2WSHAddressIndex, P2WSHAddressIndexOutputindex, Result, TypeIndex, Unit, Version,
};
use brk_store::{AnyStore, Store};
use fjall::{PersistMode, TransactionalKeyspace};
use rayon::prelude::*;

use crate::vecs::stateful::AddressTypeToTypeIndexVec;

const VERSION: Version = Version::ZERO;

#[derive(Clone)]
pub struct Stores {
    keyspace: TransactionalKeyspace,

    pub p2aaddressindex_to_addressdata: Store<P2AAddressIndex, AddressData>,
    pub p2aaddressindex_to_emptyaddressdata: Store<P2AAddressIndex, EmptyAddressData>,
    pub p2aaddressindex_to_outputs_received: Store<P2AAddressIndexOutputindex, Unit>,
    pub p2aaddressindex_to_outputs_sent: Store<P2AAddressIndexOutputindex, Unit>,

    pub p2pk33addressindex_to_addressdata: Store<P2PK33AddressIndex, AddressData>,
    pub p2pk33addressindex_to_emptyaddressdata: Store<P2PK33AddressIndex, EmptyAddressData>,
    pub p2pk33addressindex_to_outputs_received: Store<P2PK33AddressIndexOutputindex, Unit>,
    pub p2pk33addressindex_to_outputs_sent: Store<P2PK33AddressIndexOutputindex, Unit>,

    pub p2pk65addressindex_to_addressdata: Store<P2PK65AddressIndex, AddressData>,
    pub p2pk65addressindex_to_emptyaddressdata: Store<P2PK65AddressIndex, EmptyAddressData>,
    pub p2pk65addressindex_to_outputs_received: Store<P2PK65AddressIndexOutputindex, Unit>,
    pub p2pk65addressindex_to_outputs_sent: Store<P2PK65AddressIndexOutputindex, Unit>,

    pub p2pkhaddressindex_to_addressdata: Store<P2PKHAddressIndex, AddressData>,
    pub p2pkhaddressindex_to_emptyaddressdata: Store<P2PKHAddressIndex, EmptyAddressData>,
    pub p2pkhaddressindex_to_outputs_received: Store<P2PKHAddressIndexOutputindex, Unit>,
    pub p2pkhaddressindex_to_outputs_sent: Store<P2PKHAddressIndexOutputindex, Unit>,

    pub p2shaddressindex_to_addressdata: Store<P2SHAddressIndex, AddressData>,
    pub p2shaddressindex_to_emptyaddressdata: Store<P2SHAddressIndex, EmptyAddressData>,
    pub p2shaddressindex_to_outputs_received: Store<P2SHAddressIndexOutputindex, Unit>,
    pub p2shaddressindex_to_outputs_sent: Store<P2SHAddressIndexOutputindex, Unit>,

    pub p2traddressindex_to_addressdata: Store<P2TRAddressIndex, AddressData>,
    pub p2traddressindex_to_emptyaddressdata: Store<P2TRAddressIndex, EmptyAddressData>,
    pub p2traddressindex_to_outputs_received: Store<P2TRAddressIndexOutputindex, Unit>,
    pub p2traddressindex_to_outputs_sent: Store<P2TRAddressIndexOutputindex, Unit>,

    pub p2wpkhaddressindex_to_addressdata: Store<P2WPKHAddressIndex, AddressData>,
    pub p2wpkhaddressindex_to_emptyaddressdata: Store<P2WPKHAddressIndex, EmptyAddressData>,
    pub p2wpkhaddressindex_to_outputs_received: Store<P2WPKHAddressIndexOutputindex, Unit>,
    pub p2wpkhaddressindex_to_outputs_sent: Store<P2WPKHAddressIndexOutputindex, Unit>,

    pub p2wshaddressindex_to_addressdata: Store<P2WSHAddressIndex, AddressData>,
    pub p2wshaddressindex_to_emptyaddressdata: Store<P2WSHAddressIndex, EmptyAddressData>,
    pub p2wshaddressindex_to_outputs_received: Store<P2WSHAddressIndexOutputindex, Unit>,
    pub p2wshaddressindex_to_outputs_sent: Store<P2WSHAddressIndexOutputindex, Unit>,
}

impl Stores {
    pub fn import(
        path: &Path,
        version: Version,
        keyspace: &TransactionalKeyspace,
    ) -> color_eyre::Result<Self> {
        let (
            (
                p2aaddressindex_to_addressdata,
                p2aaddressindex_to_emptyaddressdata,
                p2aaddressindex_to_outputs_received,
                p2aaddressindex_to_outputs_sent,
            ),
            (
                p2pk33addressindex_to_addressdata,
                p2pk33addressindex_to_emptyaddressdata,
                p2pk33addressindex_to_outputs_received,
                p2pk33addressindex_to_outputs_sent,
            ),
            (
                p2pk65addressindex_to_addressdata,
                p2pk65addressindex_to_emptyaddressdata,
                p2pk65addressindex_to_outputs_received,
                p2pk65addressindex_to_outputs_sent,
            ),
            (
                p2pkhaddressindex_to_addressdata,
                p2pkhaddressindex_to_emptyaddressdata,
                p2pkhaddressindex_to_outputs_received,
                p2pkhaddressindex_to_outputs_sent,
            ),
            (
                p2shaddressindex_to_addressdata,
                p2shaddressindex_to_emptyaddressdata,
                p2shaddressindex_to_outputs_received,
                p2shaddressindex_to_outputs_sent,
            ),
            (
                p2traddressindex_to_addressdata,
                p2traddressindex_to_emptyaddressdata,
                p2traddressindex_to_outputs_received,
                p2traddressindex_to_outputs_sent,
            ),
            (
                p2wpkhaddressindex_to_addressdata,
                p2wpkhaddressindex_to_emptyaddressdata,
                p2wpkhaddressindex_to_outputs_received,
                p2wpkhaddressindex_to_outputs_sent,
            ),
            (
                p2wshaddressindex_to_addressdata,
                p2wshaddressindex_to_emptyaddressdata,
                p2wshaddressindex_to_outputs_received,
                p2wshaddressindex_to_outputs_sent,
            ),
        ) = thread::scope(|scope| {
            let p2a = scope.spawn(|| {
                (
                    Store::import(
                        keyspace,
                        path,
                        "p2aaddressindex_to_addressdata",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2aaddressindex_to_emptyaddressdata",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2aaddressindex_to_outputs_received",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2aaddressindex_to_outputs_sent",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                )
            });

            let p2pk33 = scope.spawn(|| {
                (
                    Store::import(
                        keyspace,
                        path,
                        "p2pk33addressindex_to_addressdata",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2pk33addressindex_to_emptyaddressdata",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2pk33addressindex_to_outputs_received",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2pk33addressindex_to_outputs_sent",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                )
            });

            let p2pk65 = scope.spawn(|| {
                (
                    Store::import(
                        keyspace,
                        path,
                        "p2pk65addressindex_to_addressdata",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2pk65addressindex_to_emptyaddressdata",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2pk65addressindex_to_outputs_received",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2pk65addressindex_to_outputs_sent",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                )
            });

            let p2pkh = scope.spawn(|| {
                (
                    Store::import(
                        keyspace,
                        path,
                        "p2pkhaddressindex_to_addressdata",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2pkhaddressindex_to_emptyaddressdata",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2pkhaddressindex_to_outputs_received",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2pkhaddressindex_to_outputs_sent",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                )
            });

            let p2sh = scope.spawn(|| {
                (
                    Store::import(
                        keyspace,
                        path,
                        "p2shaddressindex_to_addressdata",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2shaddressindex_to_emptyaddressdata",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2shaddressindex_to_outputs_received",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2shaddressindex_to_outputs_sent",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                )
            });

            let p2tr = scope.spawn(|| {
                (
                    Store::import(
                        keyspace,
                        path,
                        "p2traddressindex_to_addressdata",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2traddressindex_to_emptyaddressdata",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2traddressindex_to_outputs_received",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2traddressindex_to_outputs_sent",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                )
            });

            let p2wpkh = scope.spawn(|| {
                (
                    Store::import(
                        keyspace,
                        path,
                        "p2wpkhaddressindex_to_addressdata",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2wpkhaddressindex_to_emptyaddressdata",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2wpkhaddressindex_to_outputs_received",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2wpkhaddressindex_to_outputs_sent",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                )
            });

            let p2wsh = scope.spawn(|| {
                (
                    Store::import(
                        keyspace,
                        path,
                        "p2wshaddressindex_to_addressdata",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2wshaddressindex_to_emptyaddressdata",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2wshaddressindex_to_outputs_received",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                    Store::import(
                        keyspace,
                        path,
                        "p2wshaddressindex_to_outputs_sent",
                        version + VERSION + Version::ZERO,
                        None,
                    )
                    .unwrap(),
                )
            });

            (
                p2a.join().unwrap(),
                p2pk33.join().unwrap(),
                p2pk65.join().unwrap(),
                p2pkh.join().unwrap(),
                p2sh.join().unwrap(),
                p2tr.join().unwrap(),
                p2wpkh.join().unwrap(),
                p2wsh.join().unwrap(),
            )
        });

        Ok(Self {
            keyspace: keyspace.clone(),

            p2aaddressindex_to_addressdata,
            p2aaddressindex_to_emptyaddressdata,
            p2aaddressindex_to_outputs_received,
            p2aaddressindex_to_outputs_sent,

            p2pk33addressindex_to_addressdata,
            p2pk33addressindex_to_emptyaddressdata,
            p2pk33addressindex_to_outputs_received,
            p2pk33addressindex_to_outputs_sent,

            p2pk65addressindex_to_addressdata,
            p2pk65addressindex_to_emptyaddressdata,
            p2pk65addressindex_to_outputs_received,
            p2pk65addressindex_to_outputs_sent,

            p2pkhaddressindex_to_addressdata,
            p2pkhaddressindex_to_emptyaddressdata,
            p2pkhaddressindex_to_outputs_received,
            p2pkhaddressindex_to_outputs_sent,

            p2shaddressindex_to_addressdata,
            p2shaddressindex_to_emptyaddressdata,
            p2shaddressindex_to_outputs_received,
            p2shaddressindex_to_outputs_sent,

            p2traddressindex_to_addressdata,
            p2traddressindex_to_emptyaddressdata,
            p2traddressindex_to_outputs_received,
            p2traddressindex_to_outputs_sent,

            p2wpkhaddressindex_to_addressdata,
            p2wpkhaddressindex_to_emptyaddressdata,
            p2wpkhaddressindex_to_outputs_received,
            p2wpkhaddressindex_to_outputs_sent,

            p2wshaddressindex_to_addressdata,
            p2wshaddressindex_to_emptyaddressdata,
            p2wshaddressindex_to_outputs_received,
            p2wshaddressindex_to_outputs_sent,
        })
    }

    pub fn starting_height(&self) -> Height {
        self.as_slice()
            .into_iter()
            .map(|store| store.height().map(Height::incremented).unwrap_or_default())
            .min()
            .unwrap()
    }

    pub fn reset(&mut self) -> Result<()> {
        self.as_mut_slice()
            .into_par_iter()
            .try_for_each(|store| store.reset())?;

        self.keyspace
            .persist(PersistMode::SyncAll)
            .map_err(|e| e.into())
    }

    pub fn get_addressdata(
        &self,
        address_type: OutputType,
        type_index: TypeIndex,
    ) -> Result<Option<AddressData>> {
        Ok(match address_type {
            OutputType::P2A => self
                .p2aaddressindex_to_addressdata
                .get(&type_index.into())?
                .map(|c| c.into_owned()),
            OutputType::P2PK33 => self
                .p2pk33addressindex_to_addressdata
                .get(&type_index.into())?
                .map(|c| c.into_owned()),
            OutputType::P2PK65 => self
                .p2pk65addressindex_to_addressdata
                .get(&type_index.into())?
                .map(|c| c.into_owned()),
            OutputType::P2PKH => self
                .p2pkhaddressindex_to_addressdata
                .get(&type_index.into())?
                .map(|c| c.into_owned()),
            OutputType::P2SH => self
                .p2shaddressindex_to_addressdata
                .get(&type_index.into())?
                .map(|c| c.into_owned()),
            OutputType::P2TR => self
                .p2traddressindex_to_addressdata
                .get(&type_index.into())?
                .map(|c| c.into_owned()),
            OutputType::P2WPKH => self
                .p2wpkhaddressindex_to_addressdata
                .get(&type_index.into())?
                .map(|c| c.into_owned()),
            OutputType::P2WSH => self
                .p2wshaddressindex_to_addressdata
                .get(&type_index.into())?
                .map(|c| c.into_owned()),
            _ => unreachable!(),
        })
    }

    pub fn get_emptyaddressdata(
        &self,
        address_type: OutputType,
        type_index: TypeIndex,
    ) -> Result<Option<EmptyAddressData>> {
        Ok(match address_type {
            OutputType::P2A => self
                .p2aaddressindex_to_emptyaddressdata
                .get(&type_index.into())?
                .map(|c| c.into_owned()),
            OutputType::P2PK33 => self
                .p2pk33addressindex_to_emptyaddressdata
                .get(&type_index.into())?
                .map(|c| c.into_owned()),
            OutputType::P2PK65 => self
                .p2pk65addressindex_to_emptyaddressdata
                .get(&type_index.into())?
                .map(|c| c.into_owned()),
            OutputType::P2PKH => self
                .p2pkhaddressindex_to_emptyaddressdata
                .get(&type_index.into())?
                .map(|c| c.into_owned()),
            OutputType::P2SH => self
                .p2shaddressindex_to_emptyaddressdata
                .get(&type_index.into())?
                .map(|c| c.into_owned()),
            OutputType::P2TR => self
                .p2traddressindex_to_emptyaddressdata
                .get(&type_index.into())?
                .map(|c| c.into_owned()),
            OutputType::P2WPKH => self
                .p2wpkhaddressindex_to_emptyaddressdata
                .get(&type_index.into())?
                .map(|c| c.into_owned()),
            OutputType::P2WSH => self
                .p2wshaddressindex_to_emptyaddressdata
                .get(&type_index.into())?
                .map(|c| c.into_owned()),
            _ => unreachable!(),
        })
    }

    // pub fn emptyaddressdata_store_increase_replaced(&mut self, address_type: OutputType) {
    //     match address_type {
    //         OutputType::P2A => self.p2aaddressindex_to_addressdata.increase_replaced(),
    //         OutputType::P2PK33 => self.p2pk33addressindex_to_addressdata.increase_replaced(),
    //         OutputType::P2PK65 => self.p2pk65addressindex_to_addressdata.increase_replaced(),
    //         OutputType::P2PKH => self.p2pkhaddressindex_to_addressdata.increase_replaced(),
    //         OutputType::P2SH => self.p2shaddressindex_to_addressdata.increase_replaced(),
    //         OutputType::P2TR => self.p2traddressindex_to_addressdata.increase_replaced(),
    //         OutputType::P2WPKH => self.p2wpkhaddressindex_to_addressdata.increase_replaced(),
    //         OutputType::P2WSH => self.p2wshaddressindex_to_addressdata.increase_replaced(),
    //         _ => unreachable!(),
    //     }
    // }

    pub fn commit(
        &mut self,
        height: Height,
        sent: AddressTypeToTypeIndexVec<OutputIndex>,
        received: AddressTypeToTypeIndexVec<OutputIndex>,
    ) -> Result<()> {
        // &mut self.p2aaddressindex_to_addressdata,
        // &mut self.p2pk33addressindex_to_addressdata,
        // &mut self.p2pk65addressindex_to_addressdata,
        // &mut self.p2pkhaddressindex_to_addressdata,
        // &mut self.p2shaddressindex_to_addressdata,
        // &mut self.p2traddressindex_to_addressdata,
        // &mut self.p2wpkhaddressindex_to_addressdata,
        // &mut self.p2wshaddressindex_to_addressdata,

        // &mut self.p2aaddressindex_to_emptyaddressdata,
        // &mut self.p2pk33addressindex_to_emptyaddressdata,
        // &mut self.p2pk65addressindex_to_emptyaddressdata,
        // &mut self.p2pkhaddressindex_to_emptyaddressdata,
        // &mut self.p2shaddressindex_to_emptyaddressdata,
        // &mut self.p2traddressindex_to_emptyaddressdata,
        // &mut self.p2wpkhaddressindex_to_emptyaddressdata,
        // &mut self.p2wshaddressindex_to_emptyaddressdata,

        thread::scope(|s| {
            let GroupedByAddressType {
                p2pk65,
                p2pk33,
                p2pkh,
                p2sh,
                p2wpkh,
                p2wsh,
                p2tr,
                p2a,
            } = received.unwrap();

            s.spawn(|| {
                self.p2aaddressindex_to_outputs_received.commit_(
                    height,
                    [].into_iter(),
                    p2a.into_iter()
                        .map(P2AAddressIndexOutputindex::from)
                        .map(|i| (i, Unit)),
                )
            });
            s.spawn(|| {
                self.p2pk33addressindex_to_outputs_received.commit_(
                    height,
                    [].into_iter(),
                    p2pk33
                        .into_iter()
                        .map(P2PK33AddressIndexOutputindex::from)
                        .map(|i| (i, Unit)),
                )
            });
            s.spawn(|| {
                self.p2pk65addressindex_to_outputs_received.commit_(
                    height,
                    [].into_iter(),
                    p2pk65
                        .into_iter()
                        .map(P2PK65AddressIndexOutputindex::from)
                        .map(|i| (i, Unit)),
                )
            });
            s.spawn(|| {
                self.p2pkhaddressindex_to_outputs_received.commit_(
                    height,
                    [].into_iter(),
                    p2pkh
                        .into_iter()
                        .map(P2PKHAddressIndexOutputindex::from)
                        .map(|i| (i, Unit)),
                )
            });
            s.spawn(|| {
                self.p2shaddressindex_to_outputs_received.commit_(
                    height,
                    [].into_iter(),
                    p2sh.into_iter()
                        .map(P2SHAddressIndexOutputindex::from)
                        .map(|i| (i, Unit)),
                )
            });
            s.spawn(|| {
                self.p2traddressindex_to_outputs_received.commit_(
                    height,
                    [].into_iter(),
                    p2tr.into_iter()
                        .map(P2TRAddressIndexOutputindex::from)
                        .map(|i| (i, Unit)),
                )
            });
            s.spawn(|| {
                self.p2wpkhaddressindex_to_outputs_received.commit_(
                    height,
                    [].into_iter(),
                    p2wpkh
                        .into_iter()
                        .map(P2WPKHAddressIndexOutputindex::from)
                        .map(|i| (i, Unit)),
                )
            });
            s.spawn(|| {
                self.p2wshaddressindex_to_outputs_received.commit_(
                    height,
                    [].into_iter(),
                    p2wsh
                        .into_iter()
                        .map(P2WSHAddressIndexOutputindex::from)
                        .map(|i| (i, Unit)),
                )
            });
        });

        thread::scope(|s| {
            let GroupedByAddressType {
                p2pk65,
                p2pk33,
                p2pkh,
                p2sh,
                p2wpkh,
                p2wsh,
                p2tr,
                p2a,
            } = sent.unwrap();

            s.spawn(|| {
                self.p2aaddressindex_to_outputs_sent.commit_(
                    height,
                    [].into_iter(),
                    p2a.into_iter()
                        .map(P2AAddressIndexOutputindex::from)
                        .map(|i| (i, Unit)),
                )
            });
            s.spawn(|| {
                self.p2pk33addressindex_to_outputs_sent.commit_(
                    height,
                    [].into_iter(),
                    p2pk33
                        .into_iter()
                        .map(P2PK33AddressIndexOutputindex::from)
                        .map(|i| (i, Unit)),
                )
            });
            s.spawn(|| {
                self.p2pk65addressindex_to_outputs_sent.commit_(
                    height,
                    [].into_iter(),
                    p2pk65
                        .into_iter()
                        .map(P2PK65AddressIndexOutputindex::from)
                        .map(|i| (i, Unit)),
                )
            });
            s.spawn(|| {
                self.p2pkhaddressindex_to_outputs_sent.commit_(
                    height,
                    [].into_iter(),
                    p2pkh
                        .into_iter()
                        .map(P2PKHAddressIndexOutputindex::from)
                        .map(|i| (i, Unit)),
                )
            });
            s.spawn(|| {
                self.p2shaddressindex_to_outputs_sent.commit_(
                    height,
                    [].into_iter(),
                    p2sh.into_iter()
                        .map(P2SHAddressIndexOutputindex::from)
                        .map(|i| (i, Unit)),
                )
            });
            s.spawn(|| {
                self.p2traddressindex_to_outputs_sent.commit_(
                    height,
                    [].into_iter(),
                    p2tr.into_iter()
                        .map(P2TRAddressIndexOutputindex::from)
                        .map(|i| (i, Unit)),
                )
            });
            s.spawn(|| {
                self.p2wpkhaddressindex_to_outputs_sent.commit_(
                    height,
                    [].into_iter(),
                    p2wpkh
                        .into_iter()
                        .map(P2WPKHAddressIndexOutputindex::from)
                        .map(|i| (i, Unit)),
                )
            });
            s.spawn(|| {
                self.p2wshaddressindex_to_outputs_sent.commit_(
                    height,
                    [].into_iter(),
                    p2wsh
                        .into_iter()
                        .map(P2WSHAddressIndexOutputindex::from)
                        .map(|i| (i, Unit)),
                )
            });
        });

        self.keyspace
            .persist(PersistMode::SyncAll)
            .map_err(|e| e.into())
    }

    pub fn rotate_memtables(&self) {
        self.as_slice()
            .into_iter()
            .for_each(|store| store.rotate_memtable());
    }

    pub fn as_slice(&self) -> [&(dyn AnyStore + Send + Sync); 32] {
        [
            &self.p2aaddressindex_to_addressdata,
            &self.p2aaddressindex_to_emptyaddressdata,
            &self.p2aaddressindex_to_outputs_received,
            &self.p2aaddressindex_to_outputs_sent,
            &self.p2pk33addressindex_to_addressdata,
            &self.p2pk33addressindex_to_emptyaddressdata,
            &self.p2pk33addressindex_to_outputs_received,
            &self.p2pk33addressindex_to_outputs_sent,
            &self.p2pk65addressindex_to_addressdata,
            &self.p2pk65addressindex_to_emptyaddressdata,
            &self.p2pk65addressindex_to_outputs_received,
            &self.p2pk65addressindex_to_outputs_sent,
            &self.p2pkhaddressindex_to_addressdata,
            &self.p2pkhaddressindex_to_emptyaddressdata,
            &self.p2pkhaddressindex_to_outputs_received,
            &self.p2pkhaddressindex_to_outputs_sent,
            &self.p2shaddressindex_to_addressdata,
            &self.p2shaddressindex_to_emptyaddressdata,
            &self.p2shaddressindex_to_outputs_received,
            &self.p2shaddressindex_to_outputs_sent,
            &self.p2traddressindex_to_addressdata,
            &self.p2traddressindex_to_emptyaddressdata,
            &self.p2traddressindex_to_outputs_received,
            &self.p2traddressindex_to_outputs_sent,
            &self.p2wpkhaddressindex_to_addressdata,
            &self.p2wpkhaddressindex_to_emptyaddressdata,
            &self.p2wpkhaddressindex_to_outputs_received,
            &self.p2wpkhaddressindex_to_outputs_sent,
            &self.p2wshaddressindex_to_addressdata,
            &self.p2wshaddressindex_to_emptyaddressdata,
            &self.p2wshaddressindex_to_outputs_received,
            &self.p2wshaddressindex_to_outputs_sent,
        ]
    }

    fn as_mut_slice(&mut self) -> [&mut (dyn AnyStore + Send + Sync); 32] {
        [
            &mut self.p2aaddressindex_to_addressdata,
            &mut self.p2aaddressindex_to_emptyaddressdata,
            &mut self.p2aaddressindex_to_outputs_received,
            &mut self.p2aaddressindex_to_outputs_sent,
            &mut self.p2pk33addressindex_to_addressdata,
            &mut self.p2pk33addressindex_to_emptyaddressdata,
            &mut self.p2pk33addressindex_to_outputs_received,
            &mut self.p2pk33addressindex_to_outputs_sent,
            &mut self.p2pk65addressindex_to_addressdata,
            &mut self.p2pk65addressindex_to_emptyaddressdata,
            &mut self.p2pk65addressindex_to_outputs_received,
            &mut self.p2pk65addressindex_to_outputs_sent,
            &mut self.p2pkhaddressindex_to_addressdata,
            &mut self.p2pkhaddressindex_to_emptyaddressdata,
            &mut self.p2pkhaddressindex_to_outputs_received,
            &mut self.p2pkhaddressindex_to_outputs_sent,
            &mut self.p2shaddressindex_to_addressdata,
            &mut self.p2shaddressindex_to_emptyaddressdata,
            &mut self.p2shaddressindex_to_outputs_received,
            &mut self.p2shaddressindex_to_outputs_sent,
            &mut self.p2traddressindex_to_addressdata,
            &mut self.p2traddressindex_to_emptyaddressdata,
            &mut self.p2traddressindex_to_outputs_received,
            &mut self.p2traddressindex_to_outputs_sent,
            &mut self.p2wpkhaddressindex_to_addressdata,
            &mut self.p2wpkhaddressindex_to_emptyaddressdata,
            &mut self.p2wpkhaddressindex_to_outputs_received,
            &mut self.p2wpkhaddressindex_to_outputs_sent,
            &mut self.p2wshaddressindex_to_addressdata,
            &mut self.p2wshaddressindex_to_emptyaddressdata,
            &mut self.p2wshaddressindex_to_outputs_received,
            &mut self.p2wshaddressindex_to_outputs_sent,
        ]
    }
}
