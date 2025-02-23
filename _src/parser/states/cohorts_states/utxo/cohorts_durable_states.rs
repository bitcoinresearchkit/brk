use allocative::Allocative;
use derive_deref::{Deref, DerefMut};
use rayon::prelude::*;

use crate::{
    parser::states::DateDataVec,
    structs::{Amount, BlockData, Price, SentData, Timestamp},
};

use super::{SplitByUTXOCohort, UTXOCohortDurableStates, UTXOCohortsOneShotStates};

#[derive(Default, Deref, DerefMut, Allocative)]
pub struct UTXOCohortsDurableStates(SplitByUTXOCohort<UTXOCohortDurableStates>);

impl UTXOCohortsDurableStates {
    pub fn init(date_data_vec: &DateDataVec) -> Self {
        let mut s = Self::default();

        if let Some(last_block_data) = date_data_vec.last_block() {
            date_data_vec.iter().for_each(|date_data| {
                date_data.blocks.iter().for_each(|block_data| {
                    let amount = block_data.amount;
                    let utxo_count = block_data.utxos as f64;

                    // No need to either insert or remove if 0
                    if amount == Amount::ZERO {
                        return;
                    }

                    let increment_days_old = Timestamp::difference_in_days_between(
                        block_data.timestamp,
                        last_block_data.timestamp,
                    );

                    s.initial_filtered_apply(&increment_days_old, &block_data.height, |state| {
                        state
                            .increment(amount, utxo_count, block_data.price)
                            .unwrap();
                    });
                })
            });
        }

        s
    }

    pub fn udpate_age_if_needed(
        &mut self,
        block_data: &BlockData,
        last_block_data: &BlockData,
        previous_last_block_data: Option<&BlockData>,
    ) {
        let height = block_data.height;
        let amount = block_data.amount;
        let utxo_count = block_data.utxos as f64;
        let price = block_data.price;

        // No need to either insert or remove if 0
        if amount == Amount::ZERO {
            return;
        }

        if block_data.height == last_block_data.height {
            self.initial_filtered_apply(&0, &height, |state| {
                state.increment(amount, utxo_count, price).unwrap();
            })
        } else {
            let increment_days_old = Timestamp::difference_in_days_between(
                block_data.timestamp,
                last_block_data.timestamp,
            );

            let decrement_days_old = Timestamp::difference_in_days_between(
                block_data.timestamp,
                previous_last_block_data
                    .unwrap_or_else(|| {
                        dbg!(block_data, last_block_data, previous_last_block_data);
                        panic!()
                    })
                    .timestamp,
            );

            if increment_days_old == decrement_days_old {
                return;
            }

            self.duo_filtered_apply(
                &increment_days_old,
                &decrement_days_old,
                |state| {
                    state.increment(amount, utxo_count, price).unwrap();
                },
                |state| {
                    state.decrement(amount, utxo_count, price).unwrap();
                },
            );
        }
    }

    pub fn subtract_moved(
        &mut self,
        block_data: &BlockData,
        sent_data: &SentData,
        previous_last_block_data: &BlockData,
    ) {
        let amount = sent_data.volume;
        let utxo_count = sent_data.count as f64;

        // No need to either insert or remove if 0
        if amount == Amount::ZERO {
            return;
        }

        let days_old = Timestamp::difference_in_days_between(
            block_data.timestamp,
            previous_last_block_data.timestamp,
        );

        self.initial_filtered_apply(&days_old, &block_data.height, |state| {
            state
                .decrement(amount, utxo_count, block_data.price)
                .unwrap_or_else(|report| {
                    dbg!(
                        report.to_string(),
                        block_data,
                        sent_data,
                        previous_last_block_data
                    );
                    panic!()
                });
        })
    }

    pub fn compute_one_shot_states(
        &self,
        block_price: Price,
        date_price: Option<Price>,
    ) -> UTXOCohortsOneShotStates {
        let mut one_shot_states = UTXOCohortsOneShotStates::default();

        self.as_vec()
            .into_par_iter()
            .map(|(states, id)| (states.compute_one_shot_states(block_price, date_price), id))
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(states, id)| {
                *one_shot_states.get_mut(&id) = states;
            });

        one_shot_states
    }
}
