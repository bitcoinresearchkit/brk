mod additive;
mod amount;
mod amount_value;
mod cumulative;
mod exact;
mod term;

pub use additive::{UTXOCoreSources, UTXOSources, UTXOTypedSources};
pub use amount::AmountSources;
pub use amount_value::AmountValueSources;
pub use cumulative::{
    CumulativeUTXOCoreSources, CumulativeUTXOCoreValueSources, CumulativeUTXOSources,
    CumulativeUTXOValueSources,
};
pub use exact::ExactUTXOSources;
pub use term::UTXOTermSources;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
