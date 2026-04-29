"""GET /api/v1/transaction-times?txId[]=..."""

from _lib import show


def test_transaction_times_few(brk, mempool, live):
    """First-seen timestamps must match for a few txids."""
    txids = [b.txid for b in live.blocks[:3]]
    params = [("txId[]", t) for t in txids]
    path = "/api/v1/transaction-times"
    b = brk.get_json(path, params=params)
    m = mempool.get_json(path, params=params)
    show("GET", f"{path}?txId[]={{{len(txids)} txids}}", b, m)
    assert isinstance(b, list) and isinstance(m, list)
    assert len(b) == len(m) == len(txids)
    assert b == m, f"timestamps differ: brk={b} vs mempool={m}"


def test_transaction_times_many(brk, mempool, live):
    """A larger batch (covering all sample blocks + coinbases) must match exactly."""
    txids = [b.txid for b in live.blocks] + [b.coinbase_txid for b in live.blocks]
    params = [("txId[]", t) for t in txids]
    path = "/api/v1/transaction-times"
    b = brk.get_json(path, params=params)
    m = mempool.get_json(path, params=params)
    show("GET", f"{path}?txId[]={{{len(txids)} txids}}", f"({len(b)})", f"({len(m)})")
    assert len(b) == len(m) == len(txids)
    assert b == m, f"timestamps differ: brk={b} vs mempool={m}"


def test_transaction_times_single(brk, mempool, live):
    """A single-element batch must return a 1-element list with the same value."""
    txid = live.sample_txid
    params = [("txId[]", txid)]
    path = "/api/v1/transaction-times"
    b = brk.get_json(path, params=params)
    m = mempool.get_json(path, params=params)
    show("GET", f"{path}?txId[]={txid[:16]}...", b, m)
    assert isinstance(b, list) and isinstance(m, list)
    assert len(b) == len(m) == 1
    assert b == m, f"single timestamp differs: brk={b} vs mempool={m}"


def test_transaction_times_empty(brk, mempool):
    """An empty batch must be rejected (any non-2xx) on both servers.

    mempool.space returns 500 — technically a server-side bug (it should be a
    4xx since the request itself is malformed) — so we don't insist on exact
    status parity, only that neither server silently treats it as valid input.
    """
    path = "/api/v1/transaction-times"
    b_resp = brk.get_raw(path)
    m_resp = mempool.get_raw(path)
    show("GET", path, f"brk={b_resp.status_code}", f"mempool={m_resp.status_code}")
    assert not b_resp.ok, f"brk accepted empty batch with {b_resp.status_code}: {b_resp.text!r}"
    assert not m_resp.ok, f"mempool accepted empty batch with {m_resp.status_code}"
