use derive_deref::{Deref, DerefMut};

use crate::{
    parser::states::RealizedState,
    structs::{AddressRealizedData, LiquidityClassification, Price},
};

use super::SplitByAddressCohort;

#[derive(Deref, DerefMut, Default)]
pub struct AddressCohortsRealizedStates(SplitByAddressCohort<RealizedState>);

impl AddressCohortsRealizedStates {
    pub fn iterate_realized(
        &mut self,
        realized_data: &AddressRealizedData,
        liquidity_classification: &LiquidityClassification,
    ) -> color_eyre::Result<()> {
        let realized_profit = realized_data.profit;
        let realized_loss = realized_data.loss;
        let value_created = realized_data.value_created;
        let adjusted_value_created = realized_data.adjusted_value_created;
        let value_destroyed = realized_data.value_destroyed;
        let adjusted_value_destroyed = realized_data.adjusted_value_destroyed;

        let normal_iteration = move |state: &mut RealizedState| -> color_eyre::Result<()> {
            state.iterate(
                realized_profit,
                realized_loss,
                value_created,
                adjusted_value_created,
                value_destroyed,
                adjusted_value_destroyed,
            );
            Ok(())
        };

        let split_realized_profit =
            liquidity_classification.split(realized_profit.to_cent() as f64);
        let split_realized_loss = liquidity_classification.split(realized_loss.to_cent() as f64);
        let split_value_created = liquidity_classification.split(value_created.to_cent() as f64);
        let split_adjusted_value_created =
            liquidity_classification.split(adjusted_value_created.to_cent() as f64);
        let split_value_destroyed =
            liquidity_classification.split(value_destroyed.to_cent() as f64);
        let split_adjusted_value_destroyed =
            liquidity_classification.split(adjusted_value_destroyed.to_cent() as f64);

        let liquified_iteration =
            move |liquidity, state: &mut RealizedState| -> color_eyre::Result<()> {
                state.iterate(
                    Price::from_cent(split_realized_profit.from(liquidity) as u64),
                    Price::from_cent(split_realized_loss.from(liquidity) as u64),
                    Price::from_cent(split_value_created.from(liquidity) as u64),
                    Price::from_cent(split_adjusted_value_created.from(liquidity) as u64),
                    Price::from_cent(split_value_destroyed.from(liquidity) as u64),
                    Price::from_cent(split_adjusted_value_destroyed.from(liquidity) as u64),
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
