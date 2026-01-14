# brk-client

Python client for the [Bitcoin Research Kit](https://github.com/bitcoinresearchkit/brk) API.

[PyPI](https://pypi.org/project/brk-client/) | [API Reference](https://github.com/bitcoinresearchkit/brk/blob/main/packages/brk_client/DOCS.md)

## Installation

```bash
pip install brk-client
# or
uv add brk-client
```

## Quick Start

```python
from brk_client import BrkClient

# Use the free public API or your own instance
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

## API

```python
# Range methods
.head(n)   # First n items
.tail(n)   # Last n items
.fetch()   # Execute the request
```

## Configuration

```python
client = BrkClient("https://bitview.space", timeout=60.0)
```
