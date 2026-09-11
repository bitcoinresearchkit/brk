# Price source invariants

Truncate through the stored source's ordinary write API. A same-length rollback
rewrite invalidates its shared retained ranges; a no-op truncation keeps them warm.
The vecdb rollback/read-only regression covers this boundary.

Producer/query histogram warmup shares one fallible reader. Validate block,
transaction and output bounds before subtraction, allocation and slicing;
reject incomplete reads and impossible per-block counts. Propagate failures
without publishing a partially warmed Oracle. These resource checks do not
replace Bitcoin consensus validation.
