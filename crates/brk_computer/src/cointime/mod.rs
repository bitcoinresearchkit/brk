pub mod activity;
pub mod adjusted;
pub mod cap;
pub mod pricing;
pub mod reserve_risk;
pub mod supply;
pub mod value;

mod compute;
mod import;

use brk_traversable::Traversable;
use brk_types::Version;
use vecdb::{Database, Rw, StorageMode};

pub use activity::Vecs as ActivityVecs;
pub use adjusted::Vecs as AdjustedVecs;
pub use cap::Vecs as CapVecs;
pub use pricing::Vecs as PricingVecs;
pub use reserve_risk::Vecs as ReserveRiskVecs;
pub use supply::Vecs as SupplyVecs;
pub use value::Vecs as ValueVecs;

pub const DB_NAME: &str = "cointime";
const VERSION: Version = Version::ZERO;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub(crate) db: Database,

    pub activity: ActivityVecs<M>,
    pub supply: SupplyVecs<M>,
    pub value: ValueVecs<M>,
    pub cap: CapVecs<M>,
    pub pricing: PricingVecs<M>,
    pub adjusted: AdjustedVecs<M>,
    pub reserve_risk: ReserveRiskVecs<M>,
}
