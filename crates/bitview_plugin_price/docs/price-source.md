# Price source invariants

Truncate through `CachedVec`, not its `inner` vector. A same-length rollback
rewrite must invalidate shared read-only values; a no-op truncation retains the
warm snapshot. The vecdb rollback/read-only regression covers this boundary.

Producer/query histogram warmup shares one fallible reader. Validate block,
transaction and output bounds before subtraction, allocation and slicing;
reject incomplete reads and impossible per-block counts. Propagate failures
without publishing a partially warmed Oracle. These resource checks do not
replace Bitcoin consensus validation.
