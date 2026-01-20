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
use vecdb::Database;

pub use activity::Vecs as ActivityVecs;
pub use adjusted::Vecs as AdjustedVecs;
pub use cap::Vecs as CapVecs;
pub use pricing::Vecs as PricingVecs;
pub use reserve_risk::Vecs as ReserveRiskVecs;
pub use supply::Vecs as SupplyVecs;
pub use value::Vecs as ValueVecs;

pub const DB_NAME: &str = "cointime";
const VERSION: Version = Version::ZERO;

#[derive(Clone, Traversable)]
pub struct Vecs {
    #[traversable(skip)]
    pub(crate) db: Database,

    pub activity: ActivityVecs,
    pub supply: SupplyVecs,
    pub value: ValueVecs,
    pub cap: CapVecs,
    pub pricing: PricingVecs,
    pub adjusted: AdjustedVecs,
    pub reserve_risk: ReserveRiskVecs,
}
