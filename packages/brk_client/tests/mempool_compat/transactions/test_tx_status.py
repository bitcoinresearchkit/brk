"""GET /api/tx/{txid}/status"""

import pytest
from brk_client import BrkError

from _lib import assert_same_values, show


def test_tx_status_value_parity(brk, mempool, block):
    """Status must match for a confirmed regular tx (multi-era)."""
    path = f"/api/tx/{block.txid}/status"
    b = brk.get_tx_status(block.txid)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_values(b, m)


def test_tx_status_coinbase_value_parity(brk, mempool, block):
    """Status must match for a coinbase tx (multi-era)."""
    path = f"/api/tx/{block.coinbase_txid}/status"
    b = brk.get_tx_status(block.coinbase_txid)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_values(b, m)


def test_tx_status_invariants(brk, live):
    """Recent confirmed tx: confirmed=True, height/hash/time match the block."""
    sample = live.blocks[-1]
    s = brk.get_tx_status(sample.txid)
    show("GET", f"/api/tx/{sample.txid}/status", s, "-")
    assert s["confirmed"] is True
    assert int(s["block_height"]) == sample.height
    assert s["block_hash"] == sample.hash
    assert int(s["block_time"]) > 0


@pytest.mark.parametrize("bad", ["abc", "deadbeef"])
def test_tx_status_malformed_short(brk, bad):
    """Short txid must produce BrkError(status=400)."""
    with pytest.raises(BrkError) as exc_info:
        brk.get_text(f"/api/tx/{bad}/status")
    assert exc_info.value.status == 400


def test_tx_status_malformed_unknown(brk):
    """Valid 64-char hex with no matching tx must produce BrkError(status=404)."""
    bad = "0" * 64
    with pytest.raises(BrkError) as exc_info:
        brk.get_text(f"/api/tx/{bad}/status")
    assert exc_info.value.status == 404
