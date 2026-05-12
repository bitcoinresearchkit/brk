use brk_oracle::default_eligible_bin;
use brk_types::Transaction;
use smallvec::SmallVec;

/// Pre-bucketed oracle bins for a tx's eligible outputs. Computed once on
/// insert so `Mempool::live_histogram` can bin all live outputs without
/// re-parsing scripts or recomputing eligibility per request.
pub struct OutputBins(SmallVec<[u16; 4]>);

impl OutputBins {
    pub fn from_tx(tx: &Transaction) -> Self {
        Self(
            tx.output
                .iter()
                .filter_map(|o| default_eligible_bin(o.value, o.type_()))
                .collect(),
        )
    }

    pub fn iter(&self) -> impl Iterator<Item = u16> + '_ {
        self.0.iter().copied()
    }
}
