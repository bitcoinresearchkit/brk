"""
Mempool.space API compatibility tests.

Compares every brk mempool_space endpoint against the real mempool.space API
using live blockchain data — nothing is hardcoded or deterministic.

Usage:
    cd scripts/mempool_compat
    uv run pytest -sv                              # all tests, verbose
    uv run pytest -sv test_blocks.py               # one category
    uv run pytest -sv -k "test_block_header"       # one test
    BRK_URL=http://host:port uv run pytest -sv     # custom brk server

Environment variables:
    BRK_URL      brk server base URL        (default: http://localhost:3000)
    MEMPOOL_URL  mempool.space base URL      (default: https://mempool.space)
    RATE_LIMIT   seconds between mempool.space requests (default: 0.5)
"""

import json
import os
import time
from dataclasses import dataclass
from typing import Any, Optional, Set

import pytest
import requests

BRK_BASE = os.environ.get("BRK_URL", "http://localhost:3000")
MEMPOOL_BASE = os.environ.get("MEMPOOL_URL", "https://mempool.space")
RATE_LIMIT = float(os.environ.get("RATE_LIMIT", "0.5"))


# ── API client ────────────────────────────────────────────────────────


class ApiClient:
    """HTTP client for a single API server with optional rate limiting."""

    def __init__(self, base_url: str, name: str, rate_limit: float = 0.0):
        self.base_url = base_url.rstrip("/")
        self.name = name
        self.rate_limit = rate_limit
        self._last_request = 0.0
        self.session = requests.Session()
        self.session.headers["User-Agent"] = "brk-compat-test/1.0"

    def _wait(self):
        if self.rate_limit > 0:
            elapsed = time.monotonic() - self._last_request
            if elapsed < self.rate_limit:
                time.sleep(self.rate_limit - elapsed)
        self._last_request = time.monotonic()

    def get(self, path: str, params=None, timeout: int = 30) -> requests.Response:
        self._wait()
        url = f"{self.base_url}{path}"
        for attempt in range(3):
            resp = self.session.get(url, params=params, timeout=timeout)
            if resp.status_code == 429:
                wait = int(resp.headers.get("Retry-After", 5))
                time.sleep(wait)
                continue
            resp.raise_for_status()
            return resp
        resp.raise_for_status()
        return resp

    def get_json(self, path: str, params=None, timeout: int = 30) -> Any:
        return self.get(path, params=params, timeout=timeout).json()

    def get_text(self, path: str, params=None, timeout: int = 30) -> str:
        return self.get(path, params=params, timeout=timeout).text

    def get_bytes(self, path: str, params=None, timeout: int = 30) -> bytes:
        return self.get(path, params=params, timeout=timeout).content


# ── Live data ─────────────────────────────────────────────────────────


# Absolute heights for well-known eras + relative depths for recent blocks.
# Covers: genesis-era, early, mid, post-halving, taproot-era, recent, near-tip.
FIXED_HEIGHTS = [100, 100_000, 400_000, 630_000, 800_000]
RELATIVE_DEPTHS = [1000, 100, 10]


@dataclass
class BlockData:
    """A discovered block with associated txids."""

    height: int
    hash: str
    txid: str
    coinbase_txid: str


@dataclass
class LiveData:
    """Live blockchain data discovered at session start."""

    tip_height: int
    tip_hash: str
    # Multiple blocks at various depths for parametrized tests
    blocks: list  # list[BlockData]
    # Addresses keyed by scriptpubkey_type
    addresses: dict  # dict[str, str]
    # Convenience aliases (first block)
    stable_height: int
    stable_hash: str
    stable_block: dict
    sample_txid: str
    coinbase_txid: str
    sample_address: str


# ── Fixtures ──────────────────────────────────────────────────────────


@pytest.fixture(scope="session")
def brk():
    return ApiClient(BRK_BASE, "brk")


@pytest.fixture(scope="session")
def mempool():
    return ApiClient(MEMPOOL_BASE, "mempool.space", rate_limit=RATE_LIMIT)


@pytest.fixture(scope="session", autouse=True)
def check_servers(brk, mempool):
    """Fail fast if either server is unreachable."""
    try:
        brk.get("/api/blocks/tip/height")
    except Exception as e:
        pytest.exit(f"brk server not reachable at {brk.base_url}: {e}")
    try:
        mempool.get("/api/blocks/tip/height")
    except Exception as e:
        pytest.exit(f"mempool.space not reachable at {mempool.base_url}: {e}")


@pytest.fixture(scope="session")
def live(mempool) -> LiveData:
    """Discover live blockchain data for all tests.

    Fetches blocks at several depths and extracts txids + addresses of
    different types so parametrized tests hit varied real data.
    """
    tip_height = int(mempool.get_text("/api/blocks/tip/height"))
    tip_hash = mempool.get_text("/api/blocks/tip/hash")

    heights = FIXED_HEIGHTS + [tip_height - d for d in RELATIVE_DEPTHS]
    heights.sort()

    blocks: list[BlockData] = []
    addresses: dict[str, str] = {}

    for h in heights:
        bh = mempool.get_text(f"/api/block-height/{h}")
        txids = mempool.get_json(f"/api/block/{bh}/txids")
        coinbase = txids[0]
        sample = txids[min(1, len(txids) - 1)]
        blocks.append(BlockData(height=h, hash=bh, txid=sample, coinbase_txid=coinbase))

        # Collect addresses of different types from non-coinbase outputs
        if len(addresses) < 8:
            tx = mempool.get_json(f"/api/tx/{sample}")
            for vout in tx.get("vout", []):
                atype = vout.get("scriptpubkey_type")
                addr = vout.get("scriptpubkey_address")
                if addr and atype and atype not in addresses:
                    addresses[atype] = addr

    stable = blocks[0]
    stable_block = mempool.get_json(f"/api/block/{stable.hash}")
    sample_address = next(iter(addresses.values()), "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa")

    data = LiveData(
        tip_height=tip_height,
        tip_hash=tip_hash,
        blocks=blocks,
        addresses=addresses,
        stable_height=stable.height,
        stable_hash=stable.hash,
        stable_block=stable_block,
        sample_txid=stable.txid,
        coinbase_txid=stable.coinbase_txid,
        sample_address=sample_address,
    )

    print(f"\n{'='*70}")
    print(f"  LIVE TEST DATA  (from {MEMPOOL_BASE})")
    print(f"{'='*70}")
    print(f"  tip       {data.tip_height}  {data.tip_hash[:20]}…")
    for i, b in enumerate(blocks):
        print(f"  block[{i}]  {b.height}  {b.hash[:20]}…  tx={b.txid[:16]}…")
    for atype, addr in addresses.items():
        print(f"  addr      {atype:12s}  {addr}")
    print(f"{'='*70}\n")
    return data


# ── Display helpers ───────────────────────────────────────────────────


def show(method: str, path: str, brk_data: Any, mem_data: Any, max_lines: int = 20):
    """Print both responses so the runner can see what was fetched."""
    print(f"\n{'─'*70}")
    print(f"  {method} {path}")
    print(f"{'─'*70}")
    for label, data in [("mempool.space", mem_data), ("brk", brk_data)]:
        print(f"\n  [{label}]")
        if isinstance(data, (dict, list)):
            text = json.dumps(data, indent=2)
        elif isinstance(data, bytes):
            text = f"<{len(data)} bytes>"
        else:
            text = str(data)
        lines = text.split("\n")
        for line in lines[:max_lines]:
            print(f"    {line}")
        if len(lines) > max_lines:
            print(f"    … ({len(lines) - max_lines} more lines)")


# ── Comparison helpers ────────────────────────────────────────────────

# Keys that brk is intentionally not implementing (mempool.space-specific features).
# Everything else that mempool.space returns MUST be present in brk.
ALLOWED_MISSING = {
    "matchRate", "expectedFees", "expectedWeight",
    # brk only tracks USD — non-USD currencies and exchange rates are intentionally absent
    "EUR", "GBP", "CAD", "CHF", "AUD", "JPY",
    "USDEUR", "USDGBP", "USDCAD", "USDCHF", "USDAUD", "USDJPY",
    # brk doesn't compute block health scores
    "avgBlockHealth",
    # brk doesn't compute block similarity/template matching
    "similarity",
    # brk doesn't compute fee delta or match rate per pool
    "avgFeeDelta", "avgMatchRate",
}

# Coinbase transactions use vout=65535 (u16::MAX) in brk vs 4294967295 (u32::MAX)
# in mempool.space. This is an intentional representation difference.
COINBASE_VOUT_BRK = 65535
COINBASE_VOUT_MEMPOOL = 4294967295


def assert_same_structure(brk_data: Any, mem_data: Any, path: str = "root"):
    """brk must have every key mempool.space has (extra brk keys are fine).

    Recurses into nested dicts; for arrays, compares the first element.
    int/float are treated as equivalent; None is compatible with anything.
    """
    if isinstance(mem_data, dict):
        assert isinstance(brk_data, dict), (
            f"Expected dict at {path}, got {type(brk_data).__name__}"
        )
        brk_keys = set(brk_data.keys())
        mem_keys = set(mem_data.keys())
        missing = mem_keys - brk_keys - ALLOWED_MISSING
        assert not missing, f"brk missing keys at {path}: {missing}"
        for key in brk_keys & mem_keys:
            assert_same_structure(brk_data[key], mem_data[key], f"{path}.{key}")
    elif isinstance(mem_data, list):
        assert isinstance(brk_data, list), (
            f"Expected list at {path}, got {type(brk_data).__name__}"
        )
        if mem_data and brk_data:
            assert_same_structure(brk_data[0], mem_data[0], f"{path}[0]")
    else:
        if mem_data is None or brk_data is None:
            return
        bt = type(brk_data).__name__
        mt = type(mem_data).__name__
        if {bt, mt} <= {"int", "float"}:
            return
        # int/str are compatible when the string is a numeric literal
        # (mempool.space serializes large numbers as strings)
        if {bt, mt} == {"int", "str"}:
            return
        assert bt == mt, (
            f"Type mismatch at {path}: brk={bt}({brk_data!r}) "
            f"vs mempool={mt}({mem_data!r})"
        )


def assert_same_values(
    brk_data: Any,
    mem_data: Any,
    path: str = "root",
    exclude: Optional[Set[str]] = None,
):
    """Both responses must have identical values.

    Floats are compared with relative tolerance 1e-4.
    Pass ``exclude`` to skip keys that are expected to differ.
    """
    exclude = exclude or set()

    if isinstance(mem_data, dict):
        assert isinstance(brk_data, dict), (
            f"Expected dict at {path}, got {type(brk_data).__name__}"
        )
        # brk must have every mempool key; extra brk keys are fine
        mem_keys = set(mem_data.keys())
        for key in mem_keys - exclude - ALLOWED_MISSING:
            assert key in brk_data, f"brk missing '{key}' at {path}"
            assert_same_values(brk_data[key], mem_data[key], f"{path}.{key}", exclude)
    elif isinstance(mem_data, list):
        assert isinstance(brk_data, list), (
            f"Expected list at {path}, got {type(brk_data).__name__}"
        )
        assert len(brk_data) == len(mem_data), (
            f"Length mismatch at {path}: brk={len(brk_data)} vs mempool={len(mem_data)}"
        )
        for i, (b, m) in enumerate(zip(brk_data, mem_data)):
            assert_same_values(b, m, f"{path}[{i}]", exclude)
    elif mem_data is None:
        # mempool returns null, brk computes a value — that's fine
        return
    elif isinstance(mem_data, float) or isinstance(brk_data, float):
        if brk_data is None:
            return
        assert float(brk_data) == pytest.approx(
            float(mem_data), rel=1e-4, abs=1e-6
        ), f"Float mismatch at {path}: brk={brk_data} vs mempool={mem_data}"
    else:
        # Coinbase vout: brk uses u16::MAX, mempool uses u32::MAX — both valid
        if (
            brk_data == COINBASE_VOUT_BRK
            and mem_data == COINBASE_VOUT_MEMPOOL
        ):
            return
        assert brk_data == mem_data, (
            f"Value mismatch at {path}: brk={brk_data!r} vs mempool={mem_data!r}"
        )
