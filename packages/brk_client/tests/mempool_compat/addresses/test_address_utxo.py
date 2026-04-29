"""GET /api/address/{address}/utxo"""

import pytest

from _lib import assert_same_values, show


@pytest.fixture(params=[
    "12cbQLTFMXRnSzktFkuoG3eHoMeFtpTu3S",
    "3D2oetdNuZUqQHPJmcMDDHYoqkyNVsFk9r",
], ids=["p2pkh", "p2sh"])
def static_addr(request):
    return request.param


def test_address_utxo_static(brk, mempool, static_addr):
    """UTXO list must match — same txids, values, and statuses."""
    path = f"/api/address/{static_addr}/utxo"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, f"({len(b)} utxos)", f"({len(m)} utxos)")
    assert isinstance(b, list) and isinstance(m, list)
    key = lambda u: (u.get("txid", ""), u.get("vout", 0))
    b_sorted = sorted(b, key=key)
    m_sorted = sorted(m, key=key)
    assert_same_values(b_sorted, m_sorted)


def test_address_utxo_discovered(brk, mempool, live_addrs):
    """UTXO list must match for each discovered address type — same txids, values, and statuses."""
    for atype, addr in live_addrs:
        path = f"/api/address/{addr}/utxo"
        b = brk.get_json(path)
        m = mempool.get_json(path)
        show("GET", f"{path}  [{atype}]", f"({len(b)} utxos)", f"({len(m)} utxos)")
        assert isinstance(b, list) and isinstance(m, list)
        key = lambda u: (u.get("txid", ""), u.get("vout", 0))
        assert_same_values(sorted(b, key=key), sorted(m, key=key))


def test_address_utxo_fields(brk, live):
    """Every utxo must carry the core mempool.space fields."""
    path = f"/api/address/{live.sample_address}/utxo"
    b = brk.get_json(path)
    show("GET", path, f"({len(b)} utxos)", "—")
    if not b:
        pytest.skip("address has no utxos in brk")
    required = {"txid", "vout", "value", "status"}
    for u in b[:5]:
        missing = required - set(u.keys())
        assert not missing, f"utxo {u.get('txid', '?')}:{u.get('vout', '?')} missing fields: {missing}"
        assert isinstance(u["value"], int) and u["value"] > 0
