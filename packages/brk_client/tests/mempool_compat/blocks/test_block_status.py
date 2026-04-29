"""GET /api/block/{hash}/status"""

from _lib import assert_same_values, show


def test_block_status(brk, mempool, block):
    """Block status must be identical for a confirmed block."""
    path = f"/api/block/{block.hash}/status"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_values(b, m)
