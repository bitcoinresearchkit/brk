"""
Fee endpoint compatibility tests.

Endpoints covered:
    GET /api/v1/fees/recommended
    GET /api/v1/fees/precise
    GET /api/v1/fees/mempool-blocks
"""

from conftest import show, assert_same_structure


EXPECTED_FEE_KEYS = [
    "fastestFee", "halfHourFee", "hourFee", "economyFee", "minimumFee",
]


# ── /api/v1/fees/recommended ─────────────────────────────────────────


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


# ── /api/v1/fees/precise ─────────────────────────────────────────────


def test_fees_precise(brk, mempool):
    """Precise fees must have the same structure as recommended."""
    path = "/api/v1/fees/precise"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_structure(b, m)
    for key in EXPECTED_FEE_KEYS:
        assert key in b


# ── /api/v1/fees/mempool-blocks ──────────────────────────────────────


def test_fees_mempool_blocks(brk, mempool):
    """Projected mempool blocks must have the same element structure."""
    path = "/api/v1/fees/mempool-blocks"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, f"({len(b)} blocks)", f"({len(m)} blocks)")
    assert isinstance(b, list) and isinstance(m, list)
    assert len(b) > 0
    if b and m:
        assert_same_structure(b[0], m[0])


def test_fees_mempool_blocks_fee_range(brk, mempool):
    """Each projected block must have a 7-element feeRange."""
    path = "/api/v1/fees/mempool-blocks"
    for label, client in [("brk", brk), ("mempool", mempool)]:
        blocks = client.get_json(path)
        for i, block in enumerate(blocks[:3]):
            assert "feeRange" in block, f"{label} block {i} missing feeRange"
            assert len(block["feeRange"]) == 7, (
                f"{label} block {i} feeRange has {len(block['feeRange'])} items, expected 7"
            )
