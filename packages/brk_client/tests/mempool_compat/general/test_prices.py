"""GET /api/v1/prices"""

from _lib import assert_same_structure, show


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
