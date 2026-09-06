# Cohort alias expansion

One alias vector preserves the original priority ordering and is stably grouped
by the first byte of its first word. Binary search selects the only group that
can match a query's first normalized word; matching within it preserves priority.
No extra lookup index or result cache is retained.

Normalization guarantees nonempty ASCII words. The parity test compares every
registered alias and its word-prefix form with the original linear selection.
The retained benchmark is under `benches/unit/cohort_aliases.rs`; its timings
measure expansion only, not complete search or HTTP admission.
