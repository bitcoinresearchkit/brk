"""GET /api/v1/mining/pool/{slug}"""

from _lib import assert_same_structure, show, summary


def test_mining_pool_detail(brk, mempool, pool_slugs):
    """Pool detail must have the same structure for top pools."""
    for slug in pool_slugs:
        path = f"/api/v1/mining/pool/{slug}"
        b = brk.get_json(path)
        m = mempool.get_json(path)
        show("GET", path, summary(b), summary(m))
        assert_same_structure(b, m)
