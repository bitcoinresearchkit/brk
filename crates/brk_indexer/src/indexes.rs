use brk_error::Result;
use brk_types::{
    EmptyOutputIndex, Height, OpReturnIndex, OutputType, P2AAddressIndex, P2MSOutputIndex,
    P2PK33AddressIndex, P2PK65AddressIndex, P2PKHAddressIndex, P2SHAddressIndex, P2TRAddressIndex,
    P2WPKHAddressIndex, P2WSHAddressIndex, TxInIndex, TxIndex, TxOutIndex, TypeIndex,
    UnknownOutputIndex,
};
use vecdb::{AnyIterableVec, AnyStoredIterableVec, GenericStoredVec, StoredIndex, StoredRaw};

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
            OutputType::P2PK33 => *self.p2pkhaddressindex,
            OutputType::P2PK65 => *self.p2pk65addressindex,
            OutputType::P2PKH => *self.p2pkhaddressindex,
            OutputType::P2SH => *self.p2shaddressindex,
            OutputType::P2TR => *self.p2traddressindex,
            OutputType::P2WPKH => *self.p2wpkhaddressindex,
            OutputType::P2WSH => *self.p2wshaddressindex,
            OutputType::Unknown => *self.unknownoutputindex,
            _ => unreachable!(),
        }
    }

    pub fn push_if_needed(&self, vecs: &mut Vecs) -> Result<()> {
        let height = self.height;
        vecs.height_to_first_txindex
            .push_if_needed(height, self.txindex)?;
        vecs.height_to_first_txinindex
            .push_if_needed(height, self.txinindex)?;
        vecs.height_to_first_txoutindex
            .push_if_needed(height, self.txoutindex)?;
        vecs.height_to_first_emptyoutputindex
            .push_if_needed(height, self.emptyoutputindex)?;
        vecs.height_to_first_p2msoutputindex
            .push_if_needed(height, self.p2msoutputindex)?;
        vecs.height_to_first_opreturnindex
            .push_if_needed(height, self.opreturnindex)?;
        vecs.height_to_first_p2aaddressindex
            .push_if_needed(height, self.p2aaddressindex)?;
        vecs.height_to_first_unknownoutputindex
            .push_if_needed(height, self.unknownoutputindex)?;
        vecs.height_to_first_p2pk33addressindex
            .push_if_needed(height, self.p2pk33addressindex)?;
        vecs.height_to_first_p2pk65addressindex
            .push_if_needed(height, self.p2pk65addressindex)?;
        vecs.height_to_first_p2pkhaddressindex
            .push_if_needed(height, self.p2pkhaddressindex)?;
        vecs.height_to_first_p2shaddressindex
            .push_if_needed(height, self.p2shaddressindex)?;
        vecs.height_to_first_p2traddressindex
            .push_if_needed(height, self.p2traddressindex)?;
        vecs.height_to_first_p2wpkhaddressindex
            .push_if_needed(height, self.p2wpkhaddressindex)?;
        vecs.height_to_first_p2wshaddressindex
            .push_if_needed(height, self.p2wshaddressindex)?;

        Ok(())
    }
}

impl From<(Height, &mut Vecs, &Stores)> for Indexes {
    fn from((min_height, vecs, stores): (Height, &mut Vecs, &Stores)) -> Self {
        // Height at which we want to start: min last saved + 1 or 0
        let vecs_starting_height = vecs.starting_height();
        let stores_starting_height = stores.starting_height();
        let height = vecs_starting_height.min(stores_starting_height);
        if height < min_height {
            unreachable!()
        }

        let emptyoutputindex = starting_index(
            &vecs.height_to_first_emptyoutputindex,
            &vecs.emptyoutputindex_to_txindex,
            height,
        )
        .unwrap();

        let p2msoutputindex = starting_index(
            &vecs.height_to_first_p2msoutputindex,
            &vecs.p2msoutputindex_to_txindex,
            height,
        )
        .unwrap();

        let opreturnindex = starting_index(
            &vecs.height_to_first_opreturnindex,
            &vecs.opreturnindex_to_txindex,
            height,
        )
        .unwrap();

        let p2pk33addressindex = starting_index(
            &vecs.height_to_first_p2pk33addressindex,
            &vecs.p2pk33addressindex_to_p2pk33bytes,
            height,
        )
        .unwrap();

        let p2pk65addressindex = starting_index(
            &vecs.height_to_first_p2pk65addressindex,
            &vecs.p2pk65addressindex_to_p2pk65bytes,
            height,
        )
        .unwrap();

        let p2pkhaddressindex = starting_index(
            &vecs.height_to_first_p2pkhaddressindex,
            &vecs.p2pkhaddressindex_to_p2pkhbytes,
            height,
        )
        .unwrap();

        let p2shaddressindex = starting_index(
            &vecs.height_to_first_p2shaddressindex,
            &vecs.p2shaddressindex_to_p2shbytes,
            height,
        )
        .unwrap();

        let p2traddressindex = starting_index(
            &vecs.height_to_first_p2traddressindex,
            &vecs.p2traddressindex_to_p2trbytes,
            height,
        )
        .unwrap();

        let p2wpkhaddressindex = starting_index(
            &vecs.height_to_first_p2wpkhaddressindex,
            &vecs.p2wpkhaddressindex_to_p2wpkhbytes,
            height,
        )
        .unwrap();

        let p2wshaddressindex = starting_index(
            &vecs.height_to_first_p2wshaddressindex,
            &vecs.p2wshaddressindex_to_p2wshbytes,
            height,
        )
        .unwrap();

        let p2aaddressindex = starting_index(
            &vecs.height_to_first_p2aaddressindex,
            &vecs.p2aaddressindex_to_p2abytes,
            height,
        )
        .unwrap();

        let txindex =
            starting_index(&vecs.height_to_first_txindex, &vecs.txindex_to_txid, height).unwrap();

        let txinindex = starting_index(
            &vecs.height_to_first_txinindex,
            &vecs.txinindex_to_outpoint,
            height,
        )
        .unwrap();

        let txoutindex = starting_index(
            &vecs.height_to_first_txoutindex,
            &vecs.txoutindex_to_value,
            height,
        )
        .unwrap();

        let unknownoutputindex = starting_index(
            &vecs.height_to_first_unknownoutputindex,
            &vecs.unknownoutputindex_to_txindex,
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
    height_to_index: &impl AnyStoredIterableVec<Height, I>,
    index_to_else: &impl AnyIterableVec<I, T>,
    starting_height: Height,
) -> Option<I>
where
    I: StoredRaw + StoredIndex + From<usize>,
    T: StoredRaw,
{
    let h = Height::from(height_to_index.stamp());
    if h.is_zero() {
        None
    } else if h + 1_u32 == starting_height {
        Some(I::from(index_to_else.len()))
    } else {
        height_to_index.iter().get_inner(starting_height)
    }
}
