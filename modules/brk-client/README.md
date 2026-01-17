# brk-client

JavaScript/TypeScript client for the [Bitcoin Research Kit](https://github.com/bitcoinresearchkit/brk) API.

[npm](https://www.npmjs.com/package/brk-client) | [API Reference](https://github.com/bitcoinresearchkit/brk/blob/main/modules/brk-client/docs/globals.md)

## Installation

```bash
npm install brk-client
```

## Quick Start

```javascript
import { BrkClient } from 'brk-client';

// Use the free public API or your own instance
const client = new BrkClient('https://bitview.space');
// or: `const client = new BrkClient({ baseUrl: 'https://bitview.space', timeout: 10000 });`

// Blockchain data (mempool.space compatible)
const block = await client.getBlockByHeight(800000);
const tx = await client.getTx('abc123...');
const address = await client.getAddress('bc1q...');

// Metrics API - typed, chainable
const prices = await client.metrics.price.usd.split.close
  .by.dateindex
  .last(30); // Last 30 items

// Generic metric fetching
const data = await client.getMetric('price_close', 'dateindex', -30);
```
