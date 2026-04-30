"""GET /api/v1/block/{hash}"""

from _lib import assert_same_structure, assert_same_values, show


def test_block_v1_extras_all_values(brk, mempool, block):
    """Every shared extras field must match - exposes computation differences.

    Excluded fields:
    - medianFee, feeRange, feePercentiles: mempool computes each entry with
      a different algorithm (1st/99th percentile + first/last 2% of block
      order for the feeRange bounds, unweighted positional p10/p25/p50/p75/p90
      for the inner feeRange entries and for feePercentiles, and a vsize-
      weighted middle-0.25%-of-block-weight slice for medianFee). brk
      computes them all from a single vsize-weighted percentile distribution,
      so they diverge anywhere tx sizes vary widely.
    - avgFeeRate: mempool returns Bitcoin Core's getblockstats.avgfeerate
      (integer sat/vB), brk returns the float version. Same formula, brk
      keeps decimal precision.
    """
    path = f"/api/v1/block/{block.hash}"
    b = brk.get_json(path)["extras"]
    m = mempool.get_json(path)["extras"]
    show("GET", f"{path}  [extras]", b, m, max_lines=50)
    assert_same_structure(b, m)
    assert_same_values(
        b, m, exclude={"medianFee", "feeRange", "feePercentiles", "avgFeeRate"}
    )


def test_block_v1_extras_pool(brk, mempool, block):
    """Pool identification structure must match."""
    path = f"/api/v1/block/{block.hash}"
    bp = brk.get_json(path)["extras"]["pool"]
    mp = mempool.get_json(path)["extras"]["pool"]
    show("GET", f"{path}  [extras.pool]", bp, mp)
    assert_same_structure(bp, mp)
