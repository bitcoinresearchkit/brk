"""
Mempool endpoint compatibility tests.

Endpoints covered:
    GET /api/mempool
    GET /api/mempool/txids
    GET /api/mempool/recent
"""

from conftest import show, assert_same_structure


# ── /api/mempool ─────────────────────────────────────────────────────


def test_mempool_info(brk, mempool):
    """Mempool stats must have the same keys and types."""
    path = "/api/mempool"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m, max_lines=15)
    assert_same_structure(b, m)
    assert isinstance(b["count"], int)
    assert isinstance(b["vsize"], int)


def test_mempool_info_positive(brk, mempool):
    """Both servers must report a non-empty mempool."""
    path = "/api/mempool"
    for label, client in [("brk", brk), ("mempool", mempool)]:
        d = client.get_json(path)
        assert d["count"] > 0, f"{label} mempool count is 0"
        assert d["vsize"] > 0, f"{label} mempool vsize is 0"


# ── /api/mempool/txids ───────────────────────────────────────────────


def test_mempool_txids(brk, mempool):
    """Txid list must be a non-empty array of strings."""
    path = "/api/mempool/txids"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, f"({len(b)} txids)", f"({len(m)} txids)")
    assert isinstance(b, list) and isinstance(m, list)
    assert len(b) > 0, "brk mempool has no txids"
    assert isinstance(b[0], str) and len(b[0]) == 64


# ── /api/mempool/recent ──────────────────────────────────────────────


def test_mempool_recent(brk, mempool):
    """Recent mempool txs must have the same element structure."""
    path = "/api/mempool/recent"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert isinstance(b, list) and isinstance(m, list)
    assert len(b) > 0
    if b and m:
        assert_same_structure(b[0], m[0])


def test_mempool_recent_fields(brk, mempool):
    """Each recent tx must have txid, fee, vsize, value."""
    path = "/api/mempool/recent"
    for label, client in [("brk", brk), ("mempool", mempool)]:
        txs = client.get_json(path)
        for tx in txs[:3]:
            for key in ["txid", "fee", "vsize", "value"]:
                assert key in tx, f"{label} recent tx missing '{key}': {tx}"
