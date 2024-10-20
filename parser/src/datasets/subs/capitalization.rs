use allocative::Allocative;

use crate::{
    datasets::{AnyDataset, ComputeData, InsertData, MinInitialStates},
    states::CapitalizationState,
    structs::{AnyBiMap, BiMap, Config},
    utils::ONE_MONTH_IN_DAYS,
};

use super::RatioDataset;

#[derive(Default, Allocative)]
pub struct CapitalizationDataset {
    min_initial_states: MinInitialStates,

    // Inserted
    pub realized_cap: BiMap<f32>,

    // Computed
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

            realized_cap: BiMap::new_bin(1, &f("realized_cap")),
            realized_cap_1m_net_change: BiMap::new_bin(1, &f("realized_cap_1m_net_change")),
            realized_price: BiMap::new_bin(1, &f("realized_price")),
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

    fn to_inserted_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![&self.realized_cap]
    }

    fn to_inserted_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![&mut self.realized_cap]
    }

    fn to_computed_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        let mut v = vec![
            &self.realized_price as &(dyn AnyBiMap + Send + Sync),
            &self.realized_cap_1m_net_change,
        ];
        v.append(&mut self.realized_price_ratio.to_computed_bi_map_vec());
        v
    }

    fn to_computed_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        let mut v = vec![
            &mut self.realized_price as &mut dyn AnyBiMap,
            &mut self.realized_cap_1m_net_change,
        ];
        v.append(&mut self.realized_price_ratio.to_computed_mut_bi_map_vec());
        v
    }
}
