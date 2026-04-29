"""GET /api/tx/{txid}/raw"""

from _lib import show


def test_tx_raw(brk, mempool, block):
    """Raw transaction bytes must be identical."""
    path = f"/api/tx/{block.txid}/raw"
    b = brk.get_bytes(path)
    m = mempool.get_bytes(path)
    show("GET", path, f"<{len(b)} bytes>", f"<{len(m)} bytes>")
    assert b == m
