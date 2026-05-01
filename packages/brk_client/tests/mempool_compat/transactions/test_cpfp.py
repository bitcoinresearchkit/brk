"""GET /api/v1/cpfp/{txid}"""

import pytest
from brk_client import BrkError

from _lib import assert_same_structure, show


def test_cpfp_structure(brk, mempool, block):
    """CPFP structure must match for a confirmed regular tx (multi-era)."""
    path = f"/api/v1/cpfp/{block.txid}"
    b = brk.get_cpfp(block.txid)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_structure(b, m)


def test_cpfp_coinbase_structure(brk, mempool, block):
    """CPFP structure must match for a coinbase tx (multi-era)."""
    path = f"/api/v1/cpfp/{block.coinbase_txid}"
    b = brk.get_cpfp(block.coinbase_txid)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_structure(b, m)


def test_cpfp_invariants(brk, live):
    """Recent confirmed tx: ancestors empty, any brk-computed extras non-negative."""
    sample = live.blocks[-1]
    c = brk.get_cpfp(sample.txid)
    show("GET", f"/api/v1/cpfp/{sample.txid}", c, "-")
    assert c["ancestors"] == [], "confirmed tx must have empty ancestors"
    if "fee" in c:
        assert int(c["fee"]) >= 0
    if "effectiveFeePerVsize" in c:
        assert c["effectiveFeePerVsize"] >= 0
    if "adjustedVsize" in c:
        assert int(c["adjustedVsize"]) > 0


def test_cpfp_unknown_tx_returns_empty(brk, mempool):
    """Both servers return {ancestors: []} for any 64-char hex (no 404)."""
    bad = "0" * 64
    path = f"/api/v1/cpfp/{bad}"
    b = brk.get_cpfp(bad)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert b.get("ancestors") == []
    assert m.get("ancestors") == []


@pytest.mark.parametrize("bad", ["abc", "deadbeef"])
def test_cpfp_malformed_short(brk, bad):
    """Short txid must produce BrkError(status=400) on brk (mempool returns 501)."""
    with pytest.raises(BrkError) as exc_info:
        brk.get_text(f"/api/v1/cpfp/{bad}")
    assert exc_info.value.status == 400
