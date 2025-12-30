use brk_error::Result;
use brk_types::Height;
use log::debug;
use vecdb::{GenericStoredVec, IterableStoredVec, IterableVec, VecIndex, VecValue};

use crate::{Stores, Vecs};

pub use brk_types::Indexes;

/// Extension trait for Indexes with brk_indexer-specific functionality.
pub trait IndexesExt {
    fn checked_push(&self, vecs: &mut Vecs) -> Result<()>;
    fn from_vecs_and_stores(min_height: Height, vecs: &mut Vecs, stores: &Stores) -> Self;
}

impl IndexesExt for Indexes {
    fn checked_push(&self, vecs: &mut Vecs) -> Result<()> {
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

    fn from_vecs_and_stores(min_height: Height, vecs: &mut Vecs, stores: &Stores) -> Indexes {
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
            &vecs.txout.txoutindex_to_value,
            height,
        )
        .unwrap();

        let unknownoutputindex = starting_index(
            &vecs.output.height_to_first_unknownoutputindex,
            &vecs.output.unknownoutputindex_to_txindex,
            height,
        )
        .unwrap();

        Indexes {
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
