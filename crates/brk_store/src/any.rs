use brk_error::Result;
use brk_types::{Height, Version};
use fjall2::{InnerItem, PartitionHandle};
use fjall3::Item;

pub trait AnyStore: Send + Sync {
    fn name(&self) -> &'static str;
    fn height(&self) -> Option<Height>;
    fn has(&self, height: Height) -> bool;
    fn needs(&self, height: Height) -> bool;
    fn version(&self) -> Version;
    fn export_meta_if_needed(&mut self, height: Height) -> Result<()>;
    fn partition(&self) -> &PartitionHandle;
    fn take_all_f2(&mut self) -> Vec<InnerItem>;
    fn take_all_f3(&mut self) -> Vec<Item>;
    // fn take_all_f3(&mut self) -> Box<dyn Iterator<Item = Item>>;
}
