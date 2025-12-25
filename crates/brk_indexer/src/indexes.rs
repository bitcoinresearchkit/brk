use brk_error::Result;
use brk_types::{
    EmptyOutputIndex, Height, OpReturnIndex, OutputType, P2AAddressIndex, P2MSOutputIndex,
    P2PK33AddressIndex, P2PK65AddressIndex, P2PKHAddressIndex, P2SHAddressIndex, P2TRAddressIndex,
    P2WPKHAddressIndex, P2WSHAddressIndex, TxInIndex, TxIndex, TxOutIndex, TypeIndex,
    UnknownOutputIndex,
};
use log::debug;
use vecdb::{GenericStoredVec, IterableStoredVec, IterableVec, VecIndex, VecValue};

use crate::{Stores, Vecs};

#[derive(Debug, Default, Clone)]
pub struct Indexes {
    pub emptyoutputindex: EmptyOutputIndex,
    pub height: Height,
    pub opreturnindex: OpReturnIndex,
    pub p2msoutputindex: P2MSOutputIndex,
    pub p2pk33addressindex: P2PK33AddressIndex,
    pub p2pk65addressindex: P2PK65AddressIndex,
    pub p2pkhaddressindex: P2PKHAddressIndex,
    pub p2shaddressindex: P2SHAddressIndex,
    pub p2traddressindex: P2TRAddressIndex,
    pub p2wpkhaddressindex: P2WPKHAddressIndex,
    pub p2wshaddressindex: P2WSHAddressIndex,
    pub p2aaddressindex: P2AAddressIndex,
    pub txindex: TxIndex,
    pub txinindex: TxInIndex,
    pub txoutindex: TxOutIndex,
    pub unknownoutputindex: UnknownOutputIndex,
}

impl Indexes {
    pub fn to_typeindex(&self, outputtype: OutputType) -> TypeIndex {
        match outputtype {
            OutputType::Empty => *self.emptyoutputindex,
            OutputType::OpReturn => *self.opreturnindex,
            OutputType::P2A => *self.p2aaddressindex,
            OutputType::P2MS => *self.p2msoutputindex,
            OutputType::P2PK33 => *self.p2pk33addressindex,
            OutputType::P2PK65 => *self.p2pk65addressindex,
            OutputType::P2PKH => *self.p2pkhaddressindex,
            OutputType::P2SH => *self.p2shaddressindex,
            OutputType::P2TR => *self.p2traddressindex,
            OutputType::P2WPKH => *self.p2wpkhaddressindex,
            OutputType::P2WSH => *self.p2wshaddressindex,
            OutputType::Unknown => *self.unknownoutputindex,
        }
    }

    /// Increments the address index for the given address type and returns the previous value.
    /// Only call this for address types (P2PK65, P2PK33, P2PKH, P2SH, P2WPKH, P2WSH, P2TR, P2A).
    #[inline]
    pub fn increment_address_index(&mut self, addresstype: OutputType) -> TypeIndex {
        match addresstype {
            OutputType::P2PK65 => self.p2pk65addressindex.copy_then_increment(),
            OutputType::P2PK33 => self.p2pk33addressindex.copy_then_increment(),
            OutputType::P2PKH => self.p2pkhaddressindex.copy_then_increment(),
            OutputType::P2SH => self.p2shaddressindex.copy_then_increment(),
            OutputType::P2WPKH => self.p2wpkhaddressindex.copy_then_increment(),
            OutputType::P2WSH => self.p2wshaddressindex.copy_then_increment(),
            OutputType::P2TR => self.p2traddressindex.copy_then_increment(),
            OutputType::P2A => self.p2aaddressindex.copy_then_increment(),
            _ => unreachable!(),
        }
    }

    pub fn checked_push(&self, vecs: &mut Vecs) -> Result<()> {
        let height = self.height;
        vecs.tx
            .height_to_first_txindex
            .checked_push(height, self.txindex)?;
        vecs.txin
            .height_to_first_txinindex
            .checked_push(height, self.txinindex)?;
        vecs.txout
            .height_to_first_txoutindex
            .checked_push(height, self.txoutindex)?;
        vecs.output
            .height_to_first_emptyoutputindex
            .checked_push(height, self.emptyoutputindex)?;
        vecs.output
            .height_to_first_p2msoutputindex
            .checked_push(height, self.p2msoutputindex)?;
        vecs.output
            .height_to_first_opreturnindex
            .checked_push(height, self.opreturnindex)?;
        vecs.address
            .height_to_first_p2aaddressindex
            .checked_push(height, self.p2aaddressindex)?;
        vecs.output
            .height_to_first_unknownoutputindex
            .checked_push(height, self.unknownoutputindex)?;
        vecs.address
            .height_to_first_p2pk33addressindex
            .checked_push(height, self.p2pk33addressindex)?;
        vecs.address
            .height_to_first_p2pk65addressindex
            .checked_push(height, self.p2pk65addressindex)?;
        vecs.address
            .height_to_first_p2pkhaddressindex
            .checked_push(height, self.p2pkhaddressindex)?;
        vecs.address
            .height_to_first_p2shaddressindex
            .checked_push(height, self.p2shaddressindex)?;
        vecs.address
            .height_to_first_p2traddressindex
            .checked_push(height, self.p2traddressindex)?;
        vecs.address
            .height_to_first_p2wpkhaddressindex
            .checked_push(height, self.p2wpkhaddressindex)?;
        vecs.address
            .height_to_first_p2wshaddressindex
            .checked_push(height, self.p2wshaddressindex)?;

        Ok(())
    }
}

impl From<(Height, &mut Vecs, &Stores)> for Indexes {
    #[inline]
    fn from((min_height, vecs, stores): (Height, &mut Vecs, &Stores)) -> Self {
        debug!("Creating indexes from vecs and stores...");

        // Height at which we want to start: min last saved + 1 or 0
        let vecs_starting_height = vecs.starting_height();
        let stores_starting_height = stores.starting_height();
        let height = vecs_starting_height.min(stores_starting_height);
        if height < min_height {
            dbg!(height, min_height);
            unreachable!()
        }

        let emptyoutputindex = starting_index(
            &vecs.output.height_to_first_emptyoutputindex,
            &vecs.output.emptyoutputindex_to_txindex,
            height,
        )
        .unwrap();

        let p2msoutputindex = starting_index(
            &vecs.output.height_to_first_p2msoutputindex,
            &vecs.output.p2msoutputindex_to_txindex,
            height,
        )
        .unwrap();

        let opreturnindex = starting_index(
            &vecs.output.height_to_first_opreturnindex,
            &vecs.output.opreturnindex_to_txindex,
            height,
        )
        .unwrap();

        let p2pk33addressindex = starting_index(
            &vecs.address.height_to_first_p2pk33addressindex,
            &vecs.address.p2pk33addressindex_to_p2pk33bytes,
            height,
        )
        .unwrap();

        let p2pk65addressindex = starting_index(
            &vecs.address.height_to_first_p2pk65addressindex,
            &vecs.address.p2pk65addressindex_to_p2pk65bytes,
            height,
        )
        .unwrap();

        let p2pkhaddressindex = starting_index(
            &vecs.address.height_to_first_p2pkhaddressindex,
            &vecs.address.p2pkhaddressindex_to_p2pkhbytes,
            height,
        )
        .unwrap();

        let p2shaddressindex = starting_index(
            &vecs.address.height_to_first_p2shaddressindex,
            &vecs.address.p2shaddressindex_to_p2shbytes,
            height,
        )
        .unwrap();

        let p2traddressindex = starting_index(
            &vecs.address.height_to_first_p2traddressindex,
            &vecs.address.p2traddressindex_to_p2trbytes,
            height,
        )
        .unwrap();

        let p2wpkhaddressindex = starting_index(
            &vecs.address.height_to_first_p2wpkhaddressindex,
            &vecs.address.p2wpkhaddressindex_to_p2wpkhbytes,
            height,
        )
        .unwrap();

        let p2wshaddressindex = starting_index(
            &vecs.address.height_to_first_p2wshaddressindex,
            &vecs.address.p2wshaddressindex_to_p2wshbytes,
            height,
        )
        .unwrap();

        let p2aaddressindex = starting_index(
            &vecs.address.height_to_first_p2aaddressindex,
            &vecs.address.p2aaddressindex_to_p2abytes,
            height,
        )
        .unwrap();

        let txindex = starting_index(
            &vecs.tx.height_to_first_txindex,
            &vecs.tx.txindex_to_txid,
            height,
        )
        .unwrap();

        let txinindex = starting_index(
            &vecs.txin.height_to_first_txinindex,
            &vecs.txin.txinindex_to_outpoint,
            height,
        )
        .unwrap();

        let txoutindex = starting_index(
            &vecs.txout.height_to_first_txoutindex,
            &vecs.txout.txoutindex_to_txoutdata,
            height,
        )
        .unwrap();

        let unknownoutputindex = starting_index(
            &vecs.output.height_to_first_unknownoutputindex,
            &vecs.output.unknownoutputindex_to_txindex,
            height,
        )
        .unwrap();

        Self {
            emptyoutputindex,
            height,
            p2msoutputindex,
            opreturnindex,
            p2pk33addressindex,
            p2pk65addressindex,
            p2pkhaddressindex,
            p2shaddressindex,
            p2traddressindex,
            p2wpkhaddressindex,
            p2wshaddressindex,
            p2aaddressindex,
            txindex,
            txinindex,
            txoutindex,
            unknownoutputindex,
        }
    }
}

pub fn starting_index<I, T>(
    height_to_index: &impl IterableStoredVec<Height, I>,
    index_to_else: &impl IterableVec<I, T>,
    starting_height: Height,
) -> Option<I>
where
    I: VecValue + VecIndex + From<usize>,
    T: VecValue,
{
    let h = Height::from(height_to_index.stamp());
    if h.is_zero() {
        None
    } else if h + 1_u32 == starting_height {
        Some(I::from(index_to_else.len()))
    } else {
        height_to_index.iter().get(starting_height)
    }
}
