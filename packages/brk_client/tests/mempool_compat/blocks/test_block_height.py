"""GET /api/block-height/{height}"""

from _lib import show


def test_block_height_to_hash(brk, mempool, block):
    """Block hash at a given height must match."""
    path = f"/api/block-height/{block.height}"
    b = brk.get_text(path)
    m = mempool.get_text(path)
    show("GET", path, b, m)
    assert b == m
    assert b == block.hash
