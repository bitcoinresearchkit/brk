"""
Address endpoint compatibility tests — parametrized across address types.

Endpoints covered:
    GET /api/address/{address}
    GET /api/address/{address}/txs
    GET /api/address/{address}/txs/chain
    GET /api/address/{address}/txs/mempool
    GET /api/address/{address}/utxo
    GET /api/v1/validate-address/{address}
"""

import pytest

from conftest import show, assert_same_structure, assert_same_values


@pytest.fixture(params=[
    "12cbQLTFMXRnSzktFkuoG3eHoMeFtpTu3S",  # P2PKH — early block reward
    "3D2oetdNuZUqQHPJmcMDDHYoqkyNVsFk9r",  # P2SH
], ids=["p2pkh", "p2sh"])
def static_addr(request):
    """Well-known addresses that always exist."""
    return request.param


@pytest.fixture()
def live_addrs(live):
    """All dynamically discovered address types."""
    return list(live.addresses.items())


# ── /api/address/{address} ───────────────────────────────────────────


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


# ── /api/address/{address}/txs ───────────────────────────────────────


def test_address_txs(brk, mempool, static_addr):
    """Address transaction list structure must match."""
    path = f"/api/address/{static_addr}/txs"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, f"({len(b)} txs)", f"({len(m)} txs)")
    assert isinstance(b, list) and isinstance(m, list)
    if b and m:
        assert_same_structure(b[0], m[0])


# ── /api/address/{address}/txs/chain ─────────────────────────────────


def test_address_txs_chain(brk, mempool, static_addr):
    """Confirmed-only tx list structure must match."""
    path = f"/api/address/{static_addr}/txs/chain"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, f"({len(b)} txs)", f"({len(m)} txs)")
    assert isinstance(b, list) and isinstance(m, list)
    if b and m:
        assert_same_structure(b[0], m[0])


# ── /api/address/{address}/txs/mempool ────────────────────────────────


def test_address_txs_mempool(brk, mempool, live):
    """Mempool tx list must be an array (contents are volatile)."""
    path = f"/api/address/{live.sample_address}/txs/mempool"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, f"({len(b)} txs)", f"({len(m)} txs)")
    assert isinstance(b, list) and isinstance(m, list)


# ── /api/address/{address}/utxo ──────────────────────────────────────


def test_address_utxo(brk, mempool, static_addr):
    """UTXO list must match — same txids, values, and statuses."""
    path = f"/api/address/{static_addr}/utxo"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, f"({len(b)} utxos)", f"({len(m)} utxos)")
    assert isinstance(b, list) and isinstance(m, list)
    # Sort by txid+vout for stable comparison
    key = lambda u: (u.get("txid", ""), u.get("vout", 0))
    b_sorted = sorted(b, key=key)
    m_sorted = sorted(m, key=key)
    assert_same_values(b_sorted, m_sorted)


# ── /api/v1/validate-address/{address} ───────────────────────────────


def test_validate_address_discovered(brk, mempool, live_addrs):
    """Validation of each discovered address type must match exactly."""
    for atype, addr in live_addrs:
        path = f"/api/v1/validate-address/{addr}"
        b = brk.get_json(path)
        m = mempool.get_json(path)
        show("GET", f"{path}  [{atype}]", b, m)
        assert_same_values(b, m)
        assert b["isvalid"] is True


def test_validate_address_p2pkh(brk, mempool):
    """Satoshi's P2PKH address must validate identically."""
    path = "/api/v1/validate-address/1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_values(b, m)
    assert b["isvalid"] is True


def test_validate_address_invalid(brk, mempool):
    """Invalid address must produce the same rejection structure."""
    path = "/api/v1/validate-address/notanaddress123"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert b["isvalid"] is False
    assert m["isvalid"] is False
    assert_same_structure(b, m)
