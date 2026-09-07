# Post-restart chart replay and cache-placement comparison

## Live read-only replay

The restarted local server became query-ready at `/health` (not `/api/health`).
At readiness it reported height 965,958, zero blocks behind, version 0.12.2,
and start time `2026-09-07T18:17:46Z`. The replay ran at 18:22:40–18:22:43 UTC
against PID 32516, at height 965,959 before and after the run.

`replay_sparse_charts.py` sends four unconditional requests and one conditional
request for each of six chart shapes. It limits response sizes, pauses between
requests, and aborts on HTTP errors, timeouts, or responses exceeding two seconds.
It does not restart the service, open its database, or mutate chain data.

| Series | Resolution / rows | First request | Median of next 3 | Conditional 304 |
| --- | --- | ---: | ---: | ---: |
| `sopr_1m` | monthly / 120 | 7.350 ms | 6.839 ms | 0.914 ms |
| `sopr_1m` | daily / 365 | 0.820 ms | 2.327 ms | 1.283 ms |
| `net_pnl_change_1m_to_mcap` | monthly / 120 | 69.328 ms | 23.143 ms | 0.932 ms |
| `net_pnl_change_1m_to_mcap` | daily / 365 | 4.682 ms | 11.197 ms | 0.746 ms |
| `timestamp` | monthly / 120 | 0.601 ms | 1.359 ms | 0.883 ms |
| `timestamp` | daily / 365 | 0.836 ms | 0.905 ms | 0.951 ms |

All 24 bodies had the expected row counts. Within each shape, all four body
hashes matched. All six matching validators returned empty 304 responses. There
were no errors. The server remained healthy and caught up.

RSS was 6,359,024 KiB before and 6,384,096 KiB after: +25,072 KiB, about 24.5 MiB.
This is whole-process RSS, not retained-cache bytes; background activity and
allocator/page residency are included. The small sequential sample does not
establish production percentiles. “First” does not mean OS-cache cold: SOPR
monthly was used in an earlier readiness smoke check, and the server may have
other clients.

The live process exposes no decoder or cache-hit counters through these APIs.
No live hit rate or decode count is inferred from timings or 304 responses.
Operation counts below are from the instrumented isolated fixture. The live
server was not rebuilt or restarted for instrumentation.

## Selective aggregate cache: isolated comparison

The diagnostic chart fixture now compares **moving** the budgeted rolling cache
to the derived daily/monthly result, rather than adding a second cache to the
rolling result. Both variants retain the same cumulative source and metadata.
This is a benchmark-only candidate using the existing 2 GiB budget and unchanged
admission rule; production cache placement is unchanged.

Representative repeat run:

| Path | First fill | Warm query | Warm decodes | Retained result |
| --- | ---: | ---: | ---: | ---: |
| Existing monthly partial | — | 645.875 us | 244 | none |
| Monthly aggregate candidate | 1,255.250 us | 0.083 us | 0 | 3,712 bytes |
| Existing daily partial | — | 159.167 us | 112 | none |
| Daily aggregate candidate | 166.500 us | 160.917 us | 112 | none |

The full monthly snapshot decodes 466 pages initially, versus 244 for one
120-month partial read. Its 232 result rows fit one logical cache chunk, so the
existing admission rule retains it. The 365-day request does not cover every
chunk of the 6,945-row daily result, so that candidate remains nonresident;
its hypothetical full snapshot would occupy 111,120 bytes.

The monthly candidate can amortize its larger fill over repeated unconditional
reads between invalidations. It is not an end-to-end speedup measurement for the
live queries above: their real source graphs, update frequency, shared consumers,
and HTTP/serialization costs differ. Conditional 304 requests already avoid body
work and must not be counted as opportunities for this cache improvement.

Decision: retain the comparison and counters, not a blanket cache-policy change.
The candidate is specifically promising for repeated monthly reads; it provides
no demonstrated daily benefit, and moving a shared height cache across all
resolution consumers needs a real-data before/after validation before shipping.

## Concurrency checks

The compute test uses four readers, eight real stored sources, 512 snapshot
reads, 32 same-length replacements, publication exclusion, and capacity for only
two 64-byte resident snapshots. It checks coherent values and full byte-accounting
recovery after invalidation. This exercises capacity eviction and replacement
without allocating gigabytes or accessing live data.

The server fixture separately runs four concurrent chart clients while a cache
invalidator runs and the fixture changes real synthetic chain branches. Every
chart response must equal one complete branch's expected result, never a mixed
publication. The final branch and final chart are checked explicitly.

Results: the capacity/replacement test passed 20 consecutive runs. The combined
server test passed all 64 concurrent chart reads and four branch replacements
while cache invalidation ran (11.72 seconds including fixture setup). The chart
cache comparison and four cache-budget correctness tests passed.
`git diff --check` passed. Test-only server access to the compute budget adds one
workspace dev-dependency and its existing-package lockfile edge.

Final live health check at 18:32:19 UTC remained healthy and caught up at height
965,960, with the same process start time. The restart follow-up is complete;
the readiness heartbeat is paused.
