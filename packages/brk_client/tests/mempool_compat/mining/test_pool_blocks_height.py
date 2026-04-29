"""GET /api/v1/mining/pool/{slug}/blocks/{height}"""

from _lib import assert_same_structure, show


def test_mining_pool_blocks_at_height(brk, mempool, pool_slug, live):
    """Pool blocks before various heights must have the same element structure."""
    for block in live.blocks[::2]:  # every other block, to keep run-time bounded
        path = f"/api/v1/mining/pool/{pool_slug}/blocks/{block.height}"
        b = brk.get_json(path)
        m = mempool.get_json(path)
        show("GET", path, f"({len(b)} blocks)", f"({len(m)} blocks)")
        assert isinstance(b, list) and isinstance(m, list)
        if b and m:
            assert_same_structure(b[0], m[0])
