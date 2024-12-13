use std::{collections::BTreeMap, ops::ControlFlow, thread};

use biter::{
    bitcoin::{Block, Txid},
    bitcoincore_rpc::RpcApi,
};

use itertools::Itertools;
use rayon::prelude::*;

use crate::{
    parser::{
        databases::{
            AddressIndexToAddressData, AddressIndexToEmptyAddressData, AddressToAddressIndex,
            Databases, TxidToTxData, TxoutIndexToAddressIndex, TxoutIndexToAmount,
        },
        datasets::{Datasets, InsertData},
        states::{
            AddressCohortsInputStates, AddressCohortsOutputStates, AddressCohortsRealizedStates,
            States, UTXOCohortsOneShotStates, UTXOCohortsSentStates,
        },
    },
    structs::{
        Address, AddressData, AddressRealizedData, Amount, BlockData, BlockPath, Config, Counter,
        Date, EmptyAddressData, Height, PartialTxoutData, Price, SentData, Timestamp, TxData,
        TxoutIndex,
    },
};

pub struct ParseData<'a> {
    // pub bitcoin_cli: &'a BitcoinCli,
    pub block: Block,
    pub block_index: usize,
    pub config: &'a Config,
    pub compute_addresses: bool,
    pub databases: &'a mut Databases,
    pub datasets: &'a mut Datasets,
    pub date: Date,
    pub first_date_height: Height,
    pub height: Height,
    pub is_date_last_block: bool,
    pub rpc: &'a biter::bitcoincore_rpc::Client,
    pub states: &'a mut States,
}

pub fn parse(
    ParseData {
        block,
        block_index,
        config,
        compute_addresses,
        databases,
        datasets,
        date,
        first_date_height,
        height,
        is_date_last_block,
        rpc,
        states,
    }: ParseData,
) {
    // log(&format!("{height}"));

    let timestamp = Timestamp::wrap(block.header.time);

    // If false, expect that the code is flawless
    // or create a 0 value txid database
    let enable_check_if_txout_value_is_zero_in_db: bool = true;

    let date_index = states.date_data_vec.len() - 1;

    let previous_timestamp = height
        .checked_sub(1)
        .map(Height::new)
        .and_then(|height| datasets.block_metadata.timestamp.get_or_import(&height));

    let block_price = Price::from_dollar(
        datasets
            .price
            .get_height_ohlc(height, timestamp, previous_timestamp, config)
            .unwrap_or_else(|_| panic!("Expect {height} to have a price"))
            .close as f64,
    );

    let date_price = Price::from_dollar(
        datasets
            .price
            .get_date_ohlc(date)
            .unwrap_or_else(|_| panic!("Expect {date} to have a price"))
            .close as f64,
    );

    let difficulty = block.header.difficulty_float();
    let block_size = block.total_size();
    let block_weight = block.weight().to_wu();
    let block_vbytes = block.weight().to_vbytes_floor();
    let block_interval = previous_timestamp.map_or(Timestamp::ZERO, |previous_timestamp| {
        if previous_timestamp >= timestamp {
            Timestamp::ZERO
        } else {
            timestamp - previous_timestamp
        }
    });

    states
        .date_data_vec
        .last_mut()
        .unwrap()
        .blocks
        .push(BlockData::new(height, block_price, timestamp));

    let mut block_path_to_sent_data: BTreeMap<BlockPath, SentData> = BTreeMap::default();
    // let mut received_data: ReceivedData = ReceivedData::default();
    let mut address_index_to_address_realized_data: BTreeMap<u32, AddressRealizedData> =
        BTreeMap::default();

    let mut coinbase = Amount::ZERO;
    let mut satblocks_destroyed = Amount::ZERO;
    let mut satdays_destroyed = Amount::ZERO;
    let mut amount_sent = Amount::ZERO;
    let mut transaction_count = 0;
    let mut fees = vec![];
    let mut fees_total = Amount::ZERO;

    let (
        TxoutsParsingResults {
            op_returns: _op_returns,
            mut partial_txout_data_vec,
            provably_unspendable: _provably_unspendable,
        },
        (mut txid_to_tx_data, mut txout_index_to_amount_and_address_index),
    ) = thread::scope(|scope| {
        let output_handle = scope.spawn(|| {
            let mut txouts_parsing_results = prepare_outputs(
                &block,
                compute_addresses,
                &mut states.address_counters.multisig_addresses,
                &mut states.address_counters.op_return_addresses,
                &mut states.address_counters.push_only_addresses,
                &mut states.address_counters.unknown_addresses,
                &mut states.address_counters.empty_addresses,
                &mut databases.address_to_address_index,
            );

            // Reverse to get in order via pop later
            txouts_parsing_results.partial_txout_data_vec.reverse();

            txouts_parsing_results
        });

        let input_handle = scope.spawn(|| {
            prepare_inputs(
                &block,
                &mut databases.txid_to_tx_data,
                &mut databases.txout_index_to_amount,
                &mut databases.txout_index_to_address_index,
                compute_addresses,
            )
        });

        (output_handle.join().unwrap(), input_handle.join().unwrap())
    });

    let mut address_index_to_address_data = compute_addresses.then(|| {
        compute_address_index_to_address_data(
            &mut databases.address_index_to_address_data,
            &mut databases.address_index_to_empty_address_data,
            &partial_txout_data_vec,
            &txout_index_to_amount_and_address_index,
            compute_addresses,
        )
    });

    block
        .txdata
        .iter()
        .enumerate()
        .try_for_each(|(block_tx_index, tx)| {
            let txid = tx.compute_txid();
            let tx_index = databases.txid_to_tx_data.metadata.serial as u32;

            transaction_count += 1;

            // --
            // outputs
            // ---

            let mut utxos = BTreeMap::new();
            let mut spendable_amount = Amount::ZERO;

            let is_coinbase = tx.is_coinbase();

            if is_coinbase != (block_tx_index == 0) {
                unreachable!();
            }

            let mut inputs_sum = Amount::ZERO;
            let mut outputs_sum = Amount::ZERO;

            let last_block = states.date_data_vec.last_mut_block().unwrap();

            // Before `input` to cover outputs being used in the same block as inputs
            tx.output
                .iter()
                .enumerate()
                .filter_map(|(vout, tx_out)| {
                    if vout > (u16::MAX as usize) {
                        panic!("vout can indeed be bigger than u16::MAX !");
                    }

                    let amount = Amount::wrap(tx_out.value);

                    if is_coinbase {
                        coinbase += amount;
                    } else {
                        outputs_sum += amount;
                    }

                    partial_txout_data_vec
                        .pop()
                        .unwrap()
                        // None if not worth parsing (empty/op_return/...)
                        .map(|partial_txout_data| (vout, partial_txout_data))
                })
                .for_each(|(vout, partial_txout_data)| {
                    let vout = vout as u16;

                    let txout_index = TxoutIndex::new(tx_index, vout);

                    let PartialTxoutData {
                        address,
                        address_index_opt,
                        amount,
                    } = partial_txout_data;

                    spendable_amount += amount;

                    last_block.receive(amount);

                    utxos.insert(vout, amount);

                    databases
                        .txout_index_to_amount
                        .unsafe_insert(txout_index, amount);

                    if compute_addresses {
                        let address = address.unwrap();

                        let address_index_to_address_data =
                            address_index_to_address_data.as_mut().unwrap();

                        let (address_data, address_index) = {
                            if let Some(address_index) = address_index_opt.or_else(|| {
                                databases
                                    .address_to_address_index
                                    .unsafe_get_from_puts(&address)
                                    .cloned()
                            }) {
                                let address_data = address_index_to_address_data
                                    .get_mut(&address_index)
                                    .unwrap();

                                (address_data, address_index)
                            } else {
                                let address_index =
                                    databases.address_to_address_index.metadata.serial as u32;

                                let address_type = address.to_type();

                                if let Some(previous) = databases
                                    .address_to_address_index
                                    .insert(address, address_index)
                                {
                                    dbg!(previous);
                                    panic!(
                                        "address #{address_index} shouldn't be present during put"
                                    );
                                }

                                // Checked new
                                let address_data = address_index_to_address_data
                                    .entry(address_index)
                                    .and_modify(|_| {
                                        panic!("Shouldn't exist");
                                    })
                                    // Will always insert, it's to avoid insert + get
                                    .or_insert(AddressData::new(address_type));

                                (address_data, address_index)
                            }
                        };

                        // MUST be before received !
                        let address_realized_data = address_index_to_address_realized_data
                            .entry(address_index)
                            .or_insert_with(|| AddressRealizedData::default(address_data));

                        address_data.receive(amount, block_price);

                        address_realized_data.receive(amount);

                        databases
                            .txout_index_to_address_index
                            .unsafe_insert(txout_index, address_index);
                    }
                });

            if !utxos.is_empty() {
                databases.txid_to_tx_data.insert(
                    &txid,
                    TxData::new(
                        tx_index,
                        BlockPath::new(date_index as u16, block_index as u16),
                        utxos.len() as u16,
                    ),
                );
            }

            // ---
            // inputs
            // ---

            if !is_coinbase {
                tx.input.iter().try_for_each(|txin| {
                    let outpoint = txin.previous_output;
                    let input_txid = outpoint.txid;
                    let input_vout = outpoint.vout;

                    let remove_tx_data_from_cached_puts = {
                        let mut is_tx_data_from_cached_puts = false;

                        let input_tx_data = txid_to_tx_data
                            .get_mut(&input_txid)
                            .unwrap()
                            .as_mut()
                            .or_else(|| {
                                is_tx_data_from_cached_puts = true;

                                databases
                                    .txid_to_tx_data
                                    .unsafe_get_mut_from_puts(&input_txid)
                            });

                        // Can be none because 0 sats inputs happen
                        // https://mempool.space/tx/f329e55c2de9b821356e6f2c4bba923ea7030cad61120f5ced5d4429f5c86fda#vin=27

                        if input_tx_data.is_none() {
                            if !enable_check_if_txout_value_is_zero_in_db
                                || rpc
                                    .get_raw_transaction(&input_txid, None)
                                    .unwrap()
                                    .output
                                    .get(input_vout as usize)
                                    .unwrap()
                                    .value
                                    .to_sat()
                                    == 0
                            {
                                return ControlFlow::Continue::<()>(());
                            }

                            dbg!((input_txid, txid, tx_index, input_vout));
                            panic!("Txid to be in txid_to_tx_data");
                        }

                        let input_tx_data = input_tx_data.unwrap();
                        let input_tx_index = input_tx_data.index;
                        let input_vout = input_vout as u16;
                        let input_txout_index = TxoutIndex::new(input_tx_index, input_vout);

                        // if input_tx_index == 2516 || input_tx_index == 2490 {
                        //     dbg!(input_tx_index, &input_tx_data.utxos);
                        // }

                        // let input_amount = input_tx_data.utxos.remove(&input_vout);

                        let input_amount_and_address_index = databases
                            .txout_index_to_amount
                            .remove(&input_txout_index)
                            .map(|amount| {
                                (
                                    amount,
                                    databases
                                        .txout_index_to_address_index
                                        .remove(&input_txout_index),
                                )
                            }) // Remove from cached puts
                            .or_else(|| {
                                txout_index_to_amount_and_address_index.remove(&input_txout_index)
                            });

                        if input_amount_and_address_index.is_none() {
                            if !enable_check_if_txout_value_is_zero_in_db
                                || rpc
                                    .get_raw_transaction(&input_txid, None)
                                    .unwrap()
                                    .output
                                    .get(input_vout as usize)
                                    .unwrap()
                                    .value
                                    .to_sat()
                                    == 0
                            {
                                return ControlFlow::Continue::<()>(());
                            }

                            dbg!((
                                input_txid,
                                tx_index,
                                input_tx_index,
                                input_vout,
                                input_tx_data,
                                txid,
                            ));
                            panic!("Txout index to be in txout_index_to_txout_value");
                        }

                        input_tx_data.utxos -= 1;

                        let (input_amount, input_address_index) =
                            input_amount_and_address_index.unwrap();

                        let input_block_path = input_tx_data.block_path;

                        let BlockPath {
                            date_index: input_date_index,
                            block_index: input_block_index,
                        } = input_block_path;

                        let input_date_data = states
                            .date_data_vec
                            .get_mut(input_date_index as usize)
                            .unwrap_or_else(|| {
                                dbg!(height, &input_txid, input_block_path, input_date_index);
                                panic!()
                            });

                        let input_block_data = input_date_data
                            .blocks
                            .get_mut(input_block_index as usize)
                            .unwrap_or_else(|| {
                                dbg!(
                                    height,
                                    &input_txid,
                                    input_block_path,
                                    input_date_index,
                                    input_block_index,
                                );
                                panic!()
                            });

                        input_block_data.send(input_amount);

                        inputs_sum += input_amount;

                        block_path_to_sent_data
                            .entry(input_block_path)
                            .or_default()
                            .send(input_amount);

                        satblocks_destroyed += input_amount * (height - input_block_data.height);

                        satdays_destroyed += input_amount
                            * date.signed_duration_since(*input_date_data.date).num_days() as u64;

                        if compute_addresses {
                            let input_address_index = input_address_index.unwrap_or_else(|| {
                                dbg!(
                                    height,
                                    input_amount,
                                    &input_tx_data,
                                    input_address_index,
                                    input_txout_index,
                                    txid,
                                    input_txid,
                                    input_vout
                                );
                                panic!()
                            });

                            let address_index_to_address_data =
                                address_index_to_address_data.as_mut().unwrap();

                            let input_address_data = address_index_to_address_data
                                .get_mut(&input_address_index)
                                .unwrap_or_else(|| {
                                    dbg!(
                                        input_address_index,
                                        input_txout_index,
                                        input_txid,
                                        input_vout
                                    );
                                    panic!();
                                });

                            let input_address_realized_data =
                                address_index_to_address_realized_data
                                    .entry(input_address_index)
                                    .or_insert_with(|| {
                                        AddressRealizedData::default(input_address_data)
                                    });

                            let previous_price = input_block_data.price;

                            // MUST be after `or_insert_with`
                            input_address_data
                                .send(input_amount, previous_price)
                                .unwrap_or_else(|_| {
                                    dbg!(
                                        input_address_index,
                                        txid,
                                        input_txid,
                                        input_amount,
                                        tx_index,
                                        input_tx_index,
                                        input_vout,
                                        &input_address_data
                                    );

                                    panic!()
                                });

                            input_address_realized_data.send(
                                input_amount,
                                block_price,
                                previous_price,
                                timestamp,
                                input_block_data.timestamp,
                            );
                        };

                        is_tx_data_from_cached_puts && input_tx_data.is_empty()
                    };

                    if remove_tx_data_from_cached_puts {
                        // Pre remove tx_datas that are empty and weren't yet added to the database to avoid having it was in there or not (and thus avoid useless operations)
                        databases.txid_to_tx_data.remove_from_puts(&input_txid)
                    }

                    ControlFlow::Continue(())
                })?;
            }

            amount_sent += inputs_sum;

            let fee = inputs_sum - outputs_sum;

            fees_total += fee;
            fees.push(fee);

            ControlFlow::Continue(())
        });

    if !partial_txout_data_vec.is_empty() {
        panic!("partial_txout_data_vec should've been fully consumed");
    }

    txid_to_tx_data.into_iter().for_each(|(txid, tx_data)| {
        if let Some(tx_data) = tx_data {
            if tx_data.is_empty() {
                databases.txid_to_tx_data.remove_from_db(txid);
            } else {
                databases.txid_to_tx_data.update(txid, tx_data);
            }
        }
    });

    let mut utxo_cohorts_sent_states = UTXOCohortsSentStates::default();
    let mut utxo_cohorts_one_shot_states = UTXOCohortsOneShotStates::default();
    // let mut utxo_cohorts_received_states = UTXOCohortsReceivedStates::default();

    let mut address_cohorts_input_states = None;
    let mut address_cohorts_one_shot_states = None;
    let mut address_cohorts_output_states = None;
    let mut address_cohorts_realized_states = None;

    // log("Starting heavy work...");

    thread::scope(|scope| {
        scope.spawn(|| {
            let previous_last_block_data = states.date_data_vec.second_last_block();

            if datasets.utxo.needs_durable_states(height, date) {
                if let Some(previous_last_block_data) = previous_last_block_data {
                    block_path_to_sent_data
                        .iter()
                        .for_each(|(block_path, sent_data)| {
                            let block_data =
                                states.date_data_vec.get_block_data(block_path).unwrap();

                            if block_data.height != height {
                                states
                                    .utxo_cohorts_durable_states
                                    .as_mut()
                                    .unwrap()
                                    .subtract_moved(
                                        block_data,
                                        sent_data,
                                        previous_last_block_data,
                                    );
                            }
                        });
                }

                let last_block_data = states.date_data_vec.last_block().unwrap();

                if last_block_data.height != height {
                    unreachable!()
                }

                states
                    .date_data_vec
                    .iter()
                    .flat_map(|date_data| &date_data.blocks)
                    .for_each(|block_data| {
                        states
                            .utxo_cohorts_durable_states
                            .as_mut()
                            .unwrap()
                            .udpate_age_if_needed(
                                block_data,
                                last_block_data,
                                previous_last_block_data,
                            );
                    });
            }

            if datasets.utxo.needs_one_shot_states(height, date) {
                utxo_cohorts_one_shot_states = states
                    .utxo_cohorts_durable_states
                    .as_ref()
                    .unwrap()
                    .compute_one_shot_states(
                        block_price,
                        if is_date_last_block {
                            Some(date_price)
                        } else {
                            None
                        },
                    );
            }
        });

        // scope.spawn(|| {
        //     utxo_cohorts_received_states
        //         .compute(&states.date_data_vec, block_path_to_received_data);
        // });

        if datasets.utxo.needs_sent_states(height, date) {
            scope.spawn(|| {
                utxo_cohorts_sent_states.compute(
                    &states.date_data_vec,
                    &block_path_to_sent_data,
                    block_price,
                    timestamp,
                );
            });
        }

        if compute_addresses {
            scope.spawn(|| {
                let address_index_to_address_data = address_index_to_address_data.as_ref().unwrap();

                // TODO: Only compute if needed
                address_cohorts_realized_states.replace(AddressCohortsRealizedStates::default());

                // TODO: Only compute if needed
                address_cohorts_input_states.replace(AddressCohortsInputStates::default());

                // TODO: Only compute if needed
                address_cohorts_output_states.replace(AddressCohortsOutputStates::default());

                address_index_to_address_realized_data.iter().for_each(
                    |(address_index, address_realized_data)| {
                        let current_address_data =
                            address_index_to_address_data.get(address_index).unwrap();

                        states
                            .address_cohorts_durable_states
                            .as_mut()
                            .unwrap()
                            .iterate(address_realized_data, current_address_data)
                            .unwrap_or_else(|report| {
                                dbg!(report.to_string(), address_index);
                                panic!();
                            });

                        if !address_realized_data.initial_address_data.is_empty() {
                            // Realized == previous amount
                            // If a whale sent all its sats to another address at a loss, it's the whale that realized the loss not the now empty adress
                            let liquidity_classification = address_realized_data
                                .initial_address_data
                                .compute_liquidity_classification();

                            address_cohorts_realized_states
                                .as_mut()
                                .unwrap()
                                .iterate_realized(address_realized_data, &liquidity_classification)
                                .unwrap();

                            address_cohorts_input_states
                                .as_mut()
                                .unwrap()
                                .iterate_input(address_realized_data, &liquidity_classification)
                                .unwrap();
                        }

                        address_cohorts_output_states
                            .as_mut()
                            .unwrap()
                            .iterate_output(
                                address_realized_data,
                                &current_address_data.compute_liquidity_classification(),
                            )
                            .unwrap();
                    },
                );

                address_cohorts_one_shot_states.replace(
                    states
                        .address_cohorts_durable_states
                        .as_ref()
                        .unwrap()
                        .compute_one_shot_states(
                            block_price,
                            if is_date_last_block {
                                Some(date_price)
                            } else {
                                None
                            },
                        ),
                );
            });
        }
    });

    if compute_addresses {
        address_index_to_address_data.unwrap().into_iter().for_each(
            |(address_index, address_data)| {
                if address_data.is_empty() {
                    databases.address_index_to_empty_address_data.unsafe_insert(
                        address_index,
                        EmptyAddressData::from_non_empty(&address_data),
                    );
                } else {
                    databases
                        .address_index_to_address_data
                        .unsafe_insert(address_index, address_data);
                }
            },
        )
    }

    datasets.insert(InsertData {
        address_cohorts_input_states: &address_cohorts_input_states,
        block_size,
        block_vbytes,
        block_weight,
        address_cohorts_one_shot_states: &address_cohorts_one_shot_states,
        address_cohorts_realized_states: &address_cohorts_realized_states,
        block_interval,
        block_price,
        coinbase,
        compute_addresses,
        databases,
        date,
        date_blocks_range: &(*first_date_height..=*height),
        date_first_height: first_date_height,
        difficulty,
        fees: &fees,
        height,
        is_date_last_block,
        satblocks_destroyed,
        satdays_destroyed,
        amount_sent,
        states,
        timestamp,
        transaction_count,
        utxo_cohorts_one_shot_states: &utxo_cohorts_one_shot_states,
        utxo_cohorts_sent_states: &utxo_cohorts_sent_states,
    });
}

pub struct TxoutsParsingResults {
    partial_txout_data_vec: Vec<Option<PartialTxoutData>>,
    provably_unspendable: Amount,
    op_returns: usize,
}

#[allow(clippy::too_many_arguments)]
fn prepare_outputs(
    block: &Block,
    compute_addresses: bool,
    multisig_addresses: &mut Counter,
    op_return_addresses: &mut Counter,
    push_only_addresses: &mut Counter,
    unknown_addresses: &mut Counter,
    empty_addresses: &mut Counter,
    address_to_address_index: &mut AddressToAddressIndex,
) -> TxoutsParsingResults {
    let mut provably_unspendable = Amount::ZERO;
    let mut op_returns = 0;

    let mut partial_txout_data_vec = block
        .txdata
        .iter()
        .flat_map(|tx| &tx.output)
        .map(|txout| {
            let script = &txout.script_pubkey;
            let amount = Amount::wrap(txout.value);

            // 0 sats outputs are possible and allowed !
            // https://mempool.space/tx/2f2442f68e38b980a6c4cec21e71851b0d8a5847d85208331a27321a9967bbd6
            // https://bitcoin.stackexchange.com/questions/104937/transaction-outputs-with-value-0
            if amount == Amount::ZERO {
                return None;
            }

            // Op Return
            // https://mempool.space/tx/139c004f477101c468767983536caaeef568613fab9c2ed9237521f5ff530afd
            // Provably unspendable https://mempool.space/tx/8a68c461a2473653fe0add786f0ca6ebb99b257286166dfb00707be24716af3a#flow=&vout=0
            #[allow(deprecated)]
            if script.is_op_return() {
                // TODO: Count fee paid to write said OP_RETURN, beware of coinbase transactions
                // For coinbase transactions, count miners
                op_returns += 1;
                provably_unspendable += amount;

                // return None;
                // https://mempool.space/tx/8a68c461a2473653fe0add786f0ca6ebb99b257286166dfb00707be24716af3a#flow=&vout=0
            } else if script.is_provably_unspendable() {
                provably_unspendable += amount;
                // return None;
            }

            let address_opt = compute_addresses.then(|| {
                let address = Address::from(
                    txout,
                    multisig_addresses,
                    op_return_addresses,
                    push_only_addresses,
                    unknown_addresses,
                    empty_addresses,
                );

                address_to_address_index.open_db(&address);

                address
            });

            Some(PartialTxoutData::new(address_opt, amount, None))
        })
        .collect_vec();

    if compute_addresses {
        partial_txout_data_vec
            .par_iter_mut()
            .for_each(|partial_tx_out_data| {
                if let Some(partial_tx_out_data) = partial_tx_out_data {
                    let address_index_opt = address_to_address_index
                        .unsafe_get(partial_tx_out_data.address.as_ref().unwrap())
                        .cloned();

                    partial_tx_out_data.address_index_opt = address_index_opt;
                }
            });
    }

    TxoutsParsingResults {
        partial_txout_data_vec,
        provably_unspendable,
        op_returns,
    }
}

#[allow(clippy::type_complexity)]
fn prepare_inputs<'a>(
    block: &'a Block,
    txid_to_tx_data_db: &mut TxidToTxData,
    txout_index_to_amount_db: &mut TxoutIndexToAmount,
    txout_index_to_address_index_db: &mut TxoutIndexToAddressIndex,
    compute_addresses: bool,
) -> (
    BTreeMap<&'a Txid, Option<TxData>>,
    BTreeMap<TxoutIndex, (Amount, Option<u32>)>,
) {
    let mut txid_to_tx_data: BTreeMap<&Txid, Option<TxData>> = block
        .txdata
        .iter()
        .skip(1) // Skip coinbase transaction
        .flat_map(|transaction| &transaction.input)
        .fold(BTreeMap::default(), |mut tree, tx_in| {
            let txid = &tx_in.previous_output.txid;

            txid_to_tx_data_db.open_db(txid);

            tree.entry(txid).or_default();

            tree
        });

    let mut tx_datas = txid_to_tx_data
        .par_iter()
        .map(|(txid, _)| txid_to_tx_data_db.unsafe_get(txid))
        .collect::<Vec<_>>();

    txid_to_tx_data.values_mut().rev().for_each(|tx_data_opt| {
        *tx_data_opt = tx_datas.pop().unwrap().cloned();
    });

    let txout_index_to_amount_and_address_index = block
        .txdata
        .iter()
        .skip(1) // Skip coinbase transaction
        .flat_map(|transaction| &transaction.input)
        .flat_map(|tx_in| {
            let txid = &tx_in.previous_output.txid;

            if let Some(Some(tx_data)) = txid_to_tx_data.get(txid) {
                let txout_index = TxoutIndex::new(tx_data.index, tx_in.previous_output.vout as u16);

                txout_index_to_amount_db.open_db(&txout_index);

                if compute_addresses {
                    txout_index_to_address_index_db.open_db(&txout_index);
                }

                Some(txout_index)
            } else {
                None
            }
        })
        .collect_vec()
        .into_par_iter()
        .flat_map(|txout_index| {
            txout_index_to_amount_db
                .unsafe_get(&txout_index)
                // Will be None if value of utxo is 0
                // https://mempool.space/tx/9d8a0d851c9fb2cdf1c6d9406ce97e19e6911ae3503ab2dd5f38640bacdac996
                // which is used later as input
                .map(|amount| {
                    let address_index = compute_addresses.then(|| {
                        *txout_index_to_address_index_db
                            .unsafe_get(&txout_index)
                            .unwrap()
                    });

                    (txout_index, (*amount, address_index))
                })
        })
        .collect::<BTreeMap<_, _>>();

    // No need to call remove, it's being called later in the parse function
    // To more easily support removing cached puts

    (txid_to_tx_data, txout_index_to_amount_and_address_index)
}

fn compute_address_index_to_address_data(
    address_index_to_address_data_db: &mut AddressIndexToAddressData,
    address_index_to_empty_address_data_db: &mut AddressIndexToEmptyAddressData,
    partial_txout_data_vec: &[Option<PartialTxoutData>],
    txout_index_to_amount_and_address_index: &BTreeMap<TxoutIndex, (Amount, Option<u32>)>,
    compute_addresses: bool,
) -> BTreeMap<u32, AddressData> {
    if !compute_addresses {
        return BTreeMap::default();
    }

    let mut address_index_to_address_data = partial_txout_data_vec
        .iter()
        .flatten()
        .flat_map(|partial_txout_data| partial_txout_data.address_index_opt)
        .map(|address_index| (address_index, true))
        .chain(
            txout_index_to_amount_and_address_index
                .values()
                .map(|(_, address_index)| (*address_index.as_ref().unwrap(), false)), // False because we assume non zero inputs values
        )
        .map(|(address_index, open_empty)| {
            address_index_to_address_data_db.open_db(&address_index);

            if open_empty {
                address_index_to_empty_address_data_db.open_db(&address_index);
            }

            (address_index, AddressData::default())
        })
        .collect::<BTreeMap<_, _>>();

    address_index_to_address_data
        .par_iter_mut()
        .for_each(|(address_index, address_data)| {
            if let Some(_address_data) =
                address_index_to_address_data_db.unsafe_get_from_cache(address_index)
            {
                _address_data.clone_into(address_data);
            } else if let Some(empty_address_data) =
                address_index_to_empty_address_data_db.unsafe_get_from_cache(address_index)
            {
                *address_data = AddressData::from_empty(empty_address_data);
            } else if let Some(_address_data) =
                address_index_to_address_data_db.unsafe_get_from_db(address_index)
            {
                _address_data.clone_into(address_data);
            } else {
                let empty_address_data = address_index_to_empty_address_data_db
                    .unsafe_get_from_db(address_index)
                    .unwrap();

                *address_data = AddressData::from_empty(empty_address_data);
            }
        });

    // Parallel unsafe_get + Linear remove = Parallel-ish take
    address_index_to_address_data
        .iter()
        .for_each(|(address_index, address_data)| {
            if address_data.is_empty() {
                address_index_to_empty_address_data_db.remove(address_index);
            } else {
                address_index_to_address_data_db.remove(address_index);
            }
        });

    address_index_to_address_data
}
