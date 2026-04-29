"""GET /api/v1/mining/pools/{time_period}"""

import pytest

from _lib import assert_same_structure, show, summary


@pytest.mark.parametrize("period", ["24h", "3d", "1w", "1m", "3m", "6m", "1y", "2y", "3y", "all"])
def test_mining_pools_by_period(brk, mempool, period):
    """Pool stats for a time period must have the same structure."""
    path = f"/api/v1/mining/pools/{period}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, summary(b), summary(m))
    assert_same_structure(b, m)
