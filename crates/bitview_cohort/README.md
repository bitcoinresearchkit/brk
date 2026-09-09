# bitview_cohort

UTXO and address cohort identities and typed groups for on-chain
analytics, including `WithAddrTypes`. Generic window, percentile, and resolution
groups live in `bitview_collections`. Vector constructors live in `bitview_vecs`.

Value collections and cohort identifiers work without the storage engine.
Enable the `storage` feature for traversal, vecdb formatting traits, and
storage-enabled BRK types. The Rust client leaves this feature disabled.

`UTXOGroupCore` supplies the common logical group fields; amount and type
extensions retain it by composition. `UTXOCoreValues` contains the four shared
disjoint cohort families, and `UTXOValues` adds amount/type values. Overlapping
public groups and disjoint input values remain distinct representations.

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
    Age(AgeId),        // Range, under, or over a named age threshold
    Amount(AmountId),  // Range, under, or over a named amount threshold
    Epoch(EpochId),    // Halving epoch
    Class(ClassId),    // Creation-year class
    Entry(EntryPrice), // Entry-price valuation band
    Type(OutputType), // P2PKH, P2TR, etc.
}
```

`ByAge::get(AgeId)` and `Amount::get(AmountId)` select fields directly. Composed
UTXO groups accept `CohortId` and return `None` for unsupported families. Narrow
selectors remain useful: `AgeRangeId` selects a disjoint age bucket, whereas
`AgeId` also supports overlapping thresholds.

`CohortId::age_ranges()` and `AmountId::ranges()` enumerate the disjoint inputs
of supported aggregates. Exact and overlapping stored series remain independent
sources; selection does not reconstruct a stored series from other series.

## Example

```rust,ignore
use bitview_cohort::{CohortContext, OverAgeId, UTXOGroups};

let id = OverAgeId::Over9M.cohort();
let names = UTXOGroups::new(|id| CohortContext::Utxo.metric_name(id, "supply"));
assert_eq!(names.get(id).unwrap(), "utxos_over_9m_old_supply");

// Naming adds utxos_/addrs_ only for age and amount cohorts, and omits all_.
assert_eq!(CohortContext::Utxo.full_name(id), "utxos_over_9m_old");
```

## Built On

- `brk_error` for error handling
- `brk_types` for `Sats`, `Halving`, `OutputType`
- `bitview_traversable` for data structure traversal
