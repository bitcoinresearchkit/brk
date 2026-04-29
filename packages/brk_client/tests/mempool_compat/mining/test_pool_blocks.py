"""GET /api/v1/mining/pool/{slug}/blocks"""

from _lib import assert_same_structure, show


def test_mining_pool_blocks(brk, mempool, pool_slugs):
    """Recent blocks by pool must have the same element structure."""
    for slug in pool_slugs:
        path = f"/api/v1/mining/pool/{slug}/blocks"
        b = brk.get_json(path)
        m = mempool.get_json(path)
        show("GET", path, f"({len(b)} blocks)", f"({len(m)} blocks)")
        assert isinstance(b, list) and isinstance(m, list)
        if b and m:
            assert_same_structure(b[0], m[0])
