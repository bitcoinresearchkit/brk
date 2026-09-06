# Server/query release review

Scope: server/query changes from `v0.12.2`
(`9384e1c3f569acbf057500c5a1cb66241aa0fb63`) through the reviewed worktree,
including shared producers, storage, RPC and generated consumers.

## Decisions and fixes

- Checked date/index conversion rejects invalid February dates and width
  overflow. Native and real HTTP regression tests cover both range endpoints.
- Reads capture coherent plugin publications. Same-height reorgs, incomplete
  sources and unobserved/mismatched mempool publications cannot validate stale
  results. Confirmed transactions precede lagging mempool copies.
- Address balances, transaction/output boundaries, template/RBF traversal,
  packed URPD input, disk arithmetic and price fetcher chunks are checked.
- Dynamic address, template, mining and URPD response caches were removed.
  Immutable startup documents retain prepared representations. Native Oracle
  reuse keeps one source-validated confirmed window, never stale-on-error data.
- Admission precedes expensive work and large response allocation. Encoded-body
  permits survive compression and retained output frames. These limits are not
  a bound on total RSS or open connections.
- Broadcast is an uncached action with bounded input, deadlines and no replay
  after an ambiguous exchange. An explicit authentication rejection may refresh
  changed cookie credentials once. Native and generated clients decode its
  plain-text txid consistently.
- Legacy route aliases and orphaned generated schemas were removed. The BRK
  price fetcher uses current series endpoints. Rust date selectors are fallible.

The review included the complete release-diff and untracked-file inventory:
server/query, plugin gates/runtime, indexer/price/distribution/Bedrock producers,
mempool/RPC/reader, shared types/vecdb/quickmatch, daemon/CLI/latency, generator,
SDKs/MCP and publication scripts. Unrelated website presentation changes were
preserved. No deployment, package publication or transaction submission was done.

## Verification

The completed review ran the affected Rust library and integration suites,
reduced server feature configurations, real loopback HTTP/RPC fixtures, the
full-plugin daemon query fixture, JavaScript/Python client tests, Rust/JavaScript
quickmatch parity and fake-registry publication-script tests. Canonical
`bitview-bindgen --check` verified all ten generated outputs. Tests and retained
benchmarks now live under crate-level `tests/` and `benches/`; private tests stay
in the library harness without exposing production internals.

Key regression coverage includes validation before matching/wildcard 304s,
GET/HEAD metadata, publication waits/reorgs, source readiness, cancellation,
response-frame ownership, disk cycles, rollback/reopen and broadcast ambiguity.
Passing fixtures do not imply universal production performance or absence of
all possible bugs.

## Measured performance decisions

The one-entry native Oracle window was retained after comparing actual indexed
warmup with cloning: 12-block replay/clone P50 was 906/10.9 microseconds and
40-block P50 was 2,954/30.7 microseconds. The synthetic warm-storage fixture
checked identical EMA values. It excludes HTTP, cold disk and producer cost.

Mining timestamp hash-grouping/sort replaced tree grouping: one-million-row
algorithm-only medians were 9.22 versus 37.74 ms, with identical groups/order.
Persisted million-row size/weight preparation measured 16.86–25.07 ms across
resident/compressed timestamp cases. These are not HTTP latency claims.

After the user restarted the daemon, the deferred full-history HTTP check ran
at indexed/computed height 965,723 on 2026-09-06. The daemon's actual OpenAPI
contained the reviewed broadcast contract. Sequential HTTP/1.1 loopback requests
used one connection and identity encoding, two warmup pairs and 20 measured
pairs per route, alternating unchanged 200s and matching 304s.

| Mining route suffix | 200 P50 ms | 304 P50 ms | 304 P95 ms |
| --- | ---: | ---: | ---: |
| `blocks/sizes-weights/all` | 12.99 | 12.69 | 13.38 |
| `blocks/fees/all` | 8.22 | 8.01 | 8.23 |
| `blocks/rewards/all` | 11.01 | 10.80 | 11.25 |
| `blocks/fee-rates/all` | 29.75 | 29.56 | 31.46 |
| `hashrate` | 7.93 | 7.92 | 8.18 |
| `difficulty-adjustments` | 1.98 | 1.96 | 2.00 |
| `hashrate/pools` | 17.60 | 17.06 | 17.80 |

All 200 bodies/ETags stayed identical; matching GET/HEAD 304s were empty. Invalid
query parameters with wildcard validators returned 400 without an ETag. Health
and chain height were unchanged afterward. The first pool-hashrate request took
261 ms. These are warm serial local timings, not cold-device, compression, CDN,
concurrency or SLO evidence. Full-history content validation still computes the
result; 304 primarily saves transmission, not CPU.

## Deployment notes

Purge previously immutable CDN objects whose corrected representations may
remain fresh, especially [historical prices](historical-price.md) in Aggressive
mode. Origin headers cannot revoke an already-fresh edge object. The assistant
did not restart the daemon; the user performed the restart before live checks.
