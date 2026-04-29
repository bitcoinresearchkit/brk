"""GET /api/v1/mining/hashrate/{time_period}"""

import pytest

from _lib import assert_same_structure, show, summary


@pytest.mark.parametrize("period", ["24h", "3d", "1w", "1m", "3m", "6m", "1y", "2y", "3y"])
def test_mining_hashrate(brk, mempool, period):
    """Network hashrate + difficulty must have the same structure."""
    path = f"/api/v1/mining/hashrate/{period}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, summary(b), summary(m))
    assert_same_structure(b, m)
