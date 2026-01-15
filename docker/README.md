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
   cp docker/.env.example docker/.env
   ```
   Edit `docker/.env` and set `BITCOIN_DATA_DIR` to your Bitcoin Core data directory.

2. **Run with Docker Compose**
   ```bash
   docker compose -f docker/docker-compose.yml up -d
   ```

   Or from the docker directory:
   ```bash
   cd docker && docker compose up -d
   ```

3. **Access BRK**
   - Web interface: http://localhost:7070
   - API: http://localhost:7070/api
   - Health check: http://localhost:7070/health

## Architecture

BRK runs as a single container that includes both the blockchain processor and API server. This simplified architecture:
- Ensures processor and server are always in sync
- Simplifies deployment and monitoring
- Uses a single shared data directory

```bash
# Start BRK
docker compose -f docker/docker-compose.yml up

# Or run in background
docker compose -f docker/docker-compose.yml up -d

# Alternative: from docker directory
cd docker && docker compose up -d
```

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `BITCOIN_DATA_DIR` | Path to Bitcoin Core data directory | - |
| `BTC_RPC_HOST` | Bitcoin Core RPC host | `localhost` |
| `BTC_RPC_PORT` | Bitcoin Core RPC port | `8332` |
| `BTC_RPC_USER` | Bitcoin RPC username | - |
| `BTC_RPC_PASSWORD` | Bitcoin RPC password | - |
| `BRK_DATA_VOLUME` | Docker volume name for BRK data | `brk-data` |
| `BRK_FETCH` | Enable price fetching | `true` |

### Example .env File

```env
# Bitcoin Core paths
BITCOIN_DATA_DIR=/path/to/bitcoin/data
BRK_DATA_VOLUME=brk-data

# Bitcoin RPC configuration
BTC_RPC_HOST=localhost
BTC_RPC_PORT=8332
BTC_RPC_USER=your_username
BTC_RPC_PASSWORD=your_password

# BRK settings
BRK_FETCH=true
```

### Connecting to Bitcoin Core

#### Option 1: Cookie File Authentication (Recommended)
BRK will automatically use the `.cookie` file from your Bitcoin Core directory.

#### Option 2: Username/Password
Set `BTC_RPC_USER` and `BTC_RPC_PASSWORD` in your `docker/.env` file.

#### Network Connectivity
- **Same host**:
  - If Bitcoin Core is running natively (not in Docker): Use `host.docker.internal` on macOS/Windows or `172.17.0.1` on Linux
  - If Bitcoin Core is also in Docker: Use the service name or container IP
- **Remote host**: Use the actual IP address or hostname

## Building the Image

### Using Docker Compose
```bash
docker compose -f docker/docker-compose.yml build
```

### or ... Using Docker Build Script
```bash
# Build with default settings
./docker/docker-build.sh

# Build with custom tag
./docker/docker-build.sh --tag v1.0.0
```

## Volumes and Data Storage

BRK supports two options for storing its data:

### Option 1: Docker Named Volume (Default)
Uses a Docker-managed named volume called `brk-data`. This is the recommended approach for most users.

### Option 2: Bind Mount
Maps a specific directory on your host to the container's data directory.
This may be desirable if you want to use a specific storage location for BRK data (e.g. a different disk).

1. Set `BRK_DATA_DIR` in your `docker/.env` file to your desired host directory
2. In `docker/docker-compose.yml`, comment out the named volume line and uncomment the bind mount line

```bash
# In docker/.env file
BRK_DATA_DIR=/home/user/brk-data
```

```bash
# In docker/docker-compose.yml
# Comment out:
   - ${BRK_DATA_VOLUME:-brk-data}:/home/brk/.brk

# Uncomment:
   # - ${BRK_DATA_DIR:-./brk-data}:/home/brk/.brk
```

Can also remove or comment out the `volumes` section from the docker/docker-compose.yml file (right at the bottom):
```bash
# Comment out:
volumes:
  brk-data:
    driver: local
```

## Health Checks

The container includes a combined health check that verifies:
- The BRK process is running
- The API server is responding on port 3110

## Monitoring

### Check Container Status
```bash
# View running container
docker compose -f docker/docker-compose.yml ps

# Check health status
docker compose -f docker/docker-compose.yml ps --format \"table {{.Service}}\\t{{.Status}}\\t{{.Health}}\"
```

### View Logs
```bash
# View logs
docker compose -f docker/docker-compose.yml logs

# Follow logs in real-time
docker compose -f docker/docker-compose.yml logs -f
```

## Troubleshooting

### Server Issues

#### Server returns empty data
- This is normal if the processor hasn't indexed any blocks yet
- The server component will serve data as the processor indexes blocks

#### Server won't start
- Check Docker Compose logs: `docker compose -f docker/docker-compose.yml logs`
- Verify health endpoint: `curl http://localhost:7070/health`
- Ensure no port conflicts on 7070

### Processor Issues

#### Cannot connect to Bitcoin Core
1. Ensure Bitcoin Core is running with `-server=1`
2. Check RPC credentials are correct
3. Verify network connectivity from container
4. Test RPC connection: `docker compose -f docker/docker-compose.yml exec brk brk --help`

#### Processor fails to start
- Verify Bitcoin RPC credentials in `docker/.env`
- Ensure Bitcoin Core is running and accessible
- Check Bitcoin data directory permissions (should be readable by UID 1000)

### Performance Issues

#### Slow indexing
- Ensure adequate disk space for indexed data - a minimum of 3GB/s is recommended
- Monitor memory usage during initial indexing

#### Out of memory
- Increase Docker's memory limit
- Monitor container resource usage: `docker stats`

### Permission Issues

#### Permission denied errors
- Ensure the Bitcoin data directory is readable by the container user (UID 1000)
- Check that volumes are properly mounted
- Verify file ownership: `ls -la $BITCOIN_DATA_DIR`

### Network Issues

#### Cannot access web interface
- Verify port mapping: `docker compose -f docker/docker-compose.yml ps`
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
# Stop container
docker compose -f docker/docker-compose.yml down

# Restore from backup (named volume)
docker run --rm -v brk_brk-data:/target -v \"$(pwd)\":/backup alpine tar xzf /backup/brk-backup.tar.gz -C /target

# Start container
docker compose -f docker/docker-compose.yml up -d
```
