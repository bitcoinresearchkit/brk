"""GET /api/v1/mining/pool/{slug}/hashrate"""

from _lib import assert_same_structure, show, summary


def test_mining_pool_hashrate(brk, mempool, pool_slugs):
    """Pool hashrate history must have the same structure for top pools."""
    for slug in pool_slugs:
        path = f"/api/v1/mining/pool/{slug}/hashrate"
        b = brk.get_json(path)
        m = mempool.get_json(path)
        show("GET", path, summary(b), summary(m))
        assert_same_structure(b, m)
