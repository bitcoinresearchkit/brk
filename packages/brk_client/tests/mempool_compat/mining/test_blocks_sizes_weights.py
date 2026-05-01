"""GET /api/v1/mining/blocks/sizes-weights/{time_period}"""

import pytest

from brk_client import BrkError

from _lib import assert_same_structure, show, summary


PERIODS = ["24h", "3d", "1w", "1m", "3m", "6m", "1y", "2y", "3y", "all"]
MAX_BLOCK_WEIGHT = 4_000_000


@pytest.mark.parametrize("period", PERIODS)
def test_mining_blocks_sizes_weights_structure(brk, mempool, period):
    """Combined sizes/weights envelope must match across all periods."""
    path = f"/api/v1/mining/blocks/sizes-weights/{period}"
    b = brk.get_block_sizes_weights(period)
    m = mempool.get_json(path)
    show("GET", path, summary(b), summary(m))
    assert isinstance(b, dict) and isinstance(m, dict)
    assert_same_structure(b, m)


def test_mining_blocks_sizes_weights_invariants(brk):
    """Parallel arrays, ascending order, positive size, weight in (0, 4M] (period=1m)."""
    period = "1m"
    b = brk.get_block_sizes_weights(period)
    sizes = b["sizes"]
    weights = b["weights"]
    show("GET", f"/api/v1/mining/blocks/sizes-weights/{period}", summary(b), "-")
    assert len(sizes) > 0, "expected non-empty sizes series for 1m"
    assert len(sizes) == len(weights), (
        f"sizes/weights array lengths diverge: {len(sizes)} vs {len(weights)}"
    )
    size_heights = [e["avgHeight"] for e in sizes]
    size_ts = [e["timestamp"] for e in sizes]
    assert size_heights == sorted(size_heights), "size avgHeights not ascending"
    assert size_ts == sorted(size_ts), "size timestamps not ascending"
    assert len(set(size_heights)) == len(size_heights), "duplicate avgHeight in sizes"
    for s, w in zip(sizes, weights):
        assert s["avgHeight"] == w["avgHeight"], (
            f"size/weight height misalignment: {s['avgHeight']} vs {w['avgHeight']}"
        )
        assert s["timestamp"] == w["timestamp"], (
            f"size/weight timestamp misalignment at height {s['avgHeight']}"
        )
        assert s["avgSize"] > 0, f"non-positive avgSize at {s['avgHeight']}: {s['avgSize']}"
        assert 0 < w["avgWeight"] <= MAX_BLOCK_WEIGHT, (
            f"avgWeight out of range at {w['avgHeight']}: {w['avgWeight']}"
        )


@pytest.mark.parametrize("bad", ["9000y", "abc", "1d"])
def test_mining_blocks_sizes_weights_malformed(brk, bad):
    """Unknown time period must produce BrkError(status=400)."""
    with pytest.raises(BrkError) as exc_info:
        brk.get_text(f"/api/v1/mining/blocks/sizes-weights/{bad}")
    assert exc_info.value.status == 400, (
        f"expected status=400 for {bad!r}, got {exc_info.value.status}"
    )
