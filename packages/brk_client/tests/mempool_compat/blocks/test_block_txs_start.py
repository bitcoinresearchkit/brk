"""GET /api/block/{hash}/txs/{start_index}"""

import pytest

from _lib import assert_same_structure, show


def test_block_txs_start_index_25(brk, mempool, block):
    """Paginated txs from index 25 must match (skip small blocks)."""
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


def test_block_txs_start_index_zero(brk, mempool, block):
    """`/txs/0` must mirror `/txs` (the default page) in length and structure."""
    path0 = f"/api/block/{block.hash}/txs/0"
    pathx = f"/api/block/{block.hash}/txs"
    b0 = brk.get_json(path0)
    bx = brk.get_json(pathx)
    show("GET", path0, f"({len(b0)} txs)", f"vs /txs ({len(bx)} txs)")
    assert len(b0) == len(bx)
    if b0 and bx:
        assert b0[0]["txid"] == bx[0]["txid"]


def test_block_txs_start_aligned_pagination(brk, mempool, block):
    """Pages at 0, 25, 50 must each be aligned slices of the full txid list."""
    txids = mempool.get_json(f"/api/block/{block.hash}/txids")
    if len(txids) <= 50:
        pytest.skip(f"block has only {len(txids)} txs")
    # mempool.space orders txids tip-first inside the block payload, but
    # /txids returns them in block order (coinbase-first). Paged /txs follows
    # the same coinbase-first order — so page N starts at offset N.
    page0 = brk.get_json(f"/api/block/{block.hash}/txs/0")
    page25 = brk.get_json(f"/api/block/{block.hash}/txs/25")
    page50 = brk.get_json(f"/api/block/{block.hash}/txs/50")
    show("GET", f"/api/block/{block.hash}/txs/{{0,25,50}}",
         f"page0={len(page0)} page25={len(page25)} page50={len(page50)}", "—")
    # The paging origin is what mempool.space does; verify against the live
    # /txids list rather than re-deriving the order ourselves.
    assert page0 and page0[0]["txid"] == txids[0]
    assert page25 and page25[0]["txid"] == txids[25]
    assert page50 and page50[0]["txid"] == txids[50]


def test_block_txs_start_past_end(brk, mempool, block):
    """A start index past the last tx must produce the same response on both servers."""
    txids = mempool.get_json(f"/api/block/{block.hash}/txids")
    past = len(txids) + 1000
    path = f"/api/block/{block.hash}/txs/{past}"
    b_resp = brk.get_raw(path)
    m_resp = mempool.get_raw(path)
    show("GET", path, f"brk={b_resp.status_code}", f"mempool={m_resp.status_code}")
    assert b_resp.status_code == m_resp.status_code, (
        f"past-end status differs: brk={b_resp.status_code} vs mempool={m_resp.status_code}"
    )
    if b_resp.status_code == 200:
        assert b_resp.json() == m_resp.json(), (
            f"past-end body differs: brk={b_resp.json()} vs mempool={m_resp.json()}"
        )
