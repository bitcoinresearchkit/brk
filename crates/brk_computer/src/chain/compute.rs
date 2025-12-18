use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{
    CheckedSub, FeeRate, HalvingEpoch, Height, ONE_DAY_IN_SEC_F64, Sats, StoredF32, StoredF64,
    StoredU32, StoredU64, Timestamp, TxOutIndex, TxVersion,
};
use vecdb::{Exit, GenericStoredVec, IterableVec, TypedVecIterator, VecIndex, unlikely};

use crate::{grouped::ComputedVecsFromHeight, indexes, price, utils::OptionExt, Indexes};

use super::{Vecs, ONE_TERA_HASH, TARGET_BLOCKS_PER_DAY_F32, TARGET_BLOCKS_PER_DAY_F64};

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        price: Option<&price::Vecs>,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_(indexer, indexes, starting_indexes, price, exit)?;
        self.db.compact()?;
        Ok(())
    }

    fn compute_(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        price: Option<&price::Vecs>,
        exit: &Exit,
    ) -> Result<()> {
        self.timeindexes_to_timestamp
            .compute_all(starting_indexes, exit, |vec| {
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &indexes.dateindex_to_date,
                    |(di, d, ..)| (di, Timestamp::from(d)),
                    exit,
                )?;
                Ok(())
            })?;

        let mut height_to_timestamp_fixed_iter = indexes.height_to_timestamp_fixed.into_iter();
        let mut prev = Height::ZERO;
        self.height_to_24h_block_count.compute_transform(
            starting_indexes.height,
            &indexes.height_to_timestamp_fixed,
            |(h, t, ..)| {
                while t.difference_in_days_between(height_to_timestamp_fixed_iter.get_unwrap(prev))
                    > 0
                {
                    prev.increment();
                    if prev > h {
                        unreachable!()
                    }
                }
                (h, StoredU32::from(*h + 1 - *prev))
            },
            exit,
        )?;

        self.indexes_to_block_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_range(
                    starting_indexes.height,
                    &indexer.vecs.block.height_to_weight,
                    |h| (h, StoredU32::from(1_u32)),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_1w_block_count
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sum(
                    starting_indexes.dateindex,
                    self.indexes_to_block_count.dateindex.unwrap_sum(),
                    7,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_1m_block_count
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sum(
                    starting_indexes.dateindex,
                    self.indexes_to_block_count.dateindex.unwrap_sum(),
                    30,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_1y_block_count
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sum(
                    starting_indexes.dateindex,
                    self.indexes_to_block_count.dateindex.unwrap_sum(),
                    365,
                    exit,
                )?;
                Ok(())
            })?;

        let mut height_to_timestamp_iter = indexer.vecs.block.height_to_timestamp.iter()?;
        self.height_to_interval.compute_transform(
            starting_indexes.height,
            &indexer.vecs.block.height_to_timestamp,
            |(height, timestamp, ..)| {
                let interval = height.decremented().map_or(Timestamp::ZERO, |prev_h| {
                    let prev_timestamp = height_to_timestamp_iter.get_unwrap(prev_h);
                    timestamp
                        .checked_sub(prev_timestamp)
                        .unwrap_or(Timestamp::ZERO)
                });
                (height, interval)
            },
            exit,
        )?;

        self.indexes_to_block_interval.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_interval),
        )?;

        self.indexes_to_block_weight.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&indexer.vecs.block.height_to_weight),
        )?;

        self.indexes_to_block_size.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&indexer.vecs.block.height_to_total_size),
        )?;

        self.height_to_vbytes.compute_transform(
            starting_indexes.height,
            &indexer.vecs.block.height_to_weight,
            |(h, w, ..)| {
                (
                    h,
                    StoredU64::from(bitcoin::Weight::from(w).to_vbytes_floor()),
                )
            },
            exit,
        )?;

        self.indexes_to_block_vbytes.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_vbytes),
        )?;

        let mut height_to_timestamp_iter = indexer.vecs.block.height_to_timestamp.iter()?;

        self.difficultyepoch_to_timestamp.compute_transform(
            starting_indexes.difficultyepoch,
            &indexes.difficultyepoch_to_first_height,
            |(i, h, ..)| (i, height_to_timestamp_iter.get_unwrap(h)),
            exit,
        )?;

        self.halvingepoch_to_timestamp.compute_transform(
            starting_indexes.halvingepoch,
            &indexes.halvingepoch_to_first_height,
            |(i, h, ..)| (i, height_to_timestamp_iter.get_unwrap(h)),
            exit,
        )?;

        let mut height_to_difficultyepoch_iter = indexes.height_to_difficultyepoch.into_iter();
        self.indexes_to_difficultyepoch
            .compute_all(starting_indexes, exit, |vec| {
                let mut height_count_iter = indexes.dateindex_to_height_count.into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &indexes.dateindex_to_first_height,
                    |(di, height, ..)| {
                        (
                            di,
                            height_to_difficultyepoch_iter
                                .get_unwrap(height + (*height_count_iter.get_unwrap(di) - 1)),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        let mut height_to_halvingepoch_iter = indexes.height_to_halvingepoch.into_iter();
        self.indexes_to_halvingepoch
            .compute_all(starting_indexes, exit, |vec| {
                let mut height_count_iter = indexes.dateindex_to_height_count.into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &indexes.dateindex_to_first_height,
                    |(di, height, ..)| {
                        (
                            di,
                            height_to_halvingepoch_iter
                                .get_unwrap(height + (*height_count_iter.get_unwrap(di) - 1)),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_difficulty.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&indexer.vecs.block.height_to_difficulty),
        )?;

        self.indexes_to_tx_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.tx.height_to_first_txindex,
                    &indexer.vecs.tx.txindex_to_txid,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_input_count.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&indexes.txindex_to_input_count),
        )?;

        self.indexes_to_output_count.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&indexes.txindex_to_output_count),
        )?;

        let compute_indexes_to_tx_vany =
            |indexes_to_tx_vany: &mut ComputedVecsFromHeight<StoredU64>, txversion| {
                let mut txindex_to_txversion_iter = indexer.vecs.tx.txindex_to_txversion.iter()?;
                indexes_to_tx_vany.compute_all(indexes, starting_indexes, exit, |vec| {
                    vec.compute_filtered_count_from_indexes(
                        starting_indexes.height,
                        &indexer.vecs.tx.height_to_first_txindex,
                        &indexer.vecs.tx.txindex_to_txid,
                        |txindex| {
                            let v = txindex_to_txversion_iter.get_unwrap(txindex);
                            v == txversion
                        },
                        exit,
                    )?;
                    Ok(())
                })
            };
        compute_indexes_to_tx_vany(&mut self.indexes_to_tx_v1, TxVersion::ONE)?;
        compute_indexes_to_tx_vany(&mut self.indexes_to_tx_v2, TxVersion::TWO)?;
        compute_indexes_to_tx_vany(&mut self.indexes_to_tx_v3, TxVersion::THREE)?;

        // ---
        // TxInIndex
        // ---

        let txindex_to_first_txoutindex = &indexer.vecs.tx.txindex_to_first_txoutindex;
        let txindex_to_first_txoutindex_reader = txindex_to_first_txoutindex.create_reader();
        let txoutindex_to_value = &indexer.vecs.txout.txoutindex_to_value;
        let txoutindex_to_value_reader = indexer.vecs.txout.txoutindex_to_value.create_reader();
        self.txinindex_to_value.compute_transform(
            starting_indexes.txinindex,
            &indexer.vecs.txin.txinindex_to_outpoint,
            |(txinindex, outpoint, ..)| {
                if unlikely(outpoint.is_coinbase()) {
                    return (txinindex, Sats::MAX);
                }
                let txoutindex = txindex_to_first_txoutindex
                    .read_unwrap(outpoint.txindex(), &txindex_to_first_txoutindex_reader)
                    + outpoint.vout();

                let value = if unlikely(txoutindex == TxOutIndex::COINBASE) {
                    unreachable!()
                } else {
                    txoutindex_to_value
                        .unchecked_read(txoutindex, &txoutindex_to_value_reader)
                        .unwrap()
                };

                (txinindex, value)
            },
            exit,
        )?;

        self.txindex_to_input_value.compute_sum_from_indexes(
            starting_indexes.txindex,
            &indexer.vecs.tx.txindex_to_first_txinindex,
            &indexes.txindex_to_input_count,
            &self.txinindex_to_value,
            exit,
        )?;

        self.txindex_to_output_value.compute_sum_from_indexes(
            starting_indexes.txindex,
            &indexer.vecs.tx.txindex_to_first_txoutindex,
            &indexes.txindex_to_output_count,
            &indexer.vecs.txout.txoutindex_to_value,
            exit,
        )?;

        self.txindex_to_fee.compute_transform2(
            starting_indexes.txindex,
            &self.txindex_to_input_value,
            &self.txindex_to_output_value,
            |(i, input, output, ..)| {
                let fee = if unlikely(input.is_max()) {
                    Sats::ZERO
                } else {
                    input - output
                };
                (i, fee)
            },
            exit,
        )?;

        self.txindex_to_fee_rate.compute_transform2(
            starting_indexes.txindex,
            &self.txindex_to_fee,
            &self.txindex_to_vsize,
            |(txindex, fee, vsize, ..)| (txindex, FeeRate::from((fee, vsize))),
            exit,
        )?;

        self.indexes_to_sent_sum
            .compute_all(indexes, price, starting_indexes, exit, |v| {
                v.compute_filtered_sum_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.tx.height_to_first_txindex,
                    &indexes.height_to_txindex_count,
                    &self.txindex_to_input_value,
                    |sats| !sats.is_max(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_fee.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&self.txindex_to_fee),
            price,
        )?;

        self.indexes_to_fee_rate.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&self.txindex_to_fee_rate),
        )?;

        self.indexes_to_tx_weight.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&self.txindex_to_weight),
        )?;

        self.indexes_to_tx_vsize.compute_rest(
            indexer,
            indexes,
            starting_indexes,
            exit,
            Some(&self.txindex_to_vsize),
        )?;

        self.indexes_to_coinbase
            .compute_all(indexes, price, starting_indexes, exit, |vec| {
                let mut txindex_to_first_txoutindex_iter =
                    indexer.vecs.tx.txindex_to_first_txoutindex.iter()?;
                let mut txindex_to_output_count_iter = indexes.txindex_to_output_count.iter();
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
            &self.height_to_24h_block_count,
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
                &self.height_to_24h_block_count,
                |(h, count, ..)| {
                    let range = *h - (*count - 1)..=*h;
                    let sum = range
                        .map(Height::from)
                        .map(|h| height_to_coinbase_iter.get_unwrap(h))
                        .sum::<brk_types::Dollars>();
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
                    self.indexes_to_fee.sats.height.unwrap_sum(),
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
                let mut input_count_iter = self
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
                    self.indexes_to_output_count.height.unwrap_cumulative(),
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

        self.dateindex_to_fee_dominance.compute_transform2(
            starting_indexes.dateindex,
            self.indexes_to_fee.sats.dateindex.unwrap_sum(),
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

        self.indexes_to_difficulty_as_hash
            .compute_all(indexes, starting_indexes, exit, |v| {
                let multiplier = 2.0_f64.powi(32) / 600.0;
                v.compute_transform(
                    starting_indexes.height,
                    &indexer.vecs.block.height_to_difficulty,
                    |(i, v, ..)| (i, StoredF32::from(*v * multiplier)),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_rate
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.height,
                    &self.height_to_24h_block_count,
                    self.indexes_to_difficulty_as_hash.height.u(),
                    |(i, block_count_sum, difficulty_as_hash, ..)| {
                        (
                            i,
                            StoredF64::from(
                                (f64::from(block_count_sum) / TARGET_BLOCKS_PER_DAY_F64)
                                    * f64::from(difficulty_as_hash),
                            ),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_rate_1w_sma
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    self.indexes_to_hash_rate.dateindex.unwrap_last(),
                    7,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_rate_1m_sma
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    self.indexes_to_hash_rate.dateindex.unwrap_last(),
                    30,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_rate_2m_sma
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    self.indexes_to_hash_rate.dateindex.unwrap_last(),
                    2 * 30,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_rate_1y_sma
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    self.indexes_to_hash_rate.dateindex.unwrap_last(),
                    365,
                    exit,
                )?;
                Ok(())
            })?;

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

        self.indexes_to_difficulty_adjustment.compute_all(
            indexes,
            starting_indexes,
            exit,
            |v| {
                v.compute_percentage_change(
                    starting_indexes.height,
                    &indexer.vecs.block.height_to_difficulty,
                    1,
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_blocks_before_next_difficulty_adjustment
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &indexes.height_to_height,
                    |(h, ..)| (h, StoredU32::from(h.left_before_next_diff_adj())),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_days_before_next_difficulty_adjustment
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    self.indexes_to_blocks_before_next_difficulty_adjustment
                        .height
                        .as_ref()
                        .unwrap(),
                    |(h, blocks, ..)| (h, (*blocks as f32 / TARGET_BLOCKS_PER_DAY_F32).into()),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_blocks_before_next_halving.compute_all(
            indexes,
            starting_indexes,
            exit,
            |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &indexes.height_to_height,
                    |(h, ..)| (h, StoredU32::from(h.left_before_next_halving())),
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_days_before_next_halving.compute_all(
            indexes,
            starting_indexes,
            exit,
            |v| {
                v.compute_transform(
                    starting_indexes.height,
                    self.indexes_to_blocks_before_next_halving
                        .height
                        .as_ref()
                        .unwrap(),
                    |(h, blocks, ..)| (h, (*blocks as f32 / TARGET_BLOCKS_PER_DAY_F32).into()),
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_hash_price_ths
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.height,
                    &self.height_to_24h_coinbase_usd_sum,
                    self.indexes_to_hash_rate.height.u(),
                    |(i, coinbase_sum, hashrate, ..)| {
                        (i, (*coinbase_sum / (*hashrate / ONE_TERA_HASH)).into())
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_price_phs
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    self.indexes_to_hash_price_ths.height.u(),
                    |(i, price, ..)| (i, (*price * 1000.0).into()),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_value_ths
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.height,
                    &self.height_to_24h_coinbase_sum,
                    self.indexes_to_hash_rate.height.u(),
                    |(i, coinbase_sum, hashrate, ..)| {
                        (
                            i,
                            (*coinbase_sum as f64 / (*hashrate / ONE_TERA_HASH)).into(),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_value_phs
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    self.indexes_to_hash_value_ths.height.u(),
                    |(i, value, ..)| (i, (*value * 1000.0).into()),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_price_ths_min
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_all_time_low_(
                    starting_indexes.height,
                    self.indexes_to_hash_price_ths.height.u(),
                    exit,
                    true,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_price_phs_min
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_all_time_low_(
                    starting_indexes.height,
                    self.indexes_to_hash_price_phs.height.u(),
                    exit,
                    true,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_value_ths_min
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_all_time_low_(
                    starting_indexes.height,
                    self.indexes_to_hash_value_ths.height.u(),
                    exit,
                    true,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_value_phs_min
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_all_time_low_(
                    starting_indexes.height,
                    self.indexes_to_hash_value_phs.height.u(),
                    exit,
                    true,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_price_rebound
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_percentage_difference(
                    starting_indexes.height,
                    self.indexes_to_hash_price_phs.height.u(),
                    self.indexes_to_hash_price_phs_min.height.u(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_value_rebound
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_percentage_difference(
                    starting_indexes.height,
                    self.indexes_to_hash_value_phs.height.u(),
                    self.indexes_to_hash_value_phs_min.height.u(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_annualized_volume
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sum(
                    starting_indexes.dateindex,
                    self.indexes_to_sent_sum.sats.dateindex.unwrap_sum(),
                    365,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_annualized_volume_btc
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sum(
                    starting_indexes.dateindex,
                    self.indexes_to_sent_sum.bitcoin.dateindex.unwrap_sum(),
                    365,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_tx_btc_velocity
            .compute_all(starting_indexes, exit, |v| {
                v.compute_divide(
                    starting_indexes.dateindex,
                    self.indexes_to_annualized_volume_btc
                        .dateindex
                        .as_ref()
                        .unwrap(),
                    self.indexes_to_subsidy
                        .bitcoin
                        .dateindex
                        .unwrap_cumulative(),
                    exit,
                )?;
                Ok(())
            })?;

        if let Some(indexes_to_sent_sum) = self.indexes_to_sent_sum.dollars.as_ref() {
            self.indexes_to_annualized_volume_usd
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_sum(
                        starting_indexes.dateindex,
                        indexes_to_sent_sum.dateindex.unwrap_sum(),
                        365,
                        exit,
                    )?;
                    Ok(())
                })?;

            self.indexes_to_tx_usd_velocity
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_divide(
                        starting_indexes.dateindex,
                        self.indexes_to_annualized_volume_usd
                            .dateindex
                            .as_ref()
                            .unwrap(),
                        self.indexes_to_subsidy
                            .dollars
                            .as_ref()
                            .unwrap()
                            .dateindex
                            .unwrap_cumulative(),
                        exit,
                    )?;
                    Ok(())
                })?;
        }

        self.indexes_to_tx_per_sec
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.dateindex,
                    self.indexes_to_tx_count.dateindex.unwrap_sum(),
                    &indexes.dateindex_to_date,
                    |(i, tx_count, date, ..)| {
                        (
                            i,
                            (*tx_count as f64 / (date.completion() * ONE_DAY_IN_SEC_F64)).into(),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_inputs_per_sec
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.dateindex,
                    self.indexes_to_input_count.dateindex.unwrap_sum(),
                    &indexes.dateindex_to_date,
                    |(i, tx_count, date, ..)| {
                        (
                            i,
                            (*tx_count as f64 / (date.completion() * ONE_DAY_IN_SEC_F64)).into(),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_outputs_per_sec
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.dateindex,
                    self.indexes_to_output_count.dateindex.unwrap_sum(),
                    &indexes.dateindex_to_date,
                    |(i, tx_count, date, ..)| {
                        (
                            i,
                            (*tx_count as f64 / (date.completion() * ONE_DAY_IN_SEC_F64)).into(),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
    }
}
