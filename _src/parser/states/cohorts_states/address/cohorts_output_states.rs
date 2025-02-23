use derive_deref::{Deref, DerefMut};

use crate::{
    parser::states::OutputState,
    structs::{AddressRealizedData, Amount, LiquidityClassification},
};

use super::SplitByAddressCohort;

#[derive(Deref, DerefMut, Default)]
pub struct AddressCohortsOutputStates(SplitByAddressCohort<OutputState>);

impl AddressCohortsOutputStates {
    pub fn iterate_output(
        &mut self,
        realized_data: &AddressRealizedData,
        liquidity_classification: &LiquidityClassification,
    ) -> color_eyre::Result<()> {
        let count = realized_data.utxos_created as f64;
        let volume = realized_data.received;

        let normal_iteration = move |state: &mut OutputState| -> color_eyre::Result<()> {
            state.iterate(count, volume);
            Ok(())
        };

        let split_count = liquidity_classification.split(count);
        let split_volume = liquidity_classification.split(volume.to_sat() as f64);

        let liquified_iteration =
            move |liquidity, state: &mut OutputState| -> color_eyre::Result<()> {
                state.iterate(
                    split_count.from(liquidity),
                    Amount::from_sat(split_volume.from(liquidity).round() as u64),
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
