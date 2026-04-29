"""GET /api/tx/{txid}/outspend/{vout}"""

from _lib import assert_same_values, show


def test_tx_outspend_first(brk, mempool, block):
    """Spending status of vout 0 must match exactly."""
    path = f"/api/tx/{block.txid}/outspend/0"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_values(b, m)


def test_tx_outspend_last(brk, mempool, block):
    """Spending status of the last vout must also match exactly."""
    tx = mempool.get_json(f"/api/tx/{block.txid}")
    last_vout = len(tx["vout"]) - 1
    path = f"/api/tx/{block.txid}/outspend/{last_vout}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_values(b, m)


def test_tx_outspend_out_of_range(brk, mempool, block):
    """A vout index past the last output must produce the same response on both servers.

    Both servers return `{"spent": false}` rather than 4xx — they don't bound-check
    the vout index. The compat property is that they agree.
    """
    tx = mempool.get_json(f"/api/tx/{block.txid}")
    bad_vout = len(tx["vout"]) + 100
    path = f"/api/tx/{block.txid}/outspend/{bad_vout}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert b == m, f"out-of-range outspend disagrees: brk={b} vs mempool={m}"
