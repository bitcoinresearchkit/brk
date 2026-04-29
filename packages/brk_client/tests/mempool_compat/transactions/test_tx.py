"""GET /api/tx/{txid}"""

from _lib import assert_same_values, show


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
