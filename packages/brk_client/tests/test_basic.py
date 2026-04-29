# Run:
# uv run pytest tests/basic.py -s

from __future__ import print_function

from brk_client import BrkClient


def test_client_creation():
    BrkClient("http://localhost:3110")


def test_tree_exists():
    client = BrkClient("http://localhost:3110")
    assert hasattr(client, "series")
    assert hasattr(client.series, "prices")
    assert hasattr(client.series, "blocks")


def test_fetch_block():
    client = BrkClient("http://localhost:3110")
    print(client.get_block_by_height(800000))


def test_fetch_json_series():
    client = BrkClient("http://localhost:3110")
    a = client.get_series("price_close", "day1")
    print(a)


def test_fetch_csv_series():
    client = BrkClient("http://localhost:3110")
    a = client.get_series("price_close", "day1", -10, None, None, "csv")
    print(a)


def test_fetch_typed_series():
    client = BrkClient("http://localhost:3110")
    # Using new idiomatic API: tail(10).fetch() or [-10:].fetch()
    a = client.series.constants._0.by.day1().tail(10).fetch()
    print(a)
    b = client.series.outputs.unspent.count.by.height().tail(10).fetch()
    print(b)
    c = client.series.prices.split.close.usd.by.day1().tail(10).fetch()
    print(c)
    d = (
        client.series.investing.period.lump_sum_stack._10y.usd.by.day1()
        .tail(10)
        .fetch()
    )
    print(d)
    e = (
        client.series.investing.class_.dca_cost_basis.from_2017.usd.by.day1()
        .tail(10)
        .fetch()
    )
    print(e)
    f = client.series.prices.ohlc.usd.by.day1().tail(10).fetch()
    print(f)


def test_endpoint_len():
    client = BrkClient("http://localhost:3110")
    n = client.series.prices.split.close.usd.by.day1().len()
    assert isinstance(n, int)
    assert n > 0


def test_endpoint_version():
    client = BrkClient("http://localhost:3110")
    v = client.series.prices.split.close.usd.by.day1().version()
    assert isinstance(v, int)
    assert v >= 1
