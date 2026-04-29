"""GET /api/mempool"""

from _lib import assert_same_structure, show


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
