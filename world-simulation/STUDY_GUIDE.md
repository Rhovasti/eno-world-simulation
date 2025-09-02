# World Simulation with Eno Data - Complete Study Guide

This comprehensive guide will teach you how to use, understand, and extend the SpacetimeDB world simulation system with real Eno world data.

## Table of Contents
1. [Setup and Installation](#setup-and-installation)
2. [Understanding the Simulation](#understanding-the-simulation)
3. [Basic Operations](#basic-operations)
4. [Advanced Usage](#advanced-usage)
5. [Data Analysis](#data-analysis)
6. [Troubleshooting](#troubleshooting)
7. [Extending the System](#extending-the-system)

---

## Setup and Installation

### Prerequisites
```bash
# Install SpacetimeDB CLI
curl -sSf https://spacetimedb.com/install.sh | sh

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install build tools (Linux/WSL)
sudo apt update && sudo apt install -y build-essential
```

### Project Setup
```bash
# Navigate to the simulation directory
cd /root/Eno/simulation2/world-simulation

# Start SpacetimeDB server (keep this running)
spacetime start

# In another terminal, compile and publish the module
spacetime publish .

# Verify the module is published
spacetime list
```

**Expected Output:**
```
MODULE NAME         ADDRESS
world-simulation    <module-address>
```

---

## Understanding the Simulation

### Core Concepts

#### 1. **Three-Level Hierarchy**
```
Cities (Weekly Updates)
├── Buildings (Daily Updates)
│   └── Individuals (Hourly Updates)
```

- **Individuals**: Have needs (food, rest, work, social) that drive behavior
- **Buildings**: Homes and workplaces with operational requirements
- **Cities**: Emerge from collective building/individual activity

#### 2. **Time System**
- **1 Hour = 1 Tick**: Individuals update needs and take actions
- **24 Hours = 1 Day**: Buildings update (maintenance, production, costs)
- **168 Hours = 1 Week**: Cities update (economy, infrastructure, culture)

#### 3. **Need Hierarchy (Maslow's Model)**
```
Level 5: Self-Actualization (Progression, Specialized Roles)
Level 4: Self-Esteem (Achievements, Recognition)
Level 3: Love & Belonging (Relationships, Community)
Level 2: Safety & Security (Income, Threat, Stress)
Level 1: Physiological (Food, Rest, Environment, Waste)
```
*Higher levels only activate when lower levels are 50%+ satisfied*

### Data Integration

#### Eno World Authenticity
The simulation uses real data from 141 Eno cities:
- **Geographic Data**: Real coordinates and elevations
- **Population Data**: Actual census figures (1.8M total)
- **Building Surveys**: Townhouses, noble villas, council fires, etc.
- **Cultural Context**: Fantasy-appropriate names and structures

---

## Basic Operations

### 1. Initialize and Import Data

```bash
# Initialize the simulation system
spacetime call world-simulation init_simulation

# Import datasets (choose one):

# Option A: Single test city (~1,000 people)
spacetime call world-simulation import_test_city

# Option B: Small dataset (3 cities, ~6,000 people) - RECOMMENDED
spacetime call world-simulation import_small_dataset

# Option C: Medium dataset (5 cities, ~25,000 people)
spacetime call world-simulation import_medium_dataset
```

**Study Exercise 1:** Try each import option and observe the differences in complexity and performance.

### 2. Start the Simulation

```bash
# Start time progression
spacetime call world-simulation toggle_simulation

# Verify it's running
spacetime call world-simulation get_current_hour
```

### 3. Advance Time

```bash
# Single hour tick
spacetime call world-simulation tick_hour

# Fast-forward multiple hours
spacetime call world-simulation skip_hours 24    # 1 day
spacetime call world-simulation skip_hours 168   # 1 week
```

**Study Exercise 2:** Run `tick_hour` several times and observe the console output for individual actions.

---

## Advanced Usage

### Understanding Individual Behavior

#### Monitor Individual Actions
```bash
# Watch an individual's story develop
spacetime call world-simulation get_individual_story 1 24

# Example output analysis:
# "Current needs: Food 45%, Rest 78%, Stress 32%"
# - Food is getting low (will seek restaurant soon)
# - Rest is good (recently slept)
# - Stress is manageable
```

**Study Exercise 3:** Track 3-5 individuals over a 48-hour period. Notice patterns:
- When do they work? (Usually 8-hour blocks)
- How do needs cascade? (Low food → seeking restaurant)
- What affects stress? (Work increases, home/social decreases)

#### Analyzing Need Fulfillment
```bash
# Run for exactly 24 hours and study patterns
spacetime call world-simulation skip_hours 24
spacetime call world-simulation generate_hourly_narrative 24

# Look for patterns like:
# "12 people moved locations. 8 sought food. 15 went home to rest."
```

### Building Operations

#### Monitor Building Performance
```bash
# Track a workplace over time
spacetime call world-simulation get_building_story 1 48

# Key metrics to watch:
# - Occupancy rates (workers present)
# - Production output
# - Maintenance status
# - Visitor traffic
```

**Study Exercise 4:** Compare different building types:
1. Track a residential building (ID 1-10 usually)
2. Track a workplace building (check for production data)
3. Track an amenity building (restaurant, park)

Notice how usage patterns differ by building type.

#### Understanding Resource Flow
```bash
# Buildings consume and produce resources
# Workplaces: Input materials → Output goods
# Homes: Consume maintenance, produce satisfied residents
# Amenities: Consume upkeep, produce social/health benefits
```

### City-Level Analysis

#### Economic Monitoring
```bash
# Get comprehensive city overview
spacetime call world-simulation get_city_summary 1

# Key sections to analyze:
# ECONOMY: Tax reserve, unemployment, trade
# SOCIAL: Stability, health, safety, happiness
# DEVELOPMENT: Culture, science, prestige
```

**Study Exercise 5:** Run a city for 1 week (168 hours) and track changes:
```bash
# Baseline
spacetime call world-simulation get_city_summary 1

# After 1 week
spacetime call world-simulation skip_hours 168
spacetime call world-simulation get_city_summary 1

# Compare: Did tax reserves grow? Did culture increase?
```

---

## Data Analysis

### Tracking Simulation Health

#### Population Dynamics
```bash
# Monitor employment
# Healthy cities show ~70% employment
# Watch for unemployment spikes

# Monitor happiness
# Should stay 60-80% in stable cities
# Low happiness indicates systemic problems
```

#### Economic Indicators
```bash
# Tax Base Growth
# Should increase as more people work
# Indicates economic activity

# Import/Export Balance
# Imports cost money, exports generate revenue
# Watch for trade imbalances
```

**Study Exercise 6:** Create an "economic dashboard":
1. Record initial city metrics
2. Run simulation for 1 week intervals
3. Track: Population, Employment %, Tax Reserve, Happiness
4. Identify trends and correlations

### Identifying Patterns

#### Daily Cycles
```bash
# Typical day pattern:
# Hours 0-8: Sleep period (rest needs fulfilled)
# Hours 8-16: Work period (income generated, stress increased)
# Hours 16-24: Social/personal time (food, social needs)
```

#### Weekly Cycles
```bash
# Building maintenance accumulates
# City infrastructure degrades
# Cultural activities compound
# Economic cycles complete
```

**Study Exercise 7:** Document a complete weekly cycle:
- Log major events each day
- Note when buildings need maintenance
- Observe how individual stress affects city stability

---

## Troubleshooting

### Common Issues

#### Simulation Appears Stuck
```bash
# Check if simulation is running
spacetime call world-simulation get_current_hour

# If hour isn't advancing:
spacetime call world-simulation toggle_simulation
```

#### No Individual Movement
```bash
# This is normal if needs are satisfied
# Check individual status:
spacetime call world-simulation get_individual_story 1 1

# Look for: "Current needs: Food 80%, Rest 90%..."
# High percentages = satisfied, no urgent actions needed
```

#### Performance Issues
```bash
# Large datasets (5+ cities) may be slow
# Reduce dataset size:
# 1. Start fresh with import_small_dataset
# 2. Use skip_hours instead of tick_hour for faster progression
```

### Debugging Commands

```bash
# Check current simulation state
spacetime call world-simulation get_current_hour

# Verify data import worked
spacetime call world-simulation get_city_summary 1

# Check if individuals exist
spacetime call world-simulation get_individual_story 1 1
```

---

## Extending the System

### Understanding the Code Structure

```
src/
├── lib.rs              # Module entry point
├── types.rs            # Data type definitions
├── tables/             # Database schema
│   ├── individual.rs   # Person entities
│   ├── building.rs     # Building entities
│   ├── city.rs         # City entities
│   └── events.rs       # Event logging
├── systems/            # Game logic
│   ├── needs.rs        # Need calculations
│   ├── modifiers.rs    # Rate constants
│   └── priorities.rs   # Decision making
├── reducers/           # Database operations
│   ├── time.rs         # Time management
│   ├── individual.rs   # Person actions
│   ├── building.rs     # Building operations
│   ├── city.rs         # City updates
│   └── narrative.rs    # Story generation
└── data_import.rs      # Eno data integration
```

### Adding New Features

#### Example: Add New Building Type
1. **Update BuildingType enum** in `types.rs`
2. **Add location capabilities** in `building.rs`
3. **Update import logic** in `data_import.rs`
4. **Test with small dataset**

#### Example: Add New Individual Action
1. **Add to IndividualAction enum** in `types.rs`
2. **Implement in perform_action** in `individual.rs`
3. **Add costs/benefits** in `modifiers.rs`
4. **Update priority system** in `priorities.rs`

### Customizing Data Import

#### Import Specific Eno Cities
```rust
// Modify create_sample_eno_cities() in data_import.rs
// Add your preferred cities from the Eno dataset
let cities = vec![
    ("YourCity", population, districts, capital, port),
    // ...
];
```

#### Adjust Population Ratios
```rust
// Modify in data_import.rs
let (residential_ratio, workplace_ratio, amenity_ratio) = match district_name {
    "Your District Type" => (0.9, 0.05, 0.05), // Mostly residential
    // ...
};
```

---

## Study Exercises - Complete Curriculum

### Week 1: Basics
1. **Day 1-2**: Setup and first import
2. **Day 3-4**: Understanding individual behavior
3. **Day 5-7**: Building operations and maintenance

### Week 2: Analysis
1. **Day 1-3**: City economics and metrics
2. **Day 4-5**: Narrative generation and storytelling
3. **Day 6-7**: Performance optimization

### Week 3: Advanced
1. **Day 1-3**: Code structure and modification
2. **Day 4-5**: Custom data import
3. **Day 6-7**: New feature development

### Final Project Ideas
1. **Economic Simulation**: Track trade flows between cities
2. **Social Dynamics**: Analyze relationship formation patterns
3. **Urban Planning**: Optimize building ratios for city health
4. **Cultural Evolution**: Study how individual actions create city culture

---

## Resources

### Key Files to Study
- `modifiers.md`: All numerical parameters
- `demo.md`: Quick start examples
- `project.md`: Complete system design
- Source code with extensive comments

### SpacetimeDB Documentation
- [Official Docs](https://spacetimedb.com/docs)
- [Rust Module Guide](https://spacetimedb.com/docs/modules/rust)

### Eno World Context
- 141 cities with rich histories
- Multiple cultures and regions
- Fantasy setting with realistic demographics

This simulation provides a unique opportunity to study emergent behavior in complex systems while exploring a rich fantasy world. Take your time with each section, experiment freely, and don't hesitate to modify the code to explore your own questions about how societies function and evolve.