#!/usr/bin/env python3
"""
Analyze ALL outputs to find characteristics that correlate with accurate price signals.
Uses txoutindex directly - no pre-filtering.
"""

import urllib.request
import http.client
import json
import math
import bisect
from collections import defaultdict
from dataclasses import dataclass, field
from typing import Optional
import sys
import time

BASE_URL = "http://localhost:3110"
API_HOST = "localhost"
API_PORT = 3110

# Persistent connection
_conn = None

def get_conn():
    global _conn
    if _conn is None:
        _conn = http.client.HTTPConnection(API_HOST, API_PORT, timeout=300)
    return _conn

def reset_conn():
    global _conn
    if _conn:
        try:
            _conn.close()
        except:
            pass
    _conn = None

# Monthly prices for 2017-2018 (CoinGecko open prices)
MONTHLY_PRICES = {
    (2017, 1): 970, (2017, 2): 968, (2017, 3): 1190, (2017, 4): 1080,
    (2017, 5): 1362, (2017, 6): 2299, (2017, 7): 2455, (2017, 8): 2865,
    (2017, 9): 4738, (2017, 10): 4334, (2017, 11): 6440, (2017, 12): 9968,
    (2018, 1): 13888, (2018, 2): 10116, (2018, 3): 10307, (2018, 4): 6922,
    (2018, 5): 9243, (2018, 6): 7487, (2018, 7): 6386, (2018, 8): 7726,
    (2018, 9): 7016, (2018, 10): 6566, (2018, 11): 6305, (2018, 12): 3972,
}

def fetch(path: str, retries: int = 5):
    """Fetch JSON from API with retry logic and connection reuse."""
    for attempt in range(retries):
        try:
            conn = get_conn()
            conn.request("GET", path)
            resp = conn.getresponse()
            data = resp.read().decode('utf-8')
            return json.loads(data)
        except Exception as e:
            reset_conn()  # Reset connection on error
            if attempt < retries - 1:
                wait_time = (attempt + 1) * 3  # 3, 6, 9, 12 seconds
                print(f"  Retry {attempt + 1}/{retries} after {wait_time}s: {type(e).__name__}")
                time.sleep(wait_time)
            else:
                raise

def fetch_chunked(path_template: str, start: int, end: int, chunk_size: int = 25000) -> list:
    """Fetch data in chunks to avoid API limits."""
    result = []
    total_chunks = (end - start + chunk_size - 1) // chunk_size
    for i, chunk_start in enumerate(range(start, end, chunk_size)):
        chunk_end = min(chunk_start + chunk_size, end)
        path = path_template.format(start=chunk_start, end=chunk_end)
        if i % 20 == 0 and i > 0:
            print(f"    chunk {i}/{total_chunks}...")
        data = fetch(path)["data"]
        result.extend(data)
    return result

def get_phase_bin_and_decade(sats: int) -> tuple:
    """Get (phase_bin, decade) for sats value. Returns (None, 0) if out of range."""
    if sats < 1000 or sats > 10_000_000_000_000:
        return None, 0
    log_sats = math.log10(sats)
    decade = int(math.floor(log_sats))
    phase = log_sats - decade
    return min(int(phase * 100), 99), decade

def get_phase_bin(sats: int) -> Optional[int]:
    """Get phase bin for sats value (0-99), or None if out of range."""
    return get_phase_bin_and_decade(sats)[0]

def get_decade(sats: int) -> int:
    """Get the decade (power of 10) for sats value."""
    if sats <= 0:
        return 0
    return int(math.floor(math.log10(sats)))

def sats_to_implied_btc_price(sats: int, btc_price: float) -> float:
    """
    What BTC price does this output imply?
    If someone paid X sats for $Y worth of goods, and BTC = btc_price,
    then the implied price = (sats / 1e8) * btc_price.
    But we don't know Y. So instead, assume the output represents ~$btc_price worth,
    and see what price that implies: implied_price = btc_price * (1e8 / sats).
    """
    if sats <= 0:
        return 0
    return btc_price * (100_000_000 / sats)

def bin_to_price(bin_idx: int, anchor: float) -> float:
    """Convert bin to price using anchor for decade selection."""
    EXPONENT = 5.0
    phase = (bin_idx + 0.5) / 100
    raw_price = 10 ** (EXPONENT - phase)
    decade_ratio = round(math.log10(anchor / raw_price))
    return raw_price * (10 ** decade_ratio)

def build_bin_classifier(btc_price: float) -> dict:
    """
    Precompute classification for each bin (0-99) at given BTC price.
    Returns dict mapping bin -> category.
    """
    EXPONENT = 5.0
    result = {}

    for bin_idx in range(100):
        phase = (bin_idx + 0.5) / 100
        raw_price = 10 ** (EXPONENT - phase)

        best_error = float('inf')
        best_decade = 0

        for decade in range(-4, 5):
            price = raw_price * (10 ** decade)
            error = abs(price - btc_price) / btc_price
            if error < best_error:
                best_error = error
                best_decade = decade

        anchor_decade = round(math.log10(btc_price / raw_price))

        if best_error <= 0.15:
            result[bin_idx] = "accurate"
        elif best_error <= 0.30:
            result[bin_idx] = "close"
        elif anchor_decade != best_decade:
            result[bin_idx] = "wrong_decade"
        else:
            result[bin_idx] = "noise"

    return result

def build_bin_classifier_range(low_price: float, high_price: float) -> dict:
    """
    Precompute classification for each bin (0-99) given daily low-high range.
    An output is "accurate" if its implied price falls within the daily range.
    """
    EXPONENT = 5.0
    result = {}
    mid_price = (low_price + high_price) / 2

    for bin_idx in range(100):
        phase = (bin_idx + 0.5) / 100
        raw_price = 10 ** (EXPONENT - phase)

        # Find the best decade match using mid price as anchor
        best_error_vs_mid = float('inf')
        best_decade = 0

        for decade in range(-4, 5):
            price = raw_price * (10 ** decade)
            error = abs(price - mid_price) / mid_price
            if error < best_error_vs_mid:
                best_error_vs_mid = error
                best_decade = decade

        # Get the implied price at best decade
        implied_price = raw_price * (10 ** best_decade)

        # Check if implied price falls within the daily range (with tolerance)
        # ±5% for accurate (range already captures intraday variation)
        # ±15% for close
        range_low = low_price * 0.95
        range_high = high_price * 1.05

        anchor_decade = round(math.log10(mid_price / raw_price))

        if range_low <= implied_price <= range_high:
            result[bin_idx] = "accurate"
        elif low_price * 0.85 <= implied_price <= high_price * 1.15:
            result[bin_idx] = "close"
        elif anchor_decade != best_decade:
            result[bin_idx] = "wrong_decade"
        else:
            result[bin_idx] = "noise"

    return result

def classify_accuracy_fast(bin_idx: Optional[int], classifier: dict) -> str:
    """Fast classification using precomputed bin classifier."""
    if bin_idx is None:
        return "noise"
    return classifier.get(bin_idx, "noise")

# Precompute round BTC values as a set with tolerance ranges
_ROUND_VALUES = [1000, 10000, 20000, 30000, 50000, 100000, 200000, 300000, 500000,
                 1000000, 2000000, 3000000, 5000000, 10000000, 20000000, 30000000,
                 50000000, 100000000, 1000000000]
# Build set of all "close enough" values (within 0.1%)
_ROUND_SET = set()
for r in _ROUND_VALUES:
    tol = int(r * 0.001)
    for v in range(r - tol, r + tol + 1):
        _ROUND_SET.add(v)

def is_round_btc(sats: int) -> bool:
    """Check if sats is a round BTC amount (within 0.1%)."""
    return sats in _ROUND_SET

# Round USD values to check (in cents to avoid float issues) - sorted for binary search
_ROUND_USD_VALUES = [100, 500, 1000, 2000, 2500, 5000, 10000, 20000, 25000, 50000,
                     100000, 200000, 250000, 500000, 1000000, 2000000, 2500000,
                     5000000, 10000000]  # $1 to $100,000

def is_round_usd(sats: int, btc_low: float, btc_high: float, tolerance: float = 0.05) -> bool:
    """Check if implied USD value is close to a round amount at any price in range."""
    if sats <= 0 or btc_low <= 0 or btc_high <= 0:
        return False
    # Calculate implied USD range (low price = low USD, high price = high USD)
    implied_usd_low = int(sats * btc_low / 1_000_000)  # cents
    implied_usd_high = int(sats * btc_high / 1_000_000)  # cents

    # Check if any round USD value falls within (or near) the implied range
    for round_val in _ROUND_USD_VALUES:
        # The implied USD at some point during the day could have been round_val
        # if round_val is within [implied_low * (1-tol), implied_high * (1+tol)]
        range_low = implied_usd_low * (1 - tolerance)
        range_high = implied_usd_high * (1 + tolerance)
        if range_low <= round_val <= range_high:
            return True
    return False

# Micro-round sats: specific round values with 0.01% tolerance (UTXOracle style)
# These are values like 50000, 100000, 200000, etc. that aren't caught by is_round_btc
_MICRO_ROUND_SATS = []
# 50k-100k range (step 10k)
for v in range(50000, 100000, 10000):
    _MICRO_ROUND_SATS.append(v)
# 100k-1M range (step 10k)
for v in range(100000, 1000000, 10000):
    _MICRO_ROUND_SATS.append(v)
# 1M-10M range (step 100k)
for v in range(1000000, 10000000, 100000):
    _MICRO_ROUND_SATS.append(v)
# 10M-100M range (step 1M)
for v in range(10000000, 100000000, 1000000):
    _MICRO_ROUND_SATS.append(v)
_MICRO_ROUND_SATS = sorted(set(_MICRO_ROUND_SATS))

def is_micro_round_sats(sats: int, tolerance: float = 0.0001) -> bool:
    """Check if sats is a micro-round amount (within 0.01% of specific values)."""
    if sats <= 0:
        return False
    # Binary search for nearest
    idx = bisect.bisect_left(_MICRO_ROUND_SATS, sats)
    for i in [idx - 1, idx]:
        if 0 <= i < len(_MICRO_ROUND_SATS):
            round_val = _MICRO_ROUND_SATS[i]
            if abs(sats - round_val) <= round_val * tolerance:
                return True
    return False

# Phase bins where round USD amounts cluster (0-199 at 200 bins/decade)
# Calculated as: bin = int(frac(log10(usd_cents)) * 200)
# This is price-independent - works regardless of BTC price level!
ROUND_USD_PHASE_BINS_200 = [
    0,    # $1, $10, $100, $1000 (log10 = 0, 1, 2, 3)
    35,   # $1.50, $15, $150 (log10 = 0.176)
    60,   # $2, $20, $200 (log10 = 0.301)
    80,   # $2.50, $25, $250 (log10 = 0.398)
    95,   # $3, $30, $300 (log10 = 0.477)
    120,  # $4, $40, $400 (log10 = 0.602)
    140,  # $5, $50, $500 (log10 = 0.699)
    156,  # $6, $60, $600 (log10 = 0.778)
    169,  # $7, $70, $700 (log10 = 0.845)
    181,  # $8, $80, $800 (log10 = 0.903)
    191,  # $9, $90, $900 (log10 = 0.954)
]

def is_round_usd_phase(sats: int, tolerance_bins: int) -> bool:
    """
    Check if sats falls into a round-USD phase bin. NO PRICE NEEDED!

    Uses 200 bins/decade resolution.
    Tolerance in bins: 2 bins = 1%, 4 bins = 2%, 10 bins = 5%, 20 bins = 10%
    """
    if sats < 1000:  # Skip very small values
        return False

    phase = math.log10(sats) % 1.0  # fractional part (0.0 to 1.0)
    bin_idx = int(phase * 200)  # convert to bin (0 to 199)

    for round_bin in ROUND_USD_PHASE_BINS_200:
        diff = abs(bin_idx - round_bin)
        # Handle wraparound (bin 199 is close to bin 0)
        if diff <= tolerance_bins or (200 - diff) <= tolerance_bins:
            return True
    return False

def get_tx_pattern(input_count: int, output_count: int) -> str:
    """Categorize transaction by input/output pattern."""
    if input_count == 1:
        if output_count == 1:
            return "1-to-1"
        elif output_count == 2:
            return "1-to-2"
        else:
            return "1-to-many"
    elif input_count == 2:
        if output_count == 1:
            return "2-to-1"
        elif output_count == 2:
            return "2-to-2"
        else:
            return "2-to-many"
    else:  # many inputs (3+)
        if output_count == 1:
            return "many-to-1"
        elif output_count == 2:
            return "many-to-2"
        else:
            return "many-to-many"

@dataclass
class Stats:
    """Aggregated statistics."""
    total: int = 0
    by_output_count: dict = field(default_factory=lambda: defaultdict(int))
    by_input_count: dict = field(default_factory=lambda: defaultdict(int))
    by_output_type: dict = field(default_factory=lambda: defaultdict(int))
    by_is_round: dict = field(default_factory=lambda: defaultdict(int))
    by_same_day: dict = field(default_factory=lambda: defaultdict(int))
    by_has_opreturn: dict = field(default_factory=lambda: defaultdict(int))
    by_witness_size: dict = field(default_factory=lambda: defaultdict(int))
    by_value_range: dict = field(default_factory=lambda: defaultdict(int))
    by_both_round: dict = field(default_factory=lambda: defaultdict(int))
    by_bin: dict = field(default_factory=lambda: defaultdict(int))
    by_decade: dict = field(default_factory=lambda: defaultdict(int))
    by_implied_usd_range: dict = field(default_factory=lambda: defaultdict(int))
    # New: output position analysis (for 2-output txs only)
    by_output_index: dict = field(default_factory=lambda: defaultdict(int))
    by_is_smaller_output: dict = field(default_factory=lambda: defaultdict(int))
    by_round_pattern: dict = field(default_factory=lambda: defaultdict(int))  # "only_this", "only_other", "both", "neither"
    by_value_ratio: dict = field(default_factory=lambda: defaultdict(int))  # ratio of this output to total (for 2-out)
    by_error_pct: dict = field(default_factory=lambda: defaultdict(int))  # how close to actual price
    by_tx_total_value: dict = field(default_factory=lambda: defaultdict(int))  # total output value of tx
    by_round_usd_10pct: dict = field(default_factory=lambda: defaultdict(int)) # 10% tolerance (price-based)
    by_round_usd_5pct: dict = field(default_factory=lambda: defaultdict(int))  # 5% tolerance (price-based)
    by_round_usd_2pct: dict = field(default_factory=lambda: defaultdict(int))  # 2% tolerance (price-based)
    by_round_usd_1pct: dict = field(default_factory=lambda: defaultdict(int))  # 1% tolerance (price-based)
    # Phase-based round USD (NO PRICE NEEDED) at different tolerances
    by_phase_usd_1pct: dict = field(default_factory=lambda: defaultdict(int))  # 1% = ±2 bins
    by_phase_usd_2pct: dict = field(default_factory=lambda: defaultdict(int))  # 2% = ±4 bins
    by_phase_usd_5pct: dict = field(default_factory=lambda: defaultdict(int))  # 5% = ±10 bins
    by_phase_usd_10pct: dict = field(default_factory=lambda: defaultdict(int)) # 10% = ±20 bins
    by_tx_pattern: dict = field(default_factory=lambda: defaultdict(int))  # input->output pattern (1-to-2, many-to-1, etc)
    by_value_similarity: dict = field(default_factory=lambda: defaultdict(int))  # how similar are 2-out values (for detecting splits)
    by_is_micro_round: dict = field(default_factory=lambda: defaultdict(int))  # very specific round sat amounts (UTXOracle style)

    def record(self, output_count, input_count, output_type, is_round,
               same_day, has_opreturn, witness_size, sats, both_round, bin_idx,
               btc_price, decade, output_index=None, is_smaller=None, round_pattern=None, value_ratio=None, error_pct=None, tx_total_sats=None,
               round_usd_10pct=None, round_usd_5pct=None, round_usd_2pct=None, round_usd_1pct=None, tx_pattern=None, value_similarity=None, is_micro_round=None,
               phase_usd_1pct=None, phase_usd_2pct=None, phase_usd_5pct=None, phase_usd_10pct=None):
        self.total += 1
        self.by_output_count[min(output_count, 5)] += 1
        self.by_input_count[min(input_count, 5)] += 1
        self.by_output_type[output_type] += 1
        self.by_is_round[is_round] += 1
        self.by_same_day[same_day] += 1
        self.by_has_opreturn[has_opreturn] += 1
        self.by_both_round[both_round] += 1
        if bin_idx is not None:
            self.by_bin[bin_idx] += 1

        # Track decade (power of 10 of sats)
        self.by_decade[decade] += 1

        # Track output position (for 2-output txs)
        if output_index is not None:
            self.by_output_index[output_index] += 1
        if is_smaller is not None:
            self.by_is_smaller_output[is_smaller] += 1
        if round_pattern is not None:
            self.by_round_pattern[round_pattern] += 1
        if value_ratio is not None:
            self.by_value_ratio[value_ratio] += 1
        if error_pct is not None:
            if error_pct < 5:
                self.by_error_pct["<5%"] += 1
            elif error_pct < 10:
                self.by_error_pct["5-10%"] += 1
            elif error_pct < 15:
                self.by_error_pct["10-15%"] += 1
            elif error_pct < 25:
                self.by_error_pct["15-25%"] += 1
            elif error_pct < 50:
                self.by_error_pct["25-50%"] += 1
            else:
                self.by_error_pct["50%+"] += 1
        if tx_total_sats is not None and tx_total_sats > 0:
            # Bucket by tx total value (in BTC terms)
            if tx_total_sats < 1_000_000:  # < 0.01 BTC
                self.by_tx_total_value["<0.01 BTC"] += 1
            elif tx_total_sats < 10_000_000:  # < 0.1 BTC
                self.by_tx_total_value["0.01-0.1 BTC"] += 1
            elif tx_total_sats < 100_000_000:  # < 1 BTC
                self.by_tx_total_value["0.1-1 BTC"] += 1
            elif tx_total_sats < 1_000_000_000:  # < 10 BTC
                self.by_tx_total_value["1-10 BTC"] += 1
            else:
                self.by_tx_total_value["10+ BTC"] += 1

        # Track round USD at different tolerances
        if round_usd_10pct is not None:
            self.by_round_usd_10pct[round_usd_10pct] += 1
        if round_usd_5pct is not None:
            self.by_round_usd_5pct[round_usd_5pct] += 1
        if round_usd_2pct is not None:
            self.by_round_usd_2pct[round_usd_2pct] += 1
        if round_usd_1pct is not None:
            self.by_round_usd_1pct[round_usd_1pct] += 1

        # Track transaction pattern (input-to-output pattern)
        if tx_pattern is not None:
            self.by_tx_pattern[tx_pattern] += 1

        # Track value similarity (for 2-output txs)
        if value_similarity is not None:
            self.by_value_similarity[value_similarity] += 1

        # Track micro-round sats
        if is_micro_round is not None:
            self.by_is_micro_round[is_micro_round] += 1

        # Track phase-based round USD (no price needed)
        if phase_usd_1pct is not None:
            self.by_phase_usd_1pct[phase_usd_1pct] += 1
        if phase_usd_2pct is not None:
            self.by_phase_usd_2pct[phase_usd_2pct] += 1
        if phase_usd_5pct is not None:
            self.by_phase_usd_5pct[phase_usd_5pct] += 1
        if phase_usd_10pct is not None:
            self.by_phase_usd_10pct[phase_usd_10pct] += 1

        # Track implied USD value (sats * btc_price / 1e8)
        implied_usd = sats * btc_price / 100_000_000
        if implied_usd < 1:
            self.by_implied_usd_range["<$1"] += 1
        elif implied_usd < 10:
            self.by_implied_usd_range["$1-$10"] += 1
        elif implied_usd < 100:
            self.by_implied_usd_range["$10-$100"] += 1
        elif implied_usd < 1000:
            self.by_implied_usd_range["$100-$1k"] += 1
        elif implied_usd < 10000:
            self.by_implied_usd_range["$1k-$10k"] += 1
        else:
            self.by_implied_usd_range["$10k+"] += 1

        # Witness size buckets
        if witness_size == 0:
            self.by_witness_size["0"] += 1
        elif witness_size < 500:
            self.by_witness_size["1-499"] += 1
        elif witness_size < 1000:
            self.by_witness_size["500-999"] += 1
        elif witness_size < 2500:
            self.by_witness_size["1000-2499"] += 1
        else:
            self.by_witness_size["2500+"] += 1

        # Value ranges (sats)
        if sats < 10000:
            self.by_value_range["<10k"] += 1
        elif sats < 100000:
            self.by_value_range["10k-100k"] += 1
        elif sats < 1000000:
            self.by_value_range["100k-1M"] += 1
        elif sats < 10000000:
            self.by_value_range["1M-10M"] += 1
        elif sats < 100000000:
            self.by_value_range["10M-100M"] += 1
        else:
            self.by_value_range["100M+"] += 1

def print_stats(stats: Stats, label: str, log=print):
    """Print statistics with percentages."""
    log(f"\n{'='*50}")
    log(f"{label} (n={stats.total:,})")
    log('='*50)

    if stats.total == 0:
        log("No data")
        return

    def pct(d):
        return {k: f"{v:,} ({100*v/stats.total:.1f}%)" for k, v in sorted(d.items())}

    log(f"\nOutput count: {pct(stats.by_output_count)}")
    log(f"Input count: {pct(stats.by_input_count)}")
    log(f"Output type: {pct(stats.by_output_type)}")
    log(f"Is round BTC: {pct(stats.by_is_round)}")
    log(f"Both outputs round: {pct(stats.by_both_round)}")
    log(f"Same-day spend: {pct(stats.by_same_day)}")
    log(f"Has OP_RETURN: {pct(stats.by_has_opreturn)}")
    log(f"Witness size: {pct(stats.by_witness_size)}")
    log(f"Value range (sats): {pct(stats.by_value_range)}")
    log(f"Decade (10^N sats): {pct(stats.by_decade)}")
    log(f"Implied USD value: {pct(stats.by_implied_usd_range)}")
    if stats.by_output_index:
        log(f"Output index (2-out only): {pct(stats.by_output_index)}")
    if stats.by_is_smaller_output:
        log(f"Is smaller output (2-out): {pct(stats.by_is_smaller_output)}")
    if stats.by_round_pattern:
        log(f"Round pattern (2-out): {pct(stats.by_round_pattern)}")
    if stats.by_value_ratio:
        log(f"Value ratio (2-out): {pct(stats.by_value_ratio)}")
    if stats.by_error_pct:
        log(f"Error from actual price: {pct(stats.by_error_pct)}")
    if stats.by_tx_total_value:
        log(f"Tx total value: {pct(stats.by_tx_total_value)}")
    if stats.by_round_usd_10pct:
        log(f"Round USD (10% tol): {pct(stats.by_round_usd_10pct)}")
    if stats.by_round_usd_5pct:
        log(f"Round USD (5% tol): {pct(stats.by_round_usd_5pct)}")
    if stats.by_round_usd_2pct:
        log(f"Round USD (2% tol): {pct(stats.by_round_usd_2pct)}")
    if stats.by_round_usd_1pct:
        log(f"Round USD (1% tol): {pct(stats.by_round_usd_1pct)}")
    if stats.by_tx_pattern:
        log(f"Tx pattern: {pct(stats.by_tx_pattern)}")
    if stats.by_value_similarity:
        log(f"Value similarity (2-out): {pct(stats.by_value_similarity)}")
    if stats.by_is_micro_round:
        log(f"Micro-round sats: {pct(stats.by_is_micro_round)}")
    if stats.by_phase_usd_1pct:
        log(f"Phase USD (1% tol): {pct(stats.by_phase_usd_1pct)}")
    if stats.by_phase_usd_2pct:
        log(f"Phase USD (2% tol): {pct(stats.by_phase_usd_2pct)}")
    if stats.by_phase_usd_5pct:
        log(f"Phase USD (5% tol): {pct(stats.by_phase_usd_5pct)}")
    if stats.by_phase_usd_10pct:
        log(f"Phase USD (10% tol): {pct(stats.by_phase_usd_10pct)}")

    # Top bins
    log(f"\nTop 10 bins:")
    for bin_idx, count in sorted(stats.by_bin.items(), key=lambda x: -x[1])[:10]:
        log(f"  Bin {bin_idx}: {count:,} ({100*count/stats.total:.1f}%)")

def analyze_block_range(start_height: int, end_height: int, start_dateindex: int, end_dateindex: int):
    """Analyze all outputs in a block range using daily OHLC prices."""
    print(f"\nFetching data for heights {start_height}-{end_height}...")

    # Fetch daily OHLC prices for this date range (external price data)
    print("Fetching daily OHLC prices...")
    ohlc_data = fetch(f"/api/metric/price_ohlc/dateindex?start={start_dateindex}&end={end_dateindex}")["data"]
    # OHLC format: [open, high, low, close] in dollars
    # Store low, high, and mid price for each day (transactions happen throughout the day)
    daily_prices = {}  # dateindex -> (low, high, mid)
    for i, ohlc in enumerate(ohlc_data):
        if ohlc and len(ohlc) >= 4:
            open_p, high, low, close = ohlc[0], ohlc[1], ohlc[2], ohlc[3]
            mid = (open_p + close) / 2  # average of open/close, not low/high (avoid wick skew)
            daily_prices[start_dateindex + i] = (low, high, mid)
    all_lows = [p[0] for p in daily_prices.values()]
    all_highs = [p[1] for p in daily_prices.values()]
    print(f"  Got prices for {len(daily_prices)} days (${min(all_lows):.0f} - ${max(all_highs):.0f})")

    # Precompute bin classifiers for each unique price (cache for speed)
    bin_classifier_cache = {}
    def get_bin_classifier(low: float, high: float) -> dict:
        """Build classifier that checks if bin falls within daily low-high range."""
        # Cache key is the rounded range
        cache_key = (round(low / 10) * 10, round(high / 10) * 10)
        if cache_key not in bin_classifier_cache:
            bin_classifier_cache[cache_key] = build_bin_classifier_range(low, high)
        return bin_classifier_cache[cache_key]

    # Get transaction ranges
    first_tx = fetch(f"/api/metric/first_txindex/height?start={start_height}&end={end_height+1}")
    first_txs = first_tx["data"]
    tx_start = first_txs[0]
    tx_end = first_txs[-1] if len(first_txs) > 1 else tx_start + 10000

    print(f"Transaction range: {tx_start}-{tx_end} ({tx_end-tx_start:,} txs)")

    # Get transaction metadata (chunked for large ranges)
    print("Fetching transaction data...")
    tx_first_out = fetch_chunked("/api/metric/first_txoutindex/txindex?start={start}&end={end}", tx_start, tx_end)
    tx_first_in = fetch_chunked("/api/metric/first_txinindex/txindex?start={start}&end={end}", tx_start, tx_end)
    tx_base_size = fetch_chunked("/api/metric/base_size/txindex?start={start}&end={end}", tx_start, tx_end)
    tx_total_size = fetch_chunked("/api/metric/total_size/txindex?start={start}&end={end}", tx_start, tx_end)
    tx_output_count = fetch_chunked("/api/metric/output_count/txindex?start={start}&end={end}", tx_start, tx_end)
    tx_input_count = fetch_chunked("/api/metric/input_count/txindex?start={start}&end={end}", tx_start, tx_end)
    tx_height = fetch_chunked("/api/metric/height/txindex?start={start}&end={end}", tx_start, tx_end)

    # Get output data
    out_start = tx_first_out[0] if tx_first_out else 0
    # Estimate out_end based on last tx's output count
    last_tx_outputs = tx_output_count[-1] if tx_output_count else 10
    out_end = tx_first_out[-1] + last_tx_outputs + 1 if tx_first_out else out_start + 10000

    print(f"Output range: {out_start}-{out_end} ({out_end-out_start:,} outputs)")
    print("Fetching output data...")
    out_value = fetch_chunked("/api/metric/value/txoutindex?start={start}&end={end}", out_start, out_end)
    out_type = fetch_chunked("/api/metric/outputtype/txoutindex?start={start}&end={end}", out_start, out_end)

    # Get input data for same-day check
    in_start = tx_first_in[0] if tx_first_in else 0
    last_tx_inputs = tx_input_count[-1] if tx_input_count else 10
    in_end = tx_first_in[-1] + last_tx_inputs + 1 if tx_first_in else in_start + 10000

    print(f"Input range: {in_start}-{in_end} ({in_end-in_start:,} inputs)")
    print("Fetching input data...")
    # Get spent txoutindex for each input
    in_spent_txoutindex = fetch_chunked("/api/metric/txoutindex/txinindex?start={start}&end={end}", in_start, in_end)

    # For same-day spend detection, only check outputs created within our block range
    # (outputs from before can't be same-day by definition)
    # We'll use the output ranges we already have (out_start to out_end)

    # Get height to dateindex for same-day check
    height_dateindex = fetch_chunked("/api/metric/dateindex/height?start={start}&end={end}", start_height, end_height+1)

    # Analyze each transaction
    # Categories: accurate (<=15%), close (15-30%), wrong_decade, noise
    stats_accurate = Stats()
    stats_close = Stats()
    stats_wrong_decade = Stats()
    stats_noise = Stats()

    num_txs = len(tx_first_out) - 1
    print(f"Analyzing {num_txs:,} transactions...")
    for i in range(num_txs):
        if i % 100000 == 0 and i > 0:
            print(f"  Progress: {i:,}/{num_txs:,} ({100*i/num_txs:.0f}%)")
        txindex = tx_start + i

        out_count = tx_output_count[i] if i < len(tx_output_count) else 0
        in_count = tx_input_count[i] if i < len(tx_input_count) else 0
        base_size = tx_base_size[i] if i < len(tx_base_size) else 0
        total_size = tx_total_size[i] if i < len(tx_total_size) else 0
        witness_size = (total_size or 0) - (base_size or 0)

        first_out = tx_first_out[i] - out_start
        next_first_out = tx_first_out[i + 1] - out_start if i + 1 < len(tx_first_out) else first_out + out_count

        first_in = tx_first_in[i] - in_start if i < len(tx_first_in) else 0

        # Check for OP_RETURN
        has_opreturn = False
        output_types = []
        output_values = []
        for j in range(first_out, min(next_first_out, len(out_type))):
            ot = out_type[j] if j < len(out_type) else "unknown"
            output_types.append(ot)
            if ot and ot.lower() == "opreturn":
                has_opreturn = True
            if j < len(out_value):
                output_values.append(out_value[j])

        # Get the daily price range for this transaction
        tx_height_val = tx_height[i] if i < len(tx_height) else None
        tx_dateindex_val = None
        btc_price = None  # mid price for round USD calculations
        btc_low = None
        btc_high = None
        bin_classifier = None
        if tx_height_val is not None:
            tx_di_idx = tx_height_val - start_height
            tx_dateindex_val = height_dateindex[tx_di_idx] if 0 <= tx_di_idx < len(height_dateindex) else None
            if tx_dateindex_val is not None and tx_dateindex_val in daily_prices:
                btc_low, btc_high, btc_price = daily_prices[tx_dateindex_val]
                bin_classifier = get_bin_classifier(btc_low, btc_high)

        # Skip if no price data for this day
        if btc_price is None or bin_classifier is None:
            continue

        # Check same-day spend (was the spent output created within our analysis range?)
        # Only outputs created within our range (out_start to out_end) can be same-day
        same_day = False
        if tx_dateindex_val is not None and in_count and in_count > 0:
                for k in range(in_count):
                    in_idx = first_in + k
                    if 0 <= in_idx < len(in_spent_txoutindex):
                        spent_txoutindex = in_spent_txoutindex[in_idx]
                        if spent_txoutindex and spent_txoutindex < 2**62:
                            # Check if spent output is within our current range
                            if out_start <= spent_txoutindex < out_end:
                                # Binary search for the tx that contains this output
                                ti = bisect.bisect_right(tx_first_out, spent_txoutindex) - 1
                                if 0 <= ti < len(tx_height):
                                    spent_tx_height = tx_height[ti]
                                    if spent_tx_height:
                                        spent_di_idx = spent_tx_height - start_height
                                        if 0 <= spent_di_idx < len(height_dateindex):
                                            if height_dateindex[spent_di_idx] == tx_dateindex_val:
                                                same_day = True
                                                break

        # Compute transaction total output value
        tx_total_sats = sum(v for v in output_values if v and v > 0)

        # Check if both outputs are round (for 2-output txs)
        both_round = False
        if out_count == 2 and len(output_values) >= 2:
            both_round = is_round_btc(output_values[0]) and is_round_btc(output_values[1])

        # Precompute round status for each output (for 2-output analysis)
        output_rounds = [is_round_btc(v) if v else False for v in output_values]

        # Precompute value similarity for 2-output txs (same for both outputs)
        value_similarity = None
        if out_count == 2 and len(output_values) == 2:
            v1, v2 = output_values[0] or 0, output_values[1] or 0
            if v1 > 0 and v2 > 0:
                similarity = min(v1, v2) / max(v1, v2)
                if similarity > 0.95:
                    value_similarity = "nearly_equal"
                elif similarity > 0.8:
                    value_similarity = "similar"
                elif similarity > 0.5:
                    value_similarity = "moderate"
                elif similarity > 0.2:
                    value_similarity = "different"
                else:
                    value_similarity = "very_different"

        # Analyze each output
        for j, sats in enumerate(output_values):
            if sats is None or sats < 1000:
                continue

            ot = output_types[j] if j < len(output_types) else "unknown"
            is_round = output_rounds[j]  # Use precomputed value
            bin_idx, decade = get_phase_bin_and_decade(sats)

            # Compute error: what price does this output imply vs actual daily range?
            # Error is 0 if implied price falls within [low, high], otherwise distance to nearest edge
            error_pct = None
            if bin_idx is not None:
                implied_price = bin_to_price(bin_idx, btc_price)
                if implied_price < btc_low:
                    error_pct = 100 * (btc_low - implied_price) / btc_low
                elif implied_price > btc_high:
                    error_pct = 100 * (implied_price - btc_high) / btc_high
                else:
                    error_pct = 0  # within daily range

            # Use precomputed bin classifier for speed
            category = classify_accuracy_fast(bin_idx, bin_classifier)

            if category == "accurate":
                stats = stats_accurate
            elif category == "close":
                stats = stats_close
            elif category == "wrong_decade":
                stats = stats_wrong_decade
            else:
                stats = stats_noise

            # Compute 2-output specific metrics
            output_index = None
            is_smaller = None
            round_pattern = None
            value_ratio = None
            if out_count == 2 and len(output_values) == 2:
                output_index = j  # 0 or 1
                other_idx = 1 - j
                other_sats = output_values[other_idx] or 0
                is_smaller = sats < other_sats
                # Round pattern: which outputs are round?
                this_round = output_rounds[j]
                other_round = output_rounds[other_idx]
                if this_round and other_round:
                    round_pattern = "both_round"
                elif this_round and not other_round:
                    round_pattern = "only_this_round"
                elif not this_round and other_round:
                    round_pattern = "only_other_round"
                else:
                    round_pattern = "neither_round"
                # Value ratio: what fraction of total is this output?
                total_sats = sats + other_sats
                if total_sats > 0:
                    ratio = sats / total_sats
                    if ratio < 0.1:
                        value_ratio = "<10%"
                    elif ratio < 0.3:
                        value_ratio = "10-30%"
                    elif ratio < 0.5:
                        value_ratio = "30-50%"
                    elif ratio < 0.7:
                        value_ratio = "50-70%"
                    elif ratio < 0.9:
                        value_ratio = "70-90%"
                    else:
                        value_ratio = ">90%"

            # Compute round USD at different tolerances (price-based, using daily range)
            round_usd_10pct = is_round_usd(sats, btc_low, btc_high, tolerance=0.10)
            round_usd_5pct = is_round_usd(sats, btc_low, btc_high, tolerance=0.05)
            round_usd_2pct = is_round_usd(sats, btc_low, btc_high, tolerance=0.02)
            round_usd_1pct = is_round_usd(sats, btc_low, btc_high, tolerance=0.01)

            # Compute phase-based round USD (NO PRICE NEEDED!)
            # 200 bins/decade: 1%=±2bins, 2%=±4bins, 5%=±10bins, 10%=±20bins
            phase_usd_1pct = is_round_usd_phase(sats, tolerance_bins=2)
            phase_usd_2pct = is_round_usd_phase(sats, tolerance_bins=4)
            phase_usd_5pct = is_round_usd_phase(sats, tolerance_bins=10)
            phase_usd_10pct = is_round_usd_phase(sats, tolerance_bins=20)

            tx_pattern = get_tx_pattern(in_count, out_count)
            micro_round = is_micro_round_sats(sats)

            stats.record(
                output_count=out_count,
                input_count=in_count,
                output_type=ot,
                is_round=is_round,
                same_day=same_day,
                has_opreturn=has_opreturn,
                witness_size=witness_size,
                sats=sats,
                both_round=both_round,
                bin_idx=bin_idx,
                btc_price=btc_price,
                decade=decade,
                output_index=output_index,
                is_smaller=is_smaller,
                round_pattern=round_pattern,
                value_ratio=value_ratio,
                error_pct=error_pct,
                tx_total_sats=tx_total_sats,
                round_usd_10pct=round_usd_10pct,
                round_usd_5pct=round_usd_5pct,
                round_usd_2pct=round_usd_2pct,
                round_usd_1pct=round_usd_1pct,
                tx_pattern=tx_pattern,
                value_similarity=value_similarity,
                is_micro_round=micro_round,
                phase_usd_1pct=phase_usd_1pct,
                phase_usd_2pct=phase_usd_2pct,
                phase_usd_5pct=phase_usd_5pct,
                phase_usd_10pct=phase_usd_10pct
            )

    return stats_accurate, stats_close, stats_wrong_decade, stats_noise

def main():
    # Open report file
    report_file = open("research/price_signal_analysis_report.txt", "w")

    def log(msg=""):
        print(msg)
        report_file.write(msg + "\n")
        report_file.flush()

    log("=" * 60)
    log("PHASE ORACLE SIGNAL ANALYSIS")
    log("=" * 60)

    # Cache dates lookup (same for all months)
    log("Fetching date index...")
    dates = fetch("/api/metric/date/dateindex?start=0&end=4000")["data"]

    # Analyze all months
    for year in [2017, 2018]:
        for month in range(1, 13):  # All months
            key = (year, month)
            if key not in MONTHLY_PRICES:
                continue

            log(f"\n\n{'#'*60}")
            log(f"# {year}-{month:02d}")
            log('#'*60)

            # Get heights and dateindexes for this month
            try:
                start_di = None
                end_di = None
                for i, d in enumerate(dates):
                    if d and d.startswith(f"{year}-{month:02d}"):
                        if start_di is None:
                            start_di = i
                        end_di = i + 1  # Keep updating to find last day of month

                if start_di is None:
                    log(f"Could not find date index for {year}-{month:02d}")
                    continue

                # Get all heights for the month
                heights = fetch(f"/api/metric/first_height/dateindex?start={start_di}&end={end_di+1}")["data"]
                start_height = heights[0]
                end_height = heights[-1] if len(heights) > 1 else start_height + 1000

                log(f"Date range: {dates[start_di]} to {dates[end_di-1]} (dateindex {start_di}-{end_di})")

                # Analyze entire month with daily prices
                accurate, close, wrong_decade, noise = analyze_block_range(start_height, end_height, start_di, end_di)

                total_outputs = accurate.total + close.total + wrong_decade.total + noise.total
                log(f"\n--- SUMMARY ---")
                log(f"Total outputs analyzed: {total_outputs:,}")
                log(f"  Accurate (≤15% error): {accurate.total:,} ({100*accurate.total/total_outputs:.1f}%)")
                log(f"  Close (15-30% error): {close.total:,} ({100*close.total/total_outputs:.1f}%)")
                log(f"  Wrong decade: {wrong_decade.total:,} ({100*wrong_decade.total/total_outputs:.1f}%)")
                log(f"  Noise: {noise.total:,} ({100*noise.total/total_outputs:.1f}%)")

                print_stats(accurate, "ACCURATE (within 15% of actual price)", log)
                print_stats(noise, "NOISE (no decade matches)", log)

                # Print ratio comparison
                if accurate.total > 0 and noise.total > 0:
                    # Sanity check: show True/False split for key boolean fields
                    log(f"\n--- ROUND USD BREAKDOWN ---")
                    for name, acc_d, noise_d in [
                        ("Round USD 10%", accurate.by_round_usd_10pct, noise.by_round_usd_10pct),
                        ("Round USD 5%", accurate.by_round_usd_5pct, noise.by_round_usd_5pct),
                        ("Round USD 2%", accurate.by_round_usd_2pct, noise.by_round_usd_2pct),
                        ("Round USD 1%", accurate.by_round_usd_1pct, noise.by_round_usd_1pct),
                    ]:
                        acc_true_pct = 100 * acc_d.get(True, 0) / accurate.total
                        acc_false_pct = 100 * acc_d.get(False, 0) / accurate.total
                        noise_true_pct = 100 * noise_d.get(True, 0) / noise.total
                        noise_false_pct = 100 * noise_d.get(False, 0) / noise.total
                        log(f"{name}: accurate True={acc_true_pct:.1f}% False={acc_false_pct:.1f}% | noise True={noise_true_pct:.1f}% False={noise_false_pct:.1f}%")

                    log(f"\n{'='*50}")
                    log("KEY DIFFERENCES (accurate vs noise):")
                    log('='*50)

                    def compare(name, acc_dict, noise_dict):
                        log(f"\n{name}:")
                        all_keys = set(acc_dict.keys()) | set(noise_dict.keys())
                        for k in sorted(all_keys):
                            acc_pct = 100 * acc_dict.get(k, 0) / accurate.total
                            noise_pct = 100 * noise_dict.get(k, 0) / noise.total
                            diff = acc_pct - noise_pct
                            if abs(diff) > 2:  # Only show significant differences
                                log(f"  {k}: {acc_pct:.1f}% vs {noise_pct:.1f}% (diff: {diff:+.1f}%)")

                    compare("Output count", accurate.by_output_count, noise.by_output_count)
                    compare("Input count", accurate.by_input_count, noise.by_input_count)
                    compare("Output type", accurate.by_output_type, noise.by_output_type)
                    compare("Is round BTC", accurate.by_is_round, noise.by_is_round)
                    compare("Both round", accurate.by_both_round, noise.by_both_round)
                    compare("Same-day spend", accurate.by_same_day, noise.by_same_day)
                    compare("OP_RETURN", accurate.by_has_opreturn, noise.by_has_opreturn)
                    compare("Witness size", accurate.by_witness_size, noise.by_witness_size)
                    compare("Value range (sats)", accurate.by_value_range, noise.by_value_range)
                    compare("Decade (10^N sats)", accurate.by_decade, noise.by_decade)
                    compare("Implied USD", accurate.by_implied_usd_range, noise.by_implied_usd_range)
                    compare("Output index (2-out)", accurate.by_output_index, noise.by_output_index)
                    compare("Is smaller (2-out)", accurate.by_is_smaller_output, noise.by_is_smaller_output)
                    compare("Round pattern (2-out)", accurate.by_round_pattern, noise.by_round_pattern)
                    compare("Value ratio (2-out)", accurate.by_value_ratio, noise.by_value_ratio)
                    compare("Error from price", accurate.by_error_pct, noise.by_error_pct)
                    compare("Tx total value", accurate.by_tx_total_value, noise.by_tx_total_value)
                    compare("Round USD (10%)", accurate.by_round_usd_10pct, noise.by_round_usd_10pct)
                    compare("Round USD (5%)", accurate.by_round_usd_5pct, noise.by_round_usd_5pct)
                    compare("Round USD (2%)", accurate.by_round_usd_2pct, noise.by_round_usd_2pct)
                    compare("Round USD (1%)", accurate.by_round_usd_1pct, noise.by_round_usd_1pct)
                    compare("Phase USD (1%)", accurate.by_phase_usd_1pct, noise.by_phase_usd_1pct)
                    compare("Phase USD (2%)", accurate.by_phase_usd_2pct, noise.by_phase_usd_2pct)
                    compare("Phase USD (5%)", accurate.by_phase_usd_5pct, noise.by_phase_usd_5pct)
                    compare("Phase USD (10%)", accurate.by_phase_usd_10pct, noise.by_phase_usd_10pct)
                    compare("Tx pattern", accurate.by_tx_pattern, noise.by_tx_pattern)
                    compare("Value similarity (2-out)", accurate.by_value_similarity, noise.by_value_similarity)
                    compare("Micro-round sats", accurate.by_is_micro_round, noise.by_is_micro_round)

                    # EXCLUSION RECOMMENDATIONS
                    log(f"\n{'='*50}")
                    log("EXCLUSION CANDIDATES (overrepresented in noise):")
                    log('='*50)
                    log("Characteristics where noise% > accurate% suggest exclusion filters:\n")

                    def find_exclusions(name, acc_dict, noise_dict, threshold=3.0):
                        """Find characteristics overrepresented in noise (candidates for exclusion)."""
                        exclusions = []
                        for k in set(acc_dict.keys()) | set(noise_dict.keys()):
                            acc_pct = 100 * acc_dict.get(k, 0) / accurate.total
                            noise_pct = 100 * noise_dict.get(k, 0) / noise.total
                            diff = noise_pct - acc_pct  # positive = more in noise
                            if diff > threshold and noise_pct > 1:  # at least 1% of noise
                                exclusions.append((k, acc_pct, noise_pct, diff))
                        return sorted(exclusions, key=lambda x: -x[3])  # sort by diff descending

                    all_exclusions = []
                    for name, acc_d, noise_d in [
                        ("Value range", accurate.by_value_range, noise.by_value_range),
                        ("Implied USD", accurate.by_implied_usd_range, noise.by_implied_usd_range),
                        ("Decade", accurate.by_decade, noise.by_decade),
                        ("Output count", accurate.by_output_count, noise.by_output_count),
                        ("Is round BTC", accurate.by_is_round, noise.by_is_round),
                        ("Both round", accurate.by_both_round, noise.by_both_round),
                        ("Tx pattern", accurate.by_tx_pattern, noise.by_tx_pattern),
                        ("Value similarity", accurate.by_value_similarity, noise.by_value_similarity),
                        ("Value ratio", accurate.by_value_ratio, noise.by_value_ratio),
                        ("Round USD 10%", accurate.by_round_usd_10pct, noise.by_round_usd_10pct),
                        ("Round USD 5%", accurate.by_round_usd_5pct, noise.by_round_usd_5pct),
                        ("Round USD 2%", accurate.by_round_usd_2pct, noise.by_round_usd_2pct),
                        ("Round USD 1%", accurate.by_round_usd_1pct, noise.by_round_usd_1pct),
                        ("Phase USD 10%", accurate.by_phase_usd_10pct, noise.by_phase_usd_10pct),
                        ("Phase USD 5%", accurate.by_phase_usd_5pct, noise.by_phase_usd_5pct),
                        ("Phase USD 2%", accurate.by_phase_usd_2pct, noise.by_phase_usd_2pct),
                        ("Phase USD 1%", accurate.by_phase_usd_1pct, noise.by_phase_usd_1pct),
                        ("Tx total value", accurate.by_tx_total_value, noise.by_tx_total_value),
                        ("Micro-round sats", accurate.by_is_micro_round, noise.by_is_micro_round),
                    ]:
                        excl = find_exclusions(name, acc_d, noise_d)
                        for k, acc_pct, noise_pct, diff in excl:
                            all_exclusions.append((name, k, acc_pct, noise_pct, diff))

                    # Sort by impact (diff) and print
                    all_exclusions.sort(key=lambda x: -x[4])
                    for name, k, acc_pct, noise_pct, diff in all_exclusions[:15]:
                        log(f"  EXCLUDE {name}={k}: noise {noise_pct:.1f}% vs accurate {acc_pct:.1f}% (+{diff:.1f}%)")

                    # Also show INCLUSION candidates (overrepresented in accurate)
                    log(f"\n{'='*50}")
                    log("INCLUSION SIGNALS (overrepresented in accurate):")
                    log('='*50)
                    log("Characteristics where accurate% > noise% are good signals:\n")

                    all_inclusions = []
                    for name, acc_d, noise_d in [
                        ("Value range", accurate.by_value_range, noise.by_value_range),
                        ("Implied USD", accurate.by_implied_usd_range, noise.by_implied_usd_range),
                        ("Decade", accurate.by_decade, noise.by_decade),
                        ("Output count", accurate.by_output_count, noise.by_output_count),
                        ("Is round BTC", accurate.by_is_round, noise.by_is_round),
                        ("Is smaller (2-out)", accurate.by_is_smaller_output, noise.by_is_smaller_output),
                        ("Tx pattern", accurate.by_tx_pattern, noise.by_tx_pattern),
                        ("Value similarity", accurate.by_value_similarity, noise.by_value_similarity),
                        ("Value ratio", accurate.by_value_ratio, noise.by_value_ratio),
                        ("Round USD 10%", accurate.by_round_usd_10pct, noise.by_round_usd_10pct),
                        ("Round USD 5%", accurate.by_round_usd_5pct, noise.by_round_usd_5pct),
                        ("Round USD 2%", accurate.by_round_usd_2pct, noise.by_round_usd_2pct),
                        ("Round USD 1%", accurate.by_round_usd_1pct, noise.by_round_usd_1pct),
                        ("Phase USD 10%", accurate.by_phase_usd_10pct, noise.by_phase_usd_10pct),
                        ("Phase USD 5%", accurate.by_phase_usd_5pct, noise.by_phase_usd_5pct),
                        ("Phase USD 2%", accurate.by_phase_usd_2pct, noise.by_phase_usd_2pct),
                        ("Phase USD 1%", accurate.by_phase_usd_1pct, noise.by_phase_usd_1pct),
                    ]:
                        for k in set(acc_d.keys()) | set(noise_d.keys()):
                            acc_pct = 100 * acc_d.get(k, 0) / accurate.total
                            noise_pct = 100 * noise_d.get(k, 0) / noise.total
                            diff = acc_pct - noise_pct  # positive = more in accurate
                            if diff > 3.0 and acc_pct > 1:
                                all_inclusions.append((name, k, acc_pct, noise_pct, diff))

                    all_inclusions.sort(key=lambda x: -x[4])
                    for name, k, acc_pct, noise_pct, diff in all_inclusions[:15]:
                        log(f"  KEEP {name}={k}: accurate {acc_pct:.1f}% vs noise {noise_pct:.1f}% (+{diff:.1f}%)")

            except Exception as e:
                log(f"Error: {e}")
                import traceback
                traceback.print_exc()
                traceback.print_exc(file=report_file)

    report_file.close()
    print(f"\nReport saved to: research/price_signal_analysis_report.txt")

if __name__ == "__main__":
    main()
