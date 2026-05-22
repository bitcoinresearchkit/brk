use brk_oracle::for_each_round_dollar_bin;
use brk_types::Transaction;
use smallvec::SmallVec;

/// Pre-bucketed oracle bins for a tx's eligible outputs. Computed once on
/// insert so `Mempool::live_histogram` can bin all live outputs without
/// re-parsing scripts or recomputing eligibility per request.
pub struct OutputBins(SmallVec<[u16; 4]>);

impl OutputBins {
    pub fn from_tx(tx: &Transaction) -> Self {
        let mut bins = SmallVec::new();
        // Live mempool txs are post-tip, always above the historical max-outputs
        // cap window, so the cap never applies here.
        for_each_round_dollar_bin(
            usize::MAX,
            tx.output.iter().map(|o| (o.value, o.type_())),
            |bin| bins.push(bin),
        );
        Self(bins)
    }

    pub fn iter(&self) -> impl Iterator<Item = u16> + '_ {
        self.0.iter().copied()
    }
}
