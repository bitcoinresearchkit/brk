"""GET /api/block/{hash}/header"""

from _lib import show


def test_block_header(brk, mempool, block):
    """80-byte hex block header must be identical."""
    path = f"/api/block/{block.hash}/header"
    b = brk.get_text(path)
    m = mempool.get_text(path)
    show("GET", path, b, m)
    assert len(b) == 160, f"Expected 160 hex chars (80 bytes), got {len(b)}"
    assert b == m
