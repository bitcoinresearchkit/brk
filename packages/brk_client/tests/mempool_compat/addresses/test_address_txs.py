"""GET /api/address/{address}/txs"""

import pytest

from _lib import assert_same_structure, show


@pytest.fixture(params=[
    "12cbQLTFMXRnSzktFkuoG3eHoMeFtpTu3S",
    "3D2oetdNuZUqQHPJmcMDDHYoqkyNVsFk9r",
], ids=["p2pkh", "p2sh"])
def static_addr(request):
    return request.param


def test_address_txs_static(brk, mempool, static_addr):
    """Confirmed+mempool tx list structure must match for well-known addresses."""
    path = f"/api/address/{static_addr}/txs"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, f"({len(b)} txs)", f"({len(m)} txs)")
    assert isinstance(b, list) and isinstance(m, list)
    if b and m:
        assert_same_structure(b[0], m[0])


def test_address_txs_discovered(brk, mempool, live_addrs):
    """Confirmed+mempool tx list structure must match for each discovered type."""
    for atype, addr in live_addrs:
        path = f"/api/address/{addr}/txs"
        b = brk.get_json(path)
        m = mempool.get_json(path)
        show("GET", f"{path}  [{atype}]", f"({len(b)} txs)", f"({len(m)} txs)")
        assert isinstance(b, list) and isinstance(m, list)
        if b and m:
            assert_same_structure(b[0], m[0])


def test_address_txs_fields(brk, mempool, live):
    """Every tx in the list must carry the core mempool.space fields."""
    path = f"/api/address/{live.sample_address}/txs"
    b = brk.get_json(path)
    show("GET", path, f"({len(b)} txs)", "—")
    if not b:
        pytest.skip("address has no txs in brk")
    required = {"txid", "version", "locktime", "vin", "vout", "size", "weight", "fee", "status"}
    for tx in b[:5]:
        missing = required - set(tx.keys())
        assert not missing, f"tx {tx.get('txid', '?')} missing fields: {missing}"
