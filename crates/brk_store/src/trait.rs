use brk_core::{Height, Result};

pub trait AnyStore {
    fn commit(&mut self, height: Height) -> Result<()>;

    fn rotate_memtable(&self);

    fn height(&self) -> Option<Height>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn has(&self, height: Height) -> bool;

    fn needs(&self, height: Height) -> bool;
}
