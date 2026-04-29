"""GET /api/block/{hash}/raw"""

from _lib import show


def test_block_raw(brk, mempool, block):
    """Raw block bytes must be identical and start with the 80-byte header."""
    path = f"/api/block/{block.hash}/raw"
    b = brk.get_bytes(path)
    m = mempool.get_bytes(path)
    show("GET", path, f"<{len(b)} bytes>", f"<{len(m)} bytes>")
    assert b == m
    assert len(b) >= 80
