from __future__ import print_function

from brk_client import VERSION, BrkClient


def test_version():
    assert VERSION.startswith("v")


def test_client_creation():
    client = BrkClient("http://localhost:3110")
    assert client.base_url == "http://localhost:3110"


def test_tree_exists():
    client = BrkClient("http://localhost:3110")
    print(client.get_api_block_height_by_height(800000))
    print(client.get_api_metric_by_metric_by_index("price_close", "dateindex"))
    assert hasattr(client, "tree")
    assert hasattr(client.tree, "computed")
    assert hasattr(client.tree, "indexed")
