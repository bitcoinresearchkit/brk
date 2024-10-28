use allocative::Allocative;

use struct_iterable::Iterable;

use crate::{
    datasets::{AnyDataset, ComputeData, InsertData, MinInitialStates, SubDataset},
    states::{AddressCohortId, DurableStates},
    structs::{AddressSplit, BiMap, Config, Date, Height},
};

use super::cohort_metadata::AddressCohortMetadataDataset;

#[derive(Allocative, Iterable)]
pub struct CohortDataset {
    min_initial_states: MinInitialStates,

    split: AddressSplit,

    metadata: AddressCohortMetadataDataset,

    pub subs: SubDataset,
}

impl CohortDataset {
    pub fn import(
        parent_path: &str,
        id: AddressCohortId,
        config: &Config,
    ) -> color_eyre::Result<Self> {
        let name = id.as_name().map(|s| s.to_owned());
        let split = id.as_split();

        let mut s = Self {
            min_initial_states: MinInitialStates::default(),
            split,
            metadata: AddressCohortMetadataDataset::import(parent_path, &name, config)?,
            subs: SubDataset::import(parent_path, &name, config)?,
        };

        s.min_initial_states
            .consume(MinInitialStates::compute_from_dataset(&s, config));

        Ok(s)
    }

    pub fn sub_datasets_vec(&self) -> Vec<&SubDataset> {
        vec![&self.subs]
    }

    pub fn needs_insert_metadata(&self, height: Height, date: Date) -> bool {
        self.metadata.needs_insert(height, date)
    }

    pub fn needs_insert_utxo(&self, height: Height, date: Date) -> bool {
        self.sub_datasets_vec()
            .iter()
            .any(|sub| sub.utxo.needs_insert(height, date))
    }

    pub fn needs_insert_capitalization(&self, height: Height, date: Date) -> bool {
        self.sub_datasets_vec()
            .iter()
            .any(|sub| sub.capitalization.needs_insert(height, date))
    }

    pub fn needs_insert_supply(&self, height: Height, date: Date) -> bool {
        self.sub_datasets_vec()
            .iter()
            .any(|sub| sub.supply.needs_insert(height, date))
    }

    pub fn needs_insert_price_paid(&self, height: Height, date: Date) -> bool {
        self.sub_datasets_vec()
            .iter()
            .any(|sub| sub.price_paid.needs_insert(height, date))
    }

    pub fn needs_insert_realized(&self, height: Height, date: Date) -> bool {
        self.sub_datasets_vec()
            .iter()
            .any(|sub| sub.realized.needs_insert(height, date))
    }

    pub fn needs_insert_unrealized(&self, height: Height, date: Date) -> bool {
        self.sub_datasets_vec()
            .iter()
            .any(|sub| sub.unrealized.needs_insert(height, date))
    }

    pub fn needs_insert_input(&self, height: Height, date: Date) -> bool {
        self.sub_datasets_vec()
            .iter()
            .any(|sub| sub.input.needs_insert(height, date))
    }

    // fn needs_insert_output(&self, insert_data: &InsertData) -> bool {
    //     self.sub_datasets_vec()
    //         .iter()
    //         .any(|sub| sub.output.needs_insert(height, date))
    // }

    pub fn insert_realized_data(&mut self, insert_data: &InsertData) {
        let realized_state = insert_data
            .address_cohorts_realized_states
            .as_ref()
            .unwrap()
            .get(&self.split)
            .unwrap();

        self.subs.realized.insert(insert_data, realized_state);
    }

    fn insert_metadata(&mut self, insert_data: &InsertData) {
        let address_count = insert_data
            .states
            .address_cohorts_durable_states
            .as_ref()
            .unwrap()
            .get(&self.split)
            .unwrap()
            .address_count;

        self.metadata.insert(insert_data, address_count);
    }

    fn insert_supply_data(&mut self, insert_data: &InsertData, durable_states: &DurableStates) {
        self.subs
            .supply
            .insert(insert_data, &durable_states.supply_state);
    }

    fn insert_utxo_data(&mut self, insert_data: &InsertData, durable_states: &DurableStates) {
        self.subs
            .utxo
            .insert(insert_data, &durable_states.utxo_state);
    }

    fn insert_capitalization_data(
        &mut self,
        insert_data: &InsertData,
        durable_states: &DurableStates,
    ) {
        self.subs
            .capitalization
            .insert(insert_data, &durable_states.capitalization_state);
    }

    fn insert_unrealized_data(&mut self, insert_data: &InsertData) {
        let states = insert_data
            .address_cohorts_one_shot_states
            .as_ref()
            .unwrap()
            .get(&self.split)
            .unwrap();

        self.subs.unrealized.insert(
            insert_data,
            &states.unrealized_block_state,
            &states.unrealized_date_state,
        );
    }

    fn insert_price_paid_data(&mut self, insert_data: &InsertData) {
        let states = insert_data
            .address_cohorts_one_shot_states
            .as_ref()
            .unwrap()
            .get(&self.split)
            .unwrap();

        self.subs
            .price_paid
            .insert(insert_data, &states.price_paid_state);
    }

    fn insert_input_data(&mut self, insert_data: &InsertData) {
        let state = insert_data
            .address_cohorts_input_states
            .as_ref()
            .unwrap()
            .get(&self.split)
            .unwrap();

        self.subs.input.insert(insert_data, state);
    }

    // fn insert_output_data(&mut self, insert_data: &InsertData) {
    //     let state = insert_data
    //         .address_cohorts_output_states
    //         .as_ref()
    //         .unwrap()
    //         .get(&self.split)
    //         .unwrap();

    //     self.output.insert(insert_data, &state.all);
    //     self.illiquid.output.insert(insert_data, &state.illiquid);
    //     self.liquid.output.insert(insert_data, &state.liquid);
    //     self.highly_liquid
    //         .output
    //         .insert(insert_data, &state.highly_liquid);
    // }

    pub fn insert(&mut self, insert_data: &InsertData) {
        if !insert_data.compute_addresses {
            return;
        }

        let address_cohort_durable_states = insert_data
            .states
            .address_cohorts_durable_states
            .as_ref()
            .unwrap()
            .get(&self.split);

        if address_cohort_durable_states.is_none() {
            return; // TODO: Check if should panic instead
        }

        let address_cohort_durable_states = address_cohort_durable_states.unwrap();

        if self.needs_insert_metadata(insert_data.height, insert_data.date) {
            self.insert_metadata(insert_data);
        }

        if self.needs_insert_utxo(insert_data.height, insert_data.date) {
            self.insert_utxo_data(insert_data, &address_cohort_durable_states.durable_states);
        }

        if self.needs_insert_capitalization(insert_data.height, insert_data.date) {
            self.insert_capitalization_data(
                insert_data,
                &address_cohort_durable_states.durable_states,
            );
        }

        if self.needs_insert_supply(insert_data.height, insert_data.date) {
            self.insert_supply_data(insert_data, &address_cohort_durable_states.durable_states);
        }

        if self.needs_insert_realized(insert_data.height, insert_data.date) {
            self.insert_realized_data(insert_data);
        }

        if self.needs_insert_unrealized(insert_data.height, insert_data.date) {
            self.insert_unrealized_data(insert_data);
        }

        if self.needs_insert_price_paid(insert_data.height, insert_data.date) {
            self.insert_price_paid_data(insert_data);
        }

        if self.needs_insert_input(insert_data.height, insert_data.date) {
            self.insert_input_data(insert_data);
        }

        // if self.needs_insert_output(insert_data) {
        //     self.insert_output_data(insert_data);
        // }
    }

    // pub fn should_compute_metadata(&self, compute_data: &ComputeData) -> bool {
    //     self.metadata.should_compute(compute_data)
    // }

    // pub fn should_compute_utxo(&self, compute_data: &ComputeData) -> bool {
    //     self.sub_datasets_vec()
    //         .iter()
    //         .any(|sub| sub.utxo.should_compute(compute_data))
    // }

    pub fn should_compute_supply(&self, compute_data: &ComputeData) -> bool {
        self.sub_datasets_vec()
            .iter()
            .any(|sub| sub.supply.should_compute(compute_data))
    }

    pub fn should_compute_capitalization(&self, compute_data: &ComputeData) -> bool {
        self.sub_datasets_vec()
            .iter()
            .any(|sub| sub.capitalization.should_compute(compute_data))
    }

    fn should_compute_realized(&self, compute_data: &ComputeData) -> bool {
        self.sub_datasets_vec()
            .iter()
            .any(|sub| sub.realized.should_compute(compute_data))
    }

    fn should_compute_unrealized(&self, compute_data: &ComputeData) -> bool {
        self.sub_datasets_vec()
            .iter()
            .any(|sub| sub.unrealized.should_compute(compute_data))
    }

    // fn should_compute_input(&self, compute_data: &ComputeData) -> bool {
    //     self.sub_datasets_vec()
    //         .iter()
    //         .any(|sub| sub.input.should_compute(compute_data))
    // }

    // fn should_compute_output(&self, compute_data: &ComputeData) -> bool {
    //     self.sub_datasets_vec()
    //         .iter()
    //         .any(|sub| sub.output.should_compute(compute_data))
    // }

    fn compute_supply_data(
        &mut self,
        compute_data: &ComputeData,
        circulating_supply: &mut BiMap<f64>,
    ) {
        self.subs.supply.compute(compute_data, circulating_supply);
    }

    fn compute_unrealized_data(
        &mut self,
        compute_data: &ComputeData,
        circulating_supply: &mut BiMap<f64>,
        market_cap: &mut BiMap<f32>,
    ) {
        self.subs.unrealized.compute(
            compute_data,
            &mut self.subs.supply.supply,
            circulating_supply,
            market_cap,
        );
    }

    fn compute_realized_data(&mut self, compute_data: &ComputeData, market_cap: &mut BiMap<f32>) {
        self.subs.realized.compute(compute_data, market_cap);
    }

    fn compute_capitalization_data(&mut self, compute_data: &ComputeData, closes: &mut BiMap<f32>) {
        self.subs
            .capitalization
            .compute(compute_data, closes, &mut self.subs.supply.supply);
    }

    // fn compute_output_data(&mut self, compute_data: &ComputeData) {
    //     self.all
    //         .output
    //         .compute(compute_data, &mut self.supply.total);
    // }

    pub fn compute(
        &mut self,
        compute_data: &ComputeData,
        closes: &mut BiMap<f32>,
        circulating_supply: &mut BiMap<f64>,
        market_cap: &mut BiMap<f32>,
    ) {
        if self.should_compute_supply(compute_data) {
            self.compute_supply_data(compute_data, circulating_supply);
        }

        if self.should_compute_unrealized(compute_data) {
            self.compute_unrealized_data(compute_data, circulating_supply, market_cap);
        }

        if self.should_compute_realized(compute_data) {
            self.compute_realized_data(compute_data, market_cap);
        }

        // MUST BE after compute_supply
        if self.should_compute_capitalization(compute_data) {
            self.compute_capitalization_data(compute_data, closes);
        }

        // if self.should_compute_output(compute_data) {
        //     self.compute_output_data(compute_data);
        // }
    }
}

impl AnyDataset for CohortDataset {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }
}
