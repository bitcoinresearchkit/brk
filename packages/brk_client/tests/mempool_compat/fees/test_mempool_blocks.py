"""GET /api/v1/fees/mempool-blocks"""

from _lib import assert_same_structure, show


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
