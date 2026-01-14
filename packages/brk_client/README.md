# brk-client

Python client for the [Bitcoin Research Kit](https://github.com/bitcoinresearchkit/brk) API.

[PyPI](https://pypi.org/project/brk-client/) | [API Reference](https://github.com/bitcoinresearchkit/brk/blob/main/packages/brk_client/DOCS.md)

## Installation

```bash
pip install brk-client
```

## Quick Start

```python
from brk_client import BrkClient

client = BrkClient("http://localhost:3110")

# Blockchain data (mempool.space compatible)
block = client.get_block_by_height(800000)
tx = client.get_tx("abc123...")
address = client.get_address("bc1q...")

# Metrics API - typed, chainable
prices = client.metrics.price.usd.split.close \
    .by.dateindex() \
    .range(-30)  # Last 30 days

# Generic metric fetching
data = client.get_metric("price_close", "dateindex", -30)
```

## Configuration

```python
client = BrkClient("http://localhost:3110", timeout=60.0)
```
