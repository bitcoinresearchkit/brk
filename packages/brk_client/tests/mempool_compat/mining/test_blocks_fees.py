"""GET /api/v1/mining/blocks/fees/{time_period}"""

import pytest

from brk_client import BrkError

from _lib import assert_same_structure, show, summary


PERIODS = ["24h", "3d", "1w", "1m", "3m", "6m", "1y", "2y", "3y", "all"]


@pytest.mark.parametrize("period", PERIODS)
def test_mining_blocks_fees_structure(brk, mempool, period):
    """Average block fees envelope must match across all periods."""
    path = f"/api/v1/mining/blocks/fees/{period}"
    b = brk.get_block_fees(period)
    m = mempool.get_json(path)
    show("GET", path, summary(b), summary(m))
    assert isinstance(b, list) and isinstance(m, list)
    assert_same_structure(b, m)


def test_mining_blocks_fees_invariants(brk):
    """Series ascending by height and timestamp, fees and USD non-negative (period=1m)."""
    period = "1m"
    b = brk.get_block_fees(period)
    show("GET", f"/api/v1/mining/blocks/fees/{period}", summary(b), "-")
    assert len(b) > 0, "expected non-empty fees series for 1m"
    heights = [entry["avgHeight"] for entry in b]
    timestamps = [entry["timestamp"] for entry in b]
    assert heights == sorted(heights), "avgHeight not ascending"
    assert timestamps == sorted(timestamps), "timestamps not ascending"
    assert len(set(heights)) == len(heights), "duplicate avgHeight in series"
    for entry in b:
        assert entry["avgFees"] >= 0, f"negative avgFees: {entry}"
        assert entry["USD"] >= 0, f"negative USD: {entry}"


@pytest.mark.parametrize("bad", ["9000y", "abc", "1d"])
def test_mining_blocks_fees_malformed(brk, bad):
    """Unknown time period must produce BrkError(status=400)."""
    with pytest.raises(BrkError) as exc_info:
        brk.get_text(f"/api/v1/mining/blocks/fees/{bad}")
    assert exc_info.value.status == 400, (
        f"expected status=400 for {bad!r}, got {exc_info.value.status}"
    )
