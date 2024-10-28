use allocative::Allocative;
use struct_iterable::Iterable;

use crate::{
    datasets::{AnyDataset, ComputeData, InsertData, MinInitialStates},
    states::UnrealizedState,
    structs::{BiMap, Config, MapKind},
};

#[derive(Allocative, Iterable)]
pub struct UnrealizedSubDataset {
    min_initial_states: MinInitialStates,

    supply_in_profit: BiMap<f64>,
    unrealized_profit: BiMap<f32>,
    unrealized_loss: BiMap<f32>,
    supply_in_loss: BiMap<f64>,
    negative_unrealized_loss: BiMap<f32>,
    net_unrealized_profit_and_loss: BiMap<f32>,
    net_unrealized_profit_and_loss_to_market_cap_ratio: BiMap<f32>,
    supply_in_profit_to_own_supply_ratio: BiMap<f64>,
    supply_in_profit_to_circulating_supply_ratio: BiMap<f64>,
    supply_in_loss_to_own_supply_ratio: BiMap<f64>,
    supply_in_loss_to_circulating_supply_ratio: BiMap<f64>,
}

impl UnrealizedSubDataset {
    pub fn import(
        parent_path: &str,
        name: &Option<String>,
        config: &Config,
    ) -> color_eyre::Result<Self> {
        let f = |s: &str| {
            if let Some(name) = name {
                format!("{parent_path}/{name}/{s}")
            } else {
                format!("{parent_path}/{s}")
            }
        };

        let mut s = Self {
            min_initial_states: MinInitialStates::default(),

            // ---
            // Inserted
            // ---
            supply_in_profit: BiMap::new_bin(1, MapKind::Inserted, &f("supply_in_profit")),
            unrealized_profit: BiMap::new_bin(1, MapKind::Inserted, &f("unrealized_profit")),
            unrealized_loss: BiMap::new_bin(1, MapKind::Inserted, &f("unrealized_loss")),

            // ---
            // Inserted
            // ---
            supply_in_loss: BiMap::new_bin(1, MapKind::Computed, &f("supply_in_loss")),
            negative_unrealized_loss: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("negative_unrealized_loss"),
            ),
            net_unrealized_profit_and_loss: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("net_unrealized_profit_and_loss"),
            ),
            net_unrealized_profit_and_loss_to_market_cap_ratio: BiMap::new_bin(
                2,
                MapKind::Computed,
                &f("net_unrealized_profit_and_loss_to_market_cap_ratio"),
            ),
            supply_in_profit_to_own_supply_ratio: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("supply_in_profit_to_own_supply_ratio"),
            ),
            supply_in_profit_to_circulating_supply_ratio: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("supply_in_profit_to_circulating_supply_ratio"),
            ),
            supply_in_loss_to_own_supply_ratio: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("supply_in_loss_to_own_supply_ratio"),
            ),
            supply_in_loss_to_circulating_supply_ratio: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("supply_in_loss_to_circulating_supply_ratio"),
            ),
        };

        s.min_initial_states
            .consume(MinInitialStates::compute_from_dataset(&s, config));

        Ok(s)
    }

    pub fn insert(
        &mut self,
        &InsertData {
            height,
            date,
            is_date_last_block,
            ..
        }: &InsertData,
        block_state: &UnrealizedState,
        date_state: &Option<UnrealizedState>,
    ) {
        self.supply_in_profit
            .height
            .insert(height, block_state.supply_in_profit().to_btc());

        self.unrealized_profit
            .height
            .insert(height, block_state.unrealized_profit().to_dollar() as f32);

        self.unrealized_loss
            .height
            .insert(height, block_state.unrealized_loss().to_dollar() as f32);

        if is_date_last_block {
            let date_state = date_state.as_ref().unwrap();

            self.supply_in_profit
                .date
                .insert(date, date_state.supply_in_profit().to_btc());

            self.unrealized_profit
                .date
                .insert(date, date_state.unrealized_profit().to_dollar() as f32);

            self.unrealized_loss
                .date
                .insert(date, date_state.unrealized_loss().to_dollar() as f32);
        }
    }

    pub fn compute(
        &mut self,
        &ComputeData { heights, dates, .. }: &ComputeData,
        own_supply: &mut BiMap<f64>,
        circulating_supply: &mut BiMap<f64>,
        market_cap: &mut BiMap<f32>,
    ) {
        self.supply_in_loss.multi_insert_subtract(
            heights,
            dates,
            own_supply,
            &mut self.supply_in_profit,
        );

        self.negative_unrealized_loss.multi_insert_simple_transform(
            heights,
            dates,
            &mut self.unrealized_loss,
            &|v| v * -1.0,
        );

        self.net_unrealized_profit_and_loss.multi_insert_subtract(
            heights,
            dates,
            &mut self.unrealized_profit,
            &mut self.unrealized_loss,
        );

        self.net_unrealized_profit_and_loss_to_market_cap_ratio
            .multi_insert_percentage(
                heights,
                dates,
                &mut self.net_unrealized_profit_and_loss,
                market_cap,
            );

        self.supply_in_profit_to_own_supply_ratio
            .multi_insert_percentage(heights, dates, &mut self.supply_in_profit, own_supply);

        self.supply_in_profit_to_circulating_supply_ratio
            .multi_insert_percentage(
                heights,
                dates,
                &mut self.supply_in_profit,
                circulating_supply,
            );

        self.supply_in_loss_to_own_supply_ratio
            .multi_insert_percentage(heights, dates, &mut self.supply_in_loss, own_supply);

        self.supply_in_loss_to_circulating_supply_ratio
            .multi_insert_percentage(heights, dates, &mut self.supply_in_loss, circulating_supply);
    }
}

impl AnyDataset for UnrealizedSubDataset {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }
}
