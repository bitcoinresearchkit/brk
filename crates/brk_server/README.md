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
  <a href="https://discord.gg/Cvrwpv3zEG">
    <img src="https://img.shields.io/discord/1350431684562124850" alt="Chat" />
  </a>
</p>

A crate that serves Bitcoin data and swappable front-ends, built on top of `brk_indexer`, `brk_computer` and `brk_query`.

The file handler, will serve the website specified by the user if any, which can be *no website*, *kibo.money* or *custom* (which is a blank folder for people to experiment). If a website is specified and the server is ran outside of the brk project and thus can't find the requested website, it will download the whole project with the correct version from Github and store it in `.brk` to be able to serve to website. This is due to the crate size limit on [crates.io](https://crates.io) and the various shenanigans that need to be done to have a website in a crate.

The API uses `brk_query` and so inherites all of its features including formats.
