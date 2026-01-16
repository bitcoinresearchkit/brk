#!/usr/bin/env python3
"""
Test price phase detection from outputs alone.
The idea: Round USD outputs create a fingerprint pattern that reveals the price phase.
"""

import math
import http.client
import json
import time
from collections import defaultdict

API_HOST = "localhost"
API_PORT = 3110

# Round USD phases (fixed fingerprint)
# These are frac(log10(usd_cents)) for round USD values
ROUND_USD_PHASES = [
    0.00,  # $1, $10, $100, $1000
    0.18,  # $1.50, $15, $150
    0.30,  # $2, $20, $200
    0.40,  # $2.50, $25, $250
    0.48,  # $3, $30, $300
    0.60,  # $4, $40, $400
    0.70,  # $5, $50, $500
    0.78,  # $6, $60, $600
    0.85,  # $7, $70, $700
    0.90,  # $8, $80, $800
    0.95,  # $9, $90, $900
]

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

def fetch(path: str, retries: int = 3):
    for attempt in range(retries):
        try:
            conn = get_conn()
            conn.request("GET", path)
            resp = conn.getresponse()
            data = resp.read().decode('utf-8')
            return json.loads(data)
        except Exception as e:
            reset_conn()
            if attempt < retries - 1:
                time.sleep(2)
            else:
                raise

def fetch_chunked(path_template: str, start: int, end: int, chunk_size: int = 25000) -> list:
    result = []
    for chunk_start in range(start, end, chunk_size):
        chunk_end = min(chunk_start + chunk_size, end)
        path = path_template.format(start=chunk_start, end=chunk_end)
        data = fetch(path)["data"]
        result.extend(data)
    return result


def get_sats_phase(sats: int) -> float:
    """Get the phase (fractional part of log10) for a sats value."""
    if sats <= 0:
        return 0.0
    return math.log10(sats) % 1.0


def count_round_usd_matches(outputs: list, price_phase: float, tolerance: float = 0.02) -> int:
    """
    Count how many outputs match round USD bins at the given price phase.

    At price_phase P, round USD outputs should appear at sats_phase = (usd_phase - P) mod 1
    """
    # Compute expected sats phases for round USD at this price phase
    expected_phases = [(usd_phase - price_phase) % 1.0 for usd_phase in ROUND_USD_PHASES]

    count = 0
    for sats in outputs:
        if sats is None or sats < 1000:
            continue
        sats_phase = get_sats_phase(sats)

        # Check if sats_phase matches any expected phase
        for exp_phase in expected_phases:
            diff = abs(sats_phase - exp_phase)
            # Handle wraparound (0.99 is close to 0.01)
            if diff < tolerance or diff > (1.0 - tolerance):
                count += 1
                break

    return count


def find_best_price_phase(outputs: list, tolerance: float = 0.02, resolution: int = 100) -> tuple:
    """
    Find the price phase that maximizes round USD matches.
    Returns (best_phase, best_count, all_counts).
    """
    counts = []
    best_phase = 0.0
    best_count = 0

    for i in range(resolution):
        price_phase = i / resolution
        count = count_round_usd_matches(outputs, price_phase, tolerance)
        counts.append(count)

        if count > best_count:
            best_count = count
            best_phase = price_phase

    return best_phase, best_count, counts


def actual_price_phase(price: float) -> float:
    """Get the actual price phase from a price."""
    return math.log10(price) % 1.0


def analyze_day(date_str: str, start_height: int, end_height: int, actual_price: float):
    """Analyze a single day's outputs."""

    # Get transaction range for these heights
    first_tx = fetch(f"/api/metric/first_txindex/height?start={start_height}&end={end_height}")
    first_txs = first_tx["data"]
    if not first_txs or len(first_txs) < 2:
        return None

    tx_start = first_txs[0]
    tx_end = first_txs[-1]

    # Get output range
    tx_first_out = fetch_chunked("/api/metric/first_txoutindex/txindex?start={start}&end={end}", tx_start, tx_end)
    if not tx_first_out:
        return None

    out_start = tx_first_out[0]
    out_end = tx_first_out[-1] + 10  # estimate

    # Fetch output values
    out_values = fetch_chunked("/api/metric/value/txoutindex?start={start}&end={end}", out_start, out_end)

    # Filter to reasonable range (1000 sats to 100 BTC)
    outputs = [v for v in out_values if v and 1000 <= v <= 10_000_000_000]

    if len(outputs) < 1000:
        return None

    # Find best price phase
    detected_phase, match_count, _ = find_best_price_phase(outputs, tolerance=0.02)

    # Compare with actual
    actual_phase = actual_price_phase(actual_price)

    # Phase error (handle wraparound)
    phase_error = abs(detected_phase - actual_phase)
    if phase_error > 0.5:
        phase_error = 1.0 - phase_error

    return {
        'date': date_str,
        'actual_price': actual_price,
        'actual_phase': actual_phase,
        'detected_phase': detected_phase,
        'phase_error': phase_error,
        'match_count': match_count,
        'total_outputs': len(outputs),
        'match_pct': 100 * match_count / len(outputs),
    }


def main():
    print("=" * 60)
    print("PRICE PHASE DETECTION TEST")
    print("=" * 60)
    print("\nIdea: Round USD outputs form a fingerprint pattern.")
    print("Sliding this pattern across the histogram reveals the price phase.\n")

    # Fetch dates
    print("Fetching date index...")
    dates = fetch("/api/metric/date/dateindex?start=0&end=4000")["data"]

    # Fetch daily OHLC
    print("Fetching daily prices...")
    ohlc_data = fetch("/api/metric/price_ohlc/dateindex?start=2800&end=3600")["data"]

    # Fetch heights
    print("Fetching heights...")
    heights = fetch("/api/metric/first_height/dateindex?start=2800&end=3600")["data"]

    results = []

    # Test on 2017-2018 (roughly dateindex 2900-3600)
    # Sample every 7 days to speed up
    for di in range(2900, 3550, 7):
        if di - 2800 >= len(ohlc_data) or di - 2800 >= len(heights):
            continue

        ohlc = ohlc_data[di - 2800]
        if not ohlc or len(ohlc) < 4:
            continue

        # Use close price as "actual"
        actual_price = ohlc[3]
        if not actual_price or actual_price <= 0:
            continue

        date_str = dates[di] if di < len(dates) else f"di={di}"

        start_height = heights[di - 2800]
        end_height = heights[di - 2800 + 1] if di - 2800 + 1 < len(heights) else start_height + 144

        if not start_height:
            continue

        print(f"\nAnalyzing {date_str} (${actual_price:.0f})...")

        try:
            result = analyze_day(date_str, start_height, end_height, actual_price)
            if result:
                results.append(result)
                print(f"  Actual phase:   {result['actual_phase']:.3f}")
                print(f"  Detected phase: {result['detected_phase']:.3f}")
                print(f"  Phase error:    {result['phase_error']:.3f} ({result['phase_error']*100:.1f}%)")
                print(f"  Matches: {result['match_count']:,} / {result['total_outputs']:,} ({result['match_pct']:.1f}%)")
        except Exception as e:
            print(f"  Error: {e}")
            continue

    # Summary
    if results:
        print("\n" + "=" * 60)
        print("SUMMARY")
        print("=" * 60)

        errors = [r['phase_error'] for r in results]
        avg_error = sum(errors) / len(errors)

        # Count how many are within various thresholds
        within_01 = sum(1 for e in errors if e <= 0.01)
        within_02 = sum(1 for e in errors if e <= 0.02)
        within_05 = sum(1 for e in errors if e <= 0.05)
        within_10 = sum(1 for e in errors if e <= 0.10)

        print(f"\nTotal days analyzed: {len(results)}")
        print(f"Average phase error: {avg_error:.3f} ({avg_error*100:.1f}%)")
        print(f"\nPhase error distribution:")
        print(f"  ≤1%:  {within_01:3d} / {len(results)} ({100*within_01/len(results):.0f}%)")
        print(f"  ≤2%:  {within_02:3d} / {len(results)} ({100*within_02/len(results):.0f}%)")
        print(f"  ≤5%:  {within_05:3d} / {len(results)} ({100*within_05/len(results):.0f}%)")
        print(f"  ≤10%: {within_10:3d} / {len(results)} ({100*within_10/len(results):.0f}%)")

        # Show worst cases
        print(f"\nWorst cases:")
        worst = sorted(results, key=lambda r: -r['phase_error'])[:5]
        for r in worst:
            print(f"  {r['date']}: detected {r['detected_phase']:.2f} vs actual {r['actual_phase']:.2f} "
                  f"(error {r['phase_error']:.2f}, ${r['actual_price']:.0f})")

        # Show best cases
        print(f"\nBest cases:")
        best = sorted(results, key=lambda r: r['phase_error'])[:5]
        for r in best:
            print(f"  {r['date']}: detected {r['detected_phase']:.2f} vs actual {r['actual_phase']:.2f} "
                  f"(error {r['phase_error']:.3f}, ${r['actual_price']:.0f})")


if __name__ == "__main__":
    main()
