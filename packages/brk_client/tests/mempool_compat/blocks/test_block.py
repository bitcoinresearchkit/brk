"""GET /api/block/{hash}"""

from _lib import assert_same_values, show


def test_block_by_hash(brk, mempool, block):
    """Confirmed block info must be identical."""
    path = f"/api/block/{block.hash}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_values(b, m)
