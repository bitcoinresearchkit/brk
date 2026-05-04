use brk_types::{OutputType, SigOps, TxInIndex};
use rayon::prelude::*;
use smallvec::SmallVec;

use super::{BlockProcessor, InputSource};

impl BlockProcessor<'_> {
    /// BIP-141 sigop cost per tx in the block. Uses each input's prevout
    /// `OutputType` (already resolved by `process_inputs` for the
    /// previous-block case, looked up from `block.txdata` for the
    /// same-block case) to feed canonical-shaped synthetic prevouts into
    /// `bitcoin::Transaction::total_sigop_cost`.
    pub fn compute_sigops(&self, txins: &[(TxInIndex, InputSource)]) -> Vec<SigOps> {
        let txdata = &self.block.txdata;
        let base_tx_index = u32::from(self.indexes.tx_index);

        let mut tx_input_offsets = Vec::with_capacity(txdata.len());
        let mut offset = 0usize;
        for tx in txdata {
            tx_input_offsets.push(offset);
            offset += tx.input.len();
        }

        txdata
            .par_iter()
            .enumerate()
            .map(|(i, tx)| {
                if tx.is_coinbase() {
                    return SigOps::ZERO;
                }
                let start = tx_input_offsets[i];
                let tx_inputs = &txins[start..start + tx.input.len()];

                let kinds: SmallVec<[(bitcoin::OutPoint, OutputType); 4]> = tx
                    .input
                    .iter()
                    .zip(tx_inputs.iter())
                    .map(|(txin, (_, source))| {
                        let kind = match source {
                            InputSource::PreviousBlock { output_type, .. } => *output_type,
                            InputSource::SameBlock { outpoint, .. } => {
                                let local =
                                    (u32::from(outpoint.tx_index()) - base_tx_index) as usize;
                                let vout = u32::from(outpoint.vout()) as usize;
                                OutputType::from(&txdata[local].output[vout].script_pubkey)
                            }
                        };
                        (txin.previous_output, kind)
                    })
                    .collect();

                SigOps::of_bitcoin_tx_with_kinds(tx, |op| {
                    kinds.iter().find(|(o, _)| o == op).map(|(_, k)| *k)
                })
            })
            .collect()
    }
}
