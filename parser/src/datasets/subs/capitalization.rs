use allocative::Allocative;
use struct_iterable::Iterable;

use crate::{
    datasets::{AnyDataset, ComputeData, InsertData, MinInitialStates},
    states::CapitalizationState,
    structs::{BiMap, Config, MapKind},
    utils::ONE_MONTH_IN_DAYS,
};

use super::RatioDataset;

#[derive(Allocative, Iterable)]
pub struct CapitalizationDataset {
    min_initial_states: MinInitialStates,

    pub realized_cap: BiMap<f32>,
    pub realized_price: BiMap<f32>,
    realized_cap_1m_net_change: BiMap<f32>,
    realized_price_ratio: RatioDataset,
}

impl CapitalizationDataset {
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
            realized_cap: BiMap::new_bin(1, MapKind::Inserted, &f("realized_cap")),

            // ---
            // Computed
            // ---
            realized_cap_1m_net_change: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("realized_cap_1m_net_change"),
            ),
            realized_price: BiMap::new_bin(1, MapKind::Computed, &f("realized_price")),
            realized_price_ratio: RatioDataset::import(
                parent_path,
                &format!(
                    "{}realized_price",
                    name.as_ref().map_or("".to_owned(), |n| format!("{n}-"))
                ),
                config,
            )?,
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
        state: &CapitalizationState,
    ) {
        let realized_cap = self
            .realized_cap
            .height
            .insert(height, state.realized_cap().to_dollar() as f32);

        if is_date_last_block {
            self.realized_cap.date.insert(date, realized_cap);
        }
    }

    pub fn compute(
        &mut self,
        compute_data: &ComputeData,
        closes: &mut BiMap<f32>,
        cohort_supply: &mut BiMap<f64>,
    ) {
        let &ComputeData { heights, dates, .. } = compute_data;

        self.realized_price.multi_insert_divide(
            heights,
            dates,
            &mut self.realized_cap,
            cohort_supply,
        );

        self.realized_cap_1m_net_change.multi_insert_net_change(
            heights,
            dates,
            &mut self.realized_cap,
            ONE_MONTH_IN_DAYS,
        );

        self.realized_price_ratio
            .compute(compute_data, closes, &mut self.realized_price);
    }
}

impl AnyDataset for CapitalizationDataset {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }
}
