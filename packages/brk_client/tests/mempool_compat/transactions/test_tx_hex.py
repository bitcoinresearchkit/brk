"""GET /api/tx/{txid}/hex"""

from _lib import show


def test_tx_hex(brk, mempool, block):
    """Raw transaction hex must be identical."""
    path = f"/api/tx/{block.txid}/hex"
    b = brk.get_text(path)
    m = mempool.get_text(path)
    show("GET", path, b[:80] + "...", m[:80] + "...")
    assert b == m
