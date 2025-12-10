use brk_error::Result;
use brk_types::{Height, Version};
use fjall::Keyspace;

pub trait AnyStore: Send + Sync {
    fn name(&self) -> &'static str;
    fn height(&self) -> Option<Height>;
    fn has(&self, height: Height) -> bool;
    fn needs(&self, height: Height) -> bool;
    fn version(&self) -> Version;
    fn export_meta_if_needed(&mut self, height: Height) -> Result<()>;
    fn keyspace(&self) -> &Keyspace;
    fn commit(&mut self, height: Height) -> Result<()>;
}
