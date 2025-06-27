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
   # Multi-container mode (recommended)
   docker compose up -d brk-processor brk-server
   ```

3. **Access BRK**
   - Web interface: http://localhost:7070
   - API: http://localhost:7070/api
   - Health check: http://localhost:7070/health

## Deployment Modes

BRK supports flexible deployment modes to suit different use cases:

### 1. Multi-Container Mode (Recommended)

Deploy indexer and server as separate containers. This means the following:
- Better resource isolation
- Independent scaling of components
- Server doesn't need Bitcoin Core or RPC access
- Cleaner failure isolation
- Server can start independently (will serve empty data until processor indexes blocks)

```bash
# Start both processor and server
docker compose up brk-processor brk-server

# Or run in background
docker compose up -d brk-processor brk-server
```

### 2. Processor-Only Mode

For indexing without web interface:

```bash
docker compose up brk-processor
```

### 3. Server-Only Mode

For serving pre-indexed data:

```bash
docker compose up brk-server
```

## Configuration

### Environment Variables

| Variable | Description | Default | Required For |
|----------|-------------|---------|-------------|
| `BITCOIN_DATA_DIR` | Path to Bitcoin Core data directory | Required | Processor |
| `BTC_RPC_HOST` | Bitcoin Core RPC host | `localhost` | Processor |
| `BTC_RPC_PORT` | Bitcoin Core RPC port | `8332` | Processor |
| `BTC_RPC_USER` | Bitcoin RPC username | - | Processor |
| `BTC_RPC_PASSWORD` | Bitcoin RPC password | - | Processor |
| `BRK_DATA_VOLUME` | Docker volume name for BRK data | `brk-data` | Both |
| `BRK_COMPUTATION` | Computation mode (`lazy`, `eager`) | `lazy` | Processor |
| `BRK_FORMAT` | Data format (`raw`, `compressed`) | `raw` | Processor |
| `BRK_FETCH` | Enable price fetching | `true` | Processor |
| `BRK_MCP` | Enable MCP for AI/LLM | `true` | Server |

### Example .env File

```env
# Bitcoin Core paths
BITCOIN_DATA_DIR=/path/to/bitcoin/data
BRK_DATA_VOLUME=brk-data

# Bitcoin RPC (required for processor)
BTC_RPC_HOST=localhost
BTC_RPC_PORT=8332
BTC_RPC_USER=your_username
BTC_RPC_PASSWORD=your_password

# BRK settings
BRK_COMPUTATION=lazy
BRK_FORMAT=raw
BRK_FETCH=true
BRK_MCP=true
```

### Connecting to Bitcoin Core

#### Option 1: Cookie File Authentication (Recommended)
BRK will automatically use the `.cookie` file from your Bitcoin Core directory.

#### Option 2: Username/Password
Set `BTC_RPC_USER` and `BTC_RPC_PASSWORD` in your `.env` file.

#### Network Connectivity
- **Same host**: 
  - If Bitcoin Core is running natively (not in Docker): Use `host.docker.internal` on macOS/Windows or `172.17.0.1` on Linux
  - If Bitcoin Core is also in Docker: Use the service name or container IP
- **Remote host**: Use the actual IP address or hostname

## Building the Image

### Using Docker Compose...
```bash
docker compose build
```

### or ... Using Docker Build Script
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

### Option 2: Bind Mount
Maps a specific directory on your host to the container's data directory.
This may be desirable if you want to use a specific storage location for BRK data (e.g. a different disk).

1. Set `BRK_DATA_DIR` in your `.env` file to your desired host directory
2. In `docker-compose.yml`, comment out the named volume line and uncomment the bind mount line

```bash
# In .env file
BRK_DATA_DIR=/home/user/brk-data
```

```bash
# In docker-compose.yml, for BOTH the processor and server services.
# Comment out:
   - ${BRK_DATA_VOLUME:-brk-data}:/home/brk/.brk

# Uncomment:
   # - ${BRK_DATA_DIR:-./brk-data}:/home/brk/.brk
```

Can also remove or comment out the `volumes` section from the docker-compose.yml file (right at the bottom):
```bash
# Comment out:
volumes:
  brk-data:
    driver: local
```

## Health Checks

Both containers include health checks:

- `brk-processor`: checks that the BRK process is running
- `brk-server`: tests network connectivity on port 3110

## Monitoring

### Check Container Status
```bash
# View running containers
docker compose ps

# Check health status
docker compose ps --format \"table {{.Service}}\\t{{.Status}}\\t{{.Health}}\"
```

### View Logs
```bash
# View logs from both containers
docker compose logs brk-processor brk-server

# Follow logs in real-time
docker compose logs -f brk-processor brk-server

# View logs from specific container
docker compose logs -f brk-server
```

## Troubleshooting

### Server Issues

#### Server returns empty data
- This is normal if processor hasn't indexed any blocks yet
- Server can start before processor and will serve data as it becomes available
- Check that BRK data volume is properly shared between containers

#### Server won't start
- Check Docker Compose logs: `docker compose logs brk-server`
- Verify health endpoint: `curl http://localhost:7070/health`
- Ensure no port conflicts on 7070

### Processor Issues

#### Cannot connect to Bitcoin Core
1. Ensure Bitcoin Core is running with `-server=1`
2. Check RPC credentials are correct
3. Verify network connectivity from container
4. Test RPC connection: `docker compose exec brk-processor brk --help`

#### Processor fails to start
- Verify Bitcoin RPC credentials in `.env`
- Ensure Bitcoin Core is running and accessible
- Check Bitcoin data directory permissions (should be readable by UID 1000)

### Performance Issues

#### Slow indexing
- Ensure adequate disk space for indexed data
- Monitor memory usage during initial indexing
- Use `BRK_COMPUTATION=lazy` to reduce memory usage

#### Out of memory
- Increase Docker's memory limit
- Use `BRK_COMPUTATION=lazy` mode
- Monitor container resource usage: `docker stats`

### Permission Issues

#### Permission denied errors
- Ensure the Bitcoin data directory is readable by the container user (UID 1000)
- Check that volumes are properly mounted
- Verify file ownership: `ls -la $BITCOIN_DATA_DIR`

### Network Issues

#### Cannot access web interface
- Verify port mapping: `docker compose ps`
- Check firewall settings
- Ensure no other services are using port 7070

## Security Considerations

- Bitcoin data is mounted read-only for safety
- BRK runs as non-root user inside container
- Only necessary ports are exposed

## Backup and Recovery

### Backing Up BRK Data

```bash
# Create backup of named volume
docker run --rm -v brk_brk-data:/source -v \"$(pwd)\":/backup alpine tar czf /backup/brk-backup.tar.gz -C /source .

# Or if using bind mount
tar czf brk-backup.tar.gz -C \"$BRK_DATA_DIR\" .
```

### Restoring BRK Data

```bash
# Stop containers
docker compose down

# Restore from backup (named volume)
docker run --rm -v brk_brk-data:/target -v \"$(pwd)\":/backup alpine tar xzf /backup/brk-backup.tar.gz -C /target

# Start containers
docker compose up -d
```