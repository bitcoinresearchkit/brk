use super::AnyDataset;

pub trait AnyDatasetGroup {
    fn as_vec(&self) -> Vec<&(dyn AnyDataset + Send + Sync)>;

    fn as_mut_vec(&mut self) -> Vec<&mut dyn AnyDataset>;
}
