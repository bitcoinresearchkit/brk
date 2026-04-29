"""GET /api/v1/historical-price (with and without timestamp)"""

import pytest

from _lib import assert_same_structure, show


# Well-known timestamps from different eras
HISTORICAL_TIMESTAMPS = [
    1231006505,   # genesis block (2009-01-03)
    1354116278,   # block 210000 — first halving (2012-11-28)
    1468082773,   # block 420000 — second halving (2016-07-09)
    1588788036,   # block 630000 — third halving (2020-05-11)
    1713571767,   # block 840000 — fourth halving (2024-04-20)
]


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
