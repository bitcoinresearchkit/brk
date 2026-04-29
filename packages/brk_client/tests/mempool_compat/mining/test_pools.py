"""GET /api/v1/mining/pools"""

from _lib import assert_same_structure, show


def test_mining_pools_list_structure(brk, mempool):
    """Pool list must have the same element structure."""
    path = "/api/v1/mining/pools"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show(
        "GET", path,
        b[:3] if isinstance(b, list) else b,
        m[:3] if isinstance(m, list) else m,
    )
    assert_same_structure(b, m)


def _pools(data):
    """`pools` may live at the root or inside an envelope across versions."""
    if isinstance(data, list):
        return data
    return data.get("pools", []) if isinstance(data, dict) else []


def test_mining_pools_list_fields(brk):
    """Each pool entry must carry slug and name (period-less endpoint omits stats)."""
    b = _pools(brk.get_json("/api/v1/mining/pools"))
    show("GET", "/api/v1/mining/pools", f"({len(b)} pools)", "—")
    assert b, "no pools in brk's response"
    required = {"slug", "name"}
    for p in b[:5]:
        missing = required - set(p.keys())
        assert not missing, f"pool {p.get('slug', '?')} missing fields: {missing}"
        assert isinstance(p["name"], str) and p["name"]


def test_mining_pools_slugs_unique(brk):
    """Pool slugs must be unique across the response."""
    b = _pools(brk.get_json("/api/v1/mining/pools"))
    slugs = [p["slug"] for p in b]
    show("GET", "/api/v1/mining/pools", f"({len(slugs)} slugs)", "—")
    assert len(slugs) == len(set(slugs)), (
        f"duplicate slugs: {len(slugs) - len(set(slugs))}"
    )
