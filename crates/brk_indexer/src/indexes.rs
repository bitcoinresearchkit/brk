use bitcoincore_rpc::Client;
use brk_core::{
    BlockHash, CheckedSub, EmptyOutputIndex, Height, InputIndex, OpReturnIndex, OutputIndex,
    OutputType, P2AAddressIndex, P2MSOutputIndex, P2PK33AddressIndex, P2PK65AddressIndex,
    P2PKHAddressIndex, P2SHAddressIndex, P2TRAddressIndex, P2WPKHAddressIndex, P2WSHAddressIndex,
    Result, TxIndex, TypeIndex, UnknownOutputIndex,
};
use brk_parser::NUMBER_OF_UNSAFE_BLOCKS;
use brk_vecs::{AnyIterableVec, AnyStampedVec, AnyVec, StampedVec, StoredIndex, StoredType};
use color_eyre::eyre::ContextCompat;

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
    pub inputindex: InputIndex,
    pub outputindex: OutputIndex,
    pub unknownoutputindex: UnknownOutputIndex,
}

impl Indexes {
    pub fn typeindex(&self, outputtype: OutputType) -> TypeIndex {
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
        }
    }

    pub fn push_if_needed(&self, vecs: &mut Vecs) -> Result<()> {
        let height = self.height;
        vecs.height_to_first_txindex
            .push_if_needed(height, self.txindex)?;
        vecs.height_to_first_inputindex
            .push_if_needed(height, self.inputindex)?;
        vecs.height_to_first_outputindex
            .push_if_needed(height, self.outputindex)?;
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

impl TryFrom<(&mut Vecs, &Stores, &Client)> for Indexes {
    type Error = color_eyre::Report;
    fn try_from((vecs, stores, rpc): (&mut Vecs, &Stores, &Client)) -> color_eyre::Result<Self> {
        // Height at which we want to start: min last saved + 1 or 0
        let vecs_starting_height = vecs.starting_height();
        let stores_starting_height = stores.starting_height();
        let starting_height = vecs_starting_height.min(stores_starting_height);

        let range = u32::from(
            starting_height
                .checked_sub(NUMBER_OF_UNSAFE_BLOCKS as u32)
                .unwrap_or_default(),
        )..u32::from(starting_height);

        // But we also need to check the chain and start earlier in case of a reorg
        let height = range // ..= because of last saved + 1
            .map(Height::from)
            .find(|height| {
                let rpc_blockhash = BlockHash::try_from((rpc, *height))
                    .inspect_err(|e| {
                        dbg!(e, height);
                    })
                    .unwrap();

                vecs.height_to_blockhash
                    .iter()
                    .get(*height)
                    .is_none_or(|saved_blockhash| &rpc_blockhash != saved_blockhash.as_ref())
            })
            .unwrap_or(starting_height);

        Ok(Self {
            emptyoutputindex: starting_index(
                &vecs.height_to_first_emptyoutputindex,
                &vecs.emptyoutputindex_to_txindex,
                height,
            )
            .context("")?,
            height,
            p2msoutputindex: starting_index(
                &vecs.height_to_first_p2msoutputindex,
                &vecs.p2msoutputindex_to_txindex,
                height,
            )
            .context("")?,
            opreturnindex: starting_index(
                &vecs.height_to_first_opreturnindex,
                &vecs.opreturnindex_to_txindex,
                height,
            )
            .context("")?,
            p2pk33addressindex: starting_index(
                &vecs.height_to_first_p2pk33addressindex,
                &vecs.p2pk33addressindex_to_p2pk33bytes,
                height,
            )
            .context("")?,
            p2pk65addressindex: starting_index(
                &vecs.height_to_first_p2pk65addressindex,
                &vecs.p2pk65addressindex_to_p2pk65bytes,
                height,
            )
            .context("")?,
            p2pkhaddressindex: starting_index(
                &vecs.height_to_first_p2pkhaddressindex,
                &vecs.p2pkhaddressindex_to_p2pkhbytes,
                height,
            )
            .context("")?,
            p2shaddressindex: starting_index(
                &vecs.height_to_first_p2shaddressindex,
                &vecs.p2shaddressindex_to_p2shbytes,
                height,
            )
            .context("")?,
            p2traddressindex: starting_index(
                &vecs.height_to_first_p2traddressindex,
                &vecs.p2traddressindex_to_p2trbytes,
                height,
            )
            .context("")?,
            p2wpkhaddressindex: starting_index(
                &vecs.height_to_first_p2wpkhaddressindex,
                &vecs.p2wpkhaddressindex_to_p2wpkhbytes,
                height,
            )
            .context("")?,
            p2wshaddressindex: starting_index(
                &vecs.height_to_first_p2wshaddressindex,
                &vecs.p2wshaddressindex_to_p2wshbytes,
                height,
            )
            .context("")?,
            p2aaddressindex: starting_index(
                &vecs.height_to_first_p2aaddressindex,
                &vecs.p2aaddressindex_to_p2abytes,
                height,
            )
            .context("")?,
            txindex: starting_index(&vecs.height_to_first_txindex, &vecs.txindex_to_txid, height)
                .context("")?,
            inputindex: starting_index(
                &vecs.height_to_first_inputindex,
                &vecs.inputindex_to_outputindex,
                height,
            )
            .context("")?,
            outputindex: starting_index(
                &vecs.height_to_first_outputindex,
                &vecs.outputindex_to_value,
                height,
            )
            .context("")?,
            unknownoutputindex: starting_index(
                &vecs.height_to_first_unknownoutputindex,
                &vecs.unknownoutputindex_to_txindex,
                height,
            )
            .context("")?,
        })
    }
}

pub fn starting_index<I, T>(
    height_to_index: &StampedVec<Height, I>,
    index_to_else: &StampedVec<I, T>,
    starting_height: Height,
) -> Option<I>
where
    I: StoredType + StoredIndex + From<usize>,
    T: StoredType,
{
    let h = Height::from(u64::from(height_to_index.stamp()));
    if h.is_zero() {
        None
    } else if h + 1_u32 == starting_height {
        Some(I::from(index_to_else.len()))
    } else {
        height_to_index.iter().get_inner(starting_height)
    }
}
