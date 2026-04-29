"""GET /api/v1/blocks/{height}"""

from _lib import assert_same_values, show


def test_blocks_v1_from_height(brk, mempool, block):
    """v1 blocks from a confirmed height — all values must match."""
    path = f"/api/v1/blocks/{block.height}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, f"({len(b)} blocks)", f"({len(m)} blocks)")
    assert len(b) == len(m)
    if b and m:
        assert_same_values(b[0], m[0])
