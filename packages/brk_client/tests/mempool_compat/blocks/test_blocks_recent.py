"""GET /api/blocks (most recent confirmed blocks, no height)"""

from _lib import assert_same_structure, show


def test_blocks_recent_structure(brk, mempool):
    """Recent blocks list must have the same element structure."""
    path = "/api/blocks"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show(
        "GET", path,
        f"({len(b)} blocks, {b[-1]['height']}-{b[0]['height']})" if b else "[]",
        f"({len(m)} blocks, {m[-1]['height']}-{m[0]['height']})" if m else "[]",
    )
    assert len(b) > 0
    assert_same_structure(b, m)


def test_blocks_recent_ordering(brk):
    """Returned blocks must be ordered tip-first by strictly decreasing height."""
    b = brk.get_json("/api/blocks")
    heights = [blk["height"] for blk in b]
    show("GET", "/api/blocks", f"heights={heights[:5]}...", "—")
    assert heights == sorted(heights, reverse=True), (
        f"blocks are not strictly tip-first: {heights}"
    )
    assert len(set(heights)) == len(heights), "duplicate heights in /api/blocks"


def test_blocks_recent_count(brk):
    """mempool.space returns up to 15 blocks; brk should match that contract."""
    b = brk.get_json("/api/blocks")
    show("GET", "/api/blocks", f"({len(b)} blocks)", "—")
    assert 1 <= len(b) <= 15, f"unexpected block count: {len(b)}"
