use std::{collections::BTreeSet, time::Instant};

use export::ExportedData;
use itertools::Itertools;

use parse::ParseData;

use crate::{
    actions::{export, find_first_inserted_unsafe_height, parse},
    create_rpc,
    databases::Databases,
    datasets::{AllDatasets, ComputeData},
    io::OUTPUTS_FOLDER_PATH,
    states::{AddressCohortsDurableStates, States, UTXOCohortsDurableStates},
    structs::{DateData, MapKey, Timestamp},
    utils::{generate_allocation_files, log, time},
    Config, Exit, Height,
};

pub fn iter_blocks(
    config: &mut Config,
    rpc: &biter::bitcoincore_rpc::Client,
    approx_block_count: usize,
    exit: Exit,
) -> color_eyre::Result<()> {
    log("Starting...");

    let mut datasets = AllDatasets::import(config)?;

    log("Imported datasets");

    let mut databases = Databases::import();

    if config.first_defragment() {
        databases.defragment(&exit);
        config.disable_defragment();
    }

    log("Imported databases");

    let mut states = States::import().unwrap_or_default();

    log("Imported states");

    let first_unsafe_heights =
        find_first_inserted_unsafe_height(&mut states, &mut databases, &mut datasets);

    let mut height = first_unsafe_heights.min();

    log(&format!("Starting parsing at height: {height}"));

    let mut next_block_opt = None;
    let mut blocks_loop_date = None;

    let block_receiver = biter::new(
        config.datadir.as_ref().unwrap(),
        OUTPUTS_FOLDER_PATH,
        Some(height.to_usize()),
        None,
        create_rpc(config).unwrap(),
    );

    let mut block_iter = block_receiver.iter();

    'parsing: loop {
        let instant = Instant::now();

        let mut processed_heights = BTreeSet::new();
        let mut processed_dates = BTreeSet::new();

        'days: loop {
            let mut blocks_loop_i = 0;

            if next_block_opt.is_some() {
                blocks_loop_date.take();
            }

            'blocks: loop {
                let current_block_opt = next_block_opt.take().or_else(|| block_iter.next());

                next_block_opt = block_iter.next();

                if let Some((_current_block_height, current_block, _current_block_hash)) =
                    current_block_opt
                {
                    let timestamp = Timestamp::wrap(current_block.header.time);

                    let current_block_date = timestamp.to_date();
                    let current_block_height: Height = height + blocks_loop_i;

                    if current_block_height.to_usize() != _current_block_height {
                        dbg!(current_block_height, _current_block_height);
                        panic!()
                    }

                    let next_block_date = next_block_opt.as_ref().map(|(_, next_block, _)| {
                        Timestamp::wrap(next_block.header.time).to_date()
                    });

                    // Always run for the first block of the loop
                    if blocks_loop_date.is_none() {
                        log(&format!(
                            "Processing {current_block_date} (height: {height})..."
                        ));

                        blocks_loop_date.replace(current_block_date);

                        if states
                            .date_data_vec
                            .last()
                            .map(|date_data| *date_data.date < *current_block_date)
                            .unwrap_or(true)
                        {
                            states
                                .date_data_vec
                                .push(DateData::new(current_block_date, vec![]));
                        }

                        processed_dates.insert(current_block_date);
                    }

                    let blocks_loop_date = blocks_loop_date.unwrap();

                    if current_block_date > blocks_loop_date {
                        panic!("current block should always have the same date as the current blocks loop");
                    }

                    let is_date_last_block = next_block_date
                        // Do NOT change `blocks_loop_date` to `current_block_date` !!!
                        .map_or(true, |next_block_date| blocks_loop_date < next_block_date);

                    processed_heights.insert(current_block_height);

                    if first_unsafe_heights.inserted <= current_block_height {
                        let compute_addresses = databases.check_if_needs_to_compute_addresses(
                            current_block_height,
                            blocks_loop_date,
                        );

                        if states.address_cohorts_durable_states.is_none()
                            && (compute_addresses
                                || datasets
                                    .address
                                    .needs_durable_states(current_block_height, current_block_date))
                        {
                            states.address_cohorts_durable_states =
                                Some(AddressCohortsDurableStates::init(
                                    &mut databases.address_index_to_address_data,
                                ));
                        }

                        if states.utxo_cohorts_durable_states.is_none()
                            && datasets
                                .utxo
                                .needs_durable_states(current_block_height, current_block_date)
                        {
                            states.utxo_cohorts_durable_states =
                                Some(UTXOCohortsDurableStates::init(&states.date_data_vec));
                        }

                        parse(ParseData {
                            rpc,
                            block: current_block,
                            block_index: blocks_loop_i,
                            compute_addresses,
                            databases: &mut databases,
                            datasets: &mut datasets,
                            date: blocks_loop_date,
                            first_date_height: height,
                            height: current_block_height,
                            is_date_last_block,
                            states: &mut states,
                        });
                    }

                    blocks_loop_i += 1;

                    if is_date_last_block {
                        height += blocks_loop_i;

                        let is_check_point = next_block_date
                            .as_ref()
                            .map_or(true, |date| date.is_first_of_month());

                        let ran_for_at_least_a_minute = instant.elapsed().as_secs() >= 60;

                        if (is_check_point && ran_for_at_least_a_minute)
                            || height.is_close_to_end(approx_block_count)
                        {
                            break 'days;
                        }

                        break 'blocks;
                    }
                } else {
                    break 'parsing;
                }
            }
        }

        // Don't remember why -1
        let last_height = height - 1_u32;

        log(&format!(
            "Parsing group took {} seconds (last height: {last_height})\n",
            instant.elapsed().as_secs_f32(),
        ));

        if first_unsafe_heights.computed <= last_height {
            log("Computing datasets...");
            time("Computing datasets", || {
                let dates = processed_dates.into_iter().collect_vec();

                let heights = processed_heights.into_iter().collect_vec();

                datasets.compute(ComputeData {
                    dates: &dates,
                    heights: &heights,
                })
            });
        }

        if !config.dry_run() {
            let is_safe = height.is_safe(approx_block_count);

            export(ExportedData {
                databases: is_safe.then_some(&mut databases),
                datasets: &mut datasets,
                date: blocks_loop_date.unwrap(),
                height: last_height,
                states: is_safe.then_some(&states),
                exit: exit.clone(),
            })?;

            if config.record_ram_usage() {
                time("Exporing allocation files", || {
                    generate_allocation_files(&datasets, &databases, &states, last_height)
                })?;
            }
        } else {
            log("Skipping export");
        }

        println!();
    }

    Ok(())
}
