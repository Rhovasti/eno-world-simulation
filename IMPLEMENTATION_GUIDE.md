# World Simulation Narrative Processor - Implementation Guide

## Current Status

### ✅ Completed
1. **Design Document** - Comprehensive design for narrative generation system
2. **World Management** - Game world entities and time progression
3. **Narrative Event Queue** - Event creation, storage, and consumption system
4. **Module Structure** - Organized codebase with dedicated modules

### 🚧 In Progress
- Economic simulation module
- Political faction system
- Natural events and weather
- NPC goal system enhancements
- Narrative hooks generator

### 📋 TODO
- Multi-world batch processing
- Enonomics integration
- Frontend API connections
- Performance optimization
- Testing framework

## Development Environment Setup

### 1. Install SpacetimeDB CLI
```bash
curl -fsSL https://install.spacetimedb.com | sh
```

### 2. Build the Module
```bash
cd /root/Eno/simulation2/world-simulation
cargo build --release
```

### 3. Publish to SpacetimeDB
```bash
spacetime publish world-simulation --release
```

### 4. Create a Database Instance
```bash
spacetime database create world-sim-narrative
spacetime database publish world-sim-narrative world-simulation
```

## Module Structure

```
world-simulation/
├── src/
│   ├── lib.rs              # Main module entry point
│   ├── types.rs            # Shared type definitions
│   ├── tables/             # Existing entity tables
│   │   ├── individual.rs
│   │   ├── building.rs
│   │   ├── city.rs
│   │   └── events.rs
│   ├── systems/            # Core simulation systems
│   │   ├── needs.rs
│   │   ├── priorities.rs
│   │   └── modifiers.rs
│   ├── reducers/           # State mutation logic
│   │   ├── time.rs
│   │   ├── individual.rs
│   │   ├── building.rs
│   │   └── city.rs
│   ├── world/              # NEW: World-level components
│   │   ├── mod.rs
│   │   ├── game_world.rs   # Multi-world management
│   │   ├── factions.rs     # Political systems
│   │   └── natural_events.rs
│   ├── narrative/          # NEW: Narrative generation
│   │   ├── mod.rs
│   │   ├── event_queue.rs  # Event management
│   │   └── hooks_generator.rs
│   ├── economics/          # NEW: Economic simulation
│   │   ├── mod.rs
│   │   ├── markets.rs
│   │   └── trade_routes.rs
│   └── political/          # NEW: Political dynamics
│       ├── mod.rs
│       └── faction_relationships.rs
```

## API Endpoints

### World Management
```rust
// Create a new game world
create_game_world(name: String, climate: ClimateZone, population: u32) -> u32

// Advance world time
advance_world_time(world_id: u32, hours: u32)

// Get worlds needing update
get_worlds_needing_update() -> Vec<u32>
```

### Narrative Events
```rust
// Create narrative event
create_narrative_event(
    world_id: u32,
    game_id: u32,
    category: EventCategory,
    title: String,
    description: String,
    importance: u8
) -> u32

// Get unconsumed events
get_unconsumed_events(
    game_id: u32,
    max_events: u32,
    min_importance: u8
) -> Vec<NarrativeEvent>

// Mark events as consumed
consume_narrative_events(event_ids: Vec<u32>)
```

## Integration with Frontend

### 1. REST API Wrapper
Create a Node.js service to interface with SpacetimeDB:

```javascript
// /root/Eno/Eno-Frontend/js/services/simulationService.js

const SPACETIME_URL = process.env.SPACETIME_URL || 'http://localhost:3000';

async function getGameNarrativeEvents(gameId, maxEvents = 10) {
    const response = await fetch(`${SPACETIME_URL}/reducer/get_unconsumed_events`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            game_id: gameId,
            max_events: maxEvents,
            min_importance: 3
        })
    });
    return response.json();
}

async function consumeEvents(eventIds) {
    const response = await fetch(`${SPACETIME_URL}/reducer/consume_narrative_events`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ event_ids: eventIds })
    });
    return response.json();
}
```

### 2. Narrative Pipeline Integration
```javascript
// Process events for AI narrative generation
async function processNarrativeEvents(gameId) {
    // Get events from simulation
    const events = await getGameNarrativeEvents(gameId);

    // Transform for AI consumption
    const context = events.map(e => ({
        title: e.title,
        description: e.description,
        importance: e.importance,
        category: e.event_category,
        hooks: JSON.parse(e.narrative_hooks)
    }));

    // Generate narrative with AI
    const narrative = await generateNarrative(context);

    // Mark events as consumed
    await consumeEvents(events.map(e => e.id));

    return narrative;
}
```

## Testing Strategy

### 1. Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_creation() {
        // Test world initialization
    }

    #[test]
    fn test_event_generation() {
        // Test narrative event creation
    }

    #[test]
    fn test_time_progression() {
        // Test world time advancement
    }
}
```

### 2. Integration Tests
```bash
# Create test script
cat > test_simulation.sh << 'EOF'
#!/bin/bash

# Create test world
spacetime reducer call create_game_world "TestWorld" "Temperate" 1000

# Advance time
spacetime reducer call advance_world_time 1 24

# Check events
spacetime reducer call get_unconsumed_events 1 10 3
EOF

chmod +x test_simulation.sh
```

### 3. Performance Testing
```rust
// Benchmark multi-world processing
#[spacetimedb::reducer]
pub fn benchmark_world_processing(ctx: &ReducerContext) -> Result<(), String> {
    let start = std::time::Instant::now();

    // Create 100 test worlds
    for i in 1..=100 {
        create_game_world(ctx, format!("TestWorld{}", i), ClimateZone::Temperate, 1000)?;
    }

    // Process all worlds
    let worlds = get_worlds_needing_update(ctx)?;
    for world_id in worlds {
        advance_world_time(ctx, world_id, 1)?;
    }

    let duration = start.elapsed();
    log::info!("Processed 100 worlds in {:?}", duration);

    Ok(())
}
```

## Deployment Steps

### 1. Local Development
```bash
# Start local SpacetimeDB
spacetimedb start

# Deploy module
cd /root/Eno/simulation2/world-simulation
spacetime publish world-simulation --release

# Create database
spacetime database create world-sim-dev
spacetime database publish world-sim-dev world-simulation
```

### 2. Production Deployment
```bash
# Build optimized version
cargo build --release

# Deploy to production SpacetimeDB
spacetime publish world-simulation --release --server production.spacetimedb.com

# Create production database
spacetime database create world-sim-prod --server production.spacetimedb.com
spacetime database publish world-sim-prod world-simulation --server production.spacetimedb.com
```

## Monitoring & Debugging

### 1. View Logs
```bash
spacetime logs world-sim-narrative --follow
```

### 2. Query Database State
```bash
# Get all game worlds
spacetime query "SELECT * FROM game_world"

# Check narrative events
spacetime query "SELECT * FROM narrative_event WHERE consumed = false"
```

### 3. Performance Metrics
```rust
#[spacetimedb::reducer]
pub fn get_performance_metrics(ctx: &ReducerContext) -> Result<(), String> {
    let total_worlds = ctx.db.game_world().iter().count();
    let active_worlds = ctx.db.game_world()
        .iter()
        .filter(|w| w.is_active)
        .count();

    let total_events = ctx.db.narrative_event().iter().count();
    let unconsumed_events = ctx.db.narrative_event()
        .iter()
        .filter(|e| !e.consumed)
        .count();

    log::info!("Performance Metrics:");
    log::info!("  Total Worlds: {}", total_worlds);
    log::info!("  Active Worlds: {}", active_worlds);
    log::info!("  Total Events: {}", total_events);
    log::info!("  Unconsumed Events: {}", unconsumed_events);

    Ok(())
}
```

## Next Implementation Steps

### Phase 1: Core Systems (Current)
- [x] World management structure
- [x] Narrative event queue
- [ ] Basic event generation from existing simulation
- [ ] Time advancement system
- [ ] Multi-world support

### Phase 2: Economic & Political
- [ ] Market dynamics implementation
- [ ] Faction system
- [ ] Trade route simulation
- [ ] Political event generation

### Phase 3: Natural Events & NPCs
- [ ] Weather system
- [ ] Seasonal events
- [ ] Enhanced NPC goals
- [ ] NPC subplot generation

### Phase 4: Narrative Hooks
- [ ] Hook identification algorithms
- [ ] Tension tracking
- [ ] Mystery generation
- [ ] Consequence chains

### Phase 5: Integration & Optimization
- [ ] Frontend API integration
- [ ] Enonomics data connection
- [ ] Performance optimization
- [ ] Load testing with 100+ worlds

## Common Issues & Solutions

### Issue: Module won't compile
```bash
# Clear build cache
cargo clean
rm -rf target/

# Rebuild
cargo build --release
```

### Issue: SpacetimeDB connection errors
```bash
# Check SpacetimeDB is running
spacetimedb status

# Restart if needed
spacetimedb restart
```

### Issue: Performance degradation
```rust
// Add indexes to frequently queried fields
#[spacetimedb::table(name = narrative_event)]
#[index(btree, game_id, consumed)]
pub struct NarrativeEvent {
    // ...
}
```

## Resources

- [SpacetimeDB Documentation](https://docs.spacetimedb.com)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Game Simulation Patterns](https://gameprogrammingpatterns.com)
- Project Design: `/root/Eno/simulation2/WORLD_SIMULATION_NARRATIVE_DESIGN.md`