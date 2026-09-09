# bitview_cohort

UTXO and address cohort identities and typed groups for on-chain
analytics, including `WithAddrTypes`. Generic window, percentile, and resolution
groups live in `bitview_collections`. Vector constructors live in `bitview_vecs`.

Value collections and cohort identifiers work without the storage engine.
Enable the `storage` feature for traversal, vecdb formatting traits, and
storage-enabled BRK types. The Rust client leaves this feature disabled.

`UTXOGroupCore` supplies the common logical group fields; amount and type
extensions retain it by composition. `UTXOCoreValues` contains the four shared
disjoint cohort families, and `UTXOValues` adds amount/type values. Public groups
also retain the independently stored all/STH/LTH aggregates. Generic under/over
threshold cohorts are reconstructed by their consumers when needed.

`UTXOAndAddrGroups<T>` adds address-balance groups to `UTXOGroups<T>` as one
composed shape. It keeps output-value and controlling-address-balance cohorts
distinct, and exposes the existing UTXO paths plus `addr_balance`. Its optional
second type parameter lets the address axis own its stored sources. Use the
UTXO-only or reduced shapes for metrics that do not support address balances;
address predicates such as reused or exposed remain separate populations.

## Identity and Composition

`CohortId` identifies a supported cohort and supplies its canonical name and age
range membership. It composes the same typed selectors used by the collections;
there is no separate filter representation or caller-supplied cohort name.

```rust,ignore
pub enum CohortId {
    All,
    Term(Term),        // STH/LTH
    Age(AgeRangeId),       // Disjoint age bucket
    Amount(AmountRangeId), // Disjoint amount bucket
    Epoch(EpochId),    // Halving epoch
    Class(ClassId),    // Creation-year class
    Entry(EntryPrice), // Entry-price valuation band
    Type(OutputType), // P2PKH, P2TR, etc.
}
```

`AgeRangeId::select` and `AmountRangeId::select` select fields directly. Composed
UTXO groups accept `CohortId` and return `None` for unsupported families.

`CohortId::age_ranges()` enumerates the disjoint age inputs of all/STH/LTH.
Selection does not reconstruct a stored series from other series. Reconstruct
threshold metrics from their disjoint inputs before applying ratios or window
transforms. Exact realized prices require summed raw realized cap and supply;
capitalized prices require summed raw capitalized cap and raw realized cap.

## Example

```rust,ignore
use bitview_cohort::{AgeRangeId, CohortContext, UTXOGroups};

let id = AgeRangeId::From9MTo1Y.cohort();
let names = UTXOGroups::new(|id| CohortContext::Utxo.metric_name(id, "supply"));
assert_eq!(names.get(id).unwrap(), "utxos_9m_to_1y_old_supply");

// Naming adds utxos_/addrs_ only for age and amount cohorts, and omits all_.
assert_eq!(CohortContext::Utxo.full_name(id), "utxos_9m_to_1y_old");
```

## Built On

- `brk_error` for error handling
- `brk_types` for `Sats`, `Halving`, `OutputType`
- `bitview_traversable` for data structure traversal
