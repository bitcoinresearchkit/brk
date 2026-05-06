use brk_error::Result;
use brk_types::{
    EmptyOutputIndex, Height, OpReturnIndex, OutputType, P2AAddrIndex, P2MSOutputIndex,
    P2PK33AddrIndex, P2PK65AddrIndex, P2PKHAddrIndex, P2SHAddrIndex, P2TRAddrIndex,
    P2WPKHAddrIndex, P2WSHAddrIndex, TxInIndex, TxIndex, TxOutIndex, TypeIndex, UnknownOutputIndex,
};
use tracing::info;
use vecdb::{AnyStoredVec, PcoVec, PcoVecValue, ReadableVec, VecIndex, VecValue, WritableVec};

use crate::{Stores, Vecs};

/// Pipeline-wide length/count snapshot. Lengths semantics:
/// `bound.f = N` means positions `0..N` are fully written; readers
/// reject `pos >= bound.f`.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Lengths {
    pub empty_output_index: EmptyOutputIndex,
    pub height: Height,
    pub op_return_index: OpReturnIndex,
    pub p2ms_output_index: P2MSOutputIndex,
    pub p2pk33_addr_index: P2PK33AddrIndex,
    pub p2pk65_addr_index: P2PK65AddrIndex,
    pub p2pkh_addr_index: P2PKHAddrIndex,
    pub p2sh_addr_index: P2SHAddrIndex,
    pub p2tr_addr_index: P2TRAddrIndex,
    pub p2wpkh_addr_index: P2WPKHAddrIndex,
    pub p2wsh_addr_index: P2WSHAddrIndex,
    pub p2a_addr_index: P2AAddrIndex,
    pub tx_index: TxIndex,
    pub txin_index: TxInIndex,
    pub txout_index: TxOutIndex,
    pub unknown_output_index: UnknownOutputIndex,
}

impl Lengths {
    pub fn to_type_index(&self, output_type: OutputType) -> TypeIndex {
        match output_type {
            OutputType::Empty => *self.empty_output_index,
            OutputType::OpReturn => *self.op_return_index,
            OutputType::P2A => *self.p2a_addr_index,
            OutputType::P2MS => *self.p2ms_output_index,
            OutputType::P2PK33 => *self.p2pk33_addr_index,
            OutputType::P2PK65 => *self.p2pk65_addr_index,
            OutputType::P2PKH => *self.p2pkh_addr_index,
            OutputType::P2SH => *self.p2sh_addr_index,
            OutputType::P2TR => *self.p2tr_addr_index,
            OutputType::P2WPKH => *self.p2wpkh_addr_index,
            OutputType::P2WSH => *self.p2wsh_addr_index,
            OutputType::Unknown => *self.unknown_output_index,
        }
    }

    /// Bump per-block totals after processing a block.
    pub fn add_block(&mut self, tx_count: usize, input_count: usize, output_count: usize) {
        self.tx_index += TxIndex::from(tx_count);
        self.txin_index += TxInIndex::from(input_count);
        self.txout_index += TxOutIndex::from(output_count);
    }

    /// Increments the address index for the given address type and returns the previous value.
    /// Only call this for address types (P2PK65, P2PK33, P2PKH, P2SH, P2WPKH, P2WSH, P2TR, P2A).
    #[inline]
    pub fn increment_addr_index(&mut self, addr_type: OutputType) -> TypeIndex {
        match addr_type {
            OutputType::P2PK65 => self.p2pk65_addr_index.copy_then_increment(),
            OutputType::P2PK33 => self.p2pk33_addr_index.copy_then_increment(),
            OutputType::P2PKH => self.p2pkh_addr_index.copy_then_increment(),
            OutputType::P2SH => self.p2sh_addr_index.copy_then_increment(),
            OutputType::P2WPKH => self.p2wpkh_addr_index.copy_then_increment(),
            OutputType::P2WSH => self.p2wsh_addr_index.copy_then_increment(),
            OutputType::P2TR => self.p2tr_addr_index.copy_then_increment(),
            OutputType::P2A => self.p2a_addr_index.copy_then_increment(),
            _ => unreachable!(),
        }
    }

    pub fn checked_push(&self, vecs: &mut Vecs) -> Result<()> {
        let height = self.height;
        vecs.transactions
            .first_tx_index
            .checked_push(height, self.tx_index)?;
        vecs.inputs
            .first_txin_index
            .checked_push(height, self.txin_index)?;
        vecs.outputs
            .first_txout_index
            .checked_push(height, self.txout_index)?;
        vecs.scripts
            .empty
            .first_index
            .checked_push(height, self.empty_output_index)?;
        vecs.scripts
            .p2ms
            .first_index
            .checked_push(height, self.p2ms_output_index)?;
        vecs.scripts
            .op_return
            .first_index
            .checked_push(height, self.op_return_index)?;
        vecs.addrs
            .p2a
            .first_index
            .checked_push(height, self.p2a_addr_index)?;
        vecs.scripts
            .unknown
            .first_index
            .checked_push(height, self.unknown_output_index)?;
        vecs.addrs
            .p2pk33
            .first_index
            .checked_push(height, self.p2pk33_addr_index)?;
        vecs.addrs
            .p2pk65
            .first_index
            .checked_push(height, self.p2pk65_addr_index)?;
        vecs.addrs
            .p2pkh
            .first_index
            .checked_push(height, self.p2pkh_addr_index)?;
        vecs.addrs
            .p2sh
            .first_index
            .checked_push(height, self.p2sh_addr_index)?;
        vecs.addrs
            .p2tr
            .first_index
            .checked_push(height, self.p2tr_addr_index)?;
        vecs.addrs
            .p2wpkh
            .first_index
            .checked_push(height, self.p2wpkh_addr_index)?;
        vecs.addrs
            .p2wsh
            .first_index
            .checked_push(height, self.p2wsh_addr_index)?;

        Ok(())
    }

    /// Read current local lengths. `None` pre-genesis.
    pub fn from_local(vecs: &mut Vecs, stores: &Stores) -> Option<Self> {
        let height = vecs.next_height().min(stores.next_height());
        Self::collect_at(height, vecs)
    }

    /// Read lengths to resume at `required_height`. Reorg-aware:
    /// - if local is ahead, clamp down to `required_height`;
    /// - if local is behind, return `None` (caller must full-reset).
    pub fn resume_at(required_height: Height, vecs: &mut Vecs, stores: &Stores) -> Option<Self> {
        let local = vecs.next_height().min(stores.next_height());
        if local < required_height {
            return None;
        }
        let height = if local > required_height {
            info!(
                "Reorg detected: rolling back from {} to {}",
                local, required_height
            );
            required_height
        } else {
            local
        };
        Self::collect_at(height, vecs)
    }

    fn collect_at(height: Height, vecs: &mut Vecs) -> Option<Self> {
        Some(Self {
            empty_output_index: next_index(
                &vecs.scripts.empty.first_index,
                &vecs.scripts.empty.to_tx_index,
                height,
            )?,
            height,
            p2ms_output_index: next_index(
                &vecs.scripts.p2ms.first_index,
                &vecs.scripts.p2ms.to_tx_index,
                height,
            )?,
            op_return_index: next_index(
                &vecs.scripts.op_return.first_index,
                &vecs.scripts.op_return.to_tx_index,
                height,
            )?,
            p2pk33_addr_index: next_index(
                &vecs.addrs.p2pk33.first_index,
                &vecs.addrs.p2pk33.bytes,
                height,
            )?,
            p2pk65_addr_index: next_index(
                &vecs.addrs.p2pk65.first_index,
                &vecs.addrs.p2pk65.bytes,
                height,
            )?,
            p2pkh_addr_index: next_index(
                &vecs.addrs.p2pkh.first_index,
                &vecs.addrs.p2pkh.bytes,
                height,
            )?,
            p2sh_addr_index: next_index(
                &vecs.addrs.p2sh.first_index,
                &vecs.addrs.p2sh.bytes,
                height,
            )?,
            p2tr_addr_index: next_index(
                &vecs.addrs.p2tr.first_index,
                &vecs.addrs.p2tr.bytes,
                height,
            )?,
            p2wpkh_addr_index: next_index(
                &vecs.addrs.p2wpkh.first_index,
                &vecs.addrs.p2wpkh.bytes,
                height,
            )?,
            p2wsh_addr_index: next_index(
                &vecs.addrs.p2wsh.first_index,
                &vecs.addrs.p2wsh.bytes,
                height,
            )?,
            p2a_addr_index: next_index(&vecs.addrs.p2a.first_index, &vecs.addrs.p2a.bytes, height)?,
            tx_index: next_index(
                &vecs.transactions.first_tx_index,
                &vecs.transactions.txid,
                height,
            )?,
            txin_index: next_index(&vecs.inputs.first_txin_index, &vecs.inputs.outpoint, height)?,
            txout_index: next_index(&vecs.outputs.first_txout_index, &vecs.outputs.value, height)?,
            unknown_output_index: next_index(
                &vecs.scripts.unknown.first_index,
                &vecs.scripts.unknown.to_tx_index,
                height,
            )?,
        })
    }
}

/// Per-type next-to-write counter at `next_height`. `None` pre-genesis.
fn next_index<I, T>(
    height_to_index: &PcoVec<Height, I>,
    index_to_else: &impl ReadableVec<I, T>,
    next_height: Height,
) -> Option<I>
where
    I: VecIndex + PcoVecValue + From<usize>,
    T: VecValue,
{
    let h = Height::from(height_to_index.stamp());
    if h.is_zero() {
        None
    } else if h + 1_u32 == next_height {
        Some(I::from(index_to_else.len()))
    } else {
        height_to_index.collect_one(next_height)
    }
}
