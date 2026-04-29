"""GET /api/mempool/recent"""

from _lib import assert_same_structure, show


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
