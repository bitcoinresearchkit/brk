use super::{AnyDataset, MinInitialStates};

pub trait AnyDatasets {
    fn get_min_initial_states(&self) -> &MinInitialStates;

    fn to_any_dataset_vec(&self) -> Vec<&(dyn AnyDataset + Send + Sync)>;

    fn to_mut_any_dataset_vec(&mut self) -> Vec<&mut dyn AnyDataset>;
}
