# Docker Setup for BRK

## Prerequisites

- Docker Engine (with buildx support)
- Docker Compose v2
- A running Bitcoin Core node with RPC enabled

## Quick Start

1. **Create environment file**
   ```bash
   cp docker/.env.example docker/.env
   ```
   Edit `docker/.env` and set `BITCOIN_DATA_DIR` to your Bitcoin Core data directory.

2. **Run with Docker Compose**
   ```bash
   docker compose -f docker/docker-compose.yml up -d
   ```

3. **Access BRK**
   - Web interface: http://localhost:7070
   - API: http://localhost:7070/api
   - Health check: http://localhost:7070/health

## Configuration

All configuration is passed via CLI args in `docker-compose.yml`. Edit the `command:` section to change settings.

### Environment Variables

These variables are interpolated into `docker-compose.yml` at startup:

| Variable | Description | Default |
|----------|-------------|---------|
| `BITCOIN_DATA_DIR` | Path to Bitcoin Core data directory | - |
| `BTC_RPC_HOST` | Bitcoin Core RPC host | `localhost` |
| `BTC_RPC_USER` | Bitcoin RPC username | `bitcoin` |
| `BTC_RPC_PASSWORD` | Bitcoin RPC password | `bitcoin` |
| `BRK_DATA_VOLUME` | Docker volume name for BRK data | `brk-data` |

### Connecting to Bitcoin Core

**Cookie File Authentication (Recommended)**
Uncomment the `--rpccookiefile` lines in `docker-compose.yml` and remove `--rpcuser`/`--rpcpassword`.

**Username/Password**
Set `BTC_RPC_USER` and `BTC_RPC_PASSWORD` in your `docker/.env` file.

**Network Connectivity**
- **Same host (Bitcoin Core running natively)**: Use `host.docker.internal` on macOS/Windows or `172.17.0.1` on Linux
- **Same host (Bitcoin Core in Docker)**: Use the service name or container IP
- **Remote host**: Use the actual IP address or hostname

## Building

```bash
docker compose -f docker/docker-compose.yml build
```

## Data Storage

### Named Volume (Default)
Uses a Docker-managed volume called `brk-data`.

### Bind Mount
1. Set `BRK_DATA_DIR` in `docker/.env`
2. In `docker-compose.yml`, comment out the named volume line and uncomment the bind mount line
3. Remove the `volumes:` section at the bottom of `docker-compose.yml`

## Monitoring

```bash
docker compose -f docker/docker-compose.yml ps
docker compose -f docker/docker-compose.yml logs -f
```

## Troubleshooting

### Cannot connect to Bitcoin Core
1. Ensure Bitcoin Core is running with `-server=1`
2. Check RPC credentials are correct
3. Verify network connectivity from container

### Permission denied errors
Ensure the Bitcoin data directory is readable by the container user (UID 1000).

## Security

- Bitcoin data is mounted read-only
- BRK runs as non-root user inside container
- Only necessary ports are exposed
