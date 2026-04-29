"""GET /api/tx/{txid}/merkleblock-proof"""

from _lib import show


def test_tx_merkleblock_proof(brk, mempool, block):
    """BIP37 merkleblock proof hex must be identical."""
    path = f"/api/tx/{block.txid}/merkleblock-proof"
    b = brk.get_text(path)
    m = mempool.get_text(path)
    show("GET", path, b[:80] + "...", m[:80] + "...")
    assert b == m
