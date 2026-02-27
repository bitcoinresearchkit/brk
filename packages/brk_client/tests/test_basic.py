# Run:
# uv run pytest tests/basic.py -s

from __future__ import print_function

from brk_client import BrkClient


def test_client_creation():
    BrkClient("http://localhost:3110")


def test_tree_exists():
    client = BrkClient("http://localhost:3110")
    assert hasattr(client, "metrics")
    assert hasattr(client.metrics, "price")
    assert hasattr(client.metrics, "blocks")


def test_fetch_block():
    client = BrkClient("http://localhost:3110")
    print(client.get_block_by_height(800000))


def test_fetch_json_metric():
    client = BrkClient("http://localhost:3110")
    a = client.get_metric("price_close", "day1")
    print(a)


def test_fetch_csv_metric():
    client = BrkClient("http://localhost:3110")
    a = client.get_metric("price_close", "day1", -10, None, None, "csv")
    print(a)


def test_fetch_typed_metric():
    client = BrkClient("http://localhost:3110")
    # Using new idiomatic API: tail(10).fetch() or [-10:].fetch()
    a = client.metrics.constants.constant_0.by.day1().tail(10).fetch()
    print(a)
    b = client.metrics.outputs.count.utxo_count.by.height().tail(10).fetch()
    print(b)
    c = client.metrics.prices.split.close.usd.by.day1().tail(10).fetch()
    print(c)
    d = (
        client.metrics.market.dca.period_lump_sum_stack._10y.usd.by.day1()
        .tail(10)
        .fetch()
    )
    print(d)
    e = (
        client.metrics.market.dca.class_average_price._2017.usd.by.day1()
        .tail(10)
        .fetch()
    )
    print(e)
    f = (
        client.metrics.distribution.address_cohorts.amount_range._10k_sats_to_100k_sats.activity.sent.cumulative.usd.by.day1()
        .tail(10)
        .fetch()
    )
    print(f)
    g = client.metrics.prices.ohlc.usd.by.day1().tail(10).fetch()
    print(g)
