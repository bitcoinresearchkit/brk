use bitcoincore_rpc::Client;
use brk_core::{
    Addressindex, BlockHash, CheckedSub, Emptyindex, Height, Multisigindex, Opreturnindex,
    P2PK33index, P2PK65index, P2PKHindex, P2SHindex, P2TRindex, P2WPKHindex, P2WSHindex,
    Pushonlyindex, Txindex, Txinindex, Txoutindex, Unknownindex,
};
use brk_parser::NUMBER_OF_UNSAFE_BLOCKS;
use brk_vec::{Result, StoredIndex, StoredType, Value};
use color_eyre::eyre::ContextCompat;

use crate::{IndexedVec, Stores, Vecs};

#[derive(Debug, Default, Clone)]
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
    pub fn push_if_needed(&self, vecs: &mut Vecs) -> brk_vec::Result<()> {
        let height = self.height;
        vecs.height_to_first_txindex
            .push_if_needed(height, self.txindex)?;
        vecs.height_to_first_txinindex
            .push_if_needed(height, self.txinindex)?;
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

                vecs.height_to_blockhash.get(*height).map_or(true, |opt| {
                    opt.is_none_or(|saved_blockhash| {
                        let b = &rpc_blockhash != saved_blockhash.as_ref();
                        if b {
                            dbg!(rpc_blockhash, saved_blockhash.as_ref());
                        }
                        b
                    })
                })
            })
            .unwrap_or(starting_height);

        Ok(Self {
            addressindex: *starting_index(
                &vecs.height_to_first_addressindex,
                &vecs.addressindex_to_height,
                height,
            )?
            .context("")?,
            emptyindex: *starting_index(
                &vecs.height_to_first_emptyindex,
                &vecs.emptyindex_to_height,
                height,
            )?
            .context("")?,
            height,
            multisigindex: *starting_index(
                &vecs.height_to_first_multisigindex,
                &vecs.multisigindex_to_height,
                height,
            )?
            .context("")?,
            opreturnindex: *starting_index(
                &vecs.height_to_first_opreturnindex,
                &vecs.opreturnindex_to_height,
                height,
            )?
            .context("")?,
            p2pk33index: *starting_index(
                &vecs.height_to_first_p2pk33index,
                &vecs.p2pk33index_to_height,
                height,
            )?
            .context("")?,
            p2pk65index: *starting_index(
                &vecs.height_to_first_p2pk65index,
                &vecs.p2pk65index_to_height,
                height,
            )?
            .context("")?,
            p2pkhindex: *starting_index(
                &vecs.height_to_first_p2pkhindex,
                &vecs.p2pkhindex_to_height,
                height,
            )?
            .context("")?,
            p2shindex: *starting_index(
                &vecs.height_to_first_p2shindex,
                &vecs.p2shindex_to_height,
                height,
            )?
            .context("")?,
            p2trindex: *starting_index(
                &vecs.height_to_first_p2trindex,
                &vecs.p2trindex_to_height,
                height,
            )?
            .context("")?,
            p2wpkhindex: *starting_index(
                &vecs.height_to_first_p2wpkhindex,
                &vecs.p2wpkhindex_to_height,
                height,
            )?
            .context("")?,
            p2wshindex: *starting_index(
                &vecs.height_to_first_p2wshindex,
                &vecs.p2wshindex_to_height,
                height,
            )?
            .context("")?,
            pushonlyindex: *starting_index(
                &vecs.height_to_first_pushonlyindex,
                &vecs.pushonlyindex_to_height,
                height,
            )?
            .context("")?,
            txindex: *starting_index(
                &vecs.height_to_first_txindex,
                &vecs.txindex_to_height,
                height,
            )?
            .context("")?,
            txinindex: *starting_index(
                &vecs.height_to_first_txinindex,
                &vecs.txinindex_to_height,
                height,
            )?
            .context("")?,
            txoutindex: *starting_index(
                &vecs.height_to_first_txoutindex,
                &vecs.txoutindex_to_height,
                height,
            )?
            .context("")?,
            unknownindex: *starting_index(
                &vecs.height_to_first_unknownindex,
                &vecs.unknownindex_to_height,
                height,
            )?
            .context("")?,
        })
    }
}

pub fn starting_index<'a, I, T>(
    height_to_index: &'a IndexedVec<Height, I>,
    index_to_else: &'a IndexedVec<I, T>,
    starting_height: Height,
) -> Result<Option<Value<'a, I>>>
where
    I: StoredType + StoredIndex + From<usize>,
    T: StoredType,
{
    if height_to_index
        .height()
        .is_ok_and(|h| h + 1_u32 == starting_height)
    {
        Ok(Some(Value::Owned(I::from(index_to_else.len()))))
    } else {
        height_to_index.get(starting_height)
    }
}
