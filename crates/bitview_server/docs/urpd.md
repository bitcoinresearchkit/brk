# URPD query ownership

Resolve cohort, date, aggregation, weight, publication availability and packed
source bounds before matching conditionals. Matching tags do not skip missing
source or malformed-input errors. Raw and weighted queries share checked
decoding/aggregation; never silently truncate oversized or incomplete sources.

There is no retained URPD JSON cache. Matching requests validate captured source
entries without constructing buckets/JSON. Separate work and encoded-response
permits bound in-flight capture/build and slow transmission respectively. Body
ownership survives compression and frames retained after the HTTP future ends.
These are endpoint resource limits, not a total process-memory limit.

Regression fixtures under `tests/unit/urpd*.rs` exercise actual packed/weighted
sources, publication/reorgs, invalid inputs, conditionals and retained responses.
