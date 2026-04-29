"""GET /api/v1/fees/precise"""

from _lib import assert_same_structure, show


EXPECTED_FEE_KEYS = [
    "fastestFee", "halfHourFee", "hourFee", "economyFee", "minimumFee",
]


def test_fees_precise_structure(brk, mempool):
    """Precise fees must have the same structure as recommended."""
    path = "/api/v1/fees/precise"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_structure(b, m)
    for key in EXPECTED_FEE_KEYS:
        assert key in b


def test_fees_precise_ordering(brk, mempool):
    """Precise fee tiers must be ordered: fastest >= halfHour >= hour >= economy >= minimum."""
    path = "/api/v1/fees/precise"
    for label, client in [("brk", brk), ("mempool", mempool)]:
        d = client.get_json(path)
        assert d["fastestFee"] >= d["halfHourFee"] >= d["hourFee"], (
            f"{label}: precise fee ordering violated {d}"
        )
        assert d["hourFee"] >= d["economyFee"] >= d["minimumFee"], (
            f"{label}: precise fee ordering violated {d}"
        )


def test_fees_precise_numeric(brk):
    """Each tier in /precise must be a non-negative number."""
    d = brk.get_json("/api/v1/fees/precise")
    show("GET", "/api/v1/fees/precise", d, "—")
    for key in EXPECTED_FEE_KEYS:
        v = d[key]
        assert isinstance(v, (int, float)), f"{key} not numeric: {type(v).__name__}"
        assert v >= 0, f"{key} is negative: {v}"
