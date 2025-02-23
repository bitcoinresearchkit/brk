use derive_deref::{Deref, DerefMut};

use crate::{
    parser::states::InputState,
    structs::{AddressRealizedData, Amount, LiquidityClassification},
};

use super::SplitByAddressCohort;

#[derive(Deref, DerefMut, Default)]
pub struct AddressCohortsInputStates(SplitByAddressCohort<InputState>);

impl AddressCohortsInputStates {
    pub fn iterate_input(
        &mut self,
        realized_data: &AddressRealizedData,
        liquidity_classification: &LiquidityClassification,
    ) -> color_eyre::Result<()> {
        let count = realized_data.utxos_destroyed as f64;
        let sent = realized_data.sent;

        let normal_iteration = move |state: &mut InputState| -> color_eyre::Result<()> {
            state.iterate(count, sent);
            Ok(())
        };

        let split_count = liquidity_classification.split(count);
        let split_sent = liquidity_classification.split(sent.to_sat() as f64);

        let liquified_iteration =
            move |liquidity, state: &mut InputState| -> color_eyre::Result<()> {
                state.iterate(
                    split_count.from(liquidity),
                    Amount::from_sat(split_sent.from(liquidity).round() as u64),
                );
                Ok(())
            };

        self.iterate(
            &realized_data.initial_address_data,
            normal_iteration,
            liquified_iteration,
        )
    }
}
