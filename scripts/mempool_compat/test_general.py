"""
General endpoint compatibility tests.

Endpoints covered:
    GET /api/v1/difficulty-adjustment
    GET /api/v1/prices
    GET /api/v1/historical-price
    GET /api/v1/historical-price?timestamp=…
"""

import pytest

from conftest import show, assert_same_structure


DIFFICULTY_KEYS = [
    "progressPercent", "difficultyChange", "estimatedRetargetDate",
    "remainingBlocks", "remainingTime", "previousRetarget",
    "previousTime", "nextRetargetHeight", "timeAvg",
    "adjustedTimeAvg", "timeOffset", "expectedBlocks",
]


# ── /api/v1/difficulty-adjustment ────────────────────────────────────


def test_difficulty_adjustment(brk, mempool):
    """Difficulty adjustment must have the same structure."""
    path = "/api/v1/difficulty-adjustment"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_structure(b, m)
    for key in DIFFICULTY_KEYS:
        assert key in b, f"brk missing '{key}'"


def test_difficulty_adjustment_values_sane(brk, mempool):
    """Progress must be 0–100 %, remaining blocks must be 0–2016."""
    path = "/api/v1/difficulty-adjustment"
    for label, client in [("brk", brk), ("mempool", mempool)]:
        d = client.get_json(path)
        assert 0 <= d["progressPercent"] <= 100, (
            f"{label} progressPercent out of range: {d['progressPercent']}"
        )
        assert 0 <= d["remainingBlocks"] <= 2016, (
            f"{label} remainingBlocks out of range: {d['remainingBlocks']}"
        )


# ── /api/v1/prices ───────────────────────────────────────────────────


def test_prices(brk, mempool):
    """Current price must have the same structure."""
    path = "/api/v1/prices"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_structure(b, m)
    assert "USD" in b
    assert "time" in b


def test_prices_positive(brk, mempool):
    """USD price must be a positive number on both servers."""
    path = "/api/v1/prices"
    for label, client in [("brk", brk), ("mempool", mempool)]:
        d = client.get_json(path)
        assert d["USD"] > 0, f"{label} USD price is not positive: {d['USD']}"


# ── /api/v1/historical-price ─────────────────────────────────────────


def test_historical_price(brk, mempool):
    """Historical price must have the same structure."""
    path = "/api/v1/historical-price"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m, max_lines=15)
    assert_same_structure(b, m)
    assert "prices" in b
    assert isinstance(b["prices"], list)


def test_historical_price_at_block_timestamps(brk, mempool, live):
    """Historical price at each discovered block's timestamp must match structure."""
    for block in live.blocks:
        info = brk.get_json(f"/api/block/{block.hash}")
        ts = info["timestamp"]
        path = f"/api/v1/historical-price?timestamp={ts}"
        b = brk.get_json(path)
        m = mempool.get_json(path)
        show("GET", path, b, m)
        assert_same_structure(b, m)
        assert "prices" in b
        assert len(b["prices"]) > 0


# Well-known timestamps from different eras
HISTORICAL_TIMESTAMPS = [
    1231006505,   # genesis block (2009-01-03)
    1354116278,   # block 210000 — first halving (2012-11-28)
    1468082773,   # block 420000 — second halving (2016-07-09)
    1588788036,   # block 630000 — third halving (2020-05-11)
    1713571767,   # block 840000 — fourth halving (2024-04-20)
]


@pytest.mark.parametrize("ts", HISTORICAL_TIMESTAMPS, ids=[
    "genesis", "halving1", "halving2", "halving3", "halving4",
])
def test_historical_price_at_era(brk, mempool, ts):
    """Historical price at well-known timestamps must match structure."""
    path = f"/api/v1/historical-price?timestamp={ts}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_structure(b, m)
    assert "prices" in b
    assert len(b["prices"]) > 0
