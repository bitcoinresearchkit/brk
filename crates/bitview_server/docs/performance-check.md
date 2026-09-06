# Expanded local HTTP check — 2026-09-06

Read-only checks against the user-restarted daemon at `127.0.0.1:3110`,
version 0.12.2. No runtime performance changes were made during this pass.
The import cleanup is source-only and does not require restarting this daemon.

## Method and limits

`../benches/http_latency.py` retains the bounded probe: persistent HTTP/1.1,
identity encoding, two warmup triplets and eight measured GET/conditional GET/
HEAD triplets per route (three for the roughly 10 MB template). The first run
covered 29 routes, three gzip variants, and eight four-client burst cases.
Additional probes covered a dependent confirmed CPFP cluster and a live one.
Bodies are drained, and HEAD/304 responses are checked for empty wire bodies.
Measurements were made without concurrent Cargo builds.

These are warm loopback observations, including body transfer, not cold-device,
WAN/CDN, sustained-load or production-SLO evidence. First requests are not
controlled cold-cache measurements. There is no comparable pre-change baseline
for these additional routes, so these numbers do not establish regressions.
Small samples and selected addresses/transactions do not cover worst cases.

## Main findings

1. **Normal mempool update cycles cause visible availability gaps.** A separate
   jittered, low-rate probe took 100 observations per route over 18.7 seconds
   while indexed/computed/tip height stayed at 965745 and health remained caught
   up. `/api/mempool` returned 20 HTTP 503s; recent transactions, the selected
   confirmed address's stats, and the selected confirmed transaction's bulk
   outspends returned 21 each. `/api/mempool/hash` succeeded 100/100 times.
   Successful medians were respectively 0.879, 0.389, 0.382, 0.409 and 0.270 ms.
   This is a short-window observation, not an estimated long-term failure rate.

   Source inspection explains a recurring unavailable window: `Applier::apply`
   clears `published_tip` before mutation; the driver then resolves prevouts,
   checks the RPC tip and rebuilds the projection before publishing. Reads
   guarded by `ensure_published[_at]` fail while that marker is absent. This
   protects coherence and must not simply be removed. A follow-up should examine
   shortening the unpublished work or bounded waiting for coherent publication.

2. **Small series bursts hit response-body admission.** Four clients making
   twelve range requests produced seven successes/five 503s in the first probe.
   A second burst produced two successes/ten explicit `503/overloaded` results.
   `SeriesBodies` has two response-body permits, and the route uses non-waiting
   acquisition. Fail-fast backpressure is therefore observable at low client
   counts; the precise split depends on timing and retained response lifetimes.
   These errors must not be counted as fast successful requests. Any change
   needs to preserve the encoded-body memory bound, including slow clients.

3. **Validators do not always eliminate preparation work.** URPD 304s still
   validate source input; template HEAD still costs almost as much as GET.
   This is visible below and is separate from the publication/admission errors.
   Raw-block and template conditional GETs do avoid most body work.

## Selected warm serial medians

All times are milliseconds; GET columns contain successful 200s only.

| Request | GET | Matching 304 | HEAD |
| --- | ---: | ---: | ---: |
| Address chain history | 0.739 | 0.203 | 0.672 |
| Confirmed transaction JSON | 0.232 | 0.135 | — |
| Confirmed CPFP, isolated transaction | 0.138 | 0.082 | — |
| Confirmed CPFP, two-transaction cluster | 0.274 | 0.174 | 0.253 |
| Live CPFP, 5,447-byte response | 0.209 | 0.195 | 0.194 |
| Live transaction JSON | 0.164 | 0.159 | 0.170 |
| Block transaction page (25) | 0.355 | 0.095 | — |
| Raw block (1.53 MB) | 15.069 | 0.153 | 0.152 |
| Fuzzy series search | 3.221 | 0.129 | — |
| Daily price range, 2010–2025 | 0.372 | 0.135 | — |
| URPD latest, all/log100 | 8.348 | 6.478 | — |
| URPD historical, all/log100 | 6.258 | 4.041 | — |
| URPD weighted STH/cointime | 5.335 | 5.103 | — |
| Mempool template (9.915 MB) | 33.467 | 0.266 | 28.579 |

Raw-block gzip GET was 19.559 ms versus 15.069 ms identity on loopback;
this does not measure whether saved bytes help a remote client. Four-client
URPD GET median was 19.402 ms, including queueing, versus 8.348 ms serial.

The historical block was height 965704,
`00000000000000000001bd5ed0e28ec5f92197cf0e62438116c525a87c6aa1b5`.
The dependent CPFP seed was
`a148d3ba3bff3a4a25a373dbfcd6d30bf4c2257d3037cb9901b2b822491326e2`.
The live seed was
`67174c29fd02babcdbdd4e28820ed1603fe47adc34a1a61801141383f44b8c94`;
it is naturally not a stable future benchmark fixture.

The seven earlier full-history mining measurements remain in
`release-review.md`. Neither those nor this pass establish small-graph native
CPFP before/after performance across every topology.
