use std::{collections::BTreeSet, time::Instant};

use biter::bitcoincore_rpc::Client;
use chrono::Datelike;
use export::ExportedData;
use itertools::Itertools;

use log::info;
use parse::ParseData;

use crate::{
    parser::{
        actions::{export, find_first_inserted_unsafe_height, parse},
        databases::Databases,
        datasets::{ComputeData, Datasets},
        states::{AddressCohortsDurableStates, States, UTXOCohortsDurableStates},
    },
    structs::{Config, DateData, DisplayInstant, Exit, Height, MapKey, Timestamp},
    utils::{generate_allocation_files, time},
};

pub fn iter_blocks(
    config: &Config,
    rpc: &Client,
    approx_block_count: usize,
    exit: Exit,
    databases: &mut Databases,
    datasets: &mut Datasets,
) -> color_eyre::Result<()> {
    let mut states = States::import(config).unwrap_or_default();

    info!("Imported states");

    let first_unsafe_heights =
        find_first_inserted_unsafe_height(&mut states, databases, datasets, config);

    let mut height = first_unsafe_heights.min();

    info!("Starting parsing at height: {height}");

    let mut next_block_opt = None;
    let mut blocks_loop_date = None;
    let mut next_date_opt;

    let block_receiver = biter::new(
        &config.path_bitcoindir(),
        Some(height.to_usize()),
        None,
        Client::from(config),
    );

    let mut block_iter = block_receiver.iter();

    'parsing: loop {
        let mut processed_heights = BTreeSet::new();
        let mut processed_dates = BTreeSet::new();

        'days: loop {
            let mut blocks_loop_i = 0;

            if next_block_opt.is_some() {
                blocks_loop_date.take();
            }

            let instant = Instant::now();

            'blocks: loop {
                let current_block_opt = next_block_opt.take().or_else(|| block_iter.next());

                next_block_opt = block_iter.next();

                if let Some((_current_block_height, current_block, _current_block_hash)) =
                    current_block_opt
                {
                    let timestamp = Timestamp::from(current_block.header.time);

                    let current_block_date = timestamp.to_date();
                    let current_block_height: Height = height + blocks_loop_i;

                    if current_block_height.to_usize() != _current_block_height {
                        dbg!(current_block_height, _current_block_height);
                        panic!()
                    }

                    next_date_opt = next_block_opt.as_ref().map(|(_, next_block, _)| {
                        Timestamp::from(next_block.header.time).to_date()
                    });

                    // Always run for the first block of the loop
                    if blocks_loop_date.is_none() {
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

                    let is_date_last_block = next_date_opt
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
                            block: current_block,
                            block_index: blocks_loop_i,
                            compute_addresses,
                            config,
                            databases,
                            datasets,
                            date: blocks_loop_date,
                            first_date_height: height,
                            height: current_block_height,
                            is_date_last_block,
                            rpc,
                            states: &mut states,
                        });
                    }

                    blocks_loop_i += 1;

                    if is_date_last_block {
                        info!(
                            "Processed {current_block_date} ({height} - {current_block_height}) {}",
                            instant.display()
                        );

                        height += blocks_loop_i;

                        let is_check_point = next_date_opt
                            .as_ref()
                            .map_or(true, |date| date.is_first_of_month());

                        if (is_check_point && instant.elapsed().as_secs() >= 1)
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

        if first_unsafe_heights.computed <= last_height {
            info!("Computing datasets...");
            time("Computed datasets", || {
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

            let defragment = is_safe
                && next_date_opt.is_some_and(|date| {
                    (date.year() >= 2020 && date.is_january()
                        || date.year() >= 2022 && date.is_july())
                        && date.is_first_of_month()
                });

            export(ExportedData {
                config,
                databases: is_safe.then_some(databases),
                datasets,
                date: blocks_loop_date.unwrap(),
                defragment,
                height: last_height,
                states: is_safe.then_some(&states),
                exit: exit.clone(),
            })?;

            if config.record_ram_usage() {
                time("Exporting allocation files", || {
                    generate_allocation_files(datasets, databases, &states, last_height)
                })?;
            }
        } else {
            info!("Skipping export");
        }
    }

    Ok(())
}
