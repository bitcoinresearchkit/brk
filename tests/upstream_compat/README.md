# Upstream HTTP compatibility validators

Independent black-box tests for the Bitcoin mainnet mempool.space and Esplora
REST APIs. Only files in this directory belong to this suite. It imports neither
the Bitview SDK nor server code and is separate from the existing SDK compatibility
tests in `packages/bitview_client/tests/mempool_compat`.

These are newly written contract tests based on upstream documentation, not a
verbatim copy of upstream's internal tests. Upstream mempool's backend tests cover
internal algorithms and repositories; they are not a complete HTTP conformance
suite that can simply be pointed at a different server.

## Run

Python 3.9+ and pytest are the only dependencies. From the repository root:

```sh
python3 -m venv /tmp/upstream-compat-venv
/tmp/upstream-compat-venv/bin/pip install -r tests/upstream_compat/requirements.txt
/tmp/upstream-compat-venv/bin/python -m pytest -c tests/upstream_compat/pytest.ini tests/upstream_compat --target http://localhost:3110 --junitxml=/tmp/upstream-compat.xml
```

Or use an existing Python environment with pytest. The target must already be
running and index Bitcoin mainnet through at least height 840,000. Nothing starts,
rebuilds, changes, or repairs the server. Failed assertions remain failures; there
are no implementation-specific xfails or missing-field allowlists.

Useful selections:

```sh
# Offline validation of the test harness and inventory
python -m pytest -c tests/upstream_compat/pytest.ini tests/upstream_compat -m 'not live'
# Only the inventory's per-endpoint HTTP checks
python -m pytest -c tests/upstream_compat/pytest.ini tests/upstream_compat/test_endpoints.py
# Multi-era transactions, block headers, Merkle proofs and page boundaries
python -m pytest -c tests/upstream_compat/pytest.ini tests/upstream_compat/test_chain.py
# See every collected case without connecting to any server
python -m pytest -c tests/upstream_compat/pytest.ini tests/upstream_compat --collect-only
```

URLs are **origins**, without `/api`. `--target` defaults to `BITVIEW_URL` or
`http://localhost:3110`; `--mempool-reference` to `MEMPOOL_URL` or
`https://mempool.space`; `--esplora-reference` to `ESPLORA_URL` or
`https://blockstream.info`. Self-hosted references are supported. Requests have a
30-second timeout (`--http-timeout`) and reference requests are spaced by 0.5s
(`--reference-delay`). Run serially to respect reference rate limits. A full run
makes hundreds of requests and can take several minutes.

## Scope and provenance

`endpoints.json` inventories every REST documentation entry from these revisions,
with explicit optional-path expansions and Esplora-only Bitcoin endpoints.
`upstream_routes.json` additionally audits 147 registrations across nine upstream
route files, linking each public explorer route to a collected test and explicitly
classifying internal/hosted-service routes. The expanded inventory contains
153 GET cases and nine POST cases (including versioned aliases).
`esplora_routes.json` maps all 41 documented Bitcoin route variants to executable
cases as well:

- mempool: [`8158a230ce44265b14a44d13a785a0ac092ec478`](https://github.com/mempool/mempool/blob/8158a230ce44265b14a44d13a785a0ac092ec478/frontend/src/app/docs/api-docs/api-docs-data.ts).
- Esplora: [`1f7a21b97b013de0403ce80fb211c5370eae7a63`](https://github.com/Blockstream/esplora/blob/1f7a21b97b013de0403ce80fb211c5370eae7a63/API.md).
- Upstream internal test inventory: [mempool backend tests](https://github.com/mempool/mempool/tree/8158a230ce44265b14a44d13a785a0ac092ec478/backend/src/__tests__). Those tests are not copied here.

Sample request parameters come from the mempool documentation. The CPFP example's
literal `txid` is replaced with a documented confirmed transaction, and the rewards
example's `1d` is replaced with the documented `24h` interval. This directory
contains original test code and endpoint metadata, not vendored upstream source.

Coverage includes transactions and proofs, broadcasts, addresses, scripthashes,
address-prefix search, blocks, pagination, mempool, RBF, CPFP, fees, prices, mining,
and public Lightning GET endpoints. Unsupported target endpoints fail just like
incorrect implementations; the server's OpenAPI is not used to decide what to test.

Liquid/Elements-only routes and hosted accelerator/account/payment services are
explicitly classified as outside this Bitcoin node API suite in the inventory.
WebSocket and Electrum JSON-RPC are separate protocols and are not covered here.
This is a pinned REST contract inventory, not automatic discovery of future APIs.

## What a pass means

- Stable historical responses compare every upstream key recursively, every array
  item in order, exact integers, nulls, and raw bytes. Extra object fields at any
  depth pass. Floating values allow only `1e-12` rounding noise. Missing fields,
  strings instead of numbers, booleans instead of numbers, changed monetary values,
  omitted empty witness items and changed coinbase output indices fail.
- Changing responses compare observed types and keys, plus explicit core contracts.
  Every target array element is checked against observed reference variants. Live
  array lengths, transaction sets, prices and fee estimates are not expected to
  match between independently operated nodes. Empty reference arrays/nulls provide
  no evidence about their element/non-null schema; core contracts cover the common
  Bitcoin structures independently. Mining and Lightning checks are structural,
  not proofs that independently sampled live values must be identical.
- Blocks and transactions span genesis, early blocks, SegWit and Taproot eras.
  Header hashing, reconstructed Merkle roots, inclusion proofs, transaction fee
  accounting, raw/hex agreement, and first/second/last pages add semantic checks.
- Negative tests compare actual upstream HTTP status codes without requiring the
  same human-readable error wording.
- Default broadcast tests submit only malformed transactions/packages and require
  rejection. Successful transaction/package tests are available with explicit
  funded regtest fixtures (below). Replacement creation, orphan chains and
  deterministic mempool transitions are not exercised by the mainnet suite.
  The opt-in generated regtest suite covers RBF, CPFP, confirmation, spends,
  reorgs and package acceptance. These cases require a controlled node (below).

A missing target route is a failure, not a skip. Skips occur only when a fixture
has no regular transaction, a live scenario changes while sampled, or the regtest
profile is not configured. Reference
HTTP errors (including public rate limits or disabled endpoints such as bulk block
queries and optional mining REST) are reported explicitly as `REFERENCE` failures;
they are not evidence of a target implementation bug. Use a reference with those
features enabled to validate their success responses. No failures trigger repairs.

## Successful broadcast tests (disposable regtest only)

Prepare distinct, signed, spendable transactions with your regtest wallet: one
standalone transaction and a parent/child package in dependency order. Write:

```json
{
  "network": "regtest",
  "transaction": {"hex": "<signed hex>", "txid": "<expected txid>"},
  "package": [
    {"hex": "<signed parent hex>", "txid": "<parent txid>"},
    {"hex": "<signed child hex>", "txid": "<child txid>"}
  ]
}
```

```sh
python -m pytest -c tests/upstream_compat/pytest.ini tests/upstream_compat/test_regtest.py --target http://localhost:3110 --regtest-fixtures /tmp/regtest-transactions.json
```

These tests verify the target's genesis hash before sending anything. They submit
real regtest transactions and require the normal Bitcoin Core `submitpackage`
result (`package_msg`, `tx-results`) for package acceptance. Start with fresh
fixtures/chain state for each run. No wallet credentials are needed by this suite.

## Expanded validation depth

- Independent Bitcoin wire decoding checks every transaction in raw blocks, txids,
  weight, witness bytes and BIP141 commitments. BIP37 proofs are decoded and their
  partial Merkle trees reconstructed, including malformed/truncated payload checks.
- Address accounting and anchored pagination cover P2PKH, P2SH, P2WPKH, P2WSH and
  Taproot. Confirmed totals and quiet-address UTXOs compare actual values, so an
  empty response cannot masquerade as compatibility. Cursor comparisons use stable
  anchors; observed concurrent address changes are reported as scenario skips.
- Mining checks enforce required fields, tuple sizes, accounting relationships,
  nonempty historical datasets, and rewards across all four mainnet halvings.
  Reward statistics are checked against independently fetched coinbase outputs.
- Read-only POST helpers cover prevouts, local CPFP, PSBT parent completion and
  mempool acceptance testing. Broadcast aliases and request format boundaries have
  separate cases. Optional cached transactions and the deprecated transformed fee
  endpoint recognize upstream's documented `204` and `203` responses.
- HTTP checks cover HEAD, gzip, advertised ETags, and validation before conditional
  responses. An ETag is not required, but an advertised validator must work.
- Offline mutation tests deliberately alter fees, scripts, witness data, IDs,
  sizes, tuple/cardinality fields and proofs to verify that the harness rejects
  plausible-looking invalid responses.

Bitcoin wire references: [Core serialization](https://github.com/bitcoin/bitcoin/blob/master/src/serialize.h),
[BIP37](https://github.com/bitcoin/bips/blob/master/bip-0037.mediawiki),
[BIP141](https://github.com/bitcoin/bips/blob/master/bip-0141.mediawiki).
The genesis golden fixture is public blockchain data and records its source URL.

## Generated regtest state scenarios

`test_regtest_states.py` creates its own funded/signed transactions through an
explicitly selected **disposable** Bitcoin Core wallet RPC endpoint. It mines
coinbase maturity blocks, broadcasts, replaces transactions, builds packages,
invalidates a block and mines a competing branch. Both RPC and target genesis
hashes must be regtest before any mutation. Do not run these cases concurrently
against the same wallet/node. Default mainnet runs skip this separate profile.

Start a disposable regtest node, create/load a wallet, and point a separate target
server at that node. Then run, for example:

```sh
python -m pytest -c tests/upstream_compat/pytest.ini tests/upstream_compat/test_regtest_states.py --target http://localhost:3111 --regtest-rpc http://localhost:18443/wallet/compat --regtest-cookie /tmp/bitcoin-regtest/regtest/.cookie
```

Alternatively use `COMPAT_REGTEST_AUTH=user:password`; credentials are not saved
in reports or passed in the RPC URL. `--state-timeout` defaults to 30 seconds for
index/mempool propagation. These cases verify real state changes. Unsupported endpoints and propagation
timeouts fail normally.
The fixture generator has been smoke-tested against isolated Bitcoin Core; the
full state suite still requires a compatible regtest HTTP target.

## Keeping the upstream boundary complete

Given a local mempool checkout, run:

```sh
python tests/upstream_compat/audit_upstream.py /path/to/mempool --esplora-source /path/to/esplora
```

The audit reports added/removed routes and changed handler-source hashes, with a
nonzero exit status. It never rewrites assertions from the target's implementation.
The pinned inventory is comprehensive for the stated REST boundary, not a claim
that every possible chain state, deployment configuration, or future upstream
change can be exhausted by a finite test suite. See `VALIDATION.md` for what has
actually run; collection counts are not passing implementation results.
