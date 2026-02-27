use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Height, StoredU64};
use vecdb::Exit;

use super::Vecs;
use crate::{ComputeIndexes, blocks, indexes, inputs, scripts};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        inputs_count: &inputs::CountVecs,
        scripts_count: &scripts::CountVecs,
        blocks: &blocks::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let window_starts = blocks.count.window_starts();
        self.total_count
            .compute(starting_indexes.height, &window_starts, exit, |full| {
                full.compute_with_skip(
                    starting_indexes.height,
                    &indexes.txindex.output_count,
                    &indexer.vecs.transactions.first_txindex,
                    &indexes.height.txindex_count,
                    exit,
                    0,
                )
            })?;

        self.utxo_count.height.compute_transform3(
            starting_indexes.height,
            &*self.total_count.full.sum_cumulative.cumulative,
            &*inputs_count.full.sum_cumulative.cumulative,
            &scripts_count.opreturn.cumulative.height,
            |(h, output_count, input_count, opreturn_count, ..)| {
                let block_count = u64::from(h + 1_usize);
                // -1 > genesis output is unspendable
                let mut utxo_count =
                    *output_count - (*input_count - block_count) - *opreturn_count - 1;

                // txid dup: e3bf3d07d4b0375638d5f1db5255fe07ba2c4cb067cd81b84ee974b6585fb468
                // Block 91_722 https://mempool.space/block/00000000000271a2dc26e7667f8419f2e15416dc6955e5a6c6cdf3f2574dd08e
                // Block 91_880 https://mempool.space/block/00000000000743f190a18c5577a3c2d2a1f610ae9601ac046a38084ccb7cd721
                //
                // txid dup: d5d27987d2a3dfc724e359870c6644b40e497bdc0589a033220fe15429d88599
                // Block 91_812 https://mempool.space/block/00000000000af0aed4792b1acee3d966af36cf5def14935db8de83d6f9306f2f
                // Block 91_842 https://mempool.space/block/00000000000a4d0a398161ffc163c503763b1f4360639393e0e4c8e300e0caec
                //
                // Warning: Dups invalidate the previous coinbase according to
                // https://chainquery.com/bitcoin-cli/gettxoutsetinfo

                if h >= Height::new(91_842) {
                    utxo_count -= 1;
                }
                if h >= Height::new(91_880) {
                    utxo_count -= 1;
                }

                (h, StoredU64::from(utxo_count))
            },
            exit,
        )?;

        Ok(())
    }
}
