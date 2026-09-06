# Overengineering audit

Scope: the server/query release diff from `v0.12.2` and the existing code it
integrates with, including producers, storage, RPC, generated consumers and
release tooling. This is a simplification audit, not a proposal for another
framework. The three findings below were fixed in the approved follow-up.

## Resolved findings

1. Removed the unused public `AppState::respond_json_adaptive` pipeline.
2. Removed `AppState::{tip_strategy,height_strategy,date_strategy}`,
   `CacheStrategy::{Tip,BlockBound}`, their tag constructors, and the generic
   response path's now-unused tip checks. Deleted the four ignored comparison
   benchmarks and one unit test that only exercised the retired policies.
   Production route identity checks and `CacheParams::series` remain intact.
3. Renamed `SeriesEntry::requires_gate` to `is_mutable`, including its field,
   constructor argument and callers. It still controls immutable cache-prefix
   eligibility; source/indexer/mappings publication guards remain unconditional.

These are intentional Rust API removals/renames, without compatibility shims.
Repository callers were checked; HTTP routes, payloads and active cache policies
are unchanged. External Rust consumers using the removed APIs must migrate.

## Cleanup performed

- Consolidated 35 review notes/reports into short crate-level documents;
  removed the oversized audit journals from the repository.
- Moved 38 standalone test/benchmark files and 86 inline test modules out of
  production source directories. Private unit tests use `#[path]` from their
  original module, avoiding new public APIs or duplicated test-only facades.
- Removed four obsolete benchmark files and six one-off benchmark functions;
  preserved their useful correctness checks. Retained measurements live under
  `benches/`; regression fixtures live under `tests/` (some fixture-local
  ignored performance checks share those fixtures).
- Updated the daemon publication regression to exercise public `Query::len`
  instead of a removed internal lookup method.

Original reports and retired benchmarks are recoverable from the cleanup
archives under `/private/tmp/bitview-audit-cleanup.59jYV8/`. This is temporary
local recovery storage, not a checked-in dependency.

## Complexity worth keeping

- Publication guards and resolved snapshots prevent mixed-generation reads.
- Response permits retained through compression and output frames enforce
  actual buffer ownership; a shorter handler-only permit would not.
- The single confirmed Oracle window has measured reuse value and validates
  its source publications. It is not a general response cache.
- Small typed response/preflight paths serve different identity and readiness
  requirements. Consolidating them merely to reduce method/file counts would
  hide those requirements. `AppState` size alone is not a finding.

See [release review](release-review.md) for the retained integration and
performance evidence. No deployment or external API migration was performed.

## Cleanup verification

The affected library/integration suites ran after relocation: 987 tests passed,
zero failed, and 33 opt-in tests were ignored across 54 test binaries. This
includes the daemon's public query/publication regression and loopback HTTP/RPC
fixtures. `git diff --check` also passed.
Retained library/benchmark targets and the server's no-default-features unit
test target compiled successfully. The minimal build emits feature-dependent
dead-code warnings; it was a compile check, not an additional test run.

The approved simplification follow-up reran the server/query/daemon library
and integration suites, including the publication-boundary fixture, and
compiled the minimal-feature server target. Active HTTP policies and source
guards remain covered by the existing route and reorg regressions.
