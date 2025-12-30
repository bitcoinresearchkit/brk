use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Height, StoredU64};
use vecdb::{Exit, TypedVecIterator};

use super::Vecs;
use crate::{chain::transaction, indexes, ComputeIndexes};

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        transaction_vecs: &transaction::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.indexes_to_p2a_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.address.height_to_first_p2aaddressindex,
                    &indexer.vecs.address.p2aaddressindex_to_p2abytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2ms_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.output.height_to_first_p2msoutputindex,
                    &indexer.vecs.output.p2msoutputindex_to_txindex,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2pk33_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.address.height_to_first_p2pk33addressindex,
                    &indexer.vecs.address.p2pk33addressindex_to_p2pk33bytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2pk65_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.address.height_to_first_p2pk65addressindex,
                    &indexer.vecs.address.p2pk65addressindex_to_p2pk65bytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2pkh_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.address.height_to_first_p2pkhaddressindex,
                    &indexer.vecs.address.p2pkhaddressindex_to_p2pkhbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2sh_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.address.height_to_first_p2shaddressindex,
                    &indexer.vecs.address.p2shaddressindex_to_p2shbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2tr_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.address.height_to_first_p2traddressindex,
                    &indexer.vecs.address.p2traddressindex_to_p2trbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2wpkh_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.address.height_to_first_p2wpkhaddressindex,
                    &indexer.vecs.address.p2wpkhaddressindex_to_p2wpkhbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2wsh_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.address.height_to_first_p2wshaddressindex,
                    &indexer.vecs.address.p2wshaddressindex_to_p2wshbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_opreturn_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.output.height_to_first_opreturnindex,
                    &indexer.vecs.output.opreturnindex_to_txindex,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_unknownoutput_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.output.height_to_first_unknownoutputindex,
                    &indexer.vecs.output.unknownoutputindex_to_txindex,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_emptyoutput_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.output.height_to_first_emptyoutputindex,
                    &indexer.vecs.output.emptyoutputindex_to_txindex,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_exact_utxo_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                let mut input_count_iter = transaction_vecs
                    .indexes_to_input_count
                    .height
                    .unwrap_cumulative()
                    .into_iter();
                let mut opreturn_count_iter = self
                    .indexes_to_opreturn_count
                    .height_extra
                    .unwrap_cumulative()
                    .into_iter();
                v.compute_transform(
                    starting_indexes.height,
                    transaction_vecs
                        .indexes_to_output_count
                        .height
                        .unwrap_cumulative(),
                    |(h, output_count, ..)| {
                        let input_count = input_count_iter.get_unwrap(h);
                        let opreturn_count = opreturn_count_iter.get_unwrap(h);
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
            })?;

        Ok(())
    }
}
