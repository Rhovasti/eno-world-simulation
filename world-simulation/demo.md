# World Simulation with Eno Data - Demo Guide

This guide shows how to run the SpacetimeDB world simulation with Eno-inspired data.

## Quick Start

### 1. Start SpacetimeDB Server
```bash
spacetime start
```

### 2. Build and Publish Module
```bash
cd world-simulation
spacetime publish .
```

### 3. Initialize Simulation
```bash
# Initialize the simulation system
spacetime call world-simulation init_simulation

# Import a test city (Guild - largest in Eno)
spacetime call world-simulation import_test_city

# Or import a small dataset (3 cities, max 2000 people each)
spacetime call world-simulation import_small_dataset

# Start the simulation
spacetime call world-simulation toggle_simulation
```

### 4. Run the Simulation
```bash
# Advance time hour by hour
spacetime call world-simulation tick_hour

# Or skip ahead 24 hours (1 day)
spacetime call world-simulation skip_hours 24

# Or skip a week to see city-level changes
spacetime call world-simulation skip_hours 168
```

### 5. Query Results

#### City Summary
```bash
spacetime call world-simulation get_city_summary 1
```

#### Individual Stories
```bash
# Get the story of individual #1 for the last 24 hours
spacetime call world-simulation get_individual_story 1 24
```

#### Building Activity
```bash
# Get building activity for the last 48 hours
spacetime call world-simulation get_building_story 1 48
```

#### Hourly Narrative
```bash
# Generate narrative for hour 100
spacetime call world-simulation generate_hourly_narrative 100
```

## Data Scale Options

### Test City (1 city, ~1000 people)
- 1 city based on Eno data structure
- Multiple districts with mixed building types
- Manageable for development testing

### Small Dataset (3 cities, ~6000 people total)
- Guild (major trade hub)
- Aira (small capital)
- One other regional city
- Good for testing inter-city dynamics

### Medium Dataset (5 cities, ~25000 people total)
- Top 5 cities from Eno
- Multiple city types (capitals, ports, trade centers)
- Suitable for full simulation testing

## Eno Data Integration

The simulation incorporates realistic data from the Eno world:

### City Characteristics
- **Population**: Actual populations from Eno census
- **Geography**: Real coordinates and elevations
- **Infrastructure**: Roads, walls, and civic buildings
- **Districts**: Named districts with appropriate building distributions
- **History**: Founded dates and cultural background

### Building Types
Based on actual Eno building surveys:
- **Residential**: Townhouses, cottages, farmhouses, noble villas
- **Commercial**: Markets, trade centers, workshops
- **Civic**: Council fires, meeting halls, shrines
- **Industrial**: Ports, manufacturing, resource processing

### Population Distribution
- **Employment**: ~70% employment rate based on economic modeling
- **Housing**: 4-6 people per residential building (Eno average)
- **Names**: Fantasy-appropriate names fitting the Eno world

## Monitoring Performance

The simulation can handle:
- **Small datasets**: Sub-second updates
- **Medium datasets**: 1-2 second updates
- **Large datasets**: 5-10 second updates (not recommended for real-time)

## Example Session

```bash
# Start fresh
spacetime start
spacetime publish world-simulation

# Initialize
spacetime call world-simulation init_simulation
spacetime call world-simulation import_small_dataset
spacetime call world-simulation toggle_simulation

# Run for a day
spacetime call world-simulation skip_hours 24

# Check results
spacetime call world-simulation get_city_summary 1
spacetime call world-simulation generate_hourly_narrative 24

# Run for a week to see building/city changes
spacetime call world-simulation skip_hours 144  # +6 more days

# Check city development
spacetime call world-simulation get_city_summary 1
spacetime call world-simulation get_city_summary 2
spacetime call world-simulation get_city_summary 3
```

This will show how the three cities (Guild, Aira, and a third city) develop over a week, with individuals fulfilling needs, buildings operating, and cities evolving their economic and social metrics.