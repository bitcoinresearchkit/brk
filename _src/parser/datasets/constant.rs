use allocative::Allocative;
use struct_iterable::Iterable;

use crate::structs::{BiMap, Config, MapKind};

use super::{AnyDataset, ComputeData, MinInitialStates};

#[derive(Allocative, Iterable)]
pub struct ConstantDataset {
    min_initial_states: MinInitialStates,

    pub _0: BiMap<u16>,
    pub _1: BiMap<u16>,
    pub _50: BiMap<u16>,
    pub _100: BiMap<u16>,
}

impl ConstantDataset {
    pub fn import(config: &Config) -> color_eyre::Result<Self> {
        let f = |s: &str| config.path_datasets().join(s);

        let mut s = Self {
            min_initial_states: MinInitialStates::default(),

            // Computed
            _0: BiMap::new_bin(1, MapKind::Computed, &f("0")),
            _1: BiMap::new_bin(1, MapKind::Computed, &f("1")),
            _50: BiMap::new_bin(1, MapKind::Computed, &f("50")),
            _100: BiMap::new_bin(1, MapKind::Computed, &f("100")),
        };

        s.min_initial_states
            .consume(MinInitialStates::compute_from_dataset(&s, config));

        Ok(s)
    }

    pub fn compute(&mut self, &ComputeData { heights, dates, .. }: &ComputeData) {
        self._0.multi_insert_const(heights, dates, 0);
        self._1.multi_insert_const(heights, dates, 1);
        self._50.multi_insert_const(heights, dates, 50);
        self._100.multi_insert_const(heights, dates, 100);
    }
}

impl AnyDataset for ConstantDataset {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }
}
