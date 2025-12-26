# Hosting

## Self-Hosting

BRK is designed to be self-hosted. Install and run with:

```bash
cargo install --locked brk_cli
brk --bitcoindir ~/.bitcoin --brkdir ~/.brk
```

Requirements:
- Bitcoin Core with accessible `blk*.dat` files
- ~400 GB disk space
- 12+ GB RAM recommended

See the [CLI README](../crates/brk_cli/README.md) for configuration options.

## Professional Hosting

Need a managed instance? We offer professional hosting services.

**What's Included:**
- Dual dedicated servers (1 GB/s) with redundant ISPs
- Cloudflare integration for global performance
- 99.99% uptime SLA
- Automatic updates and maintenance
- Direct support channel
- Custom Bitcoin Core/Knots versions
- Optional branded subdomains

**Pricing:**
- Monthly: 0.01 BTC
- Yearly: 0.1 BTC

**Contact:** [hosting@bitcoinresearchkit.org](mailto:hosting@bitcoinresearchkit.org)
