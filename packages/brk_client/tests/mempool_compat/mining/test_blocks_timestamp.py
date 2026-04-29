"""GET /api/v1/mining/blocks/timestamp/{timestamp}"""

from _lib import assert_same_structure, show


def test_mining_blocks_timestamp(brk, mempool, live):
    """Block lookup by timestamp must have the same structure for various eras."""
    for block in live.blocks:
        info = brk.get_json(f"/api/block/{block.hash}")
        ts = info["timestamp"]
        path = f"/api/v1/mining/blocks/timestamp/{ts}"
        b = brk.get_json(path)
        m = mempool.get_json(path)
        show("GET", path, b, m)
        assert_same_structure(b, m)
