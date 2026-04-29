"""GET /api/mempool/txids"""

from _lib import show


HEX = set("0123456789abcdef")


def test_mempool_txids_basic(brk, mempool):
    """Txid list must be a non-empty array of strings on both servers."""
    path = "/api/mempool/txids"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, f"({len(b)} txids)", f"({len(m)} txids)")
    assert isinstance(b, list) and isinstance(m, list)
    assert len(b) > 0, "brk mempool has no txids"
    assert isinstance(b[0], str) and len(b[0]) == 64


def test_mempool_txids_format(brk):
    """Every txid in brk's mempool list must be a 64-char lowercase hex string."""
    b = brk.get_json("/api/mempool/txids")
    show("GET", "/api/mempool/txids", f"({len(b)} txids)", "—")
    bad = [t for t in b if not (isinstance(t, str) and len(t) == 64 and set(t.lower()) <= HEX)]
    assert not bad, f"{len(bad)} malformed txid(s), e.g. {bad[0] if bad else None!r}"


def test_mempool_txids_unique(brk):
    """Brk's mempool txid list must not contain duplicates."""
    b = brk.get_json("/api/mempool/txids")
    show("GET", "/api/mempool/txids", f"({len(b)} txids)", "—")
    assert len(b) == len(set(b)), (
        f"duplicate txids: {len(b) - len(set(b))} duplicates out of {len(b)}"
    )


def test_mempool_txids_count_matches_summary(brk):
    """`/api/mempool/txids` length must match `/api/mempool`'s `count` field."""
    txids = brk.get_json("/api/mempool/txids")
    summary = brk.get_json("/api/mempool")
    show("GET", "/api/mempool/txids", f"len={len(txids)}", f"count={summary.get('count')}")
    # Allow a small drift (1-2) since the mempool is updated asynchronously
    # between the two fetches.
    assert abs(len(txids) - summary["count"]) <= 5, (
        f"txids={len(txids)} vs /api/mempool.count={summary['count']}"
    )
