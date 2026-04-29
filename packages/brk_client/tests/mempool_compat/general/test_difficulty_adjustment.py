"""GET /api/v1/difficulty-adjustment"""

from _lib import assert_same_structure, show


DIFFICULTY_KEYS = [
    "progressPercent", "difficultyChange", "estimatedRetargetDate",
    "remainingBlocks", "remainingTime", "previousRetarget",
    "previousTime", "nextRetargetHeight", "timeAvg",
    "adjustedTimeAvg", "timeOffset", "expectedBlocks",
]


def test_difficulty_adjustment(brk, mempool):
    """Difficulty adjustment must have the same structure."""
    path = "/api/v1/difficulty-adjustment"
    b = brk.get_json(path)
    m = mempool.get_json(path)
    show("GET", path, b, m)
    assert_same_structure(b, m)
    for key in DIFFICULTY_KEYS:
        assert key in b, f"brk missing '{key}'"


def test_difficulty_adjustment_values_sane(brk, mempool):
    """Progress must be 0-100 %, remaining blocks must be 0-2016."""
    path = "/api/v1/difficulty-adjustment"
    for label, client in [("brk", brk), ("mempool", mempool)]:
        d = client.get_json(path)
        assert 0 <= d["progressPercent"] <= 100, (
            f"{label} progressPercent out of range: {d['progressPercent']}"
        )
        assert 0 <= d["remainingBlocks"] <= 2016, (
            f"{label} remainingBlocks out of range: {d['remainingBlocks']}"
        )
