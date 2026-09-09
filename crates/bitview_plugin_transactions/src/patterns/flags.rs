use bitview_traversable::Traversable;

#[derive(Clone, Traversable)]
pub struct Flags<V> {
    /// Whether the transaction is heuristically classified as a CoinJoin
    /// candidate: at least five inputs and outputs, neither count five times
    /// the other, sufficiently repeated input/output values, no recognized
    /// address reuse, and no detected `OP_RETURN` or inscription.
    pub is_coinjoin: V,
    /// Whether the transaction has at least five times as many inputs as
    /// outputs.
    pub is_consolidation: V,
    /// Whether the transaction is non-coinbase and has at least five times as
    /// many outputs as inputs.
    pub is_batch_payout: V,
}

impl<V> Flags<V> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut V> {
        [
            &mut self.is_coinjoin,
            &mut self.is_consolidation,
            &mut self.is_batch_payout,
        ]
        .into_iter()
    }
}
