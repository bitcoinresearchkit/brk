"""GET /api/v1/blocks/{height}"""

from _lib import assert_same_values, show


def test_blocks_v1_from_height(brk, mempool, block):
    """v1 blocks from a confirmed height - all values must match.

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
    path = f"/api/v1/blocks/{block.height}"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, f"({len(b)} blocks)", f"({len(m)} blocks)")
    assert len(b) == len(m)
    if b and m:
        assert_same_values(
            b[0],
            m[0],
            exclude={"medianFee", "feeRange", "feePercentiles", "avgFeeRate"},
        )
