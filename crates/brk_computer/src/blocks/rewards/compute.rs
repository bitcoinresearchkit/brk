use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{CheckedSub, Dollars, HalvingEpoch, Height, Sats, StoredF32, TxOutIndex};
use vecdb::{Exit, IterableVec, TypedVecIterator, VecIndex};

use super::super::count;
use super::Vecs;
use crate::{ComputeIndexes, indexes, transactions};

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        count_vecs: &count::Vecs,
        transactions_fees: &transactions::FeesVecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.coinbase
            .compute_all(indexes, starting_indexes, exit, |vec| {
                let mut txindex_to_first_txoutindex_iter =
                    indexer.vecs.transactions.first_txoutindex.iter()?;
                let mut txindex_to_output_count_iter =
                    indexes.txindex.output_count.iter();
                let mut txoutindex_to_value_iter = indexer.vecs.outputs.value.iter()?;
                vec.compute_transform(
                    starting_indexes.height,
                    &indexer.vecs.transactions.first_txindex,
                    |(height, txindex, ..)| {
                        let first_txoutindex = txindex_to_first_txoutindex_iter
                            .get_unwrap(txindex)
                            .to_usize();
                        let output_count = txindex_to_output_count_iter.get_unwrap(txindex);
                        let mut sats = Sats::ZERO;
                        (first_txoutindex..first_txoutindex + usize::from(output_count)).for_each(
                            |txoutindex| {
                                sats += txoutindex_to_value_iter
                                    .get_unwrap(TxOutIndex::from(txoutindex));
                            },
                        );
                        (height, sats)
                    },
                    exit,
                )?;
                Ok(())
            })?;

        let mut height_to_coinbase_iter = self.coinbase.sats.height.into_iter();
        self._24h_coinbase_sum.sats.compute_transform(
            starting_indexes.height,
            &count_vecs._24h_block_count.height,
            |(h, count, ..)| {
                let range = *h - (*count - 1)..=*h;
                let sum = range
                    .map(Height::from)
                    .map(|h| height_to_coinbase_iter.get_unwrap(h))
                    .sum::<Sats>();
                (h, sum)
            },
            exit,
        )?;
        drop(height_to_coinbase_iter);

        if let (Some(dollars_out), Some(dollars_in)) =
            (&mut self._24h_coinbase_sum.dollars, &self.coinbase.dollars)
        {
            let mut height_to_coinbase_iter = dollars_in.height.into_iter();
            dollars_out.compute_transform(
                starting_indexes.height,
                &count_vecs._24h_block_count.height,
                |(h, count, ..)| {
                    let range = *h - (*count - 1)..=*h;
                    let sum = range
                        .map(Height::from)
                        .map(|h| height_to_coinbase_iter.get_unwrap(h))
                        .sum::<Dollars>();
                    (h, sum)
                },
                exit,
            )?;
        }

        self.subsidy
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_transform2(
                    starting_indexes.height,
                    &self.coinbase.sats.height,
                    &transactions_fees.fee.sats.height.sum_cum.sum.0,
                    |(height, coinbase, fees, ..)| {
                        (
                            height,
                            coinbase.checked_sub(fees).unwrap_or_else(|| {
                                dbg!(height, coinbase, fees);
                                panic!()
                            }),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.unclaimed_rewards
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_transform(
                    starting_indexes.height,
                    &self.subsidy.sats.height,
                    |(height, subsidy, ..)| {
                        let halving = HalvingEpoch::from(height);
                        let expected = Sats::FIFTY_BTC / 2_usize.pow(halving.to_usize() as u32);
                        (height, expected.checked_sub(subsidy).unwrap())
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.fee_dominance.compute_transform2(
            starting_indexes.dateindex,
            &transactions_fees.fee.sats.dateindex.sum_cum.sum.0,
            &self.coinbase.sats.dateindex.sum_cum.sum.0,
            |(i, fee, coinbase, ..)| {
                let coinbase_f64 = u64::from(coinbase) as f64;
                let dominance = if coinbase_f64 == 0.0 {
                    StoredF32::NAN
                } else {
                    StoredF32::from(u64::from(fee) as f64 / coinbase_f64 * 100.0)
                };
                (i, dominance)
            },
            exit,
        )?;

        self.subsidy_dominance.compute_transform2(
            starting_indexes.dateindex,
            &self.subsidy.sats.dateindex.sum_cum.sum.0,
            &self.coinbase.sats.dateindex.sum_cum.sum.0,
            |(i, subsidy, coinbase, ..)| {
                let coinbase_f64 = u64::from(coinbase) as f64;
                let dominance = if coinbase_f64 == 0.0 {
                    StoredF32::NAN
                } else {
                    StoredF32::from(u64::from(subsidy) as f64 / coinbase_f64 * 100.0)
                };
                (i, dominance)
            },
            exit,
        )?;

        if let Some(sma) = self.subsidy_usd_1y_sma.as_mut() {
            let date_to_coinbase_usd_sum = &self
                .coinbase
                .dollars
                .as_ref()
                .unwrap()
                .dateindex
                .sum_cum
                .sum
                .0;

            sma.compute_all(starting_indexes, exit, |v| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    date_to_coinbase_usd_sum,
                    365,
                    exit,
                )?;
                Ok(())
            })?;
        }

        Ok(())
    }
}
