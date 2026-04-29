"""GET /api/block/{hash}/txids"""

from _lib import show


def test_block_txids(brk, mempool, block):
    """Ordered txid list must be identical."""
    path = f"/api/block/{block.hash}/txids"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b[:3], m[:3])
    assert b == m
