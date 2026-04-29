"""GET /api/block/{hash}/txid/{index}"""

import pytest

from _lib import show


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
