use allocative::Allocative;
use struct_iterable::Iterable;

use crate::{
    datasets::{AnyDataset, ComputeData, InsertData, MinInitialStates},
    states::RealizedState,
    structs::{BiMap, Config, MapKind, Price},
    utils::ONE_MONTH_IN_DAYS,
    DateMap, HeightMap,
};

#[derive(Allocative, Iterable)]
pub struct RealizedSubDataset {
    min_initial_states: MinInitialStates,

    realized_profit: HeightMap<f32>,
    realized_loss: HeightMap<f32>,
    value_created: HeightMap<f32>,
    adjusted_value_created: HeightMap<f32>,
    value_destroyed: HeightMap<f32>,
    adjusted_value_destroyed: HeightMap<f32>,
    realized_profit_1d_sum: DateMap<f32>,
    realized_loss_1d_sum: DateMap<f32>,
    value_created_1d_sum: DateMap<f32>,
    adjusted_value_created_1d_sum: DateMap<f32>,
    value_destroyed_1d_sum: DateMap<f32>,
    adjusted_value_destroyed_1d_sum: DateMap<f32>,
    spent_output_profit_ratio: BiMap<f32>,
    adjusted_spent_output_profit_ratio: BiMap<f32>,
    negative_realized_loss: HeightMap<f32>,
    negative_realized_loss_1d_sum: DateMap<f32>,
    net_realized_profit_and_loss: HeightMap<f32>,
    net_realized_profit_and_loss_1d_sum: DateMap<f32>,
    net_realized_profit_and_loss_1d_sum_to_market_cap_ratio: DateMap<f32>,
    cumulative_realized_profit: BiMap<f32>,
    cumulative_realized_loss: BiMap<f32>,
    cumulative_net_realized_profit_and_loss: BiMap<f32>,
    cumulative_net_realized_profit_and_loss_1m_net_change: BiMap<f32>,
    realized_value: HeightMap<f32>,
    realized_value_1d_sum: DateMap<f32>,
    sell_side_risk_ratio: DateMap<f32>,
    realized_profit_to_loss_ratio: HeightMap<f32>,
    realized_profit_to_loss_1d_sum_ratio: DateMap<f32>,
}

impl RealizedSubDataset {
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
            realized_profit: HeightMap::new_bin(1, MapKind::Inserted, &f("realized_profit")),
            realized_loss: HeightMap::new_bin(1, MapKind::Inserted, &f("realized_loss")),
            value_created: HeightMap::new_bin(1, MapKind::Inserted, &f("value_created")),
            adjusted_value_created: HeightMap::new_bin(
                1,
                MapKind::Inserted,
                &f("adjusted_value_created"),
            ),
            value_destroyed: HeightMap::new_bin(1, MapKind::Inserted, &f("value_destroyed")),
            adjusted_value_destroyed: HeightMap::new_bin(
                1,
                MapKind::Inserted,
                &f("adjusted_value_destroyed"),
            ),
            realized_profit_1d_sum: DateMap::new_bin(
                1,
                MapKind::Inserted,
                &f("realized_profit_1d_sum"),
            ),
            realized_loss_1d_sum: DateMap::new_bin(
                1,
                MapKind::Inserted,
                &f("realized_loss_1d_sum"),
            ),
            value_created_1d_sum: DateMap::new_bin(
                1,
                MapKind::Inserted,
                &f("value_created_1d_sum"),
            ),
            adjusted_value_created_1d_sum: DateMap::new_bin(
                1,
                MapKind::Inserted,
                &f("adjusted_value_created_1d_sum"),
            ),
            value_destroyed_1d_sum: DateMap::new_bin(
                1,
                MapKind::Inserted,
                &f("value_destroyed_1d_sum"),
            ),
            adjusted_value_destroyed_1d_sum: DateMap::new_bin(
                1,
                MapKind::Inserted,
                &f("adjusted_value_destroyed_1d_sum"),
            ),
            spent_output_profit_ratio: BiMap::new_bin(
                2,
                MapKind::Inserted,
                &f("spent_output_profit_ratio"),
            ),
            adjusted_spent_output_profit_ratio: BiMap::new_bin(
                2,
                MapKind::Inserted,
                &f("adjusted_spent_output_profit_ratio"),
            ),

            // ---
            // Computed
            // ---
            negative_realized_loss: HeightMap::new_bin(
                2,
                MapKind::Computed,
                &f("negative_realized_loss"),
            ),
            negative_realized_loss_1d_sum: DateMap::new_bin(
                2,
                MapKind::Computed,
                &f("negative_realized_loss_1d_sum"),
            ),
            net_realized_profit_and_loss: HeightMap::new_bin(
                1,
                MapKind::Computed,
                &f("net_realized_profit_and_loss"),
            ),
            net_realized_profit_and_loss_1d_sum: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("net_realized_profit_and_loss_1d_sum"),
            ),
            net_realized_profit_and_loss_1d_sum_to_market_cap_ratio: DateMap::new_bin(
                2,
                MapKind::Computed,
                &f("net_realized_profit_and_loss_to_market_cap_ratio"),
            ),
            cumulative_realized_profit: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("cumulative_realized_profit"),
            ),
            cumulative_realized_loss: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("cumulative_realized_loss"),
            ),
            cumulative_net_realized_profit_and_loss: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("cumulative_net_realized_profit_and_loss"),
            ),
            cumulative_net_realized_profit_and_loss_1m_net_change: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("cumulative_net_realized_profit_and_loss_1m_net_change"),
            ),
            realized_value: HeightMap::new_bin(1, MapKind::Computed, &f("realized_value")),
            realized_value_1d_sum: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("realized_value_1d_sum"),
            ),
            sell_side_risk_ratio: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("sell_side_risk_ratio"),
            ),
            realized_profit_to_loss_ratio: HeightMap::new_bin(
                1,
                MapKind::Computed,
                &f("realized_profit_to_loss_ratio"),
            ),
            realized_profit_to_loss_1d_sum_ratio: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("realized_profit_to_loss_1d_sum_ratio"),
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
            date_blocks_range,
            ..
        }: &InsertData,
        height_state: &RealizedState,
    ) {
        self.realized_profit
            .insert(height, height_state.realized_profit().to_dollar() as f32);

        self.realized_loss
            .insert(height, height_state.realized_loss().to_dollar() as f32);

        self.value_created
            .insert(height, height_state.value_created().to_dollar() as f32);

        self.adjusted_value_created.insert(
            height,
            height_state.adjusted_value_created().to_dollar() as f32,
        );

        self.value_destroyed
            .insert(height, height_state.value_destroyed().to_dollar() as f32);

        self.adjusted_value_destroyed.insert(
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
            self.realized_profit_1d_sum
                .insert(date, self.realized_profit.sum_range(date_blocks_range));

            self.realized_loss_1d_sum
                .insert(date, self.realized_loss.sum_range(date_blocks_range));

            let value_created_1d_sum = self
                .value_created_1d_sum
                .insert(date, self.value_created.sum_range(date_blocks_range));

            let adjusted_value_created_1d_sum = self.adjusted_value_created_1d_sum.insert(
                date,
                self.adjusted_value_created.sum_range(date_blocks_range),
            );

            let value_destroyed_1d_sum = self
                .value_destroyed_1d_sum
                .insert(date, self.value_destroyed.sum_range(date_blocks_range));

            let adjusted_value_destroyed_1d_sum = self.adjusted_value_destroyed_1d_sum.insert(
                date,
                self.adjusted_value_destroyed.sum_range(date_blocks_range),
            );

            self.spent_output_profit_ratio
                .date
                .insert(date, value_created_1d_sum / value_destroyed_1d_sum);

            self.adjusted_spent_output_profit_ratio.date.insert(
                date,
                adjusted_value_created_1d_sum / adjusted_value_destroyed_1d_sum,
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
            &mut self.realized_loss,
            |v, _| v * -1.0,
        );

        self.negative_realized_loss_1d_sum
            .multi_insert_simple_transform(dates, &mut self.realized_loss_1d_sum, |v, _| v * -1.0);

        self.net_realized_profit_and_loss.multi_insert_subtract(
            heights,
            &mut self.realized_profit,
            &mut self.realized_loss,
        );

        self.net_realized_profit_and_loss_1d_sum
            .multi_insert_subtract(
                dates,
                &mut self.realized_profit_1d_sum,
                &mut self.realized_loss_1d_sum,
            );

        self.net_realized_profit_and_loss_1d_sum_to_market_cap_ratio
            .multi_insert_percentage(
                dates,
                &mut self.net_realized_profit_and_loss_1d_sum,
                &mut market_cap.date,
            );

        self.cumulative_realized_profit
            .height
            .multi_insert_cumulative(heights, &mut self.realized_profit);
        self.cumulative_realized_profit
            .date
            .multi_insert_cumulative(dates, &mut self.realized_profit_1d_sum);

        self.cumulative_realized_loss
            .height
            .multi_insert_cumulative(heights, &mut self.realized_loss);
        self.cumulative_realized_loss
            .date
            .multi_insert_cumulative(dates, &mut self.realized_loss_1d_sum);

        self.cumulative_net_realized_profit_and_loss
            .height
            .multi_insert_cumulative(heights, &mut self.net_realized_profit_and_loss);
        self.cumulative_net_realized_profit_and_loss
            .date
            .multi_insert_cumulative(dates, &mut self.net_realized_profit_and_loss_1d_sum);

        self.cumulative_net_realized_profit_and_loss_1m_net_change
            .multi_insert_net_change(
                heights,
                dates,
                &mut self.cumulative_net_realized_profit_and_loss,
                ONE_MONTH_IN_DAYS,
            );

        self.realized_value.multi_insert_add(
            heights,
            &mut self.realized_profit,
            &mut self.realized_loss,
        );

        self.realized_value_1d_sum.multi_insert_add(
            dates,
            &mut self.realized_profit_1d_sum,
            &mut self.realized_loss_1d_sum,
        );

        self.sell_side_risk_ratio.multi_insert_percentage(
            dates,
            &mut self.realized_value_1d_sum,
            &mut market_cap.date,
        );

        self.realized_profit_to_loss_ratio.multi_insert_divide(
            heights,
            &mut self.realized_profit,
            &mut self.realized_loss,
        );
        self.realized_profit_to_loss_1d_sum_ratio
            .multi_insert_divide(
                dates,
                &mut self.realized_profit_1d_sum,
                &mut self.realized_loss_1d_sum,
            );
    }
}

impl AnyDataset for RealizedSubDataset {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }
}
