use allocative::Allocative;

use crate::{
    datasets::{AnyDataset, ComputeData, InsertData, MinInitialStates},
    states::RealizedState,
    structs::{AnyBiMap, AnyDateMap, BiMap, Price},
    utils::ONE_MONTH_IN_DAYS,
    DateMap,
};

#[derive(Default, Allocative)]
pub struct RealizedSubDataset {
    min_initial_states: MinInitialStates,

    // Inserted
    realized_profit: BiMap<f32>,
    realized_loss: BiMap<f32>,
    value_created: BiMap<f32>,
    adjusted_value_created: BiMap<f32>,
    value_destroyed: BiMap<f32>,
    adjusted_value_destroyed: BiMap<f32>,
    spent_output_profit_ratio: BiMap<f32>,
    adjusted_spent_output_profit_ratio: BiMap<f32>,

    // Computed
    negative_realized_loss: BiMap<f32>,
    net_realized_profit_and_loss: BiMap<f32>,
    net_realized_profit_and_loss_to_market_cap_ratio: BiMap<f32>,
    cumulative_realized_profit: BiMap<f32>,
    cumulative_realized_loss: BiMap<f32>,
    cumulative_net_realized_profit_and_loss: BiMap<f32>,
    cumulative_net_realized_profit_and_loss_1m_net_change: BiMap<f32>,
    realized_value: BiMap<f32>,
    sell_side_risk_ratio: DateMap<f32>,
}

impl RealizedSubDataset {
    pub fn import(parent_path: &str, name: &Option<String>) -> color_eyre::Result<Self> {
        let f = |s: &str| {
            if let Some(name) = name {
                format!("{parent_path}/{name}/{s}")
            } else {
                format!("{parent_path}/{s}")
            }
        };

        let mut s = Self {
            min_initial_states: MinInitialStates::default(),

            realized_profit: BiMap::new_bin(1, &f("realized_profit")),
            realized_loss: BiMap::new_bin(1, &f("realized_loss")),
            value_created: BiMap::new_bin(1, &f("value_created")),
            adjusted_value_created: BiMap::new_bin(1, &f("adjusted_value_created")),
            value_destroyed: BiMap::new_bin(1, &f("value_destroyed")),
            adjusted_value_destroyed: BiMap::new_bin(1, &f("adjusted_value_destroyed")),
            spent_output_profit_ratio: BiMap::new_bin(2, &f("spent_output_profit_ratio")),
            adjusted_spent_output_profit_ratio: BiMap::new_bin(
                2,
                &f("adjusted_spent_output_profit_ratio"),
            ),

            negative_realized_loss: BiMap::new_bin(2, &f("negative_realized_loss")),
            net_realized_profit_and_loss: BiMap::new_bin(1, &f("net_realized_profit_and_loss")),
            net_realized_profit_and_loss_to_market_cap_ratio: BiMap::new_bin(
                2,
                &f("net_realized_profit_and_loss_to_market_cap_ratio"),
            ),
            cumulative_realized_profit: BiMap::new_bin(1, &f("cumulative_realized_profit")),
            cumulative_realized_loss: BiMap::new_bin(1, &f("cumulative_realized_loss")),
            cumulative_net_realized_profit_and_loss: BiMap::new_bin(
                1,
                &f("cumulative_net_realized_profit_and_loss"),
            ),
            cumulative_net_realized_profit_and_loss_1m_net_change: BiMap::new_bin(
                1,
                &f("cumulative_net_realized_profit_and_loss_1m_net_change"),
            ),
            realized_value: BiMap::new_bin(1, &f("realized_value")),
            sell_side_risk_ratio: DateMap::new_bin(1, &f("sell_side_risk_ratio")),
        };

        s.min_initial_states
            .consume(MinInitialStates::compute_from_dataset(&s));

        Ok(s)
    }

    pub fn insert(
        &mut self,
        &InsertData {
            height,
            date,
            is_date_last_block,
            date_blocks_range,
            ..
        }: &InsertData,
        height_state: &RealizedState,
    ) {
        self.realized_profit
            .height
            .insert(height, height_state.realized_profit().to_dollar() as f32);

        self.realized_loss
            .height
            .insert(height, height_state.realized_loss().to_dollar() as f32);

        self.value_created
            .height
            .insert(height, height_state.value_created().to_dollar() as f32);

        self.adjusted_value_created.height.insert(
            height,
            height_state.adjusted_value_created().to_dollar() as f32,
        );

        self.value_destroyed
            .height
            .insert(height, height_state.value_destroyed().to_dollar() as f32);

        self.adjusted_value_destroyed.height.insert(
            height,
            height_state.adjusted_value_destroyed().to_dollar() as f32,
        );

        self.spent_output_profit_ratio.height.insert(height, {
            if height_state.value_destroyed() > Price::ZERO {
                (height_state.value_created().to_cent() as f64
                    / height_state.value_destroyed().to_cent() as f64) as f32
            } else {
                1.0
            }
        });

        self.adjusted_spent_output_profit_ratio
            .height
            .insert(height, {
                if height_state.adjusted_value_destroyed() > Price::ZERO {
                    (height_state.adjusted_value_created().to_cent() as f64
                        / height_state.adjusted_value_destroyed().to_cent() as f64)
                        as f32
                } else {
                    1.0
                }
            });

        if is_date_last_block {
            self.realized_profit
                .date_insert_sum_range(date, date_blocks_range);

            self.realized_loss
                .date_insert_sum_range(date, date_blocks_range);

            self.value_created
                .date_insert_sum_range(date, date_blocks_range);

            self.adjusted_value_created
                .date_insert_sum_range(date, date_blocks_range);

            self.value_destroyed
                .date_insert_sum_range(date, date_blocks_range);

            self.adjusted_value_destroyed
                .date_insert_sum_range(date, date_blocks_range);

            self.spent_output_profit_ratio.date.insert(
                date,
                self.value_created.height.sum_range(date_blocks_range)
                    / self.value_destroyed.height.sum_range(date_blocks_range),
            );

            self.adjusted_spent_output_profit_ratio.date.insert(
                date,
                self.adjusted_value_created
                    .height
                    .sum_range(date_blocks_range)
                    / self
                        .adjusted_value_destroyed
                        .height
                        .sum_range(date_blocks_range),
            );
        }
    }

    pub fn compute(
        &mut self,
        &ComputeData { heights, dates, .. }: &ComputeData,
        market_cap: &mut BiMap<f32>,
    ) {
        self.negative_realized_loss.multi_insert_simple_transform(
            heights,
            dates,
            &mut self.realized_loss,
            &|v| v * -1.0,
        );

        self.net_realized_profit_and_loss.multi_insert_subtract(
            heights,
            dates,
            &mut self.realized_profit,
            &mut self.realized_loss,
        );

        self.net_realized_profit_and_loss_to_market_cap_ratio
            .multi_insert_percentage(
                heights,
                dates,
                &mut self.net_realized_profit_and_loss,
                market_cap,
            );

        self.cumulative_realized_profit.multi_insert_cumulative(
            heights,
            dates,
            &mut self.realized_profit,
        );

        self.cumulative_realized_loss.multi_insert_cumulative(
            heights,
            dates,
            &mut self.realized_loss,
        );

        self.cumulative_net_realized_profit_and_loss
            .multi_insert_cumulative(heights, dates, &mut self.net_realized_profit_and_loss);

        self.cumulative_net_realized_profit_and_loss_1m_net_change
            .multi_insert_net_change(
                heights,
                dates,
                &mut self.cumulative_net_realized_profit_and_loss,
                ONE_MONTH_IN_DAYS,
            );

        self.realized_value.multi_insert_add(
            heights,
            dates,
            &mut self.realized_profit,
            &mut self.realized_loss,
        );

        self.sell_side_risk_ratio.multi_insert_percentage(
            dates,
            &mut self.realized_value.date,
            &mut market_cap.date,
        );
    }
}

impl AnyDataset for RealizedSubDataset {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }

    fn to_inserted_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![
            &self.realized_loss,
            &self.realized_profit,
            &self.value_created,
            &self.adjusted_value_created,
            &self.value_destroyed,
            &self.adjusted_value_destroyed,
            &self.spent_output_profit_ratio,
            &self.adjusted_spent_output_profit_ratio,
        ]
    }

    fn to_inserted_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![
            &mut self.realized_loss,
            &mut self.realized_profit,
            &mut self.value_created,
            &mut self.adjusted_value_created,
            &mut self.value_destroyed,
            &mut self.adjusted_value_destroyed,
            &mut self.spent_output_profit_ratio,
            &mut self.adjusted_spent_output_profit_ratio,
        ]
    }

    fn to_computed_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![
            &self.negative_realized_loss,
            &self.net_realized_profit_and_loss,
            &self.net_realized_profit_and_loss_to_market_cap_ratio,
            &self.cumulative_realized_profit,
            &self.cumulative_realized_loss,
            &self.cumulative_net_realized_profit_and_loss,
            &self.cumulative_net_realized_profit_and_loss_1m_net_change,
            &self.realized_value,
        ]
    }

    fn to_computed_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![&self.sell_side_risk_ratio]
    }

    fn to_computed_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![
            &mut self.negative_realized_loss,
            &mut self.net_realized_profit_and_loss,
            &mut self.net_realized_profit_and_loss_to_market_cap_ratio,
            &mut self.cumulative_realized_profit,
            &mut self.cumulative_realized_loss,
            &mut self.cumulative_net_realized_profit_and_loss,
            &mut self.cumulative_net_realized_profit_and_loss_1m_net_change,
            &mut self.realized_value,
        ]
    }

    fn to_computed_mut_date_map_vec(&mut self) -> Vec<&mut dyn AnyDateMap> {
        vec![&mut self.sell_side_risk_ratio]
    }
}
