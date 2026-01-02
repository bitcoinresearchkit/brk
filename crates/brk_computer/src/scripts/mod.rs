pub mod count;
pub mod value;

mod compute;
mod import;

use brk_traversable::Traversable;
use vecdb::Database;

pub use count::Vecs as CountVecs;
pub use value::Vecs as ValueVecs;

pub const DB_NAME: &str = "scripts";

#[derive(Clone, Traversable)]
pub struct Vecs {
    #[traversable(skip)]
    pub(crate) db: Database,

    pub count: CountVecs,
    pub value: ValueVecs,
}
