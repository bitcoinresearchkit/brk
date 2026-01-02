pub mod count;
pub mod spent;

mod compute;
mod import;

use brk_traversable::Traversable;
use vecdb::Database;

pub use count::Vecs as CountVecs;
pub use spent::Vecs as SpentVecs;

pub const DB_NAME: &str = "outputs";

#[derive(Clone, Traversable)]
pub struct Vecs {
    #[traversable(skip)]
    pub(crate) db: Database,

    pub spent: SpentVecs,
    pub count: CountVecs,
}
