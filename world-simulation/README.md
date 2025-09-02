# World Simulation - SpacetimeDB Implementation

A hierarchical world simulation with three interconnected levels: individuals, buildings, and cities. Built using SpacetimeDB for real-time state management and simulation.

## Overview

This simulation models a living world where:
- **Individuals** have physiological and psychological needs following Maslow's hierarchy
- **Buildings** serve as homes and workplaces with their own operational needs
- **Cities** emerge from the collective behavior of buildings and individuals

## Getting Started

### Prerequisites

Choose one of the following installation methods:

#### Option 1: Docker (Recommended)

1. Install Docker and Docker Compose on your system
2. Use the provided Docker setup (see Docker Installation below)

#### Option 2: Direct Installation

1. Install SpacetimeDB CLI:
```bash
curl -sSf https://install.spacetimedb.com | sh
```

2. Install Rust (if not already installed):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Running the Simulation

#### Docker Installation (Recommended)

1. Start the simulation with Docker:
```bash
./start-simulation.sh start
```

2. Run the demo:
```bash
./start-simulation.sh demo
```

3. Execute commands:
```bash
./start-simulation.sh exec call world-simulation get_city_summary 1
```

4. View logs:
```bash
./start-simulation.sh logs
```

5. Stop the simulation:
```bash
./start-simulation.sh stop
```

#### Direct Installation

1. Start SpacetimeDB server:
```bash
spacetime start
```

2. Build and publish the module:
```bash
cd world-simulation
spacetime publish .
```

3. Initialize the simulation:
```bash
spacetime call world-simulation init_simulation
```

4. Create a city:
```bash
spacetime call world-simulation create_city "New Angeles"
```

5. Create some buildings:
```bash
# Create homes
spacetime call world-simulation create_building "Apartment Complex 1" 1 '{"Home": {"capacity": 20, "rent": 500.0}}' 0.0 0.0
spacetime call world-simulation create_building "Apartment Complex 2" 1 '{"Home": {"capacity": 30, "rent": 400.0}}' 10.0 0.0

# Create workplaces
spacetime call world-simulation create_building "Factory 1" 1 '{"Workplace": {"job_type": "Factory", "positions": 50}}' 0.0 10.0
spacetime call world-simulation create_building "Office Park" 1 '{"Workplace": {"job_type": "Office", "positions": 100}}' 10.0 10.0

# Create amenities
spacetime call world-simulation create_building "Central Park" 1 "Park" 5.0 5.0
spacetime call world-simulation create_building "City Hospital" 1 "Hospital" -5.0 5.0
```

6. Create individuals:
```bash
# Create individuals with homes and jobs
spacetime call world-simulation create_individual "Alice Smith" 1 3
spacetime call world-simulation create_individual "Bob Johnson" 1 3
spacetime call world-simulation create_individual "Carol Williams" 2 4
spacetime call world-simulation create_individual "David Brown" 2 4
```

7. Start the simulation:
```bash
spacetime call world-simulation toggle_simulation
```

8. Advance time:
```bash
# Tick one hour
spacetime call world-simulation tick_hour

# Skip ahead 24 hours
spacetime call world-simulation skip_hours 24
```

## Querying the Simulation

### Individual Stories
```bash
# Get an individual's story for the last 24 hours
spacetime call world-simulation get_individual_story 1 24
```

### Building Activity
```bash
# Get building activity report for the last 48 hours
spacetime call world-simulation get_building_story 1 48
```

### City Summary
```bash
# Get city-wide summary
spacetime call world-simulation get_city_summary 1
```

### Hourly Narrative
```bash
# Generate narrative for current hour
spacetime call world-simulation generate_hourly_narrative 100
```

## Architecture

### Tables
- **Individual**: Tracks all individual entities with their needs and status
- **Building**: Manages buildings of various types (homes, workplaces, amenities)
- **City**: Tracks city-level metrics and development
- **Events**: Various event tables for tracking movements, work, social interactions

### Reducers
- **Time Management**: Controls simulation ticking and triggers updates
- **Individual Actions**: Handles need fulfillment, movement, and activities
- **Building Operations**: Manages daily building updates and resource flow
- **City Updates**: Processes weekly city-level changes
- **Narrative Generation**: Creates stories and summaries

### Systems
- **Needs System**: Implements the hierarchical need calculation logic
- **Modifiers**: Contains all constants for rates and thresholds
- **Priorities**: Determines action priorities and location selection

## Key Features

1. **Hierarchical Needs**: Five-level need system based on Maslow's hierarchy
2. **Real-time Updates**: Hourly individual, daily building, and weekly city updates
3. **Emergent Behavior**: Individual actions create building and city-level effects
4. **Resource Flow**: Production, consumption, and trade between entities
5. **Narrative Generation**: Rich story generation from simulation events

## Development

### Adding New Building Types

1. Add the type to `BuildingType` enum in `types.rs`
2. Update `create_location_capabilities` in `building.rs`
3. Add specific update logic in `update_building_daily`

### Adding New Needs

1. Update the `Individual` struct in `individual.rs`
2. Add update logic in `needs.rs`
3. Update priority calculations in `priorities.rs`
4. Add to the unified need mapping

### Extending Actions

1. Add to `IndividualAction` enum
2. Implement in `perform_action` function
3. Add appropriate costs/benefits in `modifiers.rs`

## Performance Considerations

- The simulation can handle 10,000+ individuals
- Batch updates minimize database operations
- In-memory state ensures fast queries
- Event logging can be toggled for performance

## Eno World Integration

This simulation includes data import capabilities for the rich Eno fantasy world:

### Real World Data
- **141 Cities**: From small villages to major trade hubs
- **1.8M Total Population**: Realistic population distribution
- **Geographic Data**: Real coordinates, elevations, and terrain
- **Detailed Buildings**: Actual building surveys with types and functions

### Scalable Import Options
- **Test Dataset**: 1 city, ~1K people for development
- **Small Dataset**: 3 cities, ~6K people for testing
- **Medium Dataset**: 5 cities, ~25K people for full simulation
- **Custom Datasets**: Import specific cities or population ranges

### Quick Start with Eno Data
```bash
# Import and run a small Eno-based simulation
spacetime call world-simulation init_simulation
spacetime call world-simulation import_small_dataset
spacetime call world-simulation toggle_simulation
spacetime call world-simulation skip_hours 168  # Run for a week
spacetime call world-simulation get_city_summary 1
```

See [demo.md](demo.md) for complete usage examples.

## Future Enhancements

- Full Eno dataset import (all 141 cities)
- Cultural and religious systems from Eno lore
- Trade routes between cities
- Political relationships and conflicts
- Seasonal and climate effects
- Historical events and their impacts
- Inter-city migration and communication
- Resource scarcity and abundance cycles