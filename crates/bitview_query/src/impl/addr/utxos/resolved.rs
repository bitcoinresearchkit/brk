use bitview_plugin::PluginReadGuard;
use bitview_plugin_indexer::Lengths;
use brk_types::{BlockHash, TxIndex, Vout};

/// A bounded UTXO selection retaining publication exclusion until consumed.
pub struct ResolvedAddrUtxos {
    guard: PluginReadGuard,
    lengths: Lengths,
    outpoints: Vec<(TxIndex, Vout)>,
    anchor: BlockHash,
}

impl ResolvedAddrUtxos {
    pub fn block_hash(&self) -> BlockHash {
        self.anchor
    }
}
pub trait RImplAddrUtxosResolvedResolvedAddrUtxosInternal: Sized {
    fn new(
        guard: PluginReadGuard,
        lengths: Lengths,
        outpoints: Vec<(TxIndex, Vout)>,
        anchor: BlockHash,
    ) -> Self;
    fn into_parts(self) -> (PluginReadGuard, Lengths, Vec<(TxIndex, Vout)>, BlockHash);
}
impl RImplAddrUtxosResolvedResolvedAddrUtxosInternal for ResolvedAddrUtxos {
    fn new(
        guard: PluginReadGuard,
        lengths: Lengths,
        outpoints: Vec<(TxIndex, Vout)>,
        anchor: BlockHash,
    ) -> Self {
        Self {
            guard,
            lengths,
            outpoints,
            anchor,
        }
    }
    fn into_parts(self) -> (PluginReadGuard, Lengths, Vec<(TxIndex, Vout)>, BlockHash) {
        (self.guard, self.lengths, self.outpoints, self.anchor)
    }
}
