"""Bounded read-only local chart replay; abort on errors or a slow response."""

import hashlib
import json
import statistics
import subprocess
import time
import urllib.error
import urllib.request

BASE = "http://127.0.0.1:3110"


def request(path, etag=None):
    headers = {"If-None-Match": etag} if etag else {}
    started = time.perf_counter()
    try:
        with urllib.request.urlopen(urllib.request.Request(BASE + path, headers=headers), timeout=8) as response:
            body = response.read(2_000_001)
            status, tag = response.status, response.headers.get("ETag")
    except urllib.error.HTTPError as error:
        if error.code != 304:
            raise
        status, tag, body = 304, error.headers.get("ETag"), error.read()
    elapsed = time.perf_counter() - started
    assert len(body) <= 2_000_000
    if elapsed > 2:
        raise RuntimeError(f"Stopping bounded replay after slow request: {path} {elapsed:.3f}s")
    return status, tag, body, elapsed * 1000


def memory():
    pids = subprocess.check_output(["lsof", "-tiTCP:3110", "-sTCP:LISTEN"], text=True).split()
    assert len(pids) == 1, pids
    return subprocess.check_output(["ps", "-p", pids[0], "-o", "pid=,rss=,%cpu="], text=True).strip()


print("health", request("/health")[2].decode(), flush=True)
print("before pid/rss_KiB/cpu_percent", memory(), flush=True)
for series in ["sopr_1m", "net_pnl_change_1m_to_mcap", "timestamp"]:
    for index, count in [("month1", 120), ("day1", 365)]:
        path = f"/api/series/{series}/{index}/data?from=-{count}"
        samples, hashes = [], []
        for _ in range(4):
            status, tag, body, elapsed = request(path)
            assert status == 200
            values = json.loads(body)
            assert isinstance(values, list) and len(values) == count
            samples.append(elapsed)
            hashes.append(hashlib.sha256(body).hexdigest())
            time.sleep(0.1)
        status, _, body, conditional_ms = request(path, tag)
        assert tag and status == 304 and not body
        print(json.dumps({"series": series, "index": index, "rows": count,
                          "first_ms": samples[0], "warm_median_ms": statistics.median(samples[1:]),
                          "warm_max_ms": max(samples[1:]), "conditional_ms": conditional_ms,
                          "stable_body": len(set(hashes)) == 1}), flush=True)
print("after pid/rss_KiB/cpu_percent", memory(), flush=True)
print("health", request("/health")[2].decode(), flush=True)
