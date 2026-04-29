"""GET /api/v1/cpfp/{txid}"""

from _lib import assert_same_structure, show


def test_cpfp(brk, mempool, block):
    """CPFP info structure must match for a confirmed tx."""
    path = f"/api/v1/cpfp/{block.txid}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_structure(b, m)
