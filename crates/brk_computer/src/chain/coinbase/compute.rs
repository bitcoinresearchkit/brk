use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{CheckedSub, Dollars, HalvingEpoch, Height, Sats, StoredF32, TxOutIndex};
use vecdb::{Exit, IterableVec, TypedVecIterator, VecIndex};

use super::Vecs;
use crate::{
    Indexes,
    chain::{block, transaction},
    indexes, price,
    utils::OptionExt,
};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        block_vecs: &block::Vecs,
        transaction_vecs: &transaction::Vecs,
        starting_indexes: &Indexes,
        price: Option<&price::Vecs>,
        exit: &Exit,
    ) -> Result<()> {
        self.indexes_to_coinbase
            .compute_all(indexes, price, starting_indexes, exit, |vec| {
                let mut txindex_to_first_txoutindex_iter =
                    indexer.vecs.tx.txindex_to_first_txoutindex.iter()?;
                let mut txindex_to_output_count_iter =
                    indexes.transaction.txindex_to_output_count.iter();
                let mut txoutindex_to_value_iter = indexer.vecs.txout.txoutindex_to_value.iter()?;
                vec.compute_transform(
                    starting_indexes.height,
                    &indexer.vecs.tx.height_to_first_txindex,
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

        let mut height_to_coinbase_iter = self
            .indexes_to_coinbase
            .sats
            .height
            .as_ref()
            .unwrap()
            .into_iter();
        self.height_to_24h_coinbase_sum.compute_transform(
            starting_indexes.height,
            &block_vecs.height_to_24h_block_count,
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

        if let Some(mut height_to_coinbase_iter) = self
            .indexes_to_coinbase
            .dollars
            .as_ref()
            .map(|c| c.height.u().into_iter())
        {
            self.height_to_24h_coinbase_usd_sum.compute_transform(
                starting_indexes.height,
                &block_vecs.height_to_24h_block_count,
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

        self.indexes_to_subsidy
            .compute_all(indexes, price, starting_indexes, exit, |vec| {
                vec.compute_transform2(
                    starting_indexes.height,
                    self.indexes_to_coinbase.sats.height.u(),
                    transaction_vecs.indexes_to_fee.sats.height.unwrap_sum(),
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

        self.indexes_to_unclaimed_rewards.compute_all(
            indexes,
            price,
            starting_indexes,
            exit,
            |vec| {
                vec.compute_transform(
                    starting_indexes.height,
                    self.indexes_to_subsidy.sats.height.u(),
                    |(height, subsidy, ..)| {
                        let halving = HalvingEpoch::from(height);
                        let expected = Sats::FIFTY_BTC / 2_usize.pow(halving.to_usize() as u32);
                        (height, expected.checked_sub(subsidy).unwrap())
                    },
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_inflation_rate
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.dateindex,
                    self.indexes_to_subsidy.sats.dateindex.unwrap_sum(),
                    self.indexes_to_subsidy.sats.dateindex.unwrap_cumulative(),
                    |(i, subsidy_1d_sum, subsidy_cumulative, ..)| {
                        (
                            i,
                            (365.0 * *subsidy_1d_sum as f64 / *subsidy_cumulative as f64 * 100.0)
                                .into(),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.dateindex_to_fee_dominance.compute_transform2(
            starting_indexes.dateindex,
            transaction_vecs.indexes_to_fee.sats.dateindex.unwrap_sum(),
            self.indexes_to_coinbase.sats.dateindex.unwrap_sum(),
            |(i, fee, coinbase, ..)| {
                (
                    i,
                    StoredF32::from(u64::from(fee) as f64 / u64::from(coinbase) as f64 * 100.0),
                )
            },
            exit,
        )?;

        self.dateindex_to_subsidy_dominance.compute_transform2(
            starting_indexes.dateindex,
            self.indexes_to_subsidy.sats.dateindex.unwrap_sum(),
            self.indexes_to_coinbase.sats.dateindex.unwrap_sum(),
            |(i, subsidy, coinbase, ..)| {
                (
                    i,
                    StoredF32::from(u64::from(subsidy) as f64 / u64::from(coinbase) as f64 * 100.0),
                )
            },
            exit,
        )?;

        if self.indexes_to_subsidy_usd_1y_sma.is_some() {
            let date_to_coinbase_usd_sum = self
                .indexes_to_coinbase
                .dollars
                .as_ref()
                .unwrap()
                .dateindex
                .unwrap_sum();

            self.indexes_to_subsidy_usd_1y_sma
                .as_mut()
                .unwrap()
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_sma(
                        starting_indexes.dateindex,
                        date_to_coinbase_usd_sum,
                        365,
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_puell_multiple
                .as_mut()
                .unwrap()
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_divide(
                        starting_indexes.dateindex,
                        date_to_coinbase_usd_sum,
                        self.indexes_to_subsidy_usd_1y_sma
                            .as_ref()
                            .unwrap()
                            .dateindex
                            .as_ref()
                            .unwrap(),
                        exit,
                    )?;
                    Ok(())
                })?;
        }

        Ok(())
    }
}
