use log::info;

use crate::{
    parser::{
        databases::Databases,
        datasets::{AnyDatasets, Datasets},
        states::States,
    },
    structs::{Config, Height},
};

#[derive(Default, Debug)]
pub struct Heights {
    pub inserted: Height,
    pub computed: Height,
}

impl Heights {
    pub fn min(&self) -> Height {
        self.inserted.min(self.computed)
    }
}

pub fn find_first_inserted_unsafe_height(
    states: &mut States,
    databases: &mut Databases,
    datasets: &mut Datasets,
    config: &Config,
) -> Heights {
    let min_initial_inserted_last_address_height = datasets
        .address
        .get_min_initial_states()
        .inserted
        .last_height
        .as_ref()
        .cloned();

    let min_initial_inserted_last_address_date = datasets
        .address
        .get_min_initial_states()
        .inserted
        .last_date
        .as_ref()
        .cloned();

    let usable_databases = databases.check_if_usable(
        min_initial_inserted_last_address_height,
        min_initial_inserted_last_address_date,
    );

    states
        .date_data_vec
        .iter()
        .last()
        .map(|date_data| date_data.date)
        .and_then(|last_safe_date| {
            if !usable_databases {
                info!("Unusable databases");

                return None;
            }

            let datasets_min_initial_states = datasets.get_min_initial_states().to_owned();

            let min_datasets_inserted_last_height = datasets_min_initial_states.inserted.last_height;
            let min_datasets_inserted_last_date = datasets_min_initial_states.inserted.last_date;

            info!("min_datasets_inserted_last_height: {:?}", min_datasets_inserted_last_height);
            info!("min_datasets_inserted_last_date: {:?}", min_datasets_inserted_last_date);

            let inserted_last_date_is_older_than_saved_state = min_datasets_inserted_last_date.map_or(true, |min_datasets_last_date| min_datasets_last_date < last_safe_date);

            if inserted_last_date_is_older_than_saved_state {
                // dbg!(min_datasets_inserted_last_date , *last_safe_date);

                return None;
            }

            datasets
                .date_metadata
                .last_height
                .get_or_import(&last_safe_date)
                .and_then(|last_safe_height| {
                    let inserted_heights_and_dates_are_out_of_sync = min_datasets_inserted_last_height.map_or(true, |min_datasets_inserted_last_height| min_datasets_inserted_last_height < last_safe_height);

                    if inserted_heights_and_dates_are_out_of_sync {
                        info!("last_safe_height ({last_safe_height}) > min_datasets_height ({min_datasets_inserted_last_height:?})");

                        None
                    } else {
                        let computed = datasets_min_initial_states.computed.last_date.and_then(
                            |last_date| datasets.date_metadata
                                .last_height
                                .get_or_import(&last_date)
                                .and_then(|last_date_height| {
                                    if datasets_min_initial_states.computed.last_height.map_or(true, |last_height| {
                                        last_height < last_date_height
                                    }) {
                                        None
                                    } else {
                                        Some(last_date_height + 1_u32)
                                    }
                                })
                        ).unwrap_or_default();

                        Some(Heights {
                            inserted: last_safe_height + 1_u32,
                            computed,
                        })
                    }
                }
            )
        })
        .unwrap_or_else(|| {
            info!("Starting over...");

            let include_addresses = !usable_databases
                || min_initial_inserted_last_address_date.is_none()
                || min_initial_inserted_last_address_height.is_none();

            states.reset(config, include_addresses);

            databases.reset(include_addresses);

            Heights::default()
        })
}
