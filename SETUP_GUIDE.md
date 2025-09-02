# World Simulation Setup Guide - SpacetimeDB 1.1.2

## Prerequisites

- Docker and Docker Compose installed
- SpacetimeDB CLI 1.1.2+ installed on host (optional for direct commands)

## Quick Setup

### 1. Start the Simulation
```bash
./start-simulation.sh start
```

This will:
- Build and start the Docker container with SpacetimeDB 1.1.2
- Build the Rust WASM module
- Publish the module as `worldsim`
- Start the server on port 3001

### 2. Initialize and Import Data
```bash
./start-simulation.sh init    # Initialize simulation
./start-simulation.sh import  # Import test city (1000 people)
```

### 3. Run the Simulation
```bash
./start-simulation.sh tick    # Advance time by 1 hour
./start-simulation.sh status  # Check current status
```

## Manual Commands

All commands use the module name `worldsim` and connect to `http://localhost:3001`:

### Core Setup Commands
```bash
# Initialize simulation (creates time tracker)
spacetime call worldsim init_simulation

# Import data
spacetime call worldsim import_test_city        # 1 city, 1000 people
spacetime call worldsim import_small_dataset    # 3 cities, 2000 each
spacetime call worldsim import_medium_dataset   # 5 cities, 5000 each
```

### Simulation Commands
```bash
# Time progression
spacetime call worldsim tick_time              # Advance 1 hour
spacetime call worldsim update_individuals     # Update all individual needs
spacetime call worldsim update_buildings_daily # Update building maintenance
spacetime call worldsim update_cities_weekly   # Update city metrics

# Status queries
spacetime call worldsim get_simulation_status
spacetime call worldsim get_city_status city_id:1
spacetime call worldsim get_individual_needs individual_id:1
spacetime call worldsim get_building_status building_id:1
```

### Manual Entity Creation
```bash
# Create city
spacetime call worldsim create_city name:"Test City"

# Create building
spacetime call worldsim create_building \
  name:"Home 1" \
  city_id:1 \
  building_type:'{"Home":{"capacity":4,"rent":500.0}}' \
  x:0.0 \
  y:0.0

# Create individual
spacetime call worldsim create_individual \
  name:"Alice" \
  home_id:1 \
  workplace_id:null
```

## Helper Script Commands

The `start-simulation.sh` script provides convenient shortcuts:

```bash
./start-simulation.sh start     # Start everything
./start-simulation.sh stop      # Stop containers
./start-simulation.sh demo      # Run full demo
./start-simulation.sh logs      # View logs
./start-simulation.sh shell     # Open container shell
./start-simulation.sh rebuild   # Rebuild and republish module
./start-simulation.sh init      # Initialize simulation
./start-simulation.sh import    # Import test data
./start-simulation.sh tick      # Advance time
./start-simulation.sh status    # Check status
```

## Database Schema

The simulation uses these main tables:
- `simulation_time` - Current hour and time tracking
- `city` - City-level metrics and data
- `building` - Building structures and maintenance
- `home_data` - Home-specific data (rent, utilities)
- `workplace_data` - Workplace production/consumption
- `individual` - People with needs and status
- `location_capability` - What services each building provides

## API Changes from 0.8.2 to 1.1.2

- Module name: `world-simulation` → `worldsim`
- Table access: `Individual::filter_by_id(&id)` → `ctx.db.individual().id().find(&id)`
- Attributes: `#[spacetimedb(table)]` → `#[spacetimedb::table(name = table_name)]`
- Reducers: `#[spacetimedb(reducer)]` → `#[spacetimedb::reducer]`
- Logging: `println!` → `log::info!`
- Context: All reducers now require `ctx: &ReducerContext` parameter

## Port Configuration

- SpacetimeDB server: `localhost:3001`
- Changed from port 3000 to avoid conflicts
- Update `docker-compose.yml` if different port needed

## Troubleshooting

### Common Issues
1. **Port 3001 in use**: Change port in `docker-compose.yml` and `Dockerfile`
2. **Module publish fails**: Check module name is `worldsim` in `spacetimedb.toml`
3. **WASM compilation errors**: Ensure `wasm32-unknown-unknown` target installed
4. **Type conflicts**: Check for duplicate type definitions between files

### Debug Commands
```bash
# Check module status
spacetime list --server http://localhost:3001

# View table contents
spacetime query --server http://localhost:3001 "SELECT * FROM simulation_time"

# Check logs
docker-compose logs spacetimedb
```