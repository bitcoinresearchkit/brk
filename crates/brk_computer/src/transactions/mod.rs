pub mod count;
pub mod fees;
pub mod size;
pub mod versions;
pub mod volume;

mod compute;
mod import;

use brk_traversable::Traversable;
use vecdb::{Database, Rw, StorageMode};

pub use count::Vecs as CountVecs;
pub use fees::Vecs as FeesVecs;
pub use size::Vecs as SizeVecs;
pub use versions::Vecs as VersionsVecs;
pub use volume::Vecs as VolumeVecs;

pub const DB_NAME: &str = "transactions";

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub(crate) db: Database,

    pub count: CountVecs<M>,
    pub size: SizeVecs<M>,
    pub fees: FeesVecs<M>,
    pub versions: VersionsVecs<M>,
    pub volume: VolumeVecs<M>,
}
