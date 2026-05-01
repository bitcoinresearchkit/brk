"""GET /api/address/{address}/txs"""

import pytest

from brk_client import BrkError

from _lib import assert_same_structure, show


# Heavy address (recently active) — stresses the 50-cap path; cannot be ordered
# exactly against mempool.space because the two indexers drift at the chain tip.
ACTIVE_ADDR = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"

# Inactive historical addresses — both indexers agree exactly on first-page
# ordering and on pagination.
STABLE_ADDRS = [
    "12cbQLTFMXRnSzktFkuoG3eHoMeFtpTu3S",  # p2pkh, ~125 txs
    "3D2oetdNuZUqQHPJmcMDDHYoqkyNVsFk9r",  # p2sh, ~5700 txs (heavy pagination)
]

STATIC_ADDRS = [ACTIVE_ADDR] + STABLE_ADDRS


@pytest.mark.parametrize("addr", STATIC_ADDRS)
def test_address_txs_shape(brk, mempool, addr):
    """Typed list response must structurally match mempool; brk's `index` extra is allowed."""
    path = f"/api/address/{addr}/txs"
    b = brk.get_address_txs(addr)
    m = mempool.get_json(path)
    show("GET", path, f"({len(b)} txs)", f"({len(m)} txs)")
    assert isinstance(b, list) and isinstance(m, list)
    if b and m:
        assert_same_structure(b[0], m[0])
        assert "index" in b[0], "brk-only `index` field missing"


def test_address_txs_shape_dynamic(brk, mempool, live_addrs):
    """Same shape contract over each live-discovered scriptpubkey type."""
    assert live_addrs, "no live addresses discovered"
    for atype, addr in live_addrs:
        path = f"/api/address/{addr}/txs"
        b = brk.get_address_txs(addr)
        m = mempool.get_json(path)
        show("GET", f"{path}  [{atype}]", f"({len(b)} txs)", f"({len(m)} txs)")
        assert isinstance(b, list) and isinstance(m, list)
        if b and m:
            assert_same_structure(b[0], m[0])


@pytest.mark.parametrize("addr", STATIC_ADDRS)
def test_address_txs_ordering(brk, addr):
    """All entries must be confirmed and heights monotonically non-increasing."""
    b = brk.get_address_txs(addr)
    if not b:
        pytest.skip(f"{addr} has no txs in brk")
    for tx in b:
        assert tx["status"]["confirmed"] is True, (
            f"{addr} returned unconfirmed tx {tx['txid']} (this endpoint is chain-only on brk)"
        )
    heights = [tx["status"]["block_height"] for tx in b]
    assert heights == sorted(heights, reverse=True), (
        f"{addr} not newest-first by height: {heights[:5]}..."
    )


@pytest.mark.parametrize("addr", STATIC_ADDRS)
def test_address_txs_limit(brk, addr):
    """Hard cap of 50 confirmed txs per call."""
    b = brk.get_address_txs(addr)
    assert len(b) <= 50, f"{addr} returned {len(b)} txs, exceeds 50-cap"


@pytest.mark.parametrize("addr", STABLE_ADDRS)
def test_address_txs_top_match_stable(brk, mempool, addr):
    """For inactive historical addresses, brk and mempool agree on first-page order."""
    b_txids = [t["txid"] for t in brk.get_address_txs(addr)]
    m_txids = [t["txid"] for t in mempool.get_json(f"/api/address/{addr}/txs")]
    assert b_txids == m_txids, (
        f"{addr} first-page txid order diverges:\n"
        f"  brk:     {b_txids[:5]}...\n"
        f"  mempool: {m_txids[:5]}..."
    )


def test_address_txs_pagination(brk, mempool):
    """`after_txid` returns a fresh, strictly-older page; matches mempool.space."""
    addr = "3D2oetdNuZUqQHPJmcMDDHYoqkyNVsFk9r"
    first = brk.get_address_txs(addr)
    assert len(first) == 50, f"expected full first page, got {len(first)}"
    last_txid = first[-1]["txid"]
    last_height = first[-1]["status"]["block_height"]

    second = brk.get_address_txs(addr, after_txid=last_txid)
    assert second, "second page must be non-empty for a 5700-tx address"

    first_txids = {t["txid"] for t in first}
    second_txids = {t["txid"] for t in second}
    assert not (first_txids & second_txids), "pagination must not return overlapping txs"

    for tx in second:
        assert tx["status"]["block_height"] <= last_height, (
            f"page 2 tx {tx['txid']} at height {tx['status']['block_height']} "
            f"exceeds page-1 tail height {last_height}"
        )

    m_second = mempool.get_json(f"/api/address/{addr}/txs?after_txid={last_txid}")
    b_ids = [t["txid"] for t in second]
    m_ids = [t["txid"] for t in m_second]
    assert b_ids == m_ids, (
        f"page-2 order diverges from mempool:\n"
        f"  brk:     {b_ids[:5]}...\n"
        f"  mempool: {m_ids[:5]}..."
    )


def test_address_txs_invalid(brk):
    """Garbage input must produce a BrkError carrying HTTP 400."""
    with pytest.raises(BrkError) as exc_info:
        brk.get_address_txs("abc")
    assert exc_info.value.status == 400, (
        f"expected status=400, got {exc_info.value.status}"
    )
