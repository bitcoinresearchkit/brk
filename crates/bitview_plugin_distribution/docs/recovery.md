# Missing reorg checkpoints

No snapshot at or before the required height is a domain-level `NotFound`.
Address/UTXO recovery uses the existing full rebuild path in that case.
Filesystem and decoding errors still propagate; do not convert every import
failure into a fresh start.

The populated server reorg fixture exercises missing UTXO snapshot recovery and
replacement publication. It is not independent coverage of every address-state
failure. No second recovery mechanism or response cache is needed.
