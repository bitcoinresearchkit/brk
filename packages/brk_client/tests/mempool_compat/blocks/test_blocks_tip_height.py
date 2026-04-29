"""GET /api/blocks/tip/height"""

from _lib import show


def test_blocks_tip_height_close(brk, mempool):
    """Tip heights must be within a few blocks of each other."""
    path = "/api/blocks/tip/height"
    b = int(brk.get_text(path))
    m = int(mempool.get_text(path))
    show("GET", path, b, m)
    assert abs(b - m) <= 3, f"Tip heights differ by {abs(b - m)}: brk={b}, mempool={m}"


def test_blocks_tip_height_resolves_to_hash(brk):
    """`tip/height` must resolve to a valid hash via `block-height/{tip}`."""
    h = int(brk.get_text("/api/blocks/tip/height"))
    bh = brk.get_text(f"/api/block-height/{h}")
    show("GET", "/api/blocks/tip/height", h, bh)
    assert len(bh) == 64 and all(c in "0123456789abcdef" for c in bh.lower()), (
        f"block-height/{h} returned non-hash: {bh!r}"
    )


def test_blocks_tip_height_matches_recent(brk):
    """`tip/height` must equal the first element's height in `/api/blocks`."""
    h = int(brk.get_text("/api/blocks/tip/height"))
    blocks = brk.get_json("/api/blocks")
    show("GET", "/api/blocks/tip/height", h, blocks[0]["height"])
    assert blocks and blocks[0]["height"] == h, (
        f"tip/height={h} but /api/blocks[0].height={blocks[0]['height']}"
    )
