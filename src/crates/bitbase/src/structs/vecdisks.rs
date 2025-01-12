use std::path::Path;

use biter::bitcoin::{BlockHash, Txid};
use color_eyre::eyre::eyre;

use super::{
    Addressbytes, Addressindex, Addresstype, Addresstypeindex, Amount, AnyVecdisk, Exit, Height, P2PK33AddressBytes,
    P2PK65AddressBytes, P2PKHAddressBytes, P2SHAddressBytes, P2TRAddressBytes, P2WPKHAddressBytes, P2WSHAddressBytes,
    Txindex, Txoutindex, Vecdisk,
};

pub struct Vecdisks {
    // TODO:
    //
    // Add
    // txindex_to_fees
    // height_to_fees
    // height_to_utc_date
    // height_to_timestamp
    //
    // NOT the following as because of reorg it's subjective
    // date_to_fees
    // date_to_first_height
    // date_to_last_height
    pub addressindex_to_addresstype: Vecdisk<Addressindex, Addresstype>,
    pub addressindex_to_addresstypeindex: Vecdisk<Addressindex, Addresstypeindex>,
    pub height_to_blockhash: Vecdisk<Height, BlockHash>,
    pub height_to_first_addressindex: Vecdisk<Height, Addressindex>,
    pub height_to_first_txindex: Vecdisk<Height, Txindex>,
    pub height_to_first_txoutindex: Vecdisk<Height, Txoutindex>,
    pub height_to_last_addressindex: Vecdisk<Height, Addressindex>,
    pub height_to_last_txindex: Vecdisk<Height, Txindex>,
    pub height_to_last_txoutindex: Vecdisk<Height, Txoutindex>,
    pub p2pk65index_to_p2pk65addressbytes: Vecdisk<Addresstypeindex, P2PK65AddressBytes>,
    pub p2pk33index_to_p2pk33addressbytes: Vecdisk<Addresstypeindex, P2PK33AddressBytes>,
    pub p2pkhindex_to_p2pkhaddressbytes: Vecdisk<Addresstypeindex, P2PKHAddressBytes>,
    pub p2shindex_to_p2shaddressbytes: Vecdisk<Addresstypeindex, P2SHAddressBytes>,
    pub p2wpkhindex_to_p2wpkhaddressbytes: Vecdisk<Addresstypeindex, P2WPKHAddressBytes>,
    pub p2wshindex_to_p2wshaddressbytes: Vecdisk<Addresstypeindex, P2WSHAddressBytes>,
    pub p2trindex_to_p2traddressbytes: Vecdisk<Addresstypeindex, P2TRAddressBytes>,
    pub txindex_to_height: Vecdisk<Txindex, Height>,
    pub txindex_to_txid: Vecdisk<Txindex, Txid>,
    pub txoutindex_to_addressindex: Vecdisk<Txoutindex, Addressindex>,
    pub txoutindex_to_amount: Vecdisk<Txoutindex, Amount>,
}

// const UNSAFE_BLOCKS: usize = 100;

impl Vecdisks {
    pub fn import(path: &Path) -> color_eyre::Result<Self> {
        Ok(Self {
            addressindex_to_addresstype: Vecdisk::import(&path.join("addressindex_to_addresstype"))?,
            addressindex_to_addresstypeindex: Vecdisk::import(&path.join("addressindex_to_addresstypeindex"))?,
            height_to_blockhash: Vecdisk::import(&path.join("height_to_blockhash"))?,
            height_to_first_addressindex: Vecdisk::import(&path.join("height_to_first_addressindex"))?,
            height_to_first_txindex: Vecdisk::import(&path.join("height_to_first_txindex"))?,
            height_to_first_txoutindex: Vecdisk::import(&path.join("height_to_first_txoutindex"))?,
            height_to_last_addressindex: Vecdisk::import(&path.join("height_to_last_addressindex"))?,
            height_to_last_txindex: Vecdisk::import(&path.join("height_to_last_txindex"))?,
            height_to_last_txoutindex: Vecdisk::import(&path.join("height_to_last_txoutindex"))?,
            p2pk65index_to_p2pk65addressbytes: Vecdisk::import(&path.join("p2pk65index_to_p2pk65addressbytes"))?,
            p2pk33index_to_p2pk33addressbytes: Vecdisk::import(&path.join("p2pk33index_to_p2pk33addressbytes"))?,
            p2pkhindex_to_p2pkhaddressbytes: Vecdisk::import(&path.join("p2pkhindex_to_p2pkhaddressbytes"))?,
            p2shindex_to_p2shaddressbytes: Vecdisk::import(&path.join("p2shindex_to_p2shaddressbytes"))?,
            p2wpkhindex_to_p2wpkhaddressbytes: Vecdisk::import(&path.join("p2wpkhindex_to_p2wpkhaddressbytes"))?,
            p2wshindex_to_p2wshaddressbytes: Vecdisk::import(&path.join("p2wshindex_to_p2wshaddressbytes"))?,
            p2trindex_to_p2traddressbytes: Vecdisk::import(&path.join("p2trindex_to_p2traddressbytes"))?,
            txindex_to_height: Vecdisk::import(&path.join("txindex_to_height"))?,
            txindex_to_txid: Vecdisk::import(&path.join("txindex_to_txid"))?,
            txoutindex_to_addressindex: Vecdisk::import(&path.join("txoutindex_to_addressindex"))?,
            txoutindex_to_amount: Vecdisk::import(&path.join("txoutindex_to_amount"))?,
        })
    }

    pub fn addresstype_to_addressvecdisk(&self, addresstype: Addresstype) -> color_eyre::Result<&dyn AnyVecdisk> {
        match addresstype {
            Addresstype::P2PK65 => Ok(&self.p2pk65index_to_p2pk65addressbytes),
            Addresstype::P2PK33 => Ok(&self.p2pk33index_to_p2pk33addressbytes),
            Addresstype::P2PKH => Ok(&self.p2pkhindex_to_p2pkhaddressbytes),
            Addresstype::P2SH => Ok(&self.p2shindex_to_p2shaddressbytes),
            Addresstype::P2WPKH => Ok(&self.p2wpkhindex_to_p2wpkhaddressbytes),
            Addresstype::P2WSH => Ok(&self.p2wshindex_to_p2wshaddressbytes),
            Addresstype::P2TR => Ok(&self.p2trindex_to_p2traddressbytes),
            _ => Err(eyre!("wrong address type")),
        }
    }

    pub fn push_addressbytes_if_needed(
        &mut self,
        index: Addresstypeindex,
        addressbytes: Addressbytes,
    ) -> color_eyre::Result<()> {
        match addressbytes {
            Addressbytes::P2PK65(bytes) => self.p2pk65index_to_p2pk65addressbytes.push_if_needed(index, bytes),
            Addressbytes::P2PK33(bytes) => self.p2pk33index_to_p2pk33addressbytes.push_if_needed(index, bytes),
            Addressbytes::P2PKH(bytes) => self.p2pkhindex_to_p2pkhaddressbytes.push_if_needed(index, bytes),
            Addressbytes::P2SH(bytes) => self.p2shindex_to_p2shaddressbytes.push_if_needed(index, bytes),
            Addressbytes::P2WPKH(bytes) => self.p2wpkhindex_to_p2wpkhaddressbytes.push_if_needed(index, bytes),
            Addressbytes::P2WSH(bytes) => self.p2wshindex_to_p2wshaddressbytes.push_if_needed(index, bytes),
            Addressbytes::P2TR(bytes) => self.p2trindex_to_p2traddressbytes.push_if_needed(index, bytes),
        }
    }

    #[allow(unused)]
    pub fn rollback_from(&mut self, _height: Height, _exit: &Exit) -> color_eyre::Result<()> {
        panic!();
        // let mut txindex = None;

        // wtx.range(self.height_to_blockhash.data(), Slice::from(height)..)
        //     .try_for_each(|slice| -> color_eyre::Result<()> {
        //         let (height_slice, slice_blockhash) = slice?;
        //         let blockhash = BlockHash::from_slice(&slice_blockhash)?;

        //         wtx.remove(self.height_to_blockhash.data(), height_slice);

        //         wtx.remove(self.blockhash_prefix_to_height.data(), blockhash.prefix());

        //         if txindex.is_none() {
        //             txindex.replace(
        //                 wtx.get(self.height_to_first_txindex.data(), height_slice)?
        //                     .context("for height to have first txindex")?,
        //             );
        //         }
        //         wtx.remove(self.height_to_first_txindex.data(), height_slice);
        //         wtx.remove(self.height_to_last_txindex.data(), height_slice);

        //         Ok(())
        //     })?;

        // let txindex = txindex.context("txindex to not be none by now")?;

        // wtx.range(self.txindex_to_txid.data(), Slice::from(txindex)..)
        //     .try_for_each(|slice| -> color_eyre::Result<()> {
        //         let (slice_txindex, slice_txid) = slice?;
        //         let txindex = Txindex::from(slice_txindex);
        //         let txid = Txid::from_slice(&slice_txid)?;

        //         wtx.remove(self.txindex_to_txid.data(), Slice::from(txindex));
        //         wtx.remove(self.txindex_to_height.data(), Slice::from(txindex));
        //         wtx.remove(self.txid_prefix_to_txindex.data(), txid.prefix());

        //         Ok(())
        //     })?;

        // let txoutindex = Txoutindex::from(txindex);

        // let mut addressindexes = BTreeSet::new();

        // wtx.range(self.txoutindex_to_amount.data(), Slice::from(txoutindex)..)
        //     .try_for_each(|slice| -> color_eyre::Result<()> {
        //         let (txoutindex_slice, _) = slice?;

        //         wtx.remove(self.txoutindex_to_amount.data(), txoutindex_slice);

        //         if let Some(addressindex_slice) =
        //             wtx.get(self.txoutindex_to_addressindex.data(), txoutindex_slice)?
        //         {
        //             wtx.remove(self.txoutindex_to_addressindex.data(), txoutindex_slice);

        //             let addressindex = Addressindex::from(addressindex_slice);
        //             addressindexes.insert(addressindex);

        //             let txoutindex = Txoutindex::from(txoutindex_slice);
        //             let addresstxoutindex = Addresstxoutindex::from((addressindex, txoutindex));

        //             wtx.remove(
        //                 self.addressindex_to_txoutindexes.data(),
        //                 Slice::from(addresstxoutindex),
        //             );
        //         }

        //         Ok(())
        //     })?;

        // addressindexes
        // .into_iter()
        // .filter(|addressindex| {
        //     let is_empty = wtx
        //         .prefix(
        //             self.addressindex_to_txoutindexes.data(),
        //             Slice::from(*addressindex),
        //         )
        //         .next()
        //         .is_none();
        //     is_empty
        // })
        // .try_for_each(|addressindex| -> color_eyre::Result<()> {
        //     let addressindex_slice = Slice::from(addressindex);

        //     let addressbytes = Addressbytes::from(
        //         wtx.get(
        //             self.addressindex_to_addressbytes.data(),
        //             &addressindex_slice,
        //         )?
        //         .context("addressindex_to_address to have value")?,
        //     );
        //     wtx.remove(
        //         self.addressbytes_prefix_to_addressindex.data(),
        //         addressbytes.prefix(),
        //     );
        //     wtx.remove(
        //         self.addressindex_to_addressbytes.data(),
        //         &addressindex_slice,
        //     );
        //     wtx.remove(self.addressindex_to_addresstype.data(), &addressindex_slice);

        //     Ok(())
        // })?;
        //

        // todo!("clear addresstxoutindexes_out")
        // todo!("clear addresstxoutindexes_in")
        // todo!("clear zero_txoutindexes")

        // Ok(())
    }

    pub fn flush(&mut self) -> color_eyre::Result<()> {
        self.as_mut_vec().into_iter().try_for_each(AnyVecdisk::flush)
    }

    fn as_mut_vec(&mut self) -> Vec<&mut dyn AnyVecdisk> {
        vec![
            &mut self.addressindex_to_addresstype,
            &mut self.addressindex_to_addresstypeindex,
            &mut self.height_to_blockhash,
            &mut self.height_to_first_addressindex,
            &mut self.height_to_first_txindex,
            &mut self.height_to_first_txoutindex,
            &mut self.height_to_last_addressindex,
            &mut self.height_to_last_txindex,
            &mut self.height_to_last_txoutindex,
            &mut self.p2pk65index_to_p2pk65addressbytes,
            &mut self.p2pk33index_to_p2pk33addressbytes,
            &mut self.p2pkhindex_to_p2pkhaddressbytes,
            &mut self.p2shindex_to_p2shaddressbytes,
            &mut self.p2wpkhindex_to_p2wpkhaddressbytes,
            &mut self.p2wshindex_to_p2wshaddressbytes,
            &mut self.p2trindex_to_p2traddressbytes,
            &mut self.txindex_to_height,
            &mut self.txindex_to_txid,
            &mut self.txoutindex_to_addressindex,
            &mut self.txoutindex_to_amount,
        ]
    }
}
