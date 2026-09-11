# Mempool read-write / read-only publication plan

Status: implemented and verified.

Written and completed: 2026-09-11. The pre-refactor observations describe the
starting worktree, which also contained unrelated changes preserved by this work.
Paths in this document are relative to the repository root.

## 1. Objective and agreed model

Give the mempool the same clear ownership split as our read-write and read-only
vecs: the writer owns updates, readers get a lean read-only handle, and immutable
data can be shared between them.

The read-only state stored in `ArcSwap` **is the published data**. There is no
additional generic snapshot layer to introduce. The existing `Snapshot` type
already represents the transaction graph and projected blocks; it is a domain
component to reuse, not a second implementation of the whole mempool state.

The normal flow is:

1. Readers use the last valid published state.
2. One writer updates private working state.
3. The writer prepares and validates the next read-only state.
4. One atomic replacement publishes it.
5. Existing readers finish with the version they acquired. Later readers see
   the replacement.

Routine refreshes must not turn valid data into `StateUpdating`. The initial
symptom is repeated 503s from `/api/mempool/price`, but the fix belongs in the
mempool ownership and publication contract, not in individual endpoints.

The result must be clean, KISS, DRY, fast, contained, and composed. This includes
writer CPU and memory costs, not merely the speed of loading a pointer.

## 2. Scope and boundaries

### In scope

- Separate writer ownership from the cloneable reader handle.
- Publish immutable read state through one shared `ArcSwap` root.
- Share transaction bodies and other substantial immutable data where useful.
- Move query methods off mutable live state.
- Make reads spanning several mempool components use one acquired version.
- Preserve confirmed-chain alignment, native query results, HTTP identities,
  template/diff semantics, address selection, CPFP, and RBF behavior.
- Migrate daemon wiring, query builders, CLI/examples, tests, and diagnostics.
- Remove superseded publication locks and update-only rejection mechanisms.
- Measure construction, publication, reads, destruction, and retained memory.

### Outside this refactor

- Changes to oracle mathematics, payment filtering, graph linearization,
  block selection, fee estimation, or transaction classification.
- Redesigning the indexer/vecdb publication mechanism.
- A generic MVCC, persistent-map, event-sourcing, or buffer-reclamation system.
- Endpoint-specific caches of prices, JSON, or histograms alongside the new
  authoritative published state.
- Blanket formatting, macros, broad dependency upgrades, or unrelated cleanup.
- Deployment or restarting a production daemon.

The earlier website chart fixes remain separate. The price poller changes are
an independent client concern; they must not be counted as fixing server
availability. Reassess their cache policy after the native fix, without using
client retries to hide a publication bug.

## 3. Pre-refactor implementation: facts preserved or replaced

| Area | Pre-refactor behavior | Consequence for this refactor |
| --- | --- | --- |
| `brk_mempool/src/lib.rs` | `Mempool` clones an `Arc<Inner>` containing the RPC client, live `RwLock<State>`, statistics publication, rebuilder, driver latch, and cycle mutex | Readers currently receive writer capability and live-state access. Split them explicitly. |
| `state/mod.rs` | `State` contains transactions, address indexes, spends, graveyard, statistics, and `published_tip` | Classify each field into working-only, read-only, or shared data. |
| `steps/applier.rs` | Clears `published_tip` before every application | This must not invalidate the last immutable publication. |
| `driver.rs` | Runs approximately every second; RPC fetch, apply, prevout fill, final tip check, rebuild, and publication are separate stages | Publication must happen only after the candidate is ready, with no visible intermediate mutation. |
| `state/mod.rs::publish_at` | Complete membership and resolved inputs are different facts | A single broad readiness flag would retain the current over-gating problem. |
| `api/histogram.rs` | Histograms read live `State` and require its fully published tip | Price currently inherits address/input readiness unnecessarily. |
| `stores/live_histograms.rs` | Raw and eligible histograms are incrementally maintained from transaction outputs alone | Preserve the incremental calculation; do not recompute by scanning all outputs at request time. |
| `Inner.info` | Already retains the last complete membership statistics, including while input resolution is incomplete | Fold this contract into the new publication rather than keeping a competing statistics store. |
| `snapshot/rebuilder.rs` | Publishes graph/projection and bounded template history under separate locks | Prepare these privately and integrate publication/history ordering. Preserve reuse checks. |
| `TxRecord` | Stores `Arc<Transaction>` plus entry metadata | Unchanged transaction payloads can already be shared. |
| `TxStore::apply_fills` | Uses `Arc::make_mut` before filling prevouts and changing sigop fields | Preserve this isolation; transaction identity alone does not imply identical response contents. |
| CPFP/RBF | Load the graph and live state separately, then check transaction revision equality | Acquire a compatible pair from one published root. Retain meaningful provenance checks until construction proves compatibility. |
| Query/server | Several resolved results already capture owned selections before asynchronous body construction | Extend this pattern; do not reload a newer publication during build. |

The historical `crates/brk_mempool/PLAN.md` is not the implementation checklist
for this work. Its proposed moves and compatibility assumptions are dated.
Use current code and `crates/bitview_server/docs/release-review.md` for the
existing safety contracts; do not reopen that entire historical cleanup.

## 4. Non-negotiable invariants

- There is exactly one authoritative publication root for current mempool read
  data. No second independently swapped graph or histogram may silently race it.
- Only the writer can replace the root. Readers cannot start a driver, tick,
  mutate stores, publish, or obtain mutable references to published data.
- The published root and everything used to compute its query results are
  immutable for their entire lifetime.
- One logical read acquires the publication once. Helper calls receive that
  state or a selection captured from it, rather than loading again.
- A failed update cannot partially replace a publication or clear valid data
  merely because the writer is working.
- An empty, successfully observed mempool is valid data. Startup with no
  observation is not equivalent to an empty mempool.
- Data combined with confirmed-chain state must match the captured full block
  hash. Height alone is insufficient, including for same-height reorgs.
- A read-only handle follows future publications; an acquired read-only state
  stays on its version. These are distinct operations with clear names.
- Template hashes, transaction-list hashes, and HTTP ETags identify the exact
  representation, not publication time, pointer address, or cycle count.
- Confirmed transactions and confirmed spends retain precedence over lagging
  mempool copies.
- Memory use includes writer state, the current publication, older publications
  still held by requests, and retained template/RBF history. Two buffers are not
  a guaranteed upper bound.

## 5. Target ownership and API

The implementation uses these concrete types:

| Type | Ownership and responsibilities |
| --- | --- |
| `Mempool` | Non-cloneable writer. Owns the client, private working state, update machinery, and access to publication. |
| `ReadOnlyMempool` | Cloneable reader handle. Privately shares the publication slot; exposes acquisition and read-only entry points. |
| `ReadOnlyState` | The actual immutable data stored in the slot. Composes read stores and the existing graph/projection where valid. |

`Mempool::read_only_clone()` should be cheap: clone the handle to the publication
slot, not the transaction maps or their payloads. Query builders should accept
`Option<ReadOnlyMempool>`, making the capability boundary visible in signatures.

The slot is `Arc<ArcSwap<ReadOnlyState>>`. Its initial read state has no pool,
no projection source tip, and an unready default graph. Native query methods
return `StateUpdating` until their first valid observation; a coherent empty
pool is a distinct successful publication. This allows a complete GBT to become
available before complete membership without introducing another atomic slot.

This follows `ReadOnlyClone` conventions without importing vecdb-specific
`StorageMode::Stored` machinery into an in-memory mempool. The trait currently
lives in vecdb, which `brk_mempool` does not depend on. Prefer an inherent
`read_only_clone()` over adding vecdb solely for that trait. Reuse an existing
lightweight shared trait if one is available by implementation time; extracting
a new generic framework is not a prerequisite for this change.

### Writer methods

Prefer `tick(&mut self)` / `tick_with(&mut self, resolver)` and a driver that
owns the writer. Rust ownership then prevents concurrent mutation. Migrate
callers before deleting the current cycle mutex and double-start latch. Retain
any synchronization that still has a demonstrated caller, not a hypothetical
future need. Do not keep a cloneable writer just to avoid changing examples.

Pipeline helpers can then take `&State` or `&mut State`, instead of a lock that
each helper reacquires. External RPC/resolver calls borrow only the needed
inputs. The resolver may read the confirmed indexer and the published mempool;
it must not require a mutable borrow of the writer or wait for its own cycle.

### Reader methods

- Synchronous methods may borrow an acquired state for their bounded work.
- A composed query loads once at its entry point and passes `&ReadOnlyState`
  through related operations.
- Results crossing a task boundary or `await` own an `Arc` or an already
  resolved, narrowly scoped selection.
- Do not pin a whole large publication just to return one transaction if
  cloning its existing `Arc<Transaction>` is sufficient.
- Convenience methods on the reader handle may load once and delegate. The
  shared query algorithm lives on the acquired read view, not in duplicated
  writer/reader implementations.

`ArcSwap::load()` is intended for short guarded access; `load_full()` provides
an owned `Arc` when the version must escape the call. Keep this choice contained
inside acquisition/resolution code instead of exposing ArcSwap strategy types
through the public API. See the [reading operations documentation](https://docs.rs/arc-swap/latest/arc_swap/struct.ArcSwapAny.html#method.load_full).

## 6. Data placement and sharing

Inventory real readers before designing read-only counterparts. Splitting a
type is useful when it removes writer-only fields or enables sharing; do not
generate paired types for every small struct mechanically.

| Data | Read requirement | Planned treatment |
| --- | --- | --- |
| RPC client, fetch batches, `CycleDiff`, resolver bookkeeping | Writer only | Keep outside published state. |
| Transaction bodies | Transactions, addresses, template bodies, RBF | Share existing `Arc<Transaction>` values; preserve copy-on-write fills. |
| Transaction entry metadata | Fees, times, ordering, RBF, graph construction | Keep one canonical record representation; copy small metadata or share an immutable record when measurement justifies it. |
| Transaction lookup/order | Membership, lists, body lookup, spends | Preserve `IndexMap` ordering and full-txid validation; freeze lookup containers once per changed version. |
| Unresolved-input worklist | Writer scheduling | Keep the worklist private; expose only the readiness information needed by read APIs. |
| Address statistics and tx membership | Address queries | Publish only a valid address view; retain deterministic ordering/limits. Account for the cost of nested txid sets. |
| Outpoint-spender index | Outspends and overlays | Freeze with the transaction view it indexes; retain exact full-outpoint checks after prefix lookup. |
| Raw/eligible histograms and pool statistics | Price, histogram, info endpoints | Include completed output/membership data in the publication. Preserve existing incremental updates. |
| Recent transactions and first-seen data | Recent/times endpoints and ordering | Preserve current recent-window semantics; it is not just the live membership list. |
| Graveyard records and replacement links | RBF and vanished-body fallback | These are partly read data. Do not classify the whole graveyard as writer-only. Share bodies; retain required links and ordering. |
| Graveyard eviction queues and revival work | Writer lifecycle | Keep private unless a current read operation actually needs part of their order; derive a lean read representation where needed. |
| Graph, chunk rates, projected blocks, fees | CPFP/RBF/fees/template | Reuse the existing `Snapshot` calculations and immutable payload; compose it into the root rather than publishing it separately. |
| Template history | Diff resolution | Retain bounded immutable body references with a coherent current-template selection. See section 10. |
| Diagnostics | CLI/debugging | Distinguish writer progress from published query data. Avoid making readers lock live state for debug counters. |

### Initial copying strategy

Start with the least complicated correct freeze operation per substantial
store: clone lookup structure and necessary metadata once, while sharing body
Arcs. Do not serialize, deep-clone transaction bodies, or rebuild already
maintained histograms to publish them.

A read-write store can contain its read-capable data plus writer-only
bookkeeping, like the existing vec types. Read methods belong to that common
read representation. Publication still needs a frozen version of any mutable
container; wrapping a live map in an Arc without isolating later writes is not
sufficient.

Measure before deciding between explicit container freezing and component-level
copy-on-write. If copying is material, share a costly unchanged component or
copy it once on its first mutation. Choose one ownership strategy per store;
do not layer eager clones, copy-on-write, a dirty cache, and a rebuild path for
the same data. Avoid `Arc` per scalar, persistent maps, sharding, and delta
replay solely to avoid an unmeasured copy.

The current `Arc::make_mut` transaction path is a useful existing mechanism:
when a published body is shared, a fill modifies a private copy. Verify pointer
reuse for unchanged bodies and isolation for changed bodies. [Rust Arc documentation](https://doc.rust-lang.org/std/sync/struct.Arc.html#method.make_mut)

## 7. Readiness: resolve the layout before migrating readers

This is the main design checkpoint. “Consistent” does not mean every possible
view must wait for every other view. Conversely, one ArcSwap pointer does not
make unrelated source observations compatible automatically.

### Required behavior matrix

| Situation | Required behavior |
| --- | --- |
| No successful observation yet | Unavailable for views with no valid data; do not fabricate zero price/empty success. |
| Writer is fetching, applying, filling, or rebuilding | Continue serving the last published data applicable to the request. |
| Coherent complete empty pool | Publish and serve valid empty results. |
| Complete membership, unresolved inputs | Statistics and output histograms can be current. Address/input-dependent views must not claim completeness. |
| Fetch cap or missing transaction bodies | Do not publish incomplete membership as a complete pool; retain prior valid membership data. |
| Exact complete GBT, but raw listing is incomplete or differs | Preserve the distinction between an authoritative block template and complete mempool membership. |
| Failed fetch, final chain observation, or candidate validation | Retain valid prior publication; no partial commit. |
| Captured indexed tip differs from the applicable publication tip | Reject the combined read; no height-only match and no silent confirmed-only price fallback. |
| Confirmed oracle source not computed or indexer publication unavailable | Preserve its existing error/wait behavior. ArcSwap does not make confirmed data ready. |
| Same pool/tip but input fills change a body or sigops | Publish changed content and compatible graph/results; do not reuse identity based only on txids. |
| Persistent upstream failure | Last completed data remains last completed data. Diagnose failure/age; do not label it as a new successful observation. |

### Recommended composition rule

Keep one publication root, with the smallest set of domain components needed
to express the matrix. Use existing membership, resolved-input, and projection
facts; avoid a generic readiness bitset or per-endpoint cached state.

The candidate membership view may be valid even when its inputs are unresolved.
Expose output-only reads from it and gate only operations that require complete
inputs. Do not carry an old address index alongside new transactions and
pretend they describe one version.

Existing template behavior is a separate source contract: the fetcher can
synthesize GBT-only bodies even when the raw listing differs. If preserving
that availability requires a separately valid template component, compose its
immutable value and source identity into the same publication root. Reuse the
same graph/body Arcs when it agrees with membership. A graph used by CPFP/RBF
must match their transaction view; it cannot be borrowed from an unrelated
newer template component.

Retaining a valid component through another component's failed refresh is
allowed only with explicit provenance and no cross-version joins. Prefer
retaining the entire prior root on a failed cycle. Do not add independently
swapped subpublications or retain a history of pool versions in the writer.

### Implemented layout and constructor invariants

`ReadOnlyState` contains an optional `Arc<Pool>`, the existing `Arc<Snapshot>`
for the last valid template projection, that projection's full source tip,
ten distinct retained template identities/bodies, and last coherent writer
counters. These values are replaced together through the one ArcSwap.

`Pool` contains one complete membership observation: full tip, statistics,
read-only transactions/recent/histograms, addresses, spends, graveyard, the
matching graph, and input completeness. CPFP and RBF use this graph, never an
independently newer template. Construction asserts that graph and transaction
content revisions match. Complete membership is validated against every full
RPC txid before publication. Histogram reads require membership and matching
tip; input-dependent reads additionally require resolved inputs.

This composed layout is necessary for the existing GBT/listing discrepancy:
an exact GBT can advance while membership retains its last complete observation.
A failed final chain observation retains the entire public root. Ordinary
private updates never invalidate a completed view.

Changed transaction containers freeze once and share transaction bodies.
Unchanged transaction/address/spend containers are shared across projection or
tip changes; graveyard sharing follows its own mutation revision. Address maps
share immutable address records, copying only changed records and their txid
sets. Measurements justified this: copying every nested address set was the
largest avoidable publication cost. No persistent map, endpoint cache, reader
cache, or custom reclamation scheme was added.

## 8. Update pipeline and commit boundary

Preserve the current fetch cap, GBT synthesis, transaction classification,
prevout resolver, graph calculation, and cycle event ordering. Change ownership
and publication around them.

1. Observe the full node tip before the fetch.
2. Fetch RPC data and validate the fetched chain observation before mutation.
   A JSON-RPC batch is not an atomic observation of the chain or pool.
3. Prepare/apply the diff to private working state. Use the previous working
   graph where removal classification/burial needs it; a stale public graph is
   not necessarily the preceding working revision after an unpublished cycle.
4. Resolve same-cycle parents and external prevouts. Keep callbacks outside
   publication operations and confirmed-indexer write ownership.
5. Complete the existing chain-observation bracket. Candidate construction
   from that owned observation must not re-read mutable RPC data inconsistently.
6. Build/reuse the graph, read stores, aggregates, and required history privately.
7. Validate completeness, source tips, and graph/body compatibility. All
   fallible construction finishes before publication.
8. Replace the publication root exactly once. No callbacks, I/O, hashing of
   large responses, or allocation belongs inside a separate commit lock.
9. Return the existing cycle events and diagnostics with explicit provenance.
   An applied private cycle and a successful public commit are not synonymous.

Do not clear the old publication before steps 3–7. If the candidate cannot be
published, the writer can continue reconciling its private state next cycle;
ordinary incomplete input resolution must not require cloning the whole pool
just to roll back.

### Failure and panic recovery

The writer marks private state as needing recovery before applying a diff.
A normal completed cycle clears that marker, including a cycle whose final tip
observation prevents publication. If a panic interrupts mutation, the next tick
restores working stores and the matching graph from the last published pool,
reconstructing the unresolved-input worklist from its immutable bodies. Before
any pool publication, recovery resets to an empty private state and fetches
again. Reader data remains intact throughout.

Fault injection covers a resolver panic after application, retained HTTP reads,
and a subsequent successful recovery tick. `Cycle` remains a record of private
work, including applied changes on a failed final observation; it does not
claim a public commit or fabricate reversal events.

### Reuse and identity

Keep the current graph reuse conditions: transaction content revision, template
ordering, minimum fee, and membership change. Audit other changes separately:
recent order, statistics, first-seen metadata, graveyard expiry/replacement
links, and readiness can change even when a txid hash does not.

Use mutation results and existing revisions where sufficient. Add a revision
only when it has a defined source and consumer. Do not hash the whole read
state each second to detect changes, or add a counter to every struct by default.
Publication sequence, if needed for diagnostics, is not a response validator.

## 9. Query and confirmed-chain integration

Migrate both `Query::build` and `AsyncQuery::build` to receive read-only mempool
handles. Audit every `mempool()`/`require_mempool()` call, including nested calls.

| Query area | Required preservation |
| --- | --- |
| Oracle price and live payment histogram | Acquire the eligible histogram at the captured confirmed tip; keep the existing confirmed-window cache keyed by tip and revision. |
| Live raw-output histogram | Read the published raw histogram with the existing safe-length/tip rules. Preserve its different indexer-lock needs. |
| Address stats and combined transaction lists | One applicable mempool view plus a compatible confirmed pin; deterministic ordering, limits, and stable captured bodies. |
| Transaction resolution | Confirmed-first lookup, vanished-body fallback excluding replaced tombstones, and existing unknown/error distinctions. |
| Outspends | One version for parent lookup, bounds, spender lookup, and all overlays. Never overwrite a confirmed spend. |
| CPFP | Matching graph, body, and sigop version; preserve traversal limits and fee calculations. |
| RBF/replacements | Matching records, replacement history, and graph rates; preserve depth/work limits and confirmed enrichment checks. |
| Aggregate/list/time reads | Hash and result from exactly the same selected data and ordering. |

Preserve the current indexer publication guards and safe-prefix pins. Acquire
the applicable mempool state within a coherent confirmed read, then use the
existing ownership/pin rules for deferred work. No writer lock should be held
while calling into the confirmed indexer.

An indexed tip change after acquisition does not justify switching only the
mempool half of a resolved response. Complete against the captured compatible
sources where the existing pin permits it, or reject/re-resolve according to
the current endpoint contract.

Do not add a hidden cache of a loaded mempool version to the process-lifetime
`Query`. Acquisition belongs to a request/resolution operation. One request
must pass the acquired value through its phases; repeated loads can mix
versions. [ArcSwap consistent-read pattern](https://docs.rs/arc-swap/latest/arc_swap/docs/patterns/index.html#consistent-snapshots)

## 10. Templates, history, and response ownership

Preserve `BlockTemplateSource` and `ResolvedBlockTemplateDiff` as captured domain
selections, or simplify them only if the same ownership contract remains clear.

- Template body, statistics, and hash must come from the same projection.
- Every advertised current template hash must have the corresponding retained
  body references required by diff resolution.
- Resolve the requested history entry and current selection coherently. Capture
  both before asynchronous handoff; later publication/eviction cannot alter them.
- Prefer publishing a small bounded immutable history index with the current
  template in the root. Reuse body Arcs; do not duplicate retained payloads.
- Audit the current `HISTORY = 10` bound and preserve its meaning. Do not count
  unrelated price/statistics publications as new template-history entries.
- Preserve reorder/removal order and exact diff round trips.
- A retained txid with changed body/prevouts must be emitted as changed. The
  current Arc/body-identity checks must not be weakened to txid equality.
- Capture only the necessary template/history Arcs for queued body work rather
  than retaining the entire mempool publication unnecessarily.
- Keep unknown-history and incomplete-template errors distinct from empty
  valid templates.

## 11. HTTP and frontend behavior

The server remains an adapter over native query results. Do not add route-
specific publication workarounds.

- Validate parameters, source availability, chain alignment, and requested
  history before honoring conditional headers, including `If-None-Match: *`.
- Use cheap exact validators before large body allocation/serialization where
  the endpoint already supports that. Preserve deferred body admission.
- Carry the captured source from validator calculation through body build.
  Separate hash-only and body calls must not load different publications.
- Preserve GET/HEAD metadata, empty 304 bodies, content identity across price
  aliases, representation/compression rules, and error `no-store` behavior.
- Retained valid data can legitimately return the same ETag during a refresh.
  A publication pointer replacement alone must not invalidate a response.
- Keep template-body and txid-body permits alive for the required output
  lifetime. An ArcSwap refactor must not bypass memory admission.
- Price errors caused by absent/mismatched confirmed data remain real errors.
  Do not return a fabricated cached price or silently change its definition.

Review `website/scripts/utils/price.js` after the server behavior is proven.
The no-overlap/visibility behavior can stand on its own. Assess whether
`cache: false` and failure backoff are still appropriate for the actual client
contract; preserve useful HTTP revalidation rather than disabling it merely
to work around stale-on-error client behavior. Changes to the generated client
must be made in its generator, never by hand, and remain a separate justified
change if needed.

## 12. File and caller migration map

Keep one public struct per new/touched Rust file. Use module-level imports,
ordinary functions/composition, and domain-local helpers. Do not manufacture
an owning service type merely to eliminate a free function. Choose names that
make the current graph `Snapshot` and whole `ReadOnlyState` unambiguous.

| Files / area | Work |
| --- | --- |
| `crates/brk_mempool/src/lib.rs` | Exports, writer construction, cheap read-only clone; remove shared writer authority. |
| `read_only.rs`, `state/read_only.rs`, and `state/pool.rs` | Lean handle, actual published data, and coherent membership, one concept per file. |
| `state/mod.rs`, `stores/*` | Separate private working fields from read data; contain freeze/share logic near its store. |
| `driver.rs`, `steps/*` | Single-owner updates, direct borrows, candidate construction, one publication point, failure recovery. |
| `snapshot/rebuilder.rs` | Make graph/history preparation private to the writer; replace independent current publication with composition. |
| `api/*`, `snapshot/cpfp.rs` | Read methods on acquired immutable data; preserve algorithm implementations. |
| `cycle/event.rs`, `diagnostics.rs` | Distinguish working-cycle outputs from committed read data. Preserve CLI event meaning. |
| `crates/bitview_query/src/lib.rs`, `src/async.rs` | Reader-only builder/storage/accessor signatures. |
| `bitview_query/src/impl/{mempool,oracle,addr,tx,cpfp}` | One acquired version per logical read and preserved confirmed pins. |
| `crates/bitview/src/lib.rs` | Give the query a read-only handle; move the sole writer into its driver thread. |
| `crates/mmpl/src/main.rs`, `brk_mempool/examples/mempool.rs` | Mutable writer for ticks, separate reader for observations where needed. |
| `crates/bitviewd/examples/{query,server}.rs` and query fixtures | Migrate builder signatures and ownership. |
| `crates/bitview_server/src/api/mempool.rs`, `src/api/oracle.rs` and resolution helpers | Audit source/validator ownership; change only what the new query boundary requires. |
| Crate tests/benches and doctests | Construct valid publications through helpers; no public production mutators for tests. |
| Workspace and `brk_mempool` manifests | Add a compatible workspace `arc-swap` dependency, verifying version/MSRV/features at implementation time. |

Keep existing public aliases only for demonstrated external compatibility.
Inventory public API changes explicitly: writer cloning, receiver mutability,
builder types, startup acquisition, `snapshot()` behavior, cycle fields, and
diagnostics. Do not leave duplicate old/new behavior indefinitely under shims.

The example that currently reads live debug counters from another thread needs
an explicit choice: display last-cycle diagnostics published as values, or
keep separate minimal operational counters. Do not reintroduce a lock on the
entire working pool to preserve that example verbatim.

## 13. Performance plan

ArcSwap changes reader synchronization; it does not remove container copies,
allocator work, graph rebuilding, or last-reader destruction. Its own docs
recommend workload-specific measurements. [ArcSwap performance guidance](https://docs.rs/arc-swap/latest/arc_swap/docs/performance/index.html)

### Baseline before changes

Capture the commit/worktree identity, build profile, machine, fixture sizes,
transaction/output/address counts, churn, and concurrent readers. Existing
working-tree changes make a bare HEAD comparison insufficient: isolate the
baseline without discarding or committing someone else's work.

Measure the current end-to-end path as well as successful-read timing. Record
503 frequency and time spent waiting on updates. A design that quickly returns
503 must not appear faster than one that successfully serves the request.

### Workloads

- Empty and small deterministic pools.
- Representative medium/large pools, e.g. 10k/50k/200k transactions when fixtures
  and the machine support them. Record actual dimensions, not only tx count.
- No membership change; metadata-only change; small churn; replacement bursts;
  block-sized eviction; input-fill-only changes; graveyard expiry.
- GBT-only membership discrepancy and startup exceeding the fetch cap.
- Short parallel price/info reads plus address/transaction/template/RBF reads.
- Slow queued body work retaining an older version while several new versions
  publish; cancellation followed by reclamation.

### Measurements

| Metric | Why |
| --- | --- |
| Whole cycle and per-stage p50/p95/p99 | Detect a freeze/build cost hidden by a fast swap. |
| Time/bytes/allocations for freeze and publication | Separate map copying, body sharing, graph work, and atomic replacement. |
| Successful read latency and throughput under updates | Verify the availability and contention improvement at the real boundary. |
| Peak/resident memory and retained generations | Account for long readers and bounded histories. |
| Changed versus unchanged transaction body clones | Prove intended sharing and input-fill isolation. |
| Last-reader and writer destruction time | Large old containers may be freed by either thread. |
| No-op cycle work | Ensure a quiet pool does not deep-copy all data gratuitously. |
| HTTP 200/304 results and latency | Preserve useful revalidation and exact bodies. |

Both histograms contain 2,400 `u32` bins: 19,200 bytes of counter payload
combined. That copy is bounded; transaction/address maps and history are the
likely costs to measure. This is a size calculation, not a benchmark.

Do not set arbitrary throughput promises before collecting the baseline. The
change is acceptable only with demonstrated successful-read availability,
preserved results, and an explicitly reviewed writer/memory cost on realistic
pools. If construction is too expensive, simplify or share the measured costly
component first. Do not jump to persistent collections or custom reclamation.

Do not use `ArcSwap::Cache`, thread-local generation caches, detached cleanup
workers, or rotating buffers in the initial design. Add any such mechanism
only for a measured problem, with bounded retention and clear ownership.

## 14. Test matrix

Use existing crate-level tests and HTTP/RPC fixtures. Add tests for behavior at
the ownership/publication boundary, not tests that merely repeat field wiring.
Use explicit barriers/channels to control update phases instead of relying on
sleep timing for the core race assertions.

### Ownership and publication

- Reader handles cannot tick, start, mutate, or publish; writer is not cloneable.
- Initial unpublished versus coherent empty pool are distinguishable.
- Hold version A, publish B/C, and prove every field of A remains unchanged.
- Slow private application, external resolution, and graph build leave A
  readable without taking the working-state lock.
- Each composed read sees entirely A or entirely B, including graph/body and
  validator/body pairs.
- Unchanged transaction bodies share pointers. Prevout/sigop changes preserve
  old bodies and change the new version appropriately.
- Failed/incomplete cycles retain valid data; panic recovery cannot publish
  corrupted private indexes.
- Dropping/canceling readers releases old versions; the writer does not retain
  an unbounded chain of prior states.

### Readiness and chain consistency

- Complete membership plus unresolved inputs still serves valid statistics and
  raw/eligible histograms.
- Address/input-dependent results never present unresolved data as complete.
- New block, lagging indexer, lagging mempool, and same-height reorg exercise
  full-hash matching and the chosen retention policy.
- Unknown/unobserved tip, failed final RPC, capped/incomplete membership, and
  GBT/listing discrepancy exercise the section 7 matrix.
- Mempool-disabled queries keep their existing behavior.
- Price/EMA values equal the reference computation from the same confirmed
  base and eligible histogram. No cumulative double-blending across requests.
- Missing confirmed price vectors still fail correctly.

### Existing domain contracts

- Address pagination, stable tie-breaking, confirmed/mempool combination, and
  selections surviving a subsequent membership change.
- Confirmed-first transaction and spend precedence, vanished fallback excluding
  replaced transactions, invalid output bounds, prefix-collision checks, and
  overlays without overwriting confirmed spends.
- CPFP graph/body/sigop alignment and existing traversal/fee results.
- RBF history order, full-RBF detection, bounded depth/work, immutable captured
  trees, and confirmed-rate enrichment.
- Template diff round trips, reorders/removals, body change under the same txid,
  unknown history, valid empty templates, and retention through eviction.
- Template history exists when its hash becomes visible. Unrelated publications
  do not consume history capacity.
- Cycle events, address transitions, graveyard revival/expiry, and CLI output
  remain intentional even when a private cycle is not publicly committed.

### Real HTTP integration

Extend `crates/bitview_server/tests/unit/addr_publication.rs`,
`mempool_publication.rs`, `transaction_publication.rs`, `oracle.rs`, and template
fixtures as applicable. Recheck their APIs before editing.

- During a blocked same-tip refresh, `/api/mempool/price`, `/api/oracle/price`,
  and both live histogram routes serve the valid retained representation.
- Matching GET/HEAD validators return 304 with correct metadata and no body;
  unmatched validators return the matching captured body.
- Genuine tip mismatch/unavailability still fails before wildcard revalidation.
- Source acquisition followed by publication before serialization cannot mix
  the old ETag with a new body.
- Template and txid admission/retained-body ownership remain effective.
- Update old tests that intentionally expected update-window 503s; preserve
  startup, unresolved-source, invalid-history, and mismatched-tip protections.
  Do not replace every 503 assertion mechanically.

## 15. Execution sequence and completion gates

Each step includes its relevant tests. Do not defer correctness tests to the
end of a broad ownership rewrite.

### A. Freeze the contract and baseline

- [x] Recheck instructions, worktree changes, callers, and existing fixtures.
- [x] Record the before API and representative output/performance baselines.
- [x] Resolve section 7's concrete readiness layout and write it into this plan.
- [x] Decide the bounded panic-recovery path and cycle-event provenance.
- [x] Record which existing locks/fields/API methods each new responsibility
  replaces. Reject additions with no clear ownership benefit.

### B. Establish the ownership split

- [x] Add the writer/read-only handle boundary and private publication slot.
- [x] Migrate daemon, query builders, CLI/examples, and test construction.
- [x] Enforce one writer through receiver/ownership types.
- [x] Remove writer sharing and live-state locks once callers no longer need
  them; keep any necessary temporary bridge local and explicitly scheduled for
  removal before completion.

### C. Build and publish read data

- [x] Implement contained freeze/share operations for actual read stores.
- [x] Move graph/projection/history preparation into the private update path.
- [x] Validate candidate readiness and compatible source identities.
- [x] Publish once, retain valid state during updates/failures, and handle empty
  startup correctly.
- [x] Prove immutable body sharing, history ordering, and failure recovery.

### D. Migrate native reads and HTTP selections

- [x] Move all mempool read APIs to the acquired read representation.
- [x] Audit and fix multi-load query composition, including deferred builds.
- [x] Preserve confirmed pins, tip validation, exact identities, and admission.
- [x] Remove `Inner.info`, separate current projection publication, and live
  `published_tip = None` read gating once their replacements are complete.
- [x] Remove obsolete lock-order comments and revision guards only when
  constructor/acquisition invariants actually replace them.

### E. Validate and measure

- [x] Run unit, domain, and real HTTP/RPC regression coverage.
- [x] Run relevant feature configurations and all migrated callers.
- [x] Compare native cycle stages, reads and memory on the same workloads;
  record the RPC/JSON timing exclusion and separate live HTTP observations.
- [x] Resolve every measured material regression or document an explicit
  accepted tradeoff before calling the refactor complete.
- [x] Perform a live local check if a daemon is available, recording its actual
  build and source tip. Do not restart production to obtain a benchmark.

### F. Finish the cleanup

- [x] Ensure there is one current publication mechanism and one read algorithm
  per operation, with no endpoint fallback caches left behind.
- [x] Remove temporary compatibility paths, redundant state, and unused locks.
- [x] Update crate docs, examples, public API notes, and relevant release notes.
- [x] Assess the earlier price polling workaround independently.
- [x] Update this plan with the implemented layout, exact commands/results,
  benchmark artifact paths, and any remaining limitations.

Suggested checks, adapted to the final feature/API changes:

```sh
cargo test -p brk_mempool
cargo test -p bitview_query --lib
cargo test -p bitview_server --lib
cargo check -p brk_mempool --all-targets
cargo check -p bitview_query --no-default-features --features price
cargo check -p bitview_server --no-default-features --features price
cargo check -p bitview -p bitviewd -p mmpl --all-targets
cargo test -p bitviewd --test query_preflight
git diff --check
```

Add the minimal/other supported feature checks exposed by the actual changes;
do not assume the full build proves reduced configurations. Run focused tests
while iterating, then the affected suites once the design is stable. Use the
repository's formatting/lint conventions and report unrelated baseline failures
separately. A documentation-only planning change does not require these builds.

## 16. Final acceptance criteria

The refactor is complete when all of the following hold:

1. Queries receive only a cheap read-only handle; update ownership is private
   and single-writer by construction.
2. The ArcSwap value is the actual latest published read-only data, with no
   redundant whole-state snapshot wrapper or competing endpoint publication.
3. Same-tip routine refreshes preserve valid reads; unavailable data is tied to
   a real source/readiness condition, not an update-in-progress flag.
4. Sharing cannot mutate a version held by an existing reader.
5. Related data, validation, and body building use one compatible acquisition.
6. Chain, template/history, input readiness, confirmed precedence, and bounded
   traversal/admission contracts remain verified at native and HTTP boundaries.
7. Freeze/build/read/destruction costs and retained memory are measured; speed
   claims describe successful requests and realistic updates.
8. Superseded state, locks, duplicate algorithms, and temporary bridges are gone.
9. File layout is contained and composed, with no speculative abstractions,
   macros, storage-mode framework, or unmeasured optimization scaffolding.

## 17. Implementation and validation record

### Public API and ownership

- `Mempool` is the update owner. `tick`, `tick_with`, `start`, and `start_with`
  require `&mut self`; the cloneable writer, cycle mutex and double-start latch
  are removed. `read_only_clone()` returns the cheap reader handle.
- `ReadOnlyMempool::load()` returns `Arc<ReadOnlyState>` for one immutable
  acquisition. Read algorithms live on this state. `Query` and `AsyncQuery`
  accept `Option<ReadOnlyMempool>`; the daemon, CLI, examples and fixtures have
  been migrated. There is no compatibility wrapper around live mutable state.
- `TxStore` contains its common read data and a private unresolved-input worklist.
  Its frozen counterpart holds no resolver bookkeeping. Transaction bodies and
  address records use ordinary Arc sharing and copy-on-write mutation.
- The old `Inner.info`, live `published_tip`, state lock, and independently
  published graph/history locks are gone. Statistics, histograms, pool stores,
  template source and bounded history all belong to the one root.
- Composed query paths were audited. Confirmed pins and narrow resolved bodies
  survive deferred work; txid admission reacquires and recomputes the array and
  validator together. Template history and current source are acquired from
  the same root. Existing HTTP serialization/admission algorithms remain intact.

### Verification

The original native suite passed 113 tests before the refactor. Final commands
and results:

| Command | Result |
| --- | --- |
| `cargo test -p brk_mempool --release --offline` | 117 unit tests; one example and two ownership compile-fail doctests pass; two benchmark tests ignored |
| `cargo test -p bitview_query --lib --offline` | 53 pass; two benchmarks ignored |
| `cargo test -p bitview_server --lib --offline -- --test-threads=1` | 87 pass; one newly extended fixture exposed its mock tip-counter sequencing bug; eight benchmarks ignored |
| `cargo test -p bitview_server --lib --offline tests::sync_success::reorganization_preserves_publication_and_validator_contracts -- --exact --test-threads=1` | Pass after correcting that test-only setup; completes verification of all 88 server tests |
| `cargo test -p bitviewd --test query_preflight --offline` | One integration test passes |
| `cargo check -p brk_mempool -p bitview_query -p bitview -p bitviewd -p mmpl --all-targets --offline` | Pass |
| `cargo check -p brk_mempool --all-targets --offline` | Pass on the final sharing implementation |
| `cargo check -p bitview_query -p bitview_server --no-default-features --features price --offline` | Pass |
| `cargo check -p bitview_query -p bitview_server --no-default-features --offline` | Pass |
| `node --test modules/bitview-client/tests/cache.js` | Eight pass |
| Price poller browser-cache/error smoke check and `node --check` | Pass |

The initial parallel server run timed out in the cache/reorg fixture under
compiler and fixture contention. That test passed both alone and in the serial
suite; no timeout threshold or unrelated cache implementation was changed.
The serial suite exposed a test setup issue: an intentionally panicking resolver
skipped the mock node's final tip read, leaving its alternating counter mid-pair.
The later final-tip-mismatch scenario now starts its pair explicitly. The
affected real RPC/HTTP regression passed after this test-only correction.

The new regression coverage checks startup versus observed empty state,
output availability with unresolved inputs, retained reads during a paused
same-tip update, coherent whole-root reads, immutable transaction/address data,
no-op sharing, dropped-version reclamation, compatible CPFP/RBF graphs,
template/history retention, full-tip mismatch before wildcard revalidation,
GET/HEAD 200/304 behavior, and recovery after a resolver panic following private
application. Existing address/transaction/confirmed-precedence tests remain.

### Measurements and operational boundary

See [the benchmark report](../benches/mempool/read-only-publication-2026-09-11/README.md)
for before/after native stages, allocation counts, retained memory, successful
reads during updates, destruction time, source identities and raw output. The
measurements include 50k and 200k starting pools. The remaining lookup-map copy
cost is an explicit tradeoff for immutable retained reads; unchanged components
and expensive unchanged address records are shared.

In the final pair without compiler/test contention, a 1% membership churn cycle
at 200k transactions took 43.65 ms before and 52.67 ms after. Current-state
allocator-requested live memory increased from 235.5 MiB to 293.0 MiB. Four
concurrent readers had zero failures in the new implementation at 50k and 200k.
The report also records the much larger cost of deliberately retaining seven
old versions and the time spent releasing them.

Native stage timings exclude RPC/JSON and HTTP serialization. They are not a
claim about end-to-end production cycle latency. HTTP/RPC integration fixtures
verify the actual availability and representation boundary. A separate live
read-only probe recorded the existing daemon's binary and full tip and reproduced
the old 503s. A separate `cargo dev` invocation then launched the new binary;
the after probe returned one 200 and 31 revalidated 304s on each of four routes,
with no failures in 128 requests. These short samples have different source tips
and are not a controlled production latency comparison. This task did not
restart or deploy a daemon.

The earlier price poller retains visibility handling, non-overlap and failure
backoff. Its option is now `memCache: false`, preserving browser HTTP caching
and revalidation while preventing the client's in-memory stale-on-error fallback
from turning failed polls into successful results. No generated client code was
changed for this server refactor.

## 18. Follow-up KISS review

Reviewed the publication root, sharing/copy boundaries, update pipeline,
recovery, native read algorithms and query handoffs. The follow-up removes
49 net production lines without adding a new type or storage mechanism:

- Graph reuse uses the existing transaction content revision for membership
  and body changes; the duplicate membership-change boolean/helper is removed.
  Template order and minimum fee remain independent reuse inputs.
- Obsolete lock scopes, redundant reference borrows and a provably unreachable
  second empty-fill check are removed. Private additions are named insertions.
- Panic recovery restores the previous graph in place, preserving the rebuild
  counter. Regression coverage also uses a real non-template removal instead
  of forcing the old rebuild boolean.

The separate RBF rate pass remains useful: filtered-out or rejected trees do
not need their rates resolved. Existing map sharing remains the simpler layout.

Verification reran `cargo test -p brk_mempool --release --offline` (117 tests
and three doctests pass), the exact RPC/HTTP publication/reorg regression from
section 17 (pass), and the five-crate all-target caller check (pass). Formatting
and diff checks pass. Four sequential before/after benchmark runs keep 200k
churn at approximately 52 ms with identical current/retained live memory.
The [follow-up benchmark record](../benches/mempool/read-only-publication-2026-09-11/README.md#follow-up-kiss-review)
includes variation, raw runs, the review patch and source identities.
