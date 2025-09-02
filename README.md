# World Simulation - SpacetimeDB 1.1.2

A hierarchical world simulation running on SpacetimeDB that models individuals, buildings, and cities through interconnected need systems.

## Quick Start

### Prerequisites
- Docker and Docker Compose
- SpacetimeDB CLI 1.1.2+ installed

### Running the Simulation

1. **Start the simulation:**
   ```bash
   docker-compose up
   ```

2. **Initialize the simulation:**
   ```bash
   spacetime call worldsim init_simulation
   ```

3. **Import test data:**
   ```bash
   # Single test city (1000 population)
   spacetime call worldsim import_test_city
   
   # Small dataset (3 cities, 2000 each)
   spacetime call worldsim import_small_dataset
   
   # Medium dataset (5 cities, 5000 each)
   spacetime call worldsim import_medium_dataset
   ```

4. **Start time progression:**
   ```bash
   spacetime call worldsim tick_time
   ```

### Key Commands

**Setup:**
- `init_simulation` - Initialize the simulation
- `import_test_city` - Create a test city with 1000 people
- `import_small_dataset` - Create 3 test cities

**Simulation Control:**
- `tick_time` - Advance time by 1 hour
- `update_individuals` - Process individual needs
- `update_buildings_daily` - Process building maintenance (every 24 hours)
- `update_cities_weekly` - Process city metrics (every 168 hours)

**Queries:**
- `get_simulation_status` - Current time and overall stats
- `get_city_status city_id:1` - Detailed city information
- `get_individual_needs individual_id:1` - Person's current needs
- `get_building_status building_id:1` - Building details

### Architecture

The simulation operates on three interconnected levels:

1. **Individual Level** - People with physiological/psychological needs
2. **Building Level** - Structures that serve human occupants  
3. **City Level** - Urban systems emerging from building/human interactions

### Time System

- **Tick Rate**: Every hour
- **Individual Updates**: Every hour
- **Building Updates**: Every 24 hours
- **City Updates**: Every 168 hours (1 week)

### Need System

All entities share 5 fundamental needs:
- **Environment** - Safety, comfort, livability
- **Consumption** - Resource intake and usage  
- **Connection** - Social bonds and networks
- **Rest** - Recovery and maintenance
- **Waste** - Byproduct management

### Development

**Building:**
```bash
cd world-simulation
cargo build --target wasm32-unknown-unknown
```

**Publishing:**
```bash
spacetime publish worldsim
```

See [CLAUDE.md](CLAUDE.md) for detailed project guidelines and [modifiers.md](modifiers.md) for all numerical rates and modifiers.