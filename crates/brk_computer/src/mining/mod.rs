pub mod hashrate;
pub mod rewards;

mod compute;
mod import;

use brk_traversable::Traversable;
use vecdb::{Database, Rw, StorageMode};

pub use hashrate::Vecs as HashrateVecs;
pub use rewards::Vecs as RewardsVecs;

pub const DB_NAME: &str = "mining";

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub(crate) db: Database,

    pub rewards: RewardsVecs<M>,
    pub hashrate: HashrateVecs<M>,
}
