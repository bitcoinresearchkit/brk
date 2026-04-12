"""
Mining endpoint compatibility tests.

Endpoints covered:
    GET /api/v1/mining/pools
    GET /api/v1/mining/pools/{period}
    GET /api/v1/mining/pool/{slug}
    GET /api/v1/mining/pool/{slug}/hashrate
    GET /api/v1/mining/pool/{slug}/blocks
    GET /api/v1/mining/pool/{slug}/blocks/{height}
    GET /api/v1/mining/hashrate/{period}
    GET /api/v1/mining/hashrate/pools/{period}
    GET /api/v1/mining/difficulty-adjustments/{period}
    GET /api/v1/mining/reward-stats/{block_count}
    GET /api/v1/mining/blocks/fees/{period}
    GET /api/v1/mining/blocks/rewards/{period}
    GET /api/v1/mining/blocks/fee-rates/{period}
    GET /api/v1/mining/blocks/sizes-weights/{period}
    GET /api/v1/mining/blocks/timestamp/{timestamp}
"""

import pytest

from conftest import show, assert_same_structure


@pytest.fixture(scope="module")
def pool_slugs(mempool):
    """Discover the top 3 active pool slugs from the last week."""
    data = mempool.get_json("/api/v1/mining/pools/1w")
    pools = data.get("pools", []) if isinstance(data, dict) else []
    slugs = [p["slug"] for p in pools if p.get("blockCount", 0) > 0][:3]
    return slugs or ["foundryusa"]


@pytest.fixture(scope="module")
def pool_slug(pool_slugs):
    return pool_slugs[0]


# ── /api/v1/mining/pools ─────────────────────────────────────────────


def test_mining_pools_list(brk, mempool):
    """Pool list must have the same element structure."""
    path = "/api/v1/mining/pools"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b[:3] if isinstance(b, list) else b, m[:3] if isinstance(m, list) else m)
    assert_same_structure(b, m)


@pytest.mark.parametrize("period", ["24h", "3d", "1w", "1m", "3m", "6m", "1y", "2y", "3y", "all"])
def test_mining_pools_by_period(brk, mempool, period):
    """Pool stats for a time period must have the same structure."""
    path = f"/api/v1/mining/pools/{period}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, _summary(b), _summary(m))
    assert_same_structure(b, m)


# ── /api/v1/mining/pool/{slug} ───────────────────────────────────────


def test_mining_pool_detail(brk, mempool, pool_slugs):
    """Pool detail must have the same structure for top pools."""
    for slug in pool_slugs:
        path = f"/api/v1/mining/pool/{slug}"
        b = brk.get_json(path)
        m = mempool.get_json(path)
        show("GET", path, _summary(b), _summary(m))
        assert_same_structure(b, m)


def test_mining_pool_hashrate(brk, mempool, pool_slugs):
    """Pool hashrate history must have the same structure for top pools."""
    for slug in pool_slugs:
        path = f"/api/v1/mining/pool/{slug}/hashrate"
        b = brk.get_json(path)
        m = mempool.get_json(path)
        show("GET", path, _summary(b), _summary(m))
        assert_same_structure(b, m)


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


def test_mining_pool_blocks_at_height(brk, mempool, pool_slug, live):
    """Pool blocks before various heights must have the same element structure."""
    for block in live.blocks[::2]:  # every other block
        path = f"/api/v1/mining/pool/{pool_slug}/blocks/{block.height}"
        b = brk.get_json(path)
        m = mempool.get_json(path)
        show("GET", path, f"({len(b)} blocks)", f"({len(m)} blocks)")
        assert isinstance(b, list) and isinstance(m, list)
        if b and m:
            assert_same_structure(b[0], m[0])


# ── /api/v1/mining/hashrate ──────────────────────────────────────────


@pytest.mark.parametrize("period", ["24h", "3d", "1w", "1m", "3m", "6m", "1y", "2y", "3y"])
def test_mining_hashrate(brk, mempool, period):
    """Network hashrate + difficulty must have the same structure."""
    path = f"/api/v1/mining/hashrate/{period}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, _summary(b), _summary(m))
    assert_same_structure(b, m)


# ── /api/v1/mining/hashrate/pools ────────────────────────────────────


@pytest.mark.parametrize("period", ["24h", "3d", "1w", "1m", "3m", "1y"])
def test_mining_hashrate_pools(brk, mempool, period):
    """Per-pool hashrate must have the same structure."""
    path = f"/api/v1/mining/hashrate/pools/{period}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, _summary(b), _summary(m))
    assert_same_structure(b, m)


# ── /api/v1/mining/difficulty-adjustments ─────────────────────────────


@pytest.mark.parametrize("period", ["24h", "3d", "1w", "1m", "3m", "6m", "1y", "2y", "3y"])
def test_mining_difficulty_adjustments(brk, mempool, period):
    """Historical difficulty adjustments must have the same structure."""
    path = f"/api/v1/mining/difficulty-adjustments/{period}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, _summary(b), _summary(m))
    assert_same_structure(b, m)


# ── /api/v1/mining/reward-stats ──────────────────────────────────────


@pytest.mark.parametrize("block_count", [10, 100, 500])
def test_mining_reward_stats(brk, mempool, block_count):
    """Reward stats must have the same structure."""
    path = f"/api/v1/mining/reward-stats/{block_count}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_structure(b, m)


# ── /api/v1/mining/blocks/fees ───────────────────────────────────────


@pytest.mark.parametrize("period", ["24h", "3d", "1w", "1m", "3m", "6m", "1y"])
def test_mining_blocks_fees(brk, mempool, period):
    """Average block fees must have the same element structure."""
    path = f"/api/v1/mining/blocks/fees/{period}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, _summary(b), _summary(m))
    assert_same_structure(b, m)


# ── /api/v1/mining/blocks/rewards ────────────────────────────────────


@pytest.mark.parametrize("period", ["24h", "3d", "1w", "1m", "3m", "6m", "1y"])
def test_mining_blocks_rewards(brk, mempool, period):
    """Average block rewards must have the same element structure."""
    path = f"/api/v1/mining/blocks/rewards/{period}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, _summary(b), _summary(m))
    assert_same_structure(b, m)


# ── /api/v1/mining/blocks/fee-rates ──────────────────────────────────


@pytest.mark.parametrize("period", ["24h", "3d", "1w", "1m", "3m", "6m", "1y"])
def test_mining_blocks_fee_rates(brk, mempool, period):
    """Block fee-rate percentiles must have the same element structure."""
    path = f"/api/v1/mining/blocks/fee-rates/{period}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, _summary(b), _summary(m))
    assert_same_structure(b, m)


# ── /api/v1/mining/blocks/sizes-weights ──────────────────────────────


@pytest.mark.parametrize("period", ["24h", "3d", "1w", "1m", "3m", "6m", "1y"])
def test_mining_blocks_sizes_weights(brk, mempool, period):
    """Block sizes and weights must have the same structure."""
    path = f"/api/v1/mining/blocks/sizes-weights/{period}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, _summary(b), _summary(m))
    assert_same_structure(b, m)


# ── /api/v1/mining/blocks/timestamp ──────────────────────────────────


def test_mining_blocks_timestamp(brk, mempool, live):
    """Block lookup by timestamp must have the same structure for various eras."""
    for block in live.blocks:
        # Get the block timestamp from brk
        info = brk.get_json(f"/api/block/{block.hash}")
        ts = info["timestamp"]
        path = f"/api/v1/mining/blocks/timestamp/{ts}"
        b = brk.get_json(path)
        m = mempool.get_json(path)
        show("GET", path, b, m)
        assert_same_structure(b, m)


# ── helpers ──────────────────────────────────────────────────────────


def _summary(data):
    """Short description of a response for the show() call."""
    if isinstance(data, list):
        return f"({len(data)} items)"
    if isinstance(data, dict):
        return f"(keys: {list(data.keys())})"
    return str(data)
