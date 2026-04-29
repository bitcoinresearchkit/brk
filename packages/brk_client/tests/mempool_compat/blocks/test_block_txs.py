"""GET /api/block/{hash}/txs"""

from _lib import assert_same_values, show


def test_block_txs_page0(brk, mempool, block):
    """First page of block transactions must match."""
    path = f"/api/block/{block.hash}/txs"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, f"({len(b)} txs)", f"({len(m)} txs)")
    assert len(b) == len(m), f"Page size: brk={len(b)} vs mempool={len(m)}"
    if b and m:
        assert_same_values(b[0], m[0], exclude={"sigops"})
