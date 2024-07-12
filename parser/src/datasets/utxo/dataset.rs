use allocative::Allocative;
use itertools::Itertools;

use crate::{
    datasets::{
        AnyDataset, AnyDatasetGroup, ComputeData, InsertData, MinInitialStates, SubDataset,
    },
    states::UTXOCohortId,
    structs::{AnyBiMap, AnyDateMap, AnyHeightMap, BiMap, WNaiveDate},
};

#[derive(Default, Allocative)]
pub struct UTXODataset {
    id: UTXOCohortId,

    min_initial_states: MinInitialStates,

    pub subs: SubDataset,
}

impl UTXODataset {
    pub fn import(parent_path: &str, id: UTXOCohortId) -> color_eyre::Result<Self> {
        let name = id.name().to_owned();

        let mut s = Self {
            min_initial_states: MinInitialStates::default(),
            id,
            subs: SubDataset::import(parent_path, &Some(name))?,
        };

        s.min_initial_states
            .consume(MinInitialStates::compute_from_dataset(&s));

        Ok(s)
    }

    pub fn insert(&mut self, insert_data: &InsertData) {
        let &InsertData {
            states,
            utxo_cohorts_one_shot_states,
            // utxo_cohorts_received_states,
            utxo_cohorts_sent_states,
            ..
        } = insert_data;

        if self.needs_insert_supply(insert_data.height, insert_data.date) {
            self.subs.supply.insert(
                insert_data,
                &states
                    .utxo_cohorts_durable_states
                    .get(&self.id)
                    .durable_states
                    .supply_state,
            );
        }

        if self.needs_insert_utxo(insert_data.height, insert_data.date) {
            self.subs.utxo.insert(
                insert_data,
                &states
                    .utxo_cohorts_durable_states
                    .get(&self.id)
                    .durable_states
                    .utxo_state,
            );
        }

        if self.needs_insert_capitalization(insert_data.height, insert_data.date) {
            self.subs.capitalization.insert(
                insert_data,
                &states
                    .utxo_cohorts_durable_states
                    .get(&self.id)
                    .durable_states
                    .capitalization_state,
            );
        }

        if self.needs_insert_unrealized(insert_data.height, insert_data.date) {
            self.subs.unrealized.insert(
                insert_data,
                &utxo_cohorts_one_shot_states
                    .get(&self.id)
                    .unrealized_block_state,
                &utxo_cohorts_one_shot_states
                    .get(&self.id)
                    .unrealized_date_state,
            );
        }

        if self.needs_insert_price_paid(insert_data.height, insert_data.date) {
            self.subs.price_paid.insert(
                insert_data,
                &utxo_cohorts_one_shot_states.get(&self.id).price_paid_state,
            );
        }

        if self.needs_insert_realized(insert_data.height, insert_data.date) {
            self.subs.realized.insert(
                insert_data,
                &utxo_cohorts_sent_states.get(&self.id).realized,
            );
        }

        if self.needs_insert_input(insert_data.height, insert_data.date) {
            self.subs
                .input
                .insert(insert_data, &utxo_cohorts_sent_states.get(&self.id).input);
        }

        // TODO: move output from common to address
        // if self.subs.output.needs_insert(insert_data) {
        //     self.subs
        //         .output
        //         .insert(insert_data, utxo_cohorts_received_states.get(&self.id));
        // }
    }

    pub fn needs_insert_utxo(&self, height: usize, date: WNaiveDate) -> bool {
        self.subs.utxo.needs_insert(height, date)
    }

    pub fn needs_insert_capitalization(&self, height: usize, date: WNaiveDate) -> bool {
        self.subs.capitalization.needs_insert(height, date)
    }

    pub fn needs_insert_supply(&self, height: usize, date: WNaiveDate) -> bool {
        self.subs.supply.needs_insert(height, date)
    }

    pub fn needs_insert_price_paid(&self, height: usize, date: WNaiveDate) -> bool {
        self.subs.price_paid.needs_insert(height, date)
    }

    pub fn needs_insert_realized(&self, height: usize, date: WNaiveDate) -> bool {
        self.subs.realized.needs_insert(height, date)
    }

    pub fn needs_insert_unrealized(&self, height: usize, date: WNaiveDate) -> bool {
        self.subs.unrealized.needs_insert(height, date)
    }

    pub fn needs_insert_input(&self, height: usize, date: WNaiveDate) -> bool {
        self.subs.input.needs_insert(height, date)
    }

    pub fn compute(
        &mut self,
        compute_data: &ComputeData,
        closes: &mut BiMap<f32>,
        circulating_supply: &mut BiMap<f64>,
        market_cap: &mut BiMap<f32>,
    ) {
        if self.subs.supply.should_compute(compute_data) {
            self.subs.supply.compute(compute_data, circulating_supply);
        }

        if self.subs.unrealized.should_compute(compute_data) {
            self.subs.unrealized.compute(
                compute_data,
                &mut self.subs.supply.supply,
                circulating_supply,
                market_cap,
            );
        }

        if self.subs.realized.should_compute(compute_data) {
            self.subs.realized.compute(compute_data, market_cap);
        }

        if self.subs.capitalization.should_compute(compute_data) {
            self.subs
                .capitalization
                .compute(compute_data, closes, &mut self.subs.supply.supply);
        }

        // if self.subs.output.should_compute(compute_data) {
        //     self.subs
        //         .output
        //         .compute(compute_data, &mut self.subs.supply.total);
        // }
    }
}

impl AnyDataset for UTXODataset {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }

    fn to_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        self.subs
            .as_vec()
            .into_iter()
            .flat_map(|d| d.to_inserted_height_map_vec())
            .collect_vec()
    }

    fn to_inserted_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        self.subs
            .as_vec()
            .into_iter()
            .flat_map(|d| d.to_inserted_date_map_vec())
            .collect_vec()
    }

    fn to_inserted_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        self.subs
            .as_vec()
            .into_iter()
            .flat_map(|d| d.to_inserted_bi_map_vec())
            .collect_vec()
    }

    fn to_inserted_mut_height_map_vec(&mut self) -> Vec<&mut dyn AnyHeightMap> {
        self.subs
            .as_mut_vec()
            .into_iter()
            .flat_map(|d| d.to_inserted_mut_height_map_vec())
            .collect_vec()
    }

    fn to_inserted_mut_date_map_vec(&mut self) -> Vec<&mut dyn AnyDateMap> {
        self.subs
            .as_mut_vec()
            .into_iter()
            .flat_map(|d| d.to_inserted_mut_date_map_vec())
            .collect_vec()
    }

    fn to_inserted_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        self.subs
            .as_mut_vec()
            .into_iter()
            .flat_map(|d| d.to_inserted_mut_bi_map_vec())
            .collect_vec()
    }

    fn to_computed_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        self.subs
            .as_vec()
            .into_iter()
            .flat_map(|d| d.to_computed_height_map_vec())
            .collect_vec()
    }

    fn to_computed_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        self.subs
            .as_vec()
            .into_iter()
            .flat_map(|d| d.to_computed_date_map_vec())
            .collect_vec()
    }

    fn to_computed_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        self.subs
            .as_vec()
            .into_iter()
            .flat_map(|d| d.to_computed_bi_map_vec())
            .collect_vec()
    }

    fn to_computed_mut_height_map_vec(&mut self) -> Vec<&mut dyn AnyHeightMap> {
        self.subs
            .as_mut_vec()
            .into_iter()
            .flat_map(|d| d.to_computed_mut_height_map_vec())
            .collect_vec()
    }

    fn to_computed_mut_date_map_vec(&mut self) -> Vec<&mut dyn AnyDateMap> {
        self.subs
            .as_mut_vec()
            .into_iter()
            .flat_map(|d| d.to_computed_mut_date_map_vec())
            .collect_vec()
    }

    fn to_computed_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        self.subs
            .as_mut_vec()
            .into_iter()
            .flat_map(|d| d.to_computed_mut_bi_map_vec())
            .collect_vec()
    }
}
