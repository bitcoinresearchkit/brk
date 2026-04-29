"""GET /api/tx/{txid}/merkle-proof"""

from _lib import assert_same_values, show


def test_tx_merkle_proof(brk, mempool, block):
    """Merkle inclusion proof must match."""
    path = f"/api/tx/{block.txid}/merkle-proof"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_values(b, m)
