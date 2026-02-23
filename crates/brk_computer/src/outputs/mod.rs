pub mod count;
pub mod spent;

mod compute;
mod import;

use brk_traversable::Traversable;
use vecdb::{Database, Rw, StorageMode};

pub use count::Vecs as CountVecs;
pub use spent::Vecs as SpentVecs;

pub const DB_NAME: &str = "outputs";

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub(crate) db: Database,

    pub spent: SpentVecs<M>,
    pub count: CountVecs<M>,
}
