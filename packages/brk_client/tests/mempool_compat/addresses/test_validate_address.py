"""GET /api/v1/validate-address/{address}"""

import pytest

from _lib import assert_same_structure, assert_same_values, show


def test_validate_address_discovered(brk, mempool, live_addrs):
    """Validation of each discovered address type must match exactly."""
    for atype, addr in live_addrs:
        path = f"/api/v1/validate-address/{addr}"
        b = brk.get_json(path)
        m = mempool.get_json(path)
        show("GET", f"{path}  [{atype}]", b, m)
        assert_same_values(b, m)
        assert b["isvalid"] is True


@pytest.mark.parametrize("addr,kind", [
    ("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", "p2pkh-genesis"),
    ("3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy", "p2sh"),
    ("bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4", "p2wpkh"),
    ("bc1p0xlxvlhemja6c4dqv22uapctqupfhlxm9h8z3k2e72q4k9hcz7vqzk5jj0", "p2tr"),
])
def test_validate_address_static_valid(brk, mempool, addr, kind):
    """Well-known addresses across all script types must validate identically."""
    path = f"/api/v1/validate-address/{addr}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", f"{path}  [{kind}]", b, m)
    assert_same_values(b, m)
    assert b["isvalid"] is True


@pytest.mark.parametrize("addr,kind", [
    ("notanaddress123", "garbage"),
    ("", "empty"),
    ("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNb", "bad-checksum-p2pkh"),
    ("bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t5", "bad-checksum-p2wpkh"),
    ("3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLz", "bad-checksum-p2sh"),
])
def test_validate_address_invalid(brk, mempool, addr, kind):
    """Invalid addresses must produce the same rejection structure."""
    path = f"/api/v1/validate-address/{addr}"
    if kind == "empty":
        # An empty path segment routes to a different endpoint — skip.
        pytest.skip("empty address routes to a different endpoint")
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", f"{path}  [{kind}]", b, m)
    assert b["isvalid"] is False
    assert m["isvalid"] is False
    assert_same_structure(b, m)
