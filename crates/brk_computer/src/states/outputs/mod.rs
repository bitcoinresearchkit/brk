use brk_vec::StoredIndex;
use rayon::prelude::*;
use std::{collections::BTreeMap, ops::ControlFlow};

use brk_core::{CheckedSub, Dollars, HalvingEpoch, Height, Timestamp};

mod by_epoch;
mod by_from;
mod by_range;
mod by_size;
mod by_spendable_type;
mod by_term;
mod by_type;
mod by_unspendable_type;
mod by_up_to;
// mod by_value;
mod filter;

pub use by_epoch::*;
pub use by_from::*;
pub use by_range::*;
pub use by_size::*;
pub use by_spendable_type::*;
pub use by_term::*;
pub use by_type::*;
pub use by_unspendable_type::*;
pub use by_up_to::*;
// pub use by_value::*;
pub use filter::*;

use crate::vecs;

use super::{BlockState, Transacted};

#[derive(Default, Clone)]
pub struct Outputs<T> {
    pub all: T,
    pub by_term: OutputsByTerm<T>,
    pub by_up_to: OutputsByUpTo<T>,
    pub by_from: OutputsByFrom<T>,
    pub by_range: OutputsByRange<T>,
    pub by_epoch: OutputsByEpoch<T>,
    pub by_type: OutputsBySpendableType<T>,
    pub by_size: OutputsBySize<T>,
    // // Needs whole UTXO set, TODO later
    // // pub by_value: OutputsByValue<T>,
}

impl<T> Outputs<T> {
    pub fn as_mut_vec(&mut self) -> Vec<&mut T> {
        [&mut self.all]
            .into_iter()
            .chain(self.by_term.as_mut_vec())
            .chain(self.by_up_to.as_mut_vec())
            .chain(self.by_from.as_mut_vec())
            .chain(self.by_range.as_mut_vec())
            .chain(self.by_epoch.as_mut_vec())
            .chain(self.by_size.as_mut_vec())
            .chain(self.by_type.as_mut_vec())
            // // .chain(self.by_value.as_mut_vec())
            .collect::<Vec<_>>()
    }
}

impl Outputs<(OutputFilter, vecs::statefull::cohort::Vecs)> {
    pub fn tick_tock_next_block(&mut self, chain_state: &[BlockState], timestamp: Timestamp) {
        if chain_state.is_empty() {
            return;
        }

        let prev_timestamp = chain_state.last().unwrap().timestamp;

        self.by_term
            .as_mut_vec()
            .into_par_iter()
            .chain(self.by_up_to.as_mut_vec())
            .chain(self.by_from.as_mut_vec())
            .chain(self.by_range.as_mut_vec())
            .for_each(|(filter, v)| {
                let state = &mut v.state;

                let mut check_days_old = |days_old: usize| -> bool {
                    match filter {
                        OutputFilter::From(from) => *from <= days_old,
                        OutputFilter::To(to) => *to > days_old,
                        OutputFilter::Range(range) => range.contains(&days_old),
                        OutputFilter::All
                        | OutputFilter::Epoch(_)
                        | OutputFilter::Size
                        | OutputFilter::Type(_) => unreachable!(),
                    }
                };

                let _ = chain_state
                    .iter()
                    .try_for_each(|block_state| -> ControlFlow<()> {
                        let prev_days_old = block_state
                            .timestamp
                            .difference_in_days_between(prev_timestamp);
                        let days_old = block_state.timestamp.difference_in_days_between(timestamp);

                        if prev_days_old == days_old {
                            return ControlFlow::Continue(());
                        }

                        let is = check_days_old(days_old);
                        let was = check_days_old(prev_days_old);

                        if is && !was {
                            state.increment(&block_state.supply, block_state.price);
                        } else if was && !is {
                            state.decrement(&block_state.supply, block_state.price);
                        }

                        ControlFlow::Continue(())
                    });
            });
    }

    pub fn send(
        &mut self,
        height_to_sent: BTreeMap<Height, Transacted>,
        chain_state: &[BlockState],
    ) {
        let mut time_based_vecs = self
            .by_term
            .as_mut_vec()
            .into_iter()
            .chain(self.by_up_to.as_mut_vec())
            .chain(self.by_from.as_mut_vec())
            .chain(self.by_range.as_mut_vec())
            .chain(self.by_epoch.as_mut_vec())
            .collect::<Vec<_>>();

        let last_timestamp = chain_state.last().unwrap().timestamp;
        let current_price = chain_state.last().unwrap().price;

        height_to_sent.into_iter().for_each(|(height, sent)| {
            let block_state = chain_state.get(height.unwrap_to_usize()).unwrap();
            let prev_price = block_state.price;

            let days_old = block_state
                .timestamp
                .difference_in_days_between(last_timestamp);

            let older_than_hour =
                jiff::Timestamp::from(last_timestamp.checked_sub(block_state.timestamp).unwrap())
                    .as_second()
                    >= 60 * 60;

            self.all.1.state.send(
                &sent.spendable_supply,
                current_price,
                prev_price,
                older_than_hour,
            );

            time_based_vecs
                .iter_mut()
                .filter(|(filter, _)| match filter {
                    OutputFilter::From(from) => *from <= days_old,
                    OutputFilter::To(to) => *to > days_old,
                    OutputFilter::Range(range) => range.contains(&days_old),
                    OutputFilter::Epoch(epoch) => *epoch == HalvingEpoch::from(height),
                    _ => unreachable!(),
                })
                .for_each(|(_, vecs)| {
                    vecs.state.send(
                        &sent.spendable_supply,
                        current_price,
                        prev_price,
                        older_than_hour,
                    );
                });

            sent.by_type.spendable.as_typed_vec().into_iter().for_each(
                |(output_type, supply_state)| {
                    self.by_type.get_mut(output_type).1.state.send(
                        supply_state,
                        current_price,
                        prev_price,
                        older_than_hour,
                    )
                },
            );

            sent.by_size.into_iter().for_each(|(group, supply_state)| {
                self.by_size.get_mut(group).1.state.send(
                    &supply_state,
                    current_price,
                    prev_price,
                    older_than_hour,
                );
            });
        });
    }

    pub fn receive(&mut self, received: Transacted, height: Height, price: Option<Dollars>) {
        let supply_state = received.spendable_supply;

        [
            &mut self.all.1,
            &mut self.by_term.short.1,
            &mut self.by_epoch.mut_vec_from_height(height).1,
            // Skip from and range as can't receive in the past
        ]
        .into_iter()
        .chain(self.by_up_to.as_mut_vec().map(|(_, v)| v))
        .for_each(|v| {
            v.state.receive(&supply_state, price);
        });

        self.by_type
            .as_mut_vec()
            .into_iter()
            .for_each(|(filter, vecs)| {
                let output_type = match filter {
                    OutputFilter::Type(output_type) => *output_type,
                    _ => unreachable!(),
                };
                vecs.state.receive(received.by_type.get(output_type), price)
            });

        received
            .by_size
            .into_iter()
            .for_each(|(group, supply_state)| {
                self.by_size
                    .get_mut(group)
                    .1
                    .state
                    .receive(&supply_state, price);
            });
    }
}

impl<T> Outputs<(OutputFilter, T)> {
    pub fn vecs(&self) -> Vec<&T> {
        [&self.all.1]
            .into_iter()
            .chain(self.by_term.vecs())
            .chain(self.by_up_to.vecs())
            .chain(self.by_from.vecs())
            .chain(self.by_range.vecs())
            .chain(self.by_epoch.vecs())
            .chain(self.by_size.vecs())
            // // .chain(self.by_value.vecs())
            .chain(self.by_type.vecs())
            .collect::<Vec<_>>()
    }
}

impl<T> From<Outputs<T>> for Outputs<(OutputFilter, T)> {
    fn from(value: Outputs<T>) -> Self {
        Self {
            all: (OutputFilter::All, value.all),
            by_term: OutputsByTerm::from(value.by_term),
            by_up_to: OutputsByUpTo::from(value.by_up_to),
            by_from: OutputsByFrom::from(value.by_from),
            by_range: OutputsByRange::from(value.by_range),
            by_epoch: OutputsByEpoch::from(value.by_epoch),
            by_size: OutputsBySize::from(value.by_size),
            // // Needs whole UTXO set, TODO later
            // // by_value: OutputsByValue<T>,
            by_type: OutputsBySpendableType::from(value.by_type),
        }
    }
}
