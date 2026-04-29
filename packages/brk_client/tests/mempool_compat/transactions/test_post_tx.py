"""POST /api/tx (broadcast)

We can't actually broadcast a real transaction in a test, so we send a
clearly malformed payload and verify both servers reject it with 4xx. The
goal is to confirm the endpoint exists and behaves like a transaction
broadcaster — not to push live transactions.
"""

from _lib import show


def test_post_tx_invalid_hex(brk, mempool):
    """Both servers must reject an obviously invalid hex payload with 4xx."""
    path = "/api/tx"
    bad_hex = "deadbeef"  # too short to be a valid serialized transaction

    b = brk.session.post(f"{brk.base_url}{path}", data=bad_hex, timeout=15)
    mempool._wait()
    m = mempool.session.post(f"{mempool.base_url}{path}", data=bad_hex, timeout=15)
    show("POST", path, f"brk={b.status_code}", f"mempool={m.status_code}")

    assert 400 <= b.status_code < 500, (
        f"brk POST /api/tx with garbage should 4xx, got {b.status_code}: {b.text!r}"
    )
    assert 400 <= m.status_code < 500, (
        f"mempool POST /api/tx with garbage should 4xx, got {m.status_code}: {m.text!r}"
    )


def test_post_tx_empty_body(brk, mempool):
    """Both servers must reject an empty body with 4xx."""
    path = "/api/tx"

    b = brk.session.post(f"{brk.base_url}{path}", data="", timeout=15)
    mempool._wait()
    m = mempool.session.post(f"{mempool.base_url}{path}", data="", timeout=15)
    show("POST", path, f"brk={b.status_code}", f"mempool={m.status_code}")

    assert 400 <= b.status_code < 500
    assert 400 <= m.status_code < 500
