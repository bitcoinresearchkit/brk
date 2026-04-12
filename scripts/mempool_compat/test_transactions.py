"""
Transaction endpoint compatibility tests — parametrized across blockchain eras.

Endpoints covered:
    GET  /api/tx/{txid}
    GET  /api/tx/{txid}/hex                text/plain
    GET  /api/tx/{txid}/raw                application/octet-stream
    GET  /api/tx/{txid}/status
    GET  /api/tx/{txid}/merkle-proof
    GET  /api/tx/{txid}/merkleblock-proof  text/plain
    GET  /api/tx/{txid}/outspend/{vout}
    GET  /api/tx/{txid}/outspends
    GET  /api/v1/cpfp/{txid}
    GET  /api/v1/transaction-times
"""

import pytest

from conftest import show, assert_same_structure, assert_same_values


@pytest.fixture(params=range(8), ids=[
    "h100", "h100k", "h400k", "h630k", "h800k", "recent1k", "recent100", "recent10",
])
def block(request, live):
    i = request.param
    if i >= len(live.blocks):
        pytest.skip("block not discovered")
    return live.blocks[i]


# ── /api/tx/{txid} ───────────────────────────────────────────────────


def test_tx_by_id(brk, mempool, block):
    """Full transaction data must match for a confirmed tx."""
    path = f"/api/tx/{block.txid}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_values(b, m, exclude={"sigops"})


def test_tx_coinbase(brk, mempool, block):
    """Coinbase transaction must match."""
    path = f"/api/tx/{block.coinbase_txid}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_values(b, m, exclude={"sigops"})


# ── /api/tx/{txid}/hex ───────────────────────────────────────────────


def test_tx_hex(brk, mempool, block):
    """Raw transaction hex must be identical."""
    path = f"/api/tx/{block.txid}/hex"
    b = brk.get_text(path)
    m = mempool.get_text(path)
    show("GET", path, b[:80] + "…", m[:80] + "…")
    assert b == m


# ── /api/tx/{txid}/raw ───────────────────────────────────────────────


def test_tx_raw(brk, mempool, block):
    """Raw transaction bytes must be identical."""
    path = f"/api/tx/{block.txid}/raw"
    b = brk.get_bytes(path)
    m = mempool.get_bytes(path)
    show("GET", path, f"<{len(b)} bytes>", f"<{len(m)} bytes>")
    assert b == m


# ── /api/tx/{txid}/status ────────────────────────────────────────────


def test_tx_status(brk, mempool, block):
    """Confirmation status must match for a confirmed tx."""
    path = f"/api/tx/{block.txid}/status"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_values(b, m)


# ── /api/tx/{txid}/merkle-proof ──────────────────────────────────────


def test_tx_merkle_proof(brk, mempool, block):
    """Merkle inclusion proof must match."""
    path = f"/api/tx/{block.txid}/merkle-proof"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_values(b, m)


# ── /api/tx/{txid}/merkleblock-proof ─────────────────────────────────


def test_tx_merkleblock_proof(brk, mempool, block):
    """BIP37 merkleblock proof hex must be identical."""
    path = f"/api/tx/{block.txid}/merkleblock-proof"
    b = brk.get_text(path)
    m = mempool.get_text(path)
    show("GET", path, b[:80] + "…", m[:80] + "…")
    assert b == m


# ── /api/tx/{txid}/outspend/{vout} ───────────────────────────────────


def test_tx_outspend(brk, mempool, block):
    """Spending status of output 0 must match exactly."""
    path = f"/api/tx/{block.txid}/outspend/0"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_values(b, m)


# ── /api/tx/{txid}/outspends ─────────────────────────────────────────


def test_tx_outspends(brk, mempool, block):
    """Spending status of all outputs must match exactly."""
    path = f"/api/tx/{block.txid}/outspends"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_values(b, m)


# ── /api/v1/cpfp/{txid} ─────────────────────────────────────────────


def test_cpfp(brk, mempool, block):
    """CPFP info structure must match for a confirmed tx."""
    path = f"/api/v1/cpfp/{block.txid}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_structure(b, m)


# ── /api/v1/transaction-times ────────────────────────────────────────


def test_transaction_times(brk, mempool, live):
    """First-seen timestamps array must have the same length."""
    txids = [b.txid for b in live.blocks[:3]]
    params = [("txId[]", t) for t in txids]
    path = "/api/v1/transaction-times"
    b = brk.get_json(path, params=params)
    m = mempool.get_json(path, params=params)
    show("GET", f"{path}?txId[]={{{len(txids)} txids}}", b, m)
    assert isinstance(b, list) and isinstance(m, list)
    assert len(b) == len(m) == len(txids)
