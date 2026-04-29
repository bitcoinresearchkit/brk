"""GET /api/tx/{txid}/status"""

from _lib import assert_same_values, show


def test_tx_status(brk, mempool, block):
    """Confirmation status must match for a confirmed tx."""
    path = f"/api/tx/{block.txid}/status"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_values(b, m)
