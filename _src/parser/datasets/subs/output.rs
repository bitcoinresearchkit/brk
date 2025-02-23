use crate::{
    datasets::{AnyDataset, ComputeData, InsertData, MinInitialStates},
    states::OutputState,
    structs::{AnyBiMap, BiMap},
    utils::ONE_YEAR_IN_DAYS,
};

pub struct OutputSubDataset {
    min_initial_states: MinInitialStates,

    // Inserted
    pub count: BiMap<f32>,
    pub volume: BiMap<f32>,

    // Computed
    pub annualized_volume: BiMap<f32>,
    pub velocity: BiMap<f32>,
    // add outputs_per_second
}

impl OutputSubDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_states: MinInitialStates::default(),

            count: BiMap::new_bin(1, &f("output_count")),
            volume: BiMap::new_bin(1, &f("output_volume")),
            annualized_volume: BiMap::new_bin(1, &f("annualized_output_volume")),
            velocity: BiMap::new_bin(1, &f("output_velocity")),
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
        state: &OutputState,
    ) {
        let count = self.count.height.insert(height, state.count);

        self.volume.height.insert(height, state.volume);

        if is_date_last_block {
            self.count.date.insert(date, count);

            self.volume.date_insert_sum_range(date, date_blocks_range);
        }
    }

    pub fn compute(
        &mut self,
        &ComputeData { heights, dates }: &ComputeData,
        cohort_supply: &mut BiMap<f32>,
    ) {
        self.annualized_volume.multi_insert_last_x_sum(
            heights,
            dates,
            &mut self.volume,
            ONE_YEAR_IN_DAYS,
        );

        self.velocity.multi_insert_divide(
            heights,
            dates,
            &mut self.annualized_volume,
            cohort_supply,
        );
    }
}

impl AnyDataset for OutputSubDataset {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }
}
