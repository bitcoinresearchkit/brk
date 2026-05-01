"""GET /api/v1/replacements

Returns up to 25 most-recent RBF replacement trees. Both servers may
report an empty list at any moment; the element structure is what's
load-bearing.
"""

from _lib import assert_same_structure, show


def test_replacements_shape(brk, mempool):
    """Replacement-tree structure must match for the first element if both lists are non-empty."""
    path = "/api/v1/replacements"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert isinstance(b, list) and isinstance(m, list)
    assert len(b) <= 25 and len(m) <= 25
    if b and m:
        assert_same_structure(b[0], m[0])
