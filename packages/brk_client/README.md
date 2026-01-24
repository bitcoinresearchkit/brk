# brk-client

Python client for the [Bitcoin Research Kit](https://github.com/bitcoinresearchkit/brk) API.

Requires Python 3.9+. Zero dependencies.

[PyPI](https://pypi.org/project/brk-client/) | [API Reference](https://github.com/bitcoinresearchkit/brk/blob/main/packages/brk_client/DOCS.md)

## Installation

```bash
pip install brk-client
# or
uv add brk-client
```

Or just copy [`brk_client/__init__.py`](./brk_client/__init__.py) into your project - it's a single file with no dependencies.

## Quick Start

```python
from brk_client import BrkClient

# Use the free public API or your own instance
# Has optional `, timeout=60.0` argument
client = BrkClient("https://bitview.space")

# Blockchain data (mempool.space compatible)
block = client.get_block_by_height(800000)
tx = client.get_tx("abc123...")
address = client.get_address("bc1q...")

# Metrics API - typed, chainable
prices = client.metrics.price.usd.split.close \
    .by.dateindex() \
    .tail(30) \
    .fetch()  # Last 30 items

# Generic metric fetching
data = client.get_metric("price_close", "dateindex", -30)
```
