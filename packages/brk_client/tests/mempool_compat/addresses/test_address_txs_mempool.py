"""GET /api/address/{address}/txs/mempool"""

from _lib import show


def test_address_txs_mempool_sample(brk, mempool, live):
    """Mempool tx list must be an array (contents are volatile)."""
    path = f"/api/address/{live.sample_address}/txs/mempool"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, f"({len(b)} txs)", f"({len(m)} txs)")
    assert isinstance(b, list) and isinstance(m, list)


def test_address_txs_mempool_discovered(brk, mempool, live_addrs):
    """Mempool tx list must be a (possibly empty) array for each discovered type."""
    for atype, addr in live_addrs:
        path = f"/api/address/{addr}/txs/mempool"
        b = brk.get_json(path)
        m = mempool.get_json(path)
        show("GET", f"{path}  [{atype}]", f"({len(b)} txs)", f"({len(m)} txs)")
        assert isinstance(b, list) and isinstance(m, list)


def test_address_txs_mempool_all_unconfirmed(brk, live):
    """Every tx returned by /txs/mempool must have confirmed=False (if any)."""
    path = f"/api/address/{live.sample_address}/txs/mempool"
    b = brk.get_json(path)
    show("GET", path, f"({len(b)} txs)", "—")
    confirmed = [t for t in b if t.get("status", {}).get("confirmed", False)]
    assert not confirmed, (
        f"{len(confirmed)} confirmed tx(s) returned by /txs/mempool"
    )
