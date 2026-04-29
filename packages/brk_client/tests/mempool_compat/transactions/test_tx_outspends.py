"""GET /api/tx/{txid}/outspends"""

from _lib import assert_same_values, show


def test_tx_outspends(brk, mempool, block):
    """Spending status of all outputs must match exactly."""
    path = f"/api/tx/{block.txid}/outspends"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_values(b, m)
