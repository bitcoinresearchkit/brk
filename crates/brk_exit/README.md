# BRK Exit

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
  <a href="https://crates.io/crates/brk_exit">
    <img src="https://img.shields.io/crates/v/brk_exit" alt="Version" />
  </a>
  <a href="https://docs.rs/brk_exit">
    <img src="https://img.shields.io/docsrs/brk_exit" alt="Documentation" />
  </a>
  <img src="https://img.shields.io/crates/size/brk_exit" alt="Size" />
  <a href="https://deps.rs/crate/brk_exit">
    <img src="https://deps.rs/crate/brk_exit/latest/status.svg" alt="Dependency status">
  </a>
  <a href="https://discord.gg/Cvrwpv3zEG">
    <img src="https://img.shields.io/discord/1350431684562124850?label=discord" alt="Discord" />
  </a>
  <a href="https://primal.net/p/nprofile1qqsfw5dacngjlahye34krvgz7u0yghhjgk7gxzl5ptm9v6n2y3sn03sqxu2e6">
    <img src="https://img.shields.io/badge/nostr-purple?link=https%3A%2F%2Fprimal.net%2Fp%2Fnprofile1qqsfw5dacngjlahye34krvgz7u0yghhjgk7gxzl5ptm9v6n2y3sn03sqxu2e6" alt="Nostr" />
  </a>
  <a href="https://bsky.app/profile/bitcoinresearchkit.org">
    <img src="https://img.shields.io/badge/bluesky-blue?link=https%3A%2F%2Fbsky.app%2Fprofile%2Fbitcoinresearchkit.org" alt="Bluesky" />
  </a>
  <a href="https://x.com/0xbrk">
    <img src="https://img.shields.io/badge/x.com-black" alt="X" />
  </a>
</p>

A simple crate that stops the program from exitting when blocking is activated until it is released. The purpose of that is to prevent exitting when a program is in the middle of saving data and thus prevent partial writes.

It's built on top of [ctrlc](https://crates.io/crates/ctrlc) which handles Ctrl + C (SIGINT), stopping the program using the `kill` command (SIGTERM) and closing the terminal (SIGHUP) but it doesn't support force kills (`kill -9`).
