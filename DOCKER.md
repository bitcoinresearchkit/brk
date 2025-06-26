# Docker Setup for BRK

This guide explains how to run BRK using Docker and Docker Compose.

## Prerequisites

- Docker Engine (with buildx support)
- Docker Compose v2
- A running Bitcoin Core node with RPC enabled
- Access to Bitcoin Core's blocks directory

## Quick Start

1. **Create environment file**
   ```bash
   cp .env.example .env
   ```
   Edit `.env` and set `BITCOIN_DATA_DIR` to your Bitcoin Core data directory.

2. **Run with Docker Compose**
   ```bash
   docker compose up -d
   ```

3. **Access BRK**
   - Web interface: http://localhost:7070
   - API: http://localhost:7070/api

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `BITCOIN_DATA_DIR` | Path to Bitcoin Core data directory | Required |
| `BTC_RPC_HOST` | Bitcoin Core RPC host | `localhost` |
| `BTC_RPC_PORT` | Bitcoin Core RPC port | `8332` |
| `BRK_SERVICES` | Services to run (`all`, `processor`, `server`) | `all` |
| `BRK_COMPUTATION` | Computation mode (`lazy`, `eager`) | `lazy` |
| `BRK_FORMAT` | Data format (`raw`, `compressed`) | `raw` |
| `BRK_FETCH` | Enable price fetching | `true` |
| `BRK_MCP` | Enable MCP for AI/LLM | `true` |

### Service Modes

- **`all`**: Run both processor and server (default)
- **`processor`**: Only process blockchain data
- **`server`**: Only serve API/web interface

### Connecting to Bitcoin Core

#### Option 1: Cookie File Authentication (Recommended)
BRK will automatically use the `.cookie` file from your Bitcoin Core directory.

#### Option 2: Username/Password
1. Uncomment the RPC user/password lines in `docker-compose.yml`
2. Set `BTC_RPC_USER` and `BTC_RPC_PASSWORD` in your `.env` file

#### Network Connectivity
- **Same host**: 
  - If Bitcoin Core is running natively (not in Docker): Use `host.docker.internal` on macOS/Windows or `172.17.0.1` on Linux
  - If Bitcoin Core is also in Docker: Use the service name or container IP
- **Remote host**: Use the actual IP address or hostname

## Building the Image

### Using Docker Compose (Simple)
```bash
docker compose build
```

### Using Docker Build Script
```bash
# Build with default settings
./docker-build.sh

# Build with custom tag
./docker-build.sh --tag v1.0.0
```

## Volumes and Data Storage

BRK supports two options for storing its data:

### Option 1: Docker Named Volume (Default)
Uses a Docker-managed named volume called `brk-data`. This is the recommended approach for most users.

**Advantages:**
- Managed by Docker
- Easy backup/restore
- Platform-independent

**Usage:** This is the default configuration - no changes needed.

### Option 2: Bind Mount
Maps a specific directory on your host to the container's data directory.

**Advantages:**
- Direct access to files from host
- Easy to locate and manage
- Can be on specific storage devices

**Usage:**
1. Set `BRK_DATA_DIR` in your `.env` file to your desired host directory
2. In `docker-compose.yml`, comment out the named volume line and uncomment the bind mount line

**Example:**
```bash
# In .env file
BRK_DATA_DIR=/home/user/brk-data
```

```yaml
# In docker-compose.yml, uncomment and change as necessary:
      # - ${BRK_DATA_VOLUME:-brk-data}:/home/brk/.brk
      - ${BRK_DATA_DIR:-./brk-data}:/home/brk/.brk
```

### Volume Details
- **BRK data**: Stores computed datasets, indexes, and application state
- **Bitcoin data**: Mounted read-only from host (always a bind mount)

## Monitoring

View logs:
```bash
docker compose logs -f brk
```

Check status:
```bash
docker compose ps
```

## Troubleshooting

### Cannot connect to Bitcoin Core
1. Ensure Bitcoin Core is running with `-server=1`
2. Check RPC credentials are correct
3. Verify network connectivity from container

### Permission denied errors
Ensure the Bitcoin data directory is readable by the container user (UID 1000).

### Out of memory
Increase Docker's memory limit or use `BRK_COMPUTATION=lazy` to reduce memory usage.

## Security Considerations

- Bitcoin data is mounted read-only for safety
- BRK runs as non-root user inside container
- Only necessary ports are exposed
