"""GET /api/v1/tx/{txid}/rbf

For confirmed transactions both servers return an empty/null replacement
set; the structure is what's load-bearing here.
"""

from _lib import assert_same_structure, show


def test_tx_rbf_for_confirmed(brk, mempool, block):
    """RBF replacement timeline structure must match for a confirmed tx."""
    path = f"/api/v1/tx/{block.txid}/rbf"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_structure(b, m)
