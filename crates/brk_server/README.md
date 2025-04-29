# BRK Server

<p align="left">
  <a href="https://github.com/bitcoinresearchkit/brk">
    <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/bitcoinresearchkit/brk?style=social">
  </a>
  <a href="https://kibo.money">
    <img alt="kibo.money" src="https://img.shields.io/badge/showcase-kib%C5%8D.money-orange">
  </a>
  <a href="https://github.com/bitcoinresearchkit/brk/blob/main/LICENSE.md">
    <img src="https://img.shields.io/crates/l/brk" alt="License" />
  </a>
  <a href="https://crates.io/crates/brk_server">
    <img src="https://img.shields.io/crates/v/brk_server" alt="Version" />
  </a>
  <a href="https://docs.rs/brk_server">
    <img src="https://img.shields.io/docsrs/brk_server" alt="Documentation" />
  </a>
  <img src="https://img.shields.io/crates/size/brk_server" alt="Size" />
  <a href="https://deps.rs/crate/brk_server">
    <img src="https://deps.rs/crate/brk_server/latest/status.svg" alt="Dependency status">
  </a>
  <a href="https://discord.gg/HaR3wpH3nr">
    <img src="https://img.shields.io/discord/1350431684562124850?label=discord" alt="Discord" />
  </a>
  <a href="https://primal.net/p/nprofile1qqsfw5dacngjlahye34krvgz7u0yghhjgk7gxzl5ptm9v6n2y3sn03sqxu2e6">
    <img src="https://img.shields.io/badge/nostr-purple?link=https%3A%2F%2Fprimal.net%2Fp%2Fnprofile1qqsfw5dacngjlahye34krvgz7u0yghhjgk7gxzl5ptm9v6n2y3sn03sqxu2e6" alt="Nostr" />
  </a>
  <a href="https://bsky.app/profile/bitcoinresearchkit.org">
    <img src="https://img.shields.io/badge/bluesky-blue?link=https%3A%2F%2Fbsky.app%2Fprofile%2Fbitcoinresearchkit.org" alt="Bluesky" />
  </a>
  <a href="https://x.com/brkdotorg">
    <img src="https://img.shields.io/badge/x.com-black" alt="X" />
  </a>
</p>

A crate that serves Bitcoin data and swappable front-ends, built on top of `brk_indexer`, `brk_computer` and `brk_query`.

The file handler, will serve the website specified by the user if any, which can be *no website*, *kibo.money* or *custom* (which is a blank folder for people to experiment). If a website is specified and the server is ran outside of the brk project and thus can't find the requested website, it will download the whole project with the correct version from Github and store it in `.brk` to be able to serve to website. This is due to the crate size limit on [crates.io](https://crates.io) and the various shenanigans that need to be done to have a website in a crate.

The API uses `brk_query` and so inherites all of its features including formats.

## Endpoints

### API

#### `GET /api/vecs/indexes`

A list of all possible vec indexes and their accepted variants

#### `GET /api/vecs/ids`

A list of all possible vec ids

#### `GET /api/vecs/id-to-indexes`

A list of all possible vec ids and their supported vec indexes

#### `GET /api/vecs/index-to-ids`

A list of all possible vec indexes and their supported vec ids

#### `GET /api/query`

This endpoint retrieves data based on the specified vector index and values.

**Parameters:**

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `index` | `VecIndex` | Yes | The vector index to query. |
| `values` | `VecId[]` | Yes | A comma or space-separated list of vector IDs to retrieve. |
| `from` | `unsigned int` | No | The starting index for pagination (default is 0). |
| `to` | `unsigned int` | No | The ending index for pagination (default is the total number of results). |
| `format` | `string` | No | The format of the response. Options include `json`, `csv`, `tsv`, or `md` (default is `json`). |

**Examples:**

```
GET /api/query?index=date&values=ohlc
GET /api/query?index=week&values=ohlc,block-interval-average&from=0&to=20&format=md
```

### Meta

#### `GET /version`

The version of the server and thus BRK.

### Files

#### `GET /*`

Catch all.

When no pattern is found, the server will look for a match inside the folder of the chosen website, if any.
