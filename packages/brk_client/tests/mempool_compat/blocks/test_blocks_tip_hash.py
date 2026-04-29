"""GET /api/blocks/tip/hash"""

from _lib import show


def test_blocks_tip_hash_format(brk, mempool):
    """Tip hash must be a valid 64-char hex string on both servers."""
    path = "/api/blocks/tip/hash"
    b = brk.get_text(path)
    m = mempool.get_text(path)
    show("GET", path, b, m)
    assert len(b) == 64 and all(c in "0123456789abcdef" for c in b.lower())
    assert len(m) == 64 and all(c in "0123456789abcdef" for c in m.lower())


def test_blocks_tip_hash_matches_height(brk):
    """`tip/hash` must equal `block-height/{tip_height}`."""
    h = int(brk.get_text("/api/blocks/tip/height"))
    by_height = brk.get_text(f"/api/block-height/{h}")
    tip_hash = brk.get_text("/api/blocks/tip/hash")
    show("GET", "/api/blocks/tip/hash", tip_hash, by_height)
    # Allow a one-block race if a new block landed between the two fetches.
    if tip_hash != by_height:
        h2 = int(brk.get_text("/api/blocks/tip/height"))
        assert h2 != h or tip_hash == by_height, (
            f"tip/hash={tip_hash} but block-height/{h}={by_height}"
        )


def test_blocks_tip_hash_matches_recent(brk):
    """`tip/hash` must equal the first hash in `/api/blocks`."""
    tip_hash = brk.get_text("/api/blocks/tip/hash")
    blocks = brk.get_json("/api/blocks")
    show("GET", "/api/blocks/tip/hash", tip_hash, blocks[0]["id"])
    assert blocks and blocks[0]["id"] == tip_hash, (
        f"tip/hash={tip_hash} but /api/blocks[0].id={blocks[0].get('id')}"
    )
