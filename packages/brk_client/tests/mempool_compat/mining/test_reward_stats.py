"""GET /api/v1/mining/reward-stats/{block_count}"""

import pytest

from _lib import assert_same_structure, show


@pytest.mark.parametrize("block_count", [10, 100, 500])
def test_mining_reward_stats(brk, mempool, block_count):
    """Reward stats must have the same structure."""
    path = f"/api/v1/mining/reward-stats/{block_count}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_structure(b, m)
