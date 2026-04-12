"""
Block endpoint compatibility tests — parametrized across blockchain eras.

Endpoints covered:
    GET /api/block/{hash}
    GET /api/v1/block/{hash}              (with extras)
    GET /api/block/{hash}/header           text/plain
    GET /api/block/{hash}/status
    GET /api/block/{hash}/txids
    GET /api/block/{hash}/txs
    GET /api/block/{hash}/txs/{start}
    GET /api/block/{hash}/txid/{index}     text/plain
    GET /api/block-height/{height}         text/plain
    GET /api/blocks
    GET /api/blocks/{height}
    GET /api/v1/blocks
    GET /api/v1/blocks/{height}
    GET /api/blocks/tip/height             text/plain
    GET /api/blocks/tip/hash               text/plain
"""

import pytest

from conftest import show, assert_same_structure, assert_same_values


def _block_ids(live):
    return [f"h{b.height}" for b in live.blocks]


def _bi(request, live):
    """Resolve parametrized block index, skip if out of range."""
    i = request.param
    if i >= len(live.blocks):
        pytest.skip("block not discovered")
    return live.blocks[i]


@pytest.fixture(params=range(8), ids=[
    "h100", "h100k", "h400k", "h630k", "h800k", "recent1k", "recent100", "recent10",
])
def block(request, live):
    i = request.param
    if i >= len(live.blocks):
        pytest.skip("block not discovered")
    return live.blocks[i]


# ── /api/block/{hash} ────────────────────────────────────────────────


def test_block_by_hash(brk, mempool, block):
    """Confirmed block info must be identical."""
    path = f"/api/block/{block.hash}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_values(b, m)


# ── /api/v1/block/{hash} ─────────────────────────────────────────────


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


# ── /api/block/{hash}/header ─────────────────────────────────────────


def test_block_header(brk, mempool, block):
    """80-byte hex block header must be identical."""
    path = f"/api/block/{block.hash}/header"
    b = brk.get_text(path)
    m = mempool.get_text(path)
    show("GET", path, b, m)
    assert len(b) == 160, f"Expected 160 hex chars (80 bytes), got {len(b)}"
    assert b == m


# ── /api/block/{hash}/status ─────────────────────────────────────────


def test_block_status(brk, mempool, block):
    """Block status must be identical for a confirmed block."""
    path = f"/api/block/{block.hash}/status"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_values(b, m)


# ── /api/block/{hash}/txids ──────────────────────────────────────────


def test_block_txids(brk, mempool, block):
    """Ordered txid list must be identical."""
    path = f"/api/block/{block.hash}/txids"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b[:3], m[:3])
    assert b == m


# ── /api/block/{hash}/txs ────────────────────────────────────────────


def test_block_txs_page0(brk, mempool, block):
    """First page of block transactions must match."""
    path = f"/api/block/{block.hash}/txs"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, f"({len(b)} txs)", f"({len(m)} txs)")
    assert len(b) == len(m), f"Page size: brk={len(b)} vs mempool={len(m)}"
    if b and m:
        assert_same_values(b[0], m[0], exclude={"sigops"})


def test_block_txs_start_index(brk, mempool, block):
    """Paginated txs from index 25 must match (skip small blocks)."""
    # Blocks with <26 txs don't have a second page
    txids = mempool.get_json(f"/api/block/{block.hash}/txids")
    if len(txids) <= 25:
        pytest.skip(f"block has only {len(txids)} txs")
    path = f"/api/block/{block.hash}/txs/25"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, f"({len(b)} txs)", f"({len(m)} txs)")
    assert len(b) == len(m)
    if b and m:
        assert_same_structure(b[0], m[0])


# ── /api/block/{hash}/txid/{index} ───────────────────────────────────


def test_block_txid_at_index_0(brk, mempool, block):
    """Txid at position 0 (coinbase) must match."""
    path = f"/api/block/{block.hash}/txid/0"
    b = brk.get_text(path)
    m = mempool.get_text(path)
    show("GET", path, b, m)
    assert b == m


def test_block_txid_at_index_1(brk, mempool, block):
    """Txid at position 1 (first non-coinbase) must match."""
    txids = mempool.get_json(f"/api/block/{block.hash}/txids")
    if len(txids) <= 1:
        pytest.skip("block has only coinbase")
    path = f"/api/block/{block.hash}/txid/1"
    b = brk.get_text(path)
    m = mempool.get_text(path)
    show("GET", path, b, m)
    assert b == m


def test_block_txid_at_last_index(brk, mempool, block):
    """Txid at last position must match."""
    txids = mempool.get_json(f"/api/block/{block.hash}/txids")
    last = len(txids) - 1
    path = f"/api/block/{block.hash}/txid/{last}"
    b = brk.get_text(path)
    m = mempool.get_text(path)
    show("GET", path, b, m)
    assert b == m


# ── /api/block-height/{height} ───────────────────────────────────────


def test_block_height_to_hash(brk, mempool, block):
    """Block hash at a given height must match."""
    path = f"/api/block-height/{block.height}"
    b = brk.get_text(path)
    m = mempool.get_text(path)
    show("GET", path, b, m)
    assert b == m
    assert b == block.hash


# ── /api/blocks/{height} ─────────────────────────────────────────────


def test_blocks_from_height(brk, mempool, block):
    """Confirmed blocks from a fixed height must match exactly."""
    path = f"/api/blocks/{block.height}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, f"({len(b)} blocks)", f"({len(m)} blocks)")
    assert len(b) == len(m)
    if b and m:
        assert_same_values(b[0], m[0])


def test_blocks_v1_from_height(brk, mempool, block):
    """v1 blocks from a confirmed height — all values must match."""
    path = f"/api/v1/blocks/{block.height}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, f"({len(b)} blocks)", f"({len(m)} blocks)")
    assert len(b) == len(m)
    if b and m:
        assert_same_values(b[0], m[0])


# ── non-parametrized (no block param) ────────────────────────────────


def test_blocks_recent(brk, mempool):
    """Recent blocks list must have the same structure."""
    path = "/api/blocks"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show(
        "GET", path,
        f"({len(b)} blocks, {b[-1]['height']}–{b[0]['height']})" if b else "[]",
        f"({len(m)} blocks, {m[-1]['height']}–{m[0]['height']})" if m else "[]",
    )
    assert len(b) > 0
    assert_same_structure(b, m)


def test_blocks_v1_recent(brk, mempool):
    """Recent v1 blocks (with extras) must have the same structure."""
    path = "/api/v1/blocks"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, f"({len(b)} blocks)", f"({len(m)} blocks)")
    assert len(b) > 0
    assert_same_structure(b, m)


def test_blocks_tip_height(brk, mempool):
    """Tip heights must be within a few blocks of each other."""
    path = "/api/blocks/tip/height"
    b = int(brk.get_text(path))
    m = int(mempool.get_text(path))
    show("GET", path, b, m)
    assert abs(b - m) <= 3, f"Tip heights differ by {abs(b - m)}: brk={b}, mempool={m}"


def test_blocks_tip_hash(brk, mempool):
    """Tip hash must be a valid 64-char hex string."""
    path = "/api/blocks/tip/hash"
    b = brk.get_text(path)
    m = mempool.get_text(path)
    show("GET", path, b, m)
    assert len(b) == 64
    assert len(m) == 64
