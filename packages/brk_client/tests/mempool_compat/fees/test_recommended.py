"""GET /api/v1/fees/recommended"""

from _lib import assert_same_structure, show


EXPECTED_FEE_KEYS = [
    "fastestFee", "halfHourFee", "hourFee", "economyFee", "minimumFee",
]


def test_fees_recommended(brk, mempool):
    """Recommended fees must have the same keys and numeric types."""
    path = "/api/v1/fees/recommended"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_structure(b, m)
    for key in EXPECTED_FEE_KEYS:
        assert key in b, f"brk missing '{key}'"
        assert isinstance(b[key], (int, float)), f"'{key}' is not numeric: {type(b[key])}"


def test_fees_recommended_ordering(brk, mempool):
    """Fee tiers must be ordered: fastest >= halfHour >= hour >= economy >= minimum."""
    path = "/api/v1/fees/recommended"
    for label, client in [("brk", brk), ("mempool", mempool)]:
        d = client.get_json(path)
        assert d["fastestFee"] >= d["halfHourFee"] >= d["hourFee"], (
            f"{label}: fee ordering violated {d}"
        )
        assert d["hourFee"] >= d["economyFee"] >= d["minimumFee"], (
            f"{label}: fee ordering violated {d}"
        )
