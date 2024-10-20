use itertools::Itertools;
use rayon::prelude::*;

use crate::{
    datasets::ComputeData,
    structs::{AnyBiMap, AnyDateMap, AnyHeightMap, AnyMap, Date, Height},
};

use super::MinInitialStates;

pub trait AnyDataset {
    fn get_min_initial_states(&self) -> &MinInitialStates;

    fn needs_insert(&self, height: Height, date: Date) -> bool {
        self.needs_insert_height(height) || self.needs_insert_date(date)
    }

    #[inline(always)]
    fn needs_insert_height(&self, height: Height) -> bool {
        !self.to_all_inserted_height_map_vec().is_empty()
            && self
                .get_min_initial_states()
                .inserted
                .first_unsafe_height
                .unwrap_or(Height::ZERO)
                <= height
    }

    #[inline(always)]
    fn needs_insert_date(&self, date: Date) -> bool {
        !self.to_all_inserted_date_map_vec().is_empty()
            && self
                .get_min_initial_states()
                .inserted
                .first_unsafe_date
                .map_or(true, |min_initial_first_unsafe_date| {
                    min_initial_first_unsafe_date <= date
                })
    }

    fn to_inserted_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![]
    }

    fn to_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![]
    }

    fn to_inserted_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![]
    }

    fn to_inserted_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![]
    }

    fn to_inserted_mut_height_map_vec(&mut self) -> Vec<&mut dyn AnyHeightMap> {
        vec![]
    }

    fn to_inserted_mut_date_map_vec(&mut self) -> Vec<&mut dyn AnyDateMap> {
        vec![]
    }

    fn to_all_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        let mut vec = self.to_inserted_height_map_vec();

        vec.append(
            &mut self
                .to_inserted_bi_map_vec()
                .iter()
                .map(|bi| bi.get_height())
                .collect_vec(),
        );

        vec
    }

    fn to_all_inserted_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        let mut vec = self.to_inserted_date_map_vec();

        vec.append(
            &mut self
                .to_inserted_bi_map_vec()
                .iter()
                .map(|bi| bi.get_date())
                .collect_vec(),
        );

        vec
    }

    fn to_all_inserted_map_vec(&self) -> Vec<&(dyn AnyMap + Send + Sync)> {
        let heights = self
            .to_all_inserted_height_map_vec()
            .into_iter()
            .map(|d| d.as_any_map());

        let dates = self
            .to_all_inserted_date_map_vec()
            .into_iter()
            .map(|d| d.as_any_map());

        heights.chain(dates).collect_vec()
    }

    #[inline(always)]
    fn should_compute(&self, compute_data: &ComputeData) -> bool {
        compute_data
            .heights
            .last()
            .map_or(false, |height| self.should_compute_height(*height))
            || compute_data
                .dates
                .last()
                .map_or(false, |date| self.should_compute_date(*date))
    }

    #[inline(always)]
    fn should_compute_height(&self, height: Height) -> bool {
        !self.to_all_computed_height_map_vec().is_empty()
            && self
                .get_min_initial_states()
                .computed
                .first_unsafe_height
                .unwrap_or(Height::ZERO)
                <= height
    }

    #[inline(always)]
    fn should_compute_date(&self, date: Date) -> bool {
        !self.to_all_computed_date_map_vec().is_empty()
            && self
                .get_min_initial_states()
                .computed
                .first_unsafe_date
                .map_or(true, |min_initial_first_unsafe_date| {
                    min_initial_first_unsafe_date <= date
                })
    }

    fn to_computed_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![]
    }

    fn to_computed_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![]
    }

    fn to_computed_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![]
    }

    fn to_computed_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![]
    }

    fn to_computed_mut_height_map_vec(&mut self) -> Vec<&mut dyn AnyHeightMap> {
        vec![]
    }

    fn to_computed_mut_date_map_vec(&mut self) -> Vec<&mut dyn AnyDateMap> {
        vec![]
    }

    fn to_all_computed_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        let mut vec = self.to_computed_height_map_vec();

        vec.append(
            &mut self
                .to_computed_bi_map_vec()
                .iter()
                .map(|bi| bi.get_height())
                .collect_vec(),
        );

        vec
    }

    fn to_all_computed_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        let mut vec = self.to_computed_date_map_vec();

        vec.append(
            &mut self
                .to_computed_bi_map_vec()
                .iter()
                .map(|bi| bi.get_date())
                .collect_vec(),
        );

        vec
    }

    fn to_all_computed_map_vec(&self) -> Vec<&(dyn AnyMap + Send + Sync)> {
        let heights = self
            .to_all_computed_height_map_vec()
            .into_iter()
            .map(|d| d.as_any_map());

        let dates = self
            .to_all_computed_date_map_vec()
            .into_iter()
            .map(|d| d.as_any_map());

        heights.chain(dates).collect_vec()
    }

    fn to_all_map_vec(&self) -> Vec<&(dyn AnyMap + Send + Sync)> {
        let mut inserted = self.to_all_inserted_map_vec();

        inserted.append(&mut self.to_all_computed_map_vec());

        inserted
    }

    // #[inline(always)]
    // fn is_empty(&self) -> bool {
    //     self.to_any_map_vec().is_empty()
    // }

    fn pre_export(&mut self) {
        self.to_inserted_mut_height_map_vec()
            .into_iter()
            .for_each(|map| map.pre_export());

        self.to_inserted_mut_date_map_vec()
            .into_iter()
            .for_each(|map| map.pre_export());

        self.to_inserted_mut_bi_map_vec().into_iter().for_each(|d| {
            d.as_any_mut_map()
                .into_iter()
                .for_each(|map| map.pre_export())
        });

        self.to_computed_mut_height_map_vec()
            .into_iter()
            .for_each(|map| map.pre_export());

        self.to_computed_mut_date_map_vec()
            .into_iter()
            .for_each(|map| map.pre_export());

        self.to_computed_mut_bi_map_vec().into_iter().for_each(|d| {
            d.as_any_mut_map()
                .into_iter()
                .for_each(|map| map.pre_export())
        });
    }

    fn export(&self) -> color_eyre::Result<()> {
        self.to_all_map_vec()
            .into_par_iter()
            .try_for_each(|map| -> color_eyre::Result<()> { map.export() })
    }

    fn post_export(&mut self) {
        self.to_inserted_mut_height_map_vec()
            .into_iter()
            .for_each(|map| map.post_export());

        self.to_inserted_mut_date_map_vec()
            .into_iter()
            .for_each(|map| map.post_export());

        self.to_inserted_mut_bi_map_vec().into_iter().for_each(|d| {
            d.as_any_mut_map()
                .into_iter()
                .for_each(|map| map.post_export())
        });

        self.to_computed_mut_height_map_vec()
            .into_iter()
            .for_each(|map| map.post_export());

        self.to_computed_mut_date_map_vec()
            .into_iter()
            .for_each(|map| map.post_export());

        self.to_computed_mut_bi_map_vec().into_iter().for_each(|d| {
            d.as_any_mut_map()
                .into_iter()
                .for_each(|map| map.post_export())
        });
    }

    fn reset_computed(&self) {
        self.to_all_computed_date_map_vec()
            .iter()
            .for_each(|map| map.delete_files());
        self.to_all_computed_height_map_vec()
            .iter()
            .for_each(|map| map.delete_files());
    }
}
