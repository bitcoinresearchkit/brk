"""GET /api/v1/mining/blocks/sizes-weights/{time_period}"""

import pytest

from _lib import assert_same_structure, show, summary


@pytest.mark.parametrize("period", ["24h", "3d", "1w", "1m", "3m", "6m", "1y"])
def test_mining_blocks_sizes_weights(brk, mempool, period):
    """Block sizes and weights must have the same structure."""
    path = f"/api/v1/mining/blocks/sizes-weights/{period}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, summary(b), summary(m))
    assert_same_structure(b, m)
