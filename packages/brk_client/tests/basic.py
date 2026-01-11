from __future__ import print_function

from brk_client import BrkClient


def test_client_creation():
    client = BrkClient("http://localhost:3110")
    assert client.base_url == "http://localhost:3110"


def test_tree_exists():
    client = BrkClient("http://localhost:3110")
    assert hasattr(client, "tree")
    assert hasattr(client.tree, "price")
    assert hasattr(client.tree, "blocks")


def test_fetch_block():
    client = BrkClient("http://localhost:3110")
    print(client.get_block_height(800000))


def test_fetch_any_metric():
    client = BrkClient("http://localhost:3110")
    print(client.get_metric_by_index("dateindex", "price_close"))


def test_fetch_typed_metric():
    client = BrkClient("http://localhost:3110")
    a = client.tree.constants.constant_0.by.dateindex().range(-10)
    print(a)
    b = client.tree.outputs.count.utxo_count.by.height().range(-10)
    print(b)
    c = client.tree.price.usd.split.close.by.dateindex().range(-10)
    print(c)
    d = client.tree.market.dca.period_lump_sum_stack._10y.dollars.by.dateindex().range(
        -10
    )
    print(d)
    e = client.tree.market.dca.class_average_price._2017.by.dateindex().range(-10)
    print(e)
    f = client.tree.distribution.address_cohorts.amount_range._10k_sats_to_100k_sats.activity.sent.dollars.cumulative.by.dateindex().range(
        -10
    )
    print(f)
    g = client.tree.price.usd.ohlc.by.dateindex().range(-10)
    print(g)
