mod class_vecs;
mod dca_sats;
mod dca_stack;
mod dependencies;
mod has;
mod lump_sum_stack;
mod period_vecs;
mod vecs;

use bitview_collections::ByDcaClass;
pub use dependencies::Dependencies;
pub use has::HasInvesting;
pub use vecs::Vecs;

use bitview_plugin::{PluginId, PluginStorage};
use brk_types::{Dollars, Version};

const STORAGE: PluginStorage = PluginStorage::new(PluginId::new("investing"), Version::new(9));
pub const ID: PluginId = STORAGE.id();
const DCA_DOLLARS_PER_DAY: f64 = 100.0;
const DCA_AMOUNT: Dollars = Dollars::mint(DCA_DOLLARS_PER_DAY);
