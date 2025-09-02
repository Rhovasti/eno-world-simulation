# World Simulation Installation Guide

This guide provides solutions for the installation issues with world-simulation.

## Problem

The direct installation of SpacetimeDB fails with "not a terminal" error when using:
```bash
curl -sSf https://install.spacetimedb.com | sh
```

## Solutions

### Solution 1: Docker (Recommended)

We've created a complete Docker setup that handles all dependencies and installation automatically.

#### Prerequisites
- Docker
- Docker Compose

#### Installation Steps

1. **Build and start the container:**
   ```bash
   ./start-simulation.sh start
   ```

2. **Run a demo simulation:**
   ```bash
   ./start-simulation.sh demo
   ```

3. **Execute SpacetimeDB commands:**
   ```bash
   # General format
   ./start-simulation.sh exec [spacetime commands]
   
   # Examples
   ./start-simulation.sh exec call world-simulation get_city_summary 1
   ./start-simulation.sh exec call world-simulation tick_hour
   ```

4. **View logs:**
   ```bash
   ./start-simulation.sh logs
   ```

5. **Stop the simulation:**
   ```bash
   ./start-simulation.sh stop
   ```

### Solution 2: Manual Installation (Alternative)

If Docker is not available, try these alternative installation methods:

#### Method A: Download Pre-built Binary

1. Download the latest SpacetimeDB binary:
   ```bash
   wget https://github.com/clockworklabs/SpacetimeDB/releases/latest/download/spacetime.linux-amd64.tar.gz
   tar -xzf spacetime.linux-amd64.tar.gz
   sudo mv spacetime /usr/local/bin/
   sudo chmod +x /usr/local/bin/spacetime
   ```

2. Verify installation:
   ```bash
   spacetime --version
   ```

#### Method B: Build from Source

1. Clone SpacetimeDB repository:
   ```bash
   git clone https://github.com/clockworklabs/SpacetimeDB.git
   cd SpacetimeDB
   ```

2. Build with Cargo:
   ```bash
   cargo build --release
   sudo cp target/release/spacetime /usr/local/bin/
   ```

## Quick Start After Installation

### With Docker:
```bash
# Start everything
./start-simulation.sh start

# Run demo
./start-simulation.sh demo
```

### With Direct Installation:
```bash
# Start server
spacetime start

# In another terminal
cd world-simulation
spacetime publish .
spacetime call world-simulation init_simulation
spacetime call world-simulation create_city "Demo City"
```

## Troubleshooting

### Docker Issues

1. **Permission denied on start-simulation.sh**
   ```bash
   chmod +x start-simulation.sh
   ```

2. **Docker not found**
   - Install Docker: https://docs.docker.com/get-docker/
   - Install Docker Compose: https://docs.docker.com/compose/install/

3. **Port 3000 already in use**
   - Edit docker-compose.yml and change the port mapping
   - Or stop the conflicting service

### Direct Installation Issues

1. **spacetime: command not found**
   - Add to PATH: `export PATH=$PATH:/usr/local/bin`
   - Or use full path: `/usr/local/bin/spacetime`

2. **Permission denied**
   - Use sudo for installation commands
   - Check file permissions

## Additional Notes

- The Docker setup includes automatic module building and publishing
- Data is persisted in Docker volumes
- The simulation runs on port 3000 by default
- All SpacetimeDB data is stored in the container's volume

## Support

If you continue to experience issues:
1. Check the SpacetimeDB documentation: https://docs.spacetimedb.com
2. Review the error logs: `./start-simulation.sh logs`
3. Ensure all prerequisites are installed correctly