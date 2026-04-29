"""GET /api/v1/block/{hash}"""

from _lib import assert_same_structure, assert_same_values, show


def test_block_v1_extras_all_values(brk, mempool, block):
    """Every shared extras field must match — exposes computation differences."""
    path = f"/api/v1/block/{block.hash}"
    b = brk.get_json(path)["extras"]
    m = mempool.get_json(path)["extras"]
    show("GET", f"{path}  [extras]", b, m, max_lines=50)
    assert_same_structure(b, m)
    assert_same_values(b, m)


def test_block_v1_extras_pool(brk, mempool, block):
    """Pool identification structure must match."""
    path = f"/api/v1/block/{block.hash}"
    bp = brk.get_json(path)["extras"]["pool"]
    mp = mempool.get_json(path)["extras"]["pool"]
    show("GET", f"{path}  [extras.pool]", bp, mp)
    assert_same_structure(bp, mp)
