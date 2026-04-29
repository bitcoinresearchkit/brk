"""GET /api/v1/blocks (with extras, no height)"""

from _lib import assert_same_structure, show


def test_blocks_v1_recent_structure(brk, mempool):
    """Recent v1 blocks (with extras) must have the same structure."""
    path = "/api/v1/blocks"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, f"({len(b)} blocks)", f"({len(m)} blocks)")
    assert len(b) > 0
    assert_same_structure(b, m)


def test_blocks_v1_recent_ordering(brk):
    """v1 blocks must also be tip-first."""
    b = brk.get_json("/api/v1/blocks")
    heights = [blk["height"] for blk in b]
    show("GET", "/api/v1/blocks", f"heights={heights[:5]}...", "—")
    assert heights == sorted(heights, reverse=True), (
        f"v1 blocks are not strictly tip-first: {heights}"
    )


def test_blocks_v1_recent_has_extras(brk):
    """Each v1 block must carry the extras envelope (v1 distinguishes itself from /api/blocks)."""
    b = brk.get_json("/api/v1/blocks")
    show("GET", "/api/v1/blocks", f"({len(b)} blocks)", "—")
    assert b
    assert "extras" in b[0], f"v1 blocks element missing 'extras': {list(b[0].keys())}"
