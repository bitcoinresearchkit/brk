use brk_error::Result;
use brk_types::{Height, Indexes};
use tracing::{debug, info};
use vecdb::{AnyStoredVec, PcoVec, PcoVecValue, ReadableVec, VecIndex, VecValue, WritableVec};

use crate::{Stores, Vecs};

/// Extension trait for Indexes with brk_indexer-specific functionality.
pub trait IndexesExt {
    fn checked_push(&self, vecs: &mut Vecs) -> Result<()>;
    fn from_vecs_and_stores(
        required_height: Height,
        vecs: &mut Vecs,
        stores: &Stores,
    ) -> Option<Self>
    where
        Self: Sized;
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
            .empty.first_index
            .checked_push(height, self.emptyoutputindex)?;
        vecs.scripts
            .p2ms.first_index
            .checked_push(height, self.p2msoutputindex)?;
        vecs.scripts
            .opreturn.first_index
            .checked_push(height, self.opreturnindex)?;
        vecs.addresses
            .p2a.first_index
            .checked_push(height, self.p2aaddressindex)?;
        vecs.scripts
            .unknown.first_index
            .checked_push(height, self.unknownoutputindex)?;
        vecs.addresses
            .p2pk33.first_index
            .checked_push(height, self.p2pk33addressindex)?;
        vecs.addresses
            .p2pk65.first_index
            .checked_push(height, self.p2pk65addressindex)?;
        vecs.addresses
            .p2pkh.first_index
            .checked_push(height, self.p2pkhaddressindex)?;
        vecs.addresses
            .p2sh.first_index
            .checked_push(height, self.p2shaddressindex)?;
        vecs.addresses
            .p2tr.first_index
            .checked_push(height, self.p2traddressindex)?;
        vecs.addresses
            .p2wpkh.first_index
            .checked_push(height, self.p2wpkhaddressindex)?;
        vecs.addresses
            .p2wsh.first_index
            .checked_push(height, self.p2wshaddressindex)?;

        Ok(())
    }

    fn from_vecs_and_stores(
        required_height: Height,
        vecs: &mut Vecs,
        stores: &Stores,
    ) -> Option<Indexes> {
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
            &vecs.scripts.empty.first_index,
            &vecs.scripts.empty.to_txindex,
            starting_height,
        )?;

        let p2msoutputindex = starting_index(
            &vecs.scripts.p2ms.first_index,
            &vecs.scripts.p2ms.to_txindex,
            starting_height,
        )?;

        let opreturnindex = starting_index(
            &vecs.scripts.opreturn.first_index,
            &vecs.scripts.opreturn.to_txindex,
            starting_height,
        )?;

        let p2pk33addressindex = starting_index(
            &vecs.addresses.p2pk33.first_index,
            &vecs.addresses.p2pk33.bytes,
            starting_height,
        )?;

        let p2pk65addressindex = starting_index(
            &vecs.addresses.p2pk65.first_index,
            &vecs.addresses.p2pk65.bytes,
            starting_height,
        )?;

        let p2pkhaddressindex = starting_index(
            &vecs.addresses.p2pkh.first_index,
            &vecs.addresses.p2pkh.bytes,
            starting_height,
        )?;

        let p2shaddressindex = starting_index(
            &vecs.addresses.p2sh.first_index,
            &vecs.addresses.p2sh.bytes,
            starting_height,
        )?;

        let p2traddressindex = starting_index(
            &vecs.addresses.p2tr.first_index,
            &vecs.addresses.p2tr.bytes,
            starting_height,
        )?;

        let p2wpkhaddressindex = starting_index(
            &vecs.addresses.p2wpkh.first_index,
            &vecs.addresses.p2wpkh.bytes,
            starting_height,
        )?;

        let p2wshaddressindex = starting_index(
            &vecs.addresses.p2wsh.first_index,
            &vecs.addresses.p2wsh.bytes,
            starting_height,
        )?;

        let p2aaddressindex = starting_index(
            &vecs.addresses.p2a.first_index,
            &vecs.addresses.p2a.bytes,
            starting_height,
        )?;

        let txindex = starting_index(
            &vecs.transactions.first_txindex,
            &vecs.transactions.txid,
            starting_height,
        )?;

        let txinindex = starting_index(
            &vecs.inputs.first_txinindex,
            &vecs.inputs.outpoint,
            starting_height,
        )?;

        let txoutindex = starting_index(
            &vecs.outputs.first_txoutindex,
            &vecs.outputs.value,
            starting_height,
        )?;

        let unknownoutputindex = starting_index(
            &vecs.scripts.unknown.first_index,
            &vecs.scripts.unknown.to_txindex,
            starting_height,
        )?;

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
    height_to_index: &PcoVec<Height, I>,
    index_to_else: &impl ReadableVec<I, T>,
    starting_height: Height,
) -> Option<I>
where
    I: VecIndex + PcoVecValue + From<usize>,
    T: VecValue,
{
    let h = Height::from(height_to_index.stamp());
    if h.is_zero() {
        None
    } else if h + 1_u32 == starting_height {
        Some(I::from(index_to_else.len()))
    } else {
        height_to_index.collect_one(starting_height)
    }
}
