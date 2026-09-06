#!/usr/bin/env python3
"""Bounded, read-only HTTP probes. JSONL on stdout; no daemon/data mutations.

Use an idle local server. First-request timings are not cold-device timings.
Serial timings include body transfer. Concurrent results include queueing and
must not be compared directly with serial or access-log measurements.
"""

import argparse
import collections
import concurrent.futures
import http.client
import json
import math
import statistics
import threading
import time
import urllib.parse


def emit(value):
    print(json.dumps(value, separators=(",", ":")), flush=True)


def summary(values):
    if not values:
        return None
    ordered = sorted(values)
    return {
        "n": len(values),
        "p50_ms": round(statistics.median(values), 3),
        "p95_ms": round(ordered[math.ceil(len(values) * .95) - 1], 3),
        "max_ms": round(max(values), 3),
    }


class Client:
    def __init__(self, origin):
        self.connection = http.client.HTTPConnection(origin.netloc, timeout=15)

    def request(self, path, method="GET", etag=None, encoding="identity", collect=False):
        headers = {"Accept-Encoding": encoding}
        if etag:
            headers["If-None-Match"] = etag
        started = time.perf_counter()
        self.connection.request(method, path, headers=headers)
        response = self.connection.getresponse()
        first_byte = (time.perf_counter() - started) * 1000
        body, size, error_body = [], 0, bytearray()
        while True:
            chunk = response.read(64 * 1024)
            if not chunk:
                break
            size += len(chunk)
            if response.status >= 400 and len(error_body) < 2048:
                error_body.extend(chunk[:2048 - len(error_body)])
            if size > 64 * 1024 * 1024:
                self.connection.close()
                raise RuntimeError("response exceeded 64 MiB wire limit: " + path)
            if collect:
                if size > 2 * 1024 * 1024:
                    self.connection.close()
                    raise RuntimeError("discovery response exceeded 2 MiB: " + path)
                body.append(chunk)
        result = {
            "status": response.status,
            "etag": response.getheader("etag"),
            "bytes": size,
            "encoding": response.getheader("content-encoding"),
            "ttfb_ms": first_byte,
            "ms": (time.perf_counter() - started) * 1000,
        }
        if error_body:
            try:
                result["error"] = json.loads(error_body).get("error", {}).get("code")
            except (ValueError, AttributeError):
                result["error"] = "non-json"
        if response.status == 304 or method == "HEAD":
            assert size == 0, (path, method, result)
        return result, b"".join(body)

    def json(self, path):
        result, body = self.request(path, collect=True)
        if result["status"] != 200:
            raise RuntimeError((path, result["status"], body[:200].decode()))
        return json.loads(body)


def discover(client):
    health = client.json("/health")
    emit({"health_before": health})
    height = max(1, health["indexed_height"] - 40)
    result, body = client.request("/api/block-height/" + str(height), collect=True)
    assert result["status"] == 200
    block = body.decode()
    transactions = client.json("/api/block/" + block + "/txs/25")
    transaction = max(transactions, key=lambda tx: len(tx["vout"]))
    txid = transaction["txid"]
    address = next(output["scriptpubkey_address"] for output in transaction["vout"]
                   if output.get("scriptpubkey_address"))
    dates = client.json("/api/urpd/all/dates")
    historical = dates[-2]
    prefix = "/api/tx/" + txid
    addr = "/api/address/" + address
    routes = [
        ("address/stats", addr), ("address/combined", addr + "/txs"),
        ("address/chain", addr + "/txs/chain"), ("address/mempool", addr + "/txs/mempool"),
        ("address/utxo", addr + "/utxo"),
        ("tx/json", prefix), ("tx/outspend", prefix + "/outspend/0"),
        ("tx/outspends", prefix + "/outspends"), ("tx/raw", prefix + "/raw"),
        ("tx/status", prefix + "/status"), ("cpfp/confirmed", "/api/v1/cpfp/" + txid),
        ("block/page", "/api/block/" + block + "/txs/25"),
        ("block/raw", "/api/block/" + block + "/raw"),
        ("series/list", "/api/series/list?per_page=1000"),
        ("series/search", "/api/series/search?q=realized%20price&limit=100"),
        ("series/search-fuzzy", "/api/series/search?q=sth%20suply%20profit&limit=100"),
        ("series/latest", "/api/series/price_close/day1/latest"),
        ("series/length", "/api/series/price_close/day1/len"),
        ("series/range", "/api/series/price_close/day1?start=2010-01-01&end=2025-01-01"),
        ("series/bulk", "/api/series/bulk?series=price_close,market_cap&index=day1&start=-1000"),
        ("urpd/dates", "/api/urpd/all/dates"),
        ("urpd/latest", "/api/urpd/all?agg=log100"),
        ("urpd/historical", "/api/urpd/all/" + historical + "?agg=log100"),
        ("urpd/weighted", "/api/urpd/sth?agg=log100&weight=cointime"),
        ("mempool/info", "/api/mempool"), ("mempool/recent", "/api/mempool/recent"),
        ("mempool/hash", "/api/mempool/hash"),
        ("mempool/blocks", "/api/v1/fees/mempool-blocks"),
        ("mempool/template", "/api/v1/mempool/block-template"),
    ]
    for _ in range(5):
        response, body = client.request("/api/mempool/recent", collect=True)
        if response["status"] == 200 and json.loads(body):
            live_tx = json.loads(body)[0]["txid"]
            routes += [("cpfp/mempool", "/api/v1/cpfp/" + live_tx),
                       ("mempool/tx", "/api/tx/" + live_tx),
                       ("mempool/outspends", "/api/tx/" + live_tx + "/outspends")]
            break
        time.sleep(.1)
    emit({"selection": {"height": height, "block": block, "txid": txid,
                        "outputs": len(transaction["vout"]), "address": address,
                        "historical_urpd": historical}})
    return routes


def serial(client, label, path, count, encoding="identity"):
    first, _ = client.request(path, encoding=encoding)
    tag = first["etag"]
    results = collections.defaultdict(list)
    for i in range(count + 2):
        for method, condition in [("GET", None), ("GET", tag), ("HEAD", None)]:
            response, _ = client.request(path, method, condition, encoding)
            if response["status"] == 200 and response["etag"]:
                tag = response["etag"]
            if i >= 2:
                key = method + (" conditional" if condition else "") + " " + str(response["status"])
                if response.get("error"):
                    key += "/" + response["error"]
                results[key].append(response["ms"])
    emit({"serial": label, "path": path, "encoding": encoding,
          "first": first, "samples": {key: summary(value) for key, value in results.items()}})


def run_concurrent(origin, routes, width):
    local = threading.local()

    def one(path):
        if not hasattr(local, "client"):
            local.client = Client(origin)
        return local.client.request(path)[0]

    with concurrent.futures.ThreadPoolExecutor(max_workers=width) as executor:
        for label, path in routes:
            started = time.perf_counter()
            results = list(executor.map(one, [path] * (width * 3)))
            groups = collections.defaultdict(list)
            for response in results:
                key = str(response["status"])
                if response.get("error"):
                    key += "/" + response["error"]
                groups[key].append(response["ms"])
            emit({"concurrent": label, "width": width,
                  "wall_ms": round((time.perf_counter() - started) * 1000, 3),
                  "samples": {key: summary(value) for key, value in groups.items()}})


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--base", default="http://127.0.0.1:3110")
    parser.add_argument("--samples", type=int, default=8, choices=range(1, 21))
    parser.add_argument("--concurrency", type=int, default=4, choices=range(1, 5))
    args = parser.parse_args()
    origin = urllib.parse.urlsplit(args.base)
    if origin.scheme != "http" or origin.path not in ["", "/"]:
        parser.error("base must be an HTTP origin")
    client = Client(origin)
    routes = discover(client)
    for label, path in routes:
        serial(client, label, path, min(args.samples, 3) if label == "mempool/template" else args.samples)
    for label, path in routes:
        if label in ["block/raw", "block/page", "series/range"]:
            serial(client, label, path, 3, "gzip")
    selected = [pair for pair in routes if pair[0] in [
        "block/page", "cpfp/confirmed", "series/list", "series/search-fuzzy",
        "series/range", "urpd/latest", "mempool/info", "tx/outspends",
    ]]
    run_concurrent(origin, selected, args.concurrency)
    emit({"health_after": client.json("/health")})
    client.connection.close()


if __name__ == "__main__":
    main()
