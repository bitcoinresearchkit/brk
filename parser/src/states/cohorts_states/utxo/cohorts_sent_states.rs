use std::{cmp::Ordering, collections::BTreeMap};

use chrono::Datelike;
use derive_deref::{Deref, DerefMut};

use crate::{
    states::{DateDataVec, InputState, RealizedState},
    structs::{BlockPath, Price, SentData, Timestamp},
};

use super::SplitByUTXOCohort;

#[derive(Default, Debug)]
pub struct SentState {
    pub input: InputState,
    pub realized: RealizedState,
}

#[derive(Deref, DerefMut, Default)]
pub struct UTXOCohortsSentStates(SplitByUTXOCohort<SentState>);

impl UTXOCohortsSentStates {
    pub fn compute(
        &mut self,
        date_data_vec: &DateDataVec,
        block_path_to_sent_data: &BTreeMap<BlockPath, SentData>,
        current_price: Price,
        current_timestamp: Timestamp,
    ) {
        if let Some(last_block_data) = date_data_vec.last_block() {
            block_path_to_sent_data
                .iter()
                .for_each(|(block_path, sent_data)| {
                    let date_data = date_data_vec.get_date_data(block_path).unwrap();

                    let year = date_data.date.year() as u32;

                    let block_data = date_data.get_block_data(block_path).unwrap();

                    let days_old = Timestamp::difference_in_days_between(
                        block_data.timestamp,
                        last_block_data.timestamp,
                    );

                    let previous_timestamp = block_data.timestamp;
                    let previous_price = block_data.price;

                    let amount_sent = sent_data.volume;

                    self.initial_filtered_apply(&days_old, &year, |state| {
                        state.input.iterate(sent_data.count as f64, amount_sent);

                        let previous_value = previous_price * amount_sent;
                        let current_value = current_price * amount_sent;

                        let mut realized_profit = Price::ZERO;
                        let mut realized_loss = Price::ZERO;
                        let value_created = current_value;
                        let mut adjusted_value_created = Price::ZERO;
                        let value_destroyed = previous_value;
                        let mut adjusted_value_destroyed = Price::ZERO;

                        match previous_value.cmp(&current_value) {
                            Ordering::Less => realized_profit = current_value - previous_value,
                            Ordering::Greater => {
                                realized_loss = previous_value - current_value;
                            }
                            Ordering::Equal => {}
                        }

                        if previous_timestamp.older_by_1h_plus_than(current_timestamp) {
                            adjusted_value_created = value_created;
                            adjusted_value_destroyed = value_destroyed;
                        }

                        state.realized.iterate(
                            realized_profit,
                            realized_loss,
                            value_created,
                            adjusted_value_created,
                            value_destroyed,
                            adjusted_value_destroyed,
                        );
                    })
                })
        }
    }
}
