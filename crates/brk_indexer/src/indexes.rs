use brk_error::Result;
use brk_types::Height;
use tracing::{debug, info};
use vecdb::{GenericStoredVec, IterableStoredVec, IterableVec, VecIndex, VecValue};

use crate::{Stores, Vecs};

pub use brk_types::Indexes;

/// Extension trait for Indexes with brk_indexer-specific functionality.
pub trait IndexesExt {
    fn checked_push(&self, vecs: &mut Vecs) -> Result<()>;
    fn from_vecs_and_stores(required_height: Height, vecs: &mut Vecs, stores: &Stores) -> Option<Self> where Self: Sized;
}

impl IndexesExt for Indexes {
    fn checked_push(&self, vecs: &mut Vecs) -> Result<()> {
        let height = self.height;
        vecs.transactions
            .first_txindex
            .checked_push(height, self.txindex)?;
        vecs.inputs
            .first_txinindex
            .checked_push(height, self.txinindex)?;
        vecs.outputs
            .first_txoutindex
            .checked_push(height, self.txoutindex)?;
        vecs.scripts
            .first_emptyoutputindex
            .checked_push(height, self.emptyoutputindex)?;
        vecs.scripts
            .first_p2msoutputindex
            .checked_push(height, self.p2msoutputindex)?;
        vecs.scripts
            .first_opreturnindex
            .checked_push(height, self.opreturnindex)?;
        vecs.addresses
            .first_p2aaddressindex
            .checked_push(height, self.p2aaddressindex)?;
        vecs.scripts
            .first_unknownoutputindex
            .checked_push(height, self.unknownoutputindex)?;
        vecs.addresses
            .first_p2pk33addressindex
            .checked_push(height, self.p2pk33addressindex)?;
        vecs.addresses
            .first_p2pk65addressindex
            .checked_push(height, self.p2pk65addressindex)?;
        vecs.addresses
            .first_p2pkhaddressindex
            .checked_push(height, self.p2pkhaddressindex)?;
        vecs.addresses
            .first_p2shaddressindex
            .checked_push(height, self.p2shaddressindex)?;
        vecs.addresses
            .first_p2traddressindex
            .checked_push(height, self.p2traddressindex)?;
        vecs.addresses
            .first_p2wpkhaddressindex
            .checked_push(height, self.p2wpkhaddressindex)?;
        vecs.addresses
            .first_p2wshaddressindex
            .checked_push(height, self.p2wshaddressindex)?;

        Ok(())
    }

    fn from_vecs_and_stores(required_height: Height, vecs: &mut Vecs, stores: &Stores) -> Option<Indexes> {
        debug!("Creating indexes from vecs and stores...");

        // Local data height: minimum of vecs and stores
        let vecs_height = vecs.starting_height();
        let stores_height = stores.starting_height();
        let local_height = vecs_height.min(stores_height);

        // Data inconsistency: local data behind required height
        if local_height < required_height {
            return None;
        }

        // Handle reorg: local data ahead of required height
        let starting_height = if local_height > required_height {
            info!(
                "Reorg detected: rolling back from {} to {}",
                local_height, required_height
            );
            required_height
        } else {
            local_height
        };

        let emptyoutputindex = starting_index(
            &vecs.scripts.first_emptyoutputindex,
            &vecs.scripts.empty_to_txindex,
            starting_height,
        )
        .unwrap();

        let p2msoutputindex = starting_index(
            &vecs.scripts.first_p2msoutputindex,
            &vecs.scripts.p2ms_to_txindex,
            starting_height,
        )
        .unwrap();

        let opreturnindex = starting_index(
            &vecs.scripts.first_opreturnindex,
            &vecs.scripts.opreturn_to_txindex,
            starting_height,
        )
        .unwrap();

        let p2pk33addressindex = starting_index(
            &vecs.addresses.first_p2pk33addressindex,
            &vecs.addresses.p2pk33bytes,
            starting_height,
        )
        .unwrap();

        let p2pk65addressindex = starting_index(
            &vecs.addresses.first_p2pk65addressindex,
            &vecs.addresses.p2pk65bytes,
            starting_height,
        )
        .unwrap();

        let p2pkhaddressindex = starting_index(
            &vecs.addresses.first_p2pkhaddressindex,
            &vecs.addresses.p2pkhbytes,
            starting_height,
        )
        .unwrap();

        let p2shaddressindex = starting_index(
            &vecs.addresses.first_p2shaddressindex,
            &vecs.addresses.p2shbytes,
            starting_height,
        )
        .unwrap();

        let p2traddressindex = starting_index(
            &vecs.addresses.first_p2traddressindex,
            &vecs.addresses.p2trbytes,
            starting_height,
        )
        .unwrap();

        let p2wpkhaddressindex = starting_index(
            &vecs.addresses.first_p2wpkhaddressindex,
            &vecs.addresses.p2wpkhbytes,
            starting_height,
        )
        .unwrap();

        let p2wshaddressindex = starting_index(
            &vecs.addresses.first_p2wshaddressindex,
            &vecs.addresses.p2wshbytes,
            starting_height,
        )
        .unwrap();

        let p2aaddressindex = starting_index(
            &vecs.addresses.first_p2aaddressindex,
            &vecs.addresses.p2abytes,
            starting_height,
        )
        .unwrap();

        let txindex = starting_index(
            &vecs.transactions.first_txindex,
            &vecs.transactions.txid,
            starting_height,
        )
        .unwrap();

        let txinindex =
            starting_index(&vecs.inputs.first_txinindex, &vecs.inputs.outpoint, starting_height).unwrap();

        let txoutindex =
            starting_index(&vecs.outputs.first_txoutindex, &vecs.outputs.value, starting_height).unwrap();

        let unknownoutputindex = starting_index(
            &vecs.scripts.first_unknownoutputindex,
            &vecs.scripts.unknown_to_txindex,
            starting_height,
        )
        .unwrap();

        Some(Indexes {
            emptyoutputindex,
            height: starting_height,
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
        })
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
