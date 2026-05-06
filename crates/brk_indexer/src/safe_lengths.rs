use std::sync::Arc;

use parking_lot::RwLock;

use crate::lengths::Lengths;

/// Pipeline-wide safe-read snapshot. All fields are lengths/counts
/// (next-to-write totals): `bound.f = N` means positions `0..N` are
/// fully written; readers reject `pos >= bound.f`. Covers vecs only:
/// reorg store rewrites can briefly tear in-flight reads.
#[derive(Clone, Default)]
pub struct SafeLengths(Arc<RwLock<Lengths>>);

impl SafeLengths {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(&self) -> Lengths {
        self.0.read().clone()
    }

    pub fn reset(&self) {
        *self.0.write() = Lengths::default();
    }

    pub fn advance(&self, next: Lengths) {
        let mut g = self.0.write();
        debug_assert!(
            next.height >= g.height
                && next.tx_index >= g.tx_index
                && next.txin_index >= g.txin_index
                && next.txout_index >= g.txout_index
                && next.empty_output_index >= g.empty_output_index
                && next.op_return_index >= g.op_return_index
                && next.p2ms_output_index >= g.p2ms_output_index
                && next.p2pk33_addr_index >= g.p2pk33_addr_index
                && next.p2pk65_addr_index >= g.p2pk65_addr_index
                && next.p2pkh_addr_index >= g.p2pkh_addr_index
                && next.p2sh_addr_index >= g.p2sh_addr_index
                && next.p2tr_addr_index >= g.p2tr_addr_index
                && next.p2wpkh_addr_index >= g.p2wpkh_addr_index
                && next.p2wsh_addr_index >= g.p2wsh_addr_index
                && next.p2a_addr_index >= g.p2a_addr_index
                && next.unknown_output_index >= g.unknown_output_index,
            "advance: per-field regression"
        );
        *g = next;
    }

    /// Drop each field to at most `starting`. Must be called BEFORE
    /// any rewrite at positions `>= starting`.
    pub fn lower_before(&self, starting: &Lengths) {
        let mut g = self.0.write();
        g.height = g.height.min(starting.height);
        g.tx_index = g.tx_index.min(starting.tx_index);
        g.txin_index = g.txin_index.min(starting.txin_index);
        g.txout_index = g.txout_index.min(starting.txout_index);
        g.empty_output_index = g.empty_output_index.min(starting.empty_output_index);
        g.op_return_index = g.op_return_index.min(starting.op_return_index);
        g.p2ms_output_index = g.p2ms_output_index.min(starting.p2ms_output_index);
        g.p2pk33_addr_index = g.p2pk33_addr_index.min(starting.p2pk33_addr_index);
        g.p2pk65_addr_index = g.p2pk65_addr_index.min(starting.p2pk65_addr_index);
        g.p2pkh_addr_index = g.p2pkh_addr_index.min(starting.p2pkh_addr_index);
        g.p2sh_addr_index = g.p2sh_addr_index.min(starting.p2sh_addr_index);
        g.p2tr_addr_index = g.p2tr_addr_index.min(starting.p2tr_addr_index);
        g.p2wpkh_addr_index = g.p2wpkh_addr_index.min(starting.p2wpkh_addr_index);
        g.p2wsh_addr_index = g.p2wsh_addr_index.min(starting.p2wsh_addr_index);
        g.p2a_addr_index = g.p2a_addr_index.min(starting.p2a_addr_index);
        g.unknown_output_index = g.unknown_output_index.min(starting.unknown_output_index);
    }
}

#[cfg(test)]
mod tests {
    use brk_types::{
        EmptyOutputIndex, Height, OpReturnIndex, P2AAddrIndex, P2MSOutputIndex, P2PK33AddrIndex,
        P2PK65AddrIndex, P2PKHAddrIndex, P2SHAddrIndex, P2TRAddrIndex, P2WPKHAddrIndex,
        P2WSHAddrIndex, TxInIndex, TxIndex, TxOutIndex, UnknownOutputIndex,
    };

    use super::*;

    #[test]
    fn lower_before_clamps_every_field() {
        let sentinel = u32::MAX as usize;
        let max = Lengths {
            empty_output_index: EmptyOutputIndex::from(sentinel),
            height: Height::from(sentinel),
            op_return_index: OpReturnIndex::from(sentinel),
            p2ms_output_index: P2MSOutputIndex::from(sentinel),
            p2pk33_addr_index: P2PK33AddrIndex::from(sentinel),
            p2pk65_addr_index: P2PK65AddrIndex::from(sentinel),
            p2pkh_addr_index: P2PKHAddrIndex::from(sentinel),
            p2sh_addr_index: P2SHAddrIndex::from(sentinel),
            p2tr_addr_index: P2TRAddrIndex::from(sentinel),
            p2wpkh_addr_index: P2WPKHAddrIndex::from(sentinel),
            p2wsh_addr_index: P2WSHAddrIndex::from(sentinel),
            p2a_addr_index: P2AAddrIndex::from(sentinel),
            tx_index: TxIndex::from(sentinel),
            txin_index: TxInIndex::from(sentinel),
            txout_index: TxOutIndex::from(sentinel),
            unknown_output_index: UnknownOutputIndex::from(sentinel),
        };

        let safe = SafeLengths::new();
        safe.advance(max);
        safe.lower_before(&Lengths::default());

        assert_eq!(safe.load(), Lengths::default());
    }
}
