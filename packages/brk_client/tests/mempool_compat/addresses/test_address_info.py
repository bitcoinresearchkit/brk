"""GET /api/address/{address}"""

import pytest

from _lib import assert_same_structure, show


@pytest.fixture(params=[
    "12cbQLTFMXRnSzktFkuoG3eHoMeFtpTu3S",  # P2PKH — early block reward
    "3D2oetdNuZUqQHPJmcMDDHYoqkyNVsFk9r",  # P2SH
], ids=["p2pkh", "p2sh"])
def static_addr(request):
    """Well-known addresses that always exist."""
    return request.param


def test_address_info_static(brk, mempool, static_addr):
    """Address stats structure must match for well-known addresses."""
    path = f"/api/address/{static_addr}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_structure(b, m)
    assert b["address"] == m["address"]


def test_address_info_discovered(brk, mempool, live_addrs):
    """Address stats structure must match for each discovered type."""
    for atype, addr in live_addrs:
        path = f"/api/address/{addr}"
        b = brk.get_json(path)
        m = mempool.get_json(path)
        show("GET", f"{path}  [{atype}]", b, m)
        assert_same_structure(b, m)
        assert b["address"] == m["address"]


def test_address_chain_stats_close(brk, mempool, live_addrs):
    """Chain stats values must be close for each discovered address."""
    for atype, addr in live_addrs:
        path = f"/api/address/{addr}"
        b = brk.get_json(path)["chain_stats"]
        m = mempool.get_json(path)["chain_stats"]
        show("GET", f"{path}  [chain_stats, {atype}]", b, m)
        assert_same_structure(b, m)
        assert abs(b["tx_count"] - m["tx_count"]) <= 5, (
            f"{atype} tx_count: brk={b['tx_count']} vs mempool={m['tx_count']}"
        )
