"""GET /api/address/{address}/txs/chain"""

import pytest

from _lib import assert_same_structure, show


@pytest.fixture(params=[
    "12cbQLTFMXRnSzktFkuoG3eHoMeFtpTu3S",
    "3D2oetdNuZUqQHPJmcMDDHYoqkyNVsFk9r",
], ids=["p2pkh", "p2sh"])
def static_addr(request):
    return request.param


def test_address_txs_chain_static(brk, mempool, static_addr):
    """Confirmed-only tx list structure must match for well-known addresses."""
    path = f"/api/address/{static_addr}/txs/chain"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, f"({len(b)} txs)", f"({len(m)} txs)")
    assert isinstance(b, list) and isinstance(m, list)
    if b and m:
        assert_same_structure(b[0], m[0])


def test_address_txs_chain_discovered(brk, mempool, live_addrs):
    """Confirmed-only tx list structure must match for each discovered type."""
    for atype, addr in live_addrs:
        path = f"/api/address/{addr}/txs/chain"
        b = brk.get_json(path)
        m = mempool.get_json(path)
        show("GET", f"{path}  [{atype}]", f"({len(b)} txs)", f"({len(m)} txs)")
        assert isinstance(b, list) and isinstance(m, list)
        if b and m:
            assert_same_structure(b[0], m[0])


def test_address_txs_chain_all_confirmed(brk, live):
    """Every tx returned by /txs/chain must have confirmed=True in its status."""
    path = f"/api/address/{live.sample_address}/txs/chain"
    b = brk.get_json(path)
    show("GET", path, f"({len(b)} txs)", "—")
    if not b:
        pytest.skip("address has no confirmed txs in brk")
    unconfirmed = [t for t in b if not t.get("status", {}).get("confirmed", False)]
    assert not unconfirmed, (
        f"{len(unconfirmed)} unconfirmed tx(s) returned by /txs/chain"
    )
