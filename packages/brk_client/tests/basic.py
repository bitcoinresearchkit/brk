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
    print(client.get_block_height(800000))


def test_fetch_json_metric():
    client = BrkClient("http://localhost:3110")
    a = client.get_metric_by_index("price_close", "dateindex")
    print(a)


def test_fetch_csv_metric():
    client = BrkClient("http://localhost:3110")
    a = client.get_metric_by_index("price_close", "dateindex", -10, None, None, "csv")
    print(a)


def test_fetch_typed_metric():
    client = BrkClient("http://localhost:3110")
    a = client.metrics.constants.constant_0.by.dateindex().range(-10)
    print(a)
    b = client.metrics.outputs.count.utxo_count.by.height().range(-10)
    print(b)
    c = client.metrics.price.usd.split.close.by.dateindex().range(-10)
    print(c)
    d = client.metrics.market.dca.period_lump_sum_stack._10y.dollars.by.dateindex().range(
        -10
    )
    print(d)
    e = client.metrics.market.dca.class_average_price._2017.by.dateindex().range(-10)
    print(e)
    f = client.metrics.distribution.address_cohorts.amount_range._10k_sats_to_100k_sats.activity.sent.dollars.cumulative.by.dateindex().range(
        -10
    )
    print(f)
    g = client.metrics.price.usd.ohlc.by.dateindex().range(-10)
    print(g)
