use brk_core::{Height, Result, Version};

pub trait AnyStore {
    fn commit(&mut self, height: Height) -> Result<()>;

    fn persist(&self) -> Result<()>;

    fn reset(&mut self) -> Result<()>;

    fn name(&self) -> &'static str;

    fn rotate_memtable(&self);

    fn height(&self) -> Option<Height>;

    fn has(&self, height: Height) -> bool;

    fn needs(&self, height: Height) -> bool;

    fn version(&self) -> Version;
}
