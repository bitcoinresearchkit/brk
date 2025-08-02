# BRK Server

<p align="left">
  <a href="https://github.com/bitcoinresearchkit/brk">
    <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/bitcoinresearchkit/brk?style=social">
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
</p>

A crate that serves Bitcoin data and swappable front-ends, built on top of `brk_indexer`, `brk_computer` and `brk_interface`.

The file handler, will serve the website specified by the user if any, which can be *no website*, *default* or *custom* (which is a blank folder for people to experiment). If a website is specified and the server is ran outside of the brk project and thus can't find the requested website, it will download the whole project with the correct version from Github and store it in `.brk` to be able to serve to website. This is due to the crate size limit on [crates.io](https://crates.io) and the various shenanigans that need to be done to have a website in a crate.

The API uses `brk_interface` and so inherites all of its features including formats.

## Endpoints

### API

#### [`GET /api/vecs/index-count`](https://bitcoinresearchkit.org/api/vecs/index-count)

Get the count of all existing indexes.

#### [`GET /api/vecs/id-count`](https://bitcoinresearchkit.org/api/vecs/id-count)

Get the count of all existing vec ids.

#### [`GET /api/vecs/vec-count`](https://bitcoinresearchkit.org/api/vecs/vec-count)

Get the count of all existing vecs. \
Equals to the sum of supported Indexes of each vec id.

#### [`GET /api/vecs/indexes`](https://bitcoinresearchkit.org/api/vecs/indexes)

Get the list of all existing indexes.

#### [`GET /api/vecs/accepted-indexes`](https://bitcoinresearchkit.org/api/vecs/accepted-indexes)

Get an object which has all existing indexes as keys and a list of their accepted variants as values.

#### [`GET /api/vecs/ids`](https://bitcoinresearchkit.org/api/vecs/ids)

Get a paginated list of all existing vec ids. \
There are up to 1,000 values per page. \
If the `page` param is omitted, it will default to page `0`.

#### [`GET /api/vecs/index-to-ids`](https://bitcoinresearchkit.org/api/vecs/index-to-ids)

Get a paginated list of all vec ids which support a given index.
There are up to 1,000 values per page.
If the `page` param is omitted, it will default to the first page.

#### [`GET /api/vecs/id-to-indexes`](https://bitcoinresearchkit.org/api/vecs/id-to-indexes)

Get a list of all indexes supported by a given vec id.
The list will be empty if the vec id isn't correct.

#### `GET /api/vecs/{INDEX}-to-{ID}`

This endpoint retrieves data based on the specified vector index and id.

**Parameters:**

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `from` | `signed int` | No | Inclusive starting index for pagination (default is 0). |
| `to` | `signed int` | No | Exclusive ending index for pagination (default is the total number of results). Overrides `count` |
| `count` | `unsigned int` | No | The number of values requested |
| `format` | `string` | No | The format of the response. Options include `json`, `csv`, `tsv`, or `md` (default is `json`). |

**Examples:**

```sh
# GET /api/vecs/date-to-close
curl https://bitcoinresearchkit.org/api/vecs/date-to-close

# GET /api/vecs/date-to-close?from=-100
curl https://bitcoinresearchkit.org/api/vecs/date-to-close?from=-100

# GET /api/vecs/date-to-close?count=100&format=csv
curl https://bitcoinresearchkit.org/api/vecs/date-to-close?count=100&format=csv
```

#### `GET /api/vecs/query`

Get one or multiple vecs depending on given parameters.
If you'd like to request multiple vec ids, simply separate them with a ','. \
To get the last value set `-1` to the `from` parameter. \
The response's format will depend on the given parameters, it will be:
- A value: If requested only one vec and the given range returns one value (for example: `from=-1`)
- A list: If requested only one vec and the given range returns multiple values (for example: `from=-1000&count=100` or `from=-444&to=-333`)
- A matrix: When multiple vecs are requested, even if they each return one value.

**Parameters:**

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `index` | `VecIndex` | Yes | The vector index to query. |
| `ids` | `VecId[]` | Yes | A comma or space-separated list of vector IDs to retrieve. |
| `from` | `signed int` | No | Inclusive starting index for pagination (default is 0). |
| `to` | `signed int` | No | Exclusive ending index for pagination (default is the total number of results). Overrides `count` |
| `count` | `unsigned int` | No | The number of values requested |
| `format` | `string` | No | The format of the response. Options include `json`, `csv`, `tsv`, or `md` (default is `json`). |

**Examples:**

```sh
# GET /api/vecs/query?index=date&ids=ohlc
curl https://bitcoinresearchkit.org/api/vecs/query?index=date&ids=ohlc

# GET /api/vecs/query?index=week&ids=ohlc,block-interval-average&from=0&to=20&format=md
curl https://bitcoinresearchkit.org/api/vecs/query?index=week&ids=ohlc,block-interval-average&from=0&to=20&format=md
```

### Meta

#### [`GET /version`](https://bitcoinresearchkit.org/version)

The version of the server and thus BRK.

### Files

#### `GET /*`

Catch all.

When no pattern is found, the server will look for a match inside the folder of the chosen website, if any.
