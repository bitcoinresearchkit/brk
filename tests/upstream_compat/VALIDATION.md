# Initial validation — 2026-09-08

This is a partial execution report, not a claim that the implementation passes the
whole suite. No implementation files were changed by this work.

- 271 cases collect successfully.
- 17 offline harness checks pass.
- Local target: `http://localhost:3110`, observed mainnet height 965978.
- Five endpoint checks passed using `https://blockstream.info` as the reference:
  block-height lookup, tip height, tip hash, confirmed transaction status, and
  recent mempool transactions.
- Height-100 semantic checks: 1 passed, 2 failed, 1 skipped (no regular transaction
  exists in that block).
- Both failures expose the same wire incompatibility: coinbase `vin[0].vout` is
  `65535` locally and `4294967295` upstream. One failure is the direct transaction
  response; the other is the transaction inside the block page. Assertions and
  implementation were left unchanged.
- mempool.space connectivity timed out. Its control run was stopped. The full
  mempool-specific surface has not been verified live.
- Successful broadcasts were not run; they require explicit funded regtest fixtures.

Commands used for the live subsets (with a Python environment containing pytest):

```sh
python -m pytest -c tests/upstream_compat/pytest.ini tests/upstream_compat/test_endpoints.py --target http://localhost:3110 --mempool-reference https://blockstream.info --http-timeout 5 -k 'get-block-height or get-block-tip or get-transaction-status or get-mempool-recent' -q --tb=short
python -m pytest -c tests/upstream_compat/pytest.ini tests/upstream_compat/test_chain.py --target http://localhost:3110 --mempool-reference https://blockstream.info --http-timeout 10 -k 'height-100 and not height-100000' -q --tb=short
```

Use Blockstream only for shared Esplora routes; it does not implement mempool's
`/api/v1` extensions. Use mempool.space or a compatible self-hosted reference for
the full suite.

## Expanded audit and verification

The follow-up audit added backend route coverage beyond the public documentation,
semantic address/mining tests, independent binary/proof validators, utility POST
cases, HTTP checks and generated regtest transitions. The resulting suite collects
520 cases. The route inventory contains 153 GET cases and nine POST cases, and the
source audit accounts for 147 registrations across nine pinned route modules,
plus all 41 documented Esplora Bitcoin route variants.

Verified during this expansion:

- 62 offline harness/mutation checks pass, including genesis golden bytes,
  malformed serialization/proofs, and deliberately incorrect response fields.
- The pinned source audit passes against the downloaded original route files.
- 11 upstream binary/proof cases passed across heights 100 and 800000; one regular
  transaction scenario was correctly skipped because height 100 has only coinbase.
- The added BIP141 witness-commitment check passed separately on block 800000.
- 12 upstream address/script-type controls passed, including empty histories and
  P2PKH, P2SH, P2WPKH, P2WSH and Taproot transaction fixtures.
- The fixture generator successfully funded, signed, replaced, mined and submitted
  a parent/child package on a temporary isolated Bitcoin Core regtest instance.
  The instance was stopped and removed. This verifies fixture generation, not
  the target server's full regtest HTTP state-machine compatibility.
- The local HTTP control run passed 11 cases and failed one: an unknown all-zero
  transaction ID returned HTTP 503 rather than the expected 400/404/422. No
  implementation change was made. The final HTTP set restricts byte-for-byte
  comparisons to immutable text/binary representations so changing extra JSON
  fields cannot produce false cache failures.
- mempool.space still timed out, so its full live surface remains unverified.

The full regtest state scenarios are implemented but were not executed against a
regtest-compatible target HTTP server. They are explicitly opt-in; a passing
mainnet run with those scenarios skipped is not a full state-machine validation.
