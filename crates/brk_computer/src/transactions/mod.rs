pub mod count;
pub mod fees;
pub mod input_types;
pub mod output_types;
pub mod size;
pub mod versions;
pub mod volume;

mod type_counts;
mod compute;
mod import;

use brk_traversable::Traversable;
use vecdb::{Database, Rw, StorageMode};

pub use count::Vecs as CountVecs;
pub use fees::Vecs as FeesVecs;
pub use input_types::Vecs as InputTypesVecs;
pub use output_types::Vecs as OutputTypesVecs;
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
    pub input_types: InputTypesVecs<M>,
    pub output_types: OutputTypesVecs<M>,
}
