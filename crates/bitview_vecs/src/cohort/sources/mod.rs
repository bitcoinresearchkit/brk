mod additive;
mod age;
mod amount;
mod amount_value;
mod cumulative;
mod term;

pub use additive::{UTXOCoreSources, UTXOSources, UTXOTypedSources};
pub use age::UTXOAgeSources;
pub use amount::AmountSources;
pub use amount_value::AmountValueSources;
pub use cumulative::{
    CumulativeUTXOCoreSources, CumulativeUTXOCoreValueSources, CumulativeUTXOSources,
    CumulativeUTXOValueSources,
};
pub use term::UTXOTermSources;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
