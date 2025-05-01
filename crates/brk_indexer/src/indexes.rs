use bitcoincore_rpc::Client;
use brk_core::{
    BlockHash, CheckedSub, EmptyOutputIndex, Height, InputIndex, OpReturnIndex, OutputIndex,
    OutputType, OutputTypeIndex, P2AIndex, P2MSIndex, P2PK33Index, P2PK65Index, P2PKHIndex,
    P2SHIndex, P2TRIndex, P2WPKHIndex, P2WSHIndex, TxIndex, UnknownOutputIndex,
};
use brk_parser::NUMBER_OF_UNSAFE_BLOCKS;
use brk_vec::{StoredIndex, StoredType, VecIterator};
use color_eyre::eyre::ContextCompat;

use crate::{IndexedVec, Stores, Vecs};

#[derive(Debug, Default, Clone)]
pub struct Indexes {
    pub emptyoutputindex: EmptyOutputIndex,
    pub height: Height,
    pub opreturnindex: OpReturnIndex,
    pub p2msindex: P2MSIndex,
    pub p2pk33index: P2PK33Index,
    pub p2pk65index: P2PK65Index,
    pub p2pkhindex: P2PKHIndex,
    pub p2shindex: P2SHIndex,
    pub p2trindex: P2TRIndex,
    pub p2wpkhindex: P2WPKHIndex,
    pub p2wshindex: P2WSHIndex,
    pub p2aindex: P2AIndex,
    pub txindex: TxIndex,
    pub inputindex: InputIndex,
    pub outputindex: OutputIndex,
    pub unknownoutputindex: UnknownOutputIndex,
}

impl Indexes {
    pub fn outputtypeindex(&self, outputtype: OutputType) -> OutputTypeIndex {
        match outputtype {
            OutputType::Empty => *self.emptyoutputindex,
            OutputType::OpReturn => *self.opreturnindex,
            OutputType::P2A => *self.p2aindex,
            OutputType::P2MS => *self.p2msindex,
            OutputType::P2PK33 => *self.p2pkhindex,
            OutputType::P2PK65 => *self.p2pk65index,
            OutputType::P2PKH => *self.p2pkhindex,
            OutputType::P2SH => *self.p2shindex,
            OutputType::P2TR => *self.p2trindex,
            OutputType::P2WPKH => *self.p2wpkhindex,
            OutputType::P2WSH => *self.p2wshindex,
            OutputType::Unknown => *self.unknownoutputindex,
        }
    }

    pub fn push_if_needed(&self, vecs: &mut Vecs) -> brk_vec::Result<()> {
        let height = self.height;
        vecs.height_to_first_txindex
            .push_if_needed(height, self.txindex)?;
        vecs.height_to_first_inputindex
            .push_if_needed(height, self.inputindex)?;
        vecs.height_to_first_outputindex
            .push_if_needed(height, self.outputindex)?;
        vecs.height_to_first_emptyoutputindex
            .push_if_needed(height, self.emptyoutputindex)?;
        vecs.height_to_first_p2msindex
            .push_if_needed(height, self.p2msindex)?;
        vecs.height_to_first_opreturnindex
            .push_if_needed(height, self.opreturnindex)?;
        vecs.height_to_first_p2aindex
            .push_if_needed(height, self.p2aindex)?;
        vecs.height_to_first_unknownoutputindex
            .push_if_needed(height, self.unknownoutputindex)?;
        vecs.height_to_first_p2pk33index
            .push_if_needed(height, self.p2pk33index)?;
        vecs.height_to_first_p2pk65index
            .push_if_needed(height, self.p2pk65index)?;
        vecs.height_to_first_p2pkhindex
            .push_if_needed(height, self.p2pkhindex)?;
        vecs.height_to_first_p2shindex
            .push_if_needed(height, self.p2shindex)?;
        vecs.height_to_first_p2trindex
            .push_if_needed(height, self.p2trindex)?;
        vecs.height_to_first_p2wpkhindex
            .push_if_needed(height, self.p2wpkhindex)?;
        vecs.height_to_first_p2wshindex
            .push_if_needed(height, self.p2wshindex)?;

        Ok(())
    }
}

impl TryFrom<(&mut Vecs, &Stores, &Client)> for Indexes {
    type Error = color_eyre::Report;
    fn try_from((vecs, stores, rpc): (&mut Vecs, &Stores, &Client)) -> color_eyre::Result<Self> {
        // Height at which we wanna start: min last saved + 1 or 0
        let starting_height = vecs.starting_height().min(stores.starting_height());

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
                    .is_none_or(|saved_blockhash| {
                        let b = &rpc_blockhash != saved_blockhash.as_ref();
                        if b {
                            dbg!(rpc_blockhash, saved_blockhash.as_ref());
                        }
                        b
                    })
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
            p2msindex: starting_index(
                &vecs.height_to_first_p2msindex,
                &vecs.p2msindex_to_txindex,
                height,
            )
            .context("")?,
            opreturnindex: starting_index(
                &vecs.height_to_first_opreturnindex,
                &vecs.opreturnindex_to_txindex,
                height,
            )
            .context("")?,
            p2pk33index: starting_index(
                &vecs.height_to_first_p2pk33index,
                &vecs.p2pk33index_to_p2pk33bytes,
                height,
            )
            .context("")?,
            p2pk65index: starting_index(
                &vecs.height_to_first_p2pk65index,
                &vecs.p2pk65index_to_p2pk65bytes,
                height,
            )
            .context("")?,
            p2pkhindex: starting_index(
                &vecs.height_to_first_p2pkhindex,
                &vecs.p2pkhindex_to_p2pkhbytes,
                height,
            )
            .context("")?,
            p2shindex: starting_index(
                &vecs.height_to_first_p2shindex,
                &vecs.p2shindex_to_p2shbytes,
                height,
            )
            .context("")?,
            p2trindex: starting_index(
                &vecs.height_to_first_p2trindex,
                &vecs.p2trindex_to_p2trbytes,
                height,
            )
            .context("")?,
            p2wpkhindex: starting_index(
                &vecs.height_to_first_p2wpkhindex,
                &vecs.p2wpkhindex_to_p2wpkhbytes,
                height,
            )
            .context("")?,
            p2wshindex: starting_index(
                &vecs.height_to_first_p2wshindex,
                &vecs.p2wshindex_to_p2wshbytes,
                height,
            )
            .context("")?,
            p2aindex: starting_index(
                &vecs.height_to_first_p2aindex,
                &vecs.p2aindex_to_p2abytes,
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
    height_to_index: &IndexedVec<Height, I>,
    index_to_else: &IndexedVec<I, T>,
    starting_height: Height,
) -> Option<I>
where
    I: StoredType + StoredIndex + From<usize>,
    T: StoredType,
{
    if height_to_index
        .height()
        .is_ok_and(|h| h + 1_u32 == starting_height)
    {
        Some(I::from(index_to_else.len()))
    } else {
        height_to_index.iter().get_inner(starting_height)
    }
}
