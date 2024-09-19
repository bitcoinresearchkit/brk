# Changelog

<!--
## v. 0.X.Y | WIP
![Image of the kibō Web App version 0.X.Y](./assets/v0.X.Y.jpg)
-->

## v. 0.4.1 | WIP

<!-- ![Image of the kibō Web App version 0.4.1](./assets/v0.4.1.jpg) -->

## Website

- Fixed service worker not passing 304 (not modified) response and instead serving cached responses
- Fixed history not being properly registered
- Fixed prices on charts not having a wide enough background due to the font not being fully loaded during the creation of the chart
- Fixed window being moveable on iOS when in standalone mode when it shouldn't be

## Server

- Fixed links in several places missing the `/api` part and thus not working

## v. 0.4.0 | [861950](https://mempool.space/block/00000000000000000000530d0e30ccf7deeace122dcc99f2668a06c6dad83629) - 2024/09/19

![Image of the kibō Web App version 0.4.0](./assets/v0.4.0.jpg)

### Brand

- **Satonomics** is now **kibō** 🎉

### Website

- Complete redesign of the website
- Rewrote the whole application and removed `node`/`npm`/`pnpm` dependencies in favor for pure `HTML`/`CSS`/`Javascript`
- Website is now served by the server
- Added Trading View attribution link to the settings frame and file in the lightweight charts folder
- Many other changes

### Parser

- Changed the block iterator from a custom version of [bitcoin-explorer](https://crates.io/crates/bitcoin-explorer) to the homemade [biter](https://crates.io/crates/biter) which allows the parser to run alongside `bitcoind`
- Added datasets compression thanks to [zstd](https://crates.io/crates/zstd) to reduce disk usage
- Use the Bitcoin RPC server for various calls instead of running cli commands and then parsing the JSON from the output
- **Important database changes that will need a full rescan**:
  - Changed databases page size from 1MB to 4KB for improved disk usage
  - Split txid_to_tx_data database in 4096 chunks (from 256) for improved disk usage
  - Split address_index_to_X databases to chunks of 25_000 instead of 50_000
  - Removed local Multisig database
- Updated the config, run with `-h` to see possible args
- Moved outputs from `/target/outputs` to `/out` to allow to run commands like `cargo clean` without side effects
- Various first run fixes
- Added to `-h` which arguments are saved, which is all of them at the time of writing

### Server

- Updated the code to support compressed binaries
- Added serving of the website
- Improved `Cache-Control` behavior

## v. 0.3.0 | [853930](https://mempool.space/block/00000000000000000002eb5e9a7950ca2d5d98bd1ed28fc9098aa630d417985d) - 2024/07/26

![Image of the Satonomics Web App version 0.3.0](./assets/v0.3.0.jpg)

### Parser

- Global
  - Improved self-hosting by:
    - Fixing an incredibly annoying bug that made the program panic because of a wrong utxo/address durable state after a or many new datasets were added/changed after a first successful parse of the chain
    - Fixing a bug that would crash the program if launched for the first time ever
    - Auto fetch prices from the main Satonomics instance if missing instead of only trying Kraken's and Binance's API which are limited to the last 16 hours
  - Merged the core of `HeightMap` and `DateMap` structs into `GenericMap`
  - Added `Height` struct and many others
  - Reorganized outputs of both the parser and the server for ease of use and easier sync compatibility
- CLI
  - Added an argument parser for improved UX with several options
- Datasets
  - Added the following datasets for all entities:
    - Value destroyed
    - Value created
    - Spent Output Profit Ratio (SOPR)
  - Added the following ratio datasets and their variations to all prices {realized, moving average, any cointime, etc}:
    - Market Price to {X}
    - Market Price to {X} Ratio
    - Market Price to {X} Ratio 1 Week SMA
    - Market Price to {X} Ratio 1 Month SMA
    - Market Price to {X} Ratio 1 Year SMA
    - Market Price to {X} Ratio 1 Year SMA Momentum Oscillator
    - Market Price to {X} Ratio 99th Percentile
    - Market Price to {X} Ratio 99.5th Percentile
    - Market Price to {X} Ratio 99.9th Percentile
    - Market Price to {X} Ratio 1st Percentile
    - Market Price to {X} Ratio 0.5th Percentile
    - {X} 1% Top Probability
    - {X} 0.5% Top Probability
    - {X} 0.1% Top Probability
    - {X} 1% Bottom Probability
    - {X} 0.5% Bottom Probability
    - {X} 0.1% Bottom Probability
  - Added block metadatasets and their variants (raw/sum/average/min/max/percentiles):
    - Block size
    - Block weight
    - Block VBytes
    - Block interval
- Price
  - Improved error message when price cannot be found

### App

- General
  - Added chart scroll button for nice animations à la Wicked
  - Added scale mode switch (Linear/Logarithmic) at the bottom right of all charts
  - Added unit at the top left of all charts
  - Added a backup API in case the main one fails or is offline
  - Complete redesign of the datasets object
  - Removed import of routes in JSON in favor for hardcoded typed routes in string format which resulted in:
    - \+ A much lighter app
    - \+ Better Lighthouse score
    - \- Slower Typescript server
  - Fixed datasets with null values crashing their fetch function
  - Added a 'Go to a random chart' button in several places
- Chart
  - Fixed series color being set to default ones after hovering the legend
  - Fixed chart starting showing candlesticks and quickly switching to a line when it should've started directly with the line
  - Separated the QRCode generator library from the main chunk and made it imported on click
  - Fixed timescale changing on small screen after changing charts
- Folders
  - Added the size in the "filename" of address cohorts grouped by size
- Favorites
  - Added a 'favorite' and 'unfavorite' button at the bottom
- Settings
  - Removed the horizontal scroll bar which was unintended

### Server

- Run file
  - Only run with a watcher if `cargo watch` is available
  - Removed id_to_path file in favor for only `paths.d.ts` in `app/src/types`

## v. 0.2.0 | [851286](https://mempool.space/block/0000000000000000000281ca7f1bf8c50702bfca168c7af1bdc67c977c1ac8ed) - 2024/07/08

![Image of the Satonomics Web App version 0.2.0](./assets/v0.2.0.jpg)

### App

- General
  - Added the height version of all datasets and many optimizations to make them usable but only available on desktop and tablets for now
  - Added a light theme
- Charts
  - Added split panes in order to have the vertical axis visible for all datasets
  - Added min and max values on the charts
  - Fixed legend hovering on mobile not resetting on touch end
  - Added "3 months" and yearly time scale setters (from year 2009 to today)
  - Hide scrollbar of timescale setters and instead added scroll buttons to the legend only visible on desktop
  - Improved Share/QR Code screen
  - Changed all Area series to Line series
  - Fixed horizontal scrollable legend not updating on preset change
- Performance
  - Improved app's reactivity
  - Added some chunk splitting for a faster initial load
  - Global improvements that increased the Lighthouse's performance score
- Settings
  - Finally made a proper component where you can chose the app's theme, between a moving or static background and its text opacity
  - Added donations section with a leaderboard
  - Added various links that are visible on the bottom side of the strip on desktop to mobile users
  - Added install instructions when not installed for Apple users
- Misc
  - Support mini window size, could be useful for embedded views
  - Hopefully made scrollbars a little more subtle on WIndows and Linux, can't test
  - Generale style updates

### Parser

- Fixed ulimit only being run in Mac OS instead of whenever the program is detected

## v. 0.1.1 | [849240](https://mempool.space/block/000000000000000000002b8653988655071c07bb5f7181c038f9326bc86db741) - 2024/06/24

![Image of the Satonomics Web App version 0.1.1](./assets/v0.1.1.jpg)

### Parser

- Fixed overflow in `Price` struct which caused many Realized Caps and Realized Prices to have completely bogus data
- Fixed Realized Cap computation which was using rounded prices instead normal ones

### Server

- Added the chunk, date and time of the request to the terminal logs

### App

- Chart
  - Added double click option on a legend to toggle the visibility of all other series
  - Added highlight effect to a legend by darkening the color of all the other series on the chart while hovering it with the mouse
  - Added an API link in the legend for each dataset where applicable (when isn't generated locally)
  - Save fullscreen preference in local storage and url
  - Improved resize bar on desktop
  - Changed resize button logo
  - Changed the share button to visible on small screen too
  - Improved share screen
  - Fixed time range shifting not being the one in url params or saved in local storage
  - Fixed time range shifting on series toggling via the legend
  - Fixed time range shifting on fullscreen
  - Fixed time range shifting on resize of the sidebar
  - Set default view at first load to last 6 months
  - Added some padding around the datasets (year 1970 to 2100)
- History
  - Changed background for the sticky dates from blur to a solid color as it didn't appear properly in Firefox
- Build
  - Tried to add lazy loads to have split chunks after build, to have much faster load times and they worked great ! But they completely broke Safari on iOS, we can't have nice things
  - Removed many libraries and did some things manually instead to improve build size
- Strip
  - Temporarily removed the Home button on the strip bar on desktop as there is no landing page yet
- Settings
  - Added version
- PWA
  - Fixed background update
  - Changed update check frequency to 1 minute (~1kb to fetch every minute which is very reasonable)
  - Added a nice banner to ask the user to install the update
- Misc
  - Removed tracker even though it was a very privacy friendly as it appeared to not be working properly

### Price

- Deleted old price datasets and their backups

## v. 0.1.0 | [848642](https://mempool.space/block/000000000000000000020be5761d70751252219a9557f55e91ecdfb86c4e026a) - 2024/06/19

![Image of the Satonomics Web App version 0.1.0](./assets/v0.1.0.jpg)

## v. 0.0.X | [835444](https://mempool.space/block/000000000000000000009f93907a0dd83c080d5585cc7ec82c076d45f6d7c872) - 2024/03/20

![Image of the Satonomics Web App version 0.0.X](./assets/v0.0.X.jpg)
