use brk_parser::NUMBER_OF_UNSAFE_BLOCKS;
use brk_parser::{Height, rpc::Client};
use color_eyre::eyre::ContextCompat;
use storable_vec::CACHED_GETS;

use crate::storage::{Fjalls, StorableVecs};

use super::{
    Addressindex, BlockHash, Emptyindex, Multisigindex, Opreturnindex, P2PK33index, P2PK65index, P2PKHindex, P2SHindex,
    P2TRindex, P2WPKHindex, P2WSHindex, Pushonlyindex, Txindex, Txinindex, Txoutindex, Unknownindex,
};

#[derive(Debug, Default)]
pub struct Indexes {
    pub addressindex: Addressindex,
    pub emptyindex: Emptyindex,
    pub height: Height,
    pub multisigindex: Multisigindex,
    pub opreturnindex: Opreturnindex,
    pub p2pk33index: P2PK33index,
    pub p2pk65index: P2PK65index,
    pub p2pkhindex: P2PKHindex,
    pub p2shindex: P2SHindex,
    pub p2trindex: P2TRindex,
    pub p2wpkhindex: P2WPKHindex,
    pub p2wshindex: P2WSHindex,
    pub pushonlyindex: Pushonlyindex,
    pub txindex: Txindex,
    pub txinindex: Txinindex,
    pub txoutindex: Txoutindex,
    pub unknownindex: Unknownindex,
}

impl Indexes {
    pub fn push_if_needed(&self, vecs: &mut StorableVecs<CACHED_GETS>) -> storable_vec::Result<()> {
        let height = self.height;
        vecs.height_to_first_txindex.push_if_needed(height, self.txindex)?;
        vecs.height_to_first_txinindex.push_if_needed(height, self.txinindex)?;
        vecs.height_to_first_txoutindex
            .push_if_needed(height, self.txoutindex)?;
        vecs.height_to_first_addressindex
            .push_if_needed(height, self.addressindex)?;
        vecs.height_to_first_emptyindex
            .push_if_needed(height, self.emptyindex)?;
        vecs.height_to_first_multisigindex
            .push_if_needed(height, self.multisigindex)?;
        vecs.height_to_first_opreturnindex
            .push_if_needed(height, self.opreturnindex)?;
        vecs.height_to_first_pushonlyindex
            .push_if_needed(height, self.pushonlyindex)?;
        vecs.height_to_first_unknownindex
            .push_if_needed(height, self.unknownindex)?;
        vecs.height_to_first_p2pk33index
            .push_if_needed(height, self.p2pk33index)?;
        vecs.height_to_first_p2pk65index
            .push_if_needed(height, self.p2pk65index)?;
        vecs.height_to_first_p2pkhindex
            .push_if_needed(height, self.p2pkhindex)?;
        vecs.height_to_first_p2shindex.push_if_needed(height, self.p2shindex)?;
        vecs.height_to_first_p2trindex.push_if_needed(height, self.p2trindex)?;
        vecs.height_to_first_p2wpkhindex
            .push_if_needed(height, self.p2wpkhindex)?;
        vecs.height_to_first_p2wshindex
            .push_if_needed(height, self.p2wshindex)?;
        Ok(())
    }

    pub fn push_future_if_needed(&mut self, vecs: &mut StorableVecs<CACHED_GETS>) -> storable_vec::Result<()> {
        self.height.increment();
        self.push_if_needed(vecs)?;
        self.height.decrement();
        Ok(())
    }
}

impl TryFrom<(&mut StorableVecs<CACHED_GETS>, &Fjalls, &Client)> for Indexes {
    type Error = color_eyre::Report;
    fn try_from((vecs, trees, rpc): (&mut StorableVecs<CACHED_GETS>, &Fjalls, &Client)) -> color_eyre::Result<Self> {
        // Height at which we wanna start: min last saved + 1 or 0
        let starting_height = vecs.starting_height().min(trees.starting_height());

        // But we also need to check the chain and start earlier in case of a reorg
        let height = (starting_height
            .checked_sub(NUMBER_OF_UNSAFE_BLOCKS as u32)
            .unwrap_or_default()..*starting_height) // ..= because of last saved + 1
            .map(Height::from)
            .find(|height| {
                let rpc_blockhash = BlockHash::try_from((rpc, *height)).unwrap();
                let saved_blockhash = vecs.height_to_blockhash.get(*height).unwrap().unwrap();
                &rpc_blockhash != saved_blockhash.as_ref()
            })
            .unwrap_or(starting_height);

        Ok(Self {
            addressindex: *vecs.height_to_first_addressindex.get(height)?.context("")?,
            emptyindex: *vecs.height_to_first_emptyindex.get(height)?.context("")?,
            height,
            multisigindex: *vecs.height_to_first_multisigindex.get(height)?.context("")?,
            opreturnindex: *vecs.height_to_first_opreturnindex.get(height)?.context("")?,
            p2pk33index: *vecs.height_to_first_p2pk33index.get(height)?.context("")?,
            p2pk65index: *vecs.height_to_first_p2pk65index.get(height)?.context("")?,
            p2pkhindex: *vecs.height_to_first_p2pkhindex.get(height)?.context("")?,
            p2shindex: *vecs.height_to_first_p2shindex.get(height)?.context("")?,
            p2trindex: *vecs.height_to_first_p2trindex.get(height)?.context("")?,
            p2wpkhindex: *vecs.height_to_first_p2wpkhindex.get(height)?.context("")?,
            p2wshindex: *vecs.height_to_first_p2wshindex.get(height)?.context("")?,
            pushonlyindex: *vecs.height_to_first_pushonlyindex.get(height)?.context("")?,
            txindex: *vecs.height_to_first_txindex.get(height)?.context("")?,
            txinindex: *vecs.height_to_first_txinindex.get(height)?.context("")?,
            txoutindex: *vecs.height_to_first_txoutindex.get(height)?.context("")?,
            unknownindex: *vecs.height_to_first_unknownindex.get(height)?.context("")?,
        })
    }
}
