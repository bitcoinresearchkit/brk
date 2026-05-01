"""GET /api/v1/fullrbf/replacements

Like `/api/v1/replacements`, but limited to trees where at least one
predecessor was non-signaling (full-RBF).
"""

from _lib import assert_same_structure, show


def test_fullrbf_replacements_shape(brk, mempool):
    """Full-RBF replacement-tree structure must match for the first element if both lists are non-empty."""
    path = "/api/v1/fullrbf/replacements"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert isinstance(b, list) and isinstance(m, list)
    assert len(b) <= 25 and len(m) <= 25
    if b and m:
        assert_same_structure(b[0], m[0])
