# World Simulation Narrative Processor Design

## Overview

This document outlines the design for extending the existing simulation2 system to support narrative generation for async gameplay. The system will evolve game worlds between player interactions, creating emergent storylines and dynamic context for AI narrative generation.

## Current System Analysis

### Existing Components
- **SpacetimeDB-based simulation** with individuals, buildings, and cities
- **Need-based system** (Environment, Consumption, Connection, Rest, Waste)
- **Hierarchical time ticks** (hourly for individuals, daily for buildings, weekly for cities)
- **Basic narrative reporting** (generate_hourly_narrative, get_individual_story, etc.)
- **Event tracking** (movement, work, social, building, city events)

### What's Missing for Narrative Generation
1. **World-level events** that affect multiple cities
2. **Political dynamics** between factions/leaders
3. **Economic market forces** and trade relationships
4. **Natural events** (weather, seasons, disasters)
5. **Event queue** for narrative pipeline consumption
6. **Multi-world support** (100+ game worlds)
7. **Narrative hooks** generation for player engagement

## Extended Data Model

### New Tables for SpacetimeDB

```rust
// World-level entities
#[spacetimedb::table(name = game_world)]
pub struct GameWorld {
    #[primary_key]
    pub id: u32,
    pub name: String,
    pub current_cycle: i32,  // Game time in cycles (360 days each)
    pub current_day: u16,     // Day within cycle (1-360)
    pub season: Season,
    pub climate_zone: ClimateZone,
    pub active_players: u32,
    pub narrative_speed: NarrativeSpeed, // How fast time progresses
    pub last_update: i64,     // Unix timestamp
}

#[spacetimedb::table(name = faction)]
pub struct Faction {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub name: String,
    pub faction_type: FactionType, // Political, Religious, Economic, Military
    pub leader_id: u32,  // Links to Individual
    pub influence: f32,
    pub treasury: f32,
    pub stability: f32,
    pub ideology: String, // JSON of faction beliefs/goals
}

#[spacetimedb::table(name = faction_relationship)]
pub struct FactionRelationship {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub faction1_id: u32,
    pub faction2_id: u32,
    pub relationship: f32, // -100 (war) to +100 (alliance)
    pub trade_volume: f32,
    pub recent_events: String, // JSON array of events
}

// Economic extensions
#[spacetimedb::table(name = market)]
pub struct Market {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub city_id: u32,
    pub resource_type: ResourceType,
    pub supply: f32,
    pub demand: f32,
    pub price: f32,
    pub price_history: String, // JSON array of recent prices
    pub volatility: f32,
}

#[spacetimedb::table(name = trade_route)]
pub struct TradeRoute {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub from_city_id: u32,
    pub to_city_id: u32,
    pub resource_type: ResourceType,
    pub volume: f32,
    pub profitability: f32,
    pub safety: f32, // Affected by political stability
}

// Natural world events
#[spacetimedb::table(name = natural_event)]
pub struct NaturalEvent {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub event_type: NaturalEventType,
    pub severity: f32,
    pub affected_cities: String, // JSON array of city IDs
    pub start_hour: u64,
    pub duration_hours: u32,
    pub impact_description: String,
}

// Narrative event queue for game consumption
#[spacetimedb::table(name = narrative_event)]
pub struct NarrativeEvent {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub game_id: u32, // Links to specific game session
    pub event_category: EventCategory,
    pub importance: u8, // 1-10 scale
    pub title: String,
    pub description: String,
    pub participants: String, // JSON array of character/faction IDs
    pub location_context: String, // JSON with city/building info
    pub consequences: String, // JSON of potential impacts
    pub narrative_hooks: String, // JSON of story possibilities
    pub created_hour: u64,
    pub consumed: bool,
    pub consumed_at: Option<i64>,
}

// Enhanced NPC tracking
#[spacetimedb::table(name = npc_goal)]
pub struct NPCGoal {
    #[primary_key]
    pub id: u32,
    pub individual_id: u32,
    pub goal_type: GoalType,
    pub target_value: f32,
    pub current_progress: f32,
    pub deadline_hour: u64,
    pub motivation: f32,
    pub related_npcs: String, // JSON array of IDs
}
```

### Type Definitions

```rust
#[derive(SpacetimeType)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[derive(SpacetimeType)]
pub enum ClimateZone {
    Tropical,
    Temperate,
    Desert,
    Arctic,
    Mountain,
}

#[derive(SpacetimeType)]
pub enum NarrativeSpeed {
    Paused,
    Slow,    // 1 day per real hour
    Normal,  // 1 week per real hour
    Fast,    // 1 month per real hour
}

#[derive(SpacetimeType)]
pub enum FactionType {
    Political,
    Religious,
    Economic,
    Military,
    Cultural,
}

#[derive(SpacetimeType)]
pub enum ResourceType {
    Food,
    Materials,
    Luxury,
    Knowledge,
    Military,
}

#[derive(SpacetimeType)]
pub enum NaturalEventType {
    Weather(WeatherEvent),
    Disaster(DisasterType),
    Seasonal(SeasonalEvent),
    Astronomical(AstronomicalEvent),
}

#[derive(SpacetimeType)]
pub enum EventCategory {
    Political,
    Economic,
    Social,
    Military,
    Natural,
    Personal,
    Mystery,
}

#[derive(SpacetimeType)]
pub enum GoalType {
    WealthAccumulation,
    SocialAdvancement,
    SkillMastery,
    Relationship,
    Revenge,
    Discovery,
    Power,
}
```

## Simulation Components

### 1. Economic Simulation Module

**Purpose**: Generate market dynamics, trade flows, and economic events

**Key Functions**:
- Update market prices based on supply/demand
- Process trade routes and calculate profits
- Generate economic crises/booms
- Create merchant NPC activities
- Integrate with Enonomics data for realistic economics

### 2. Political Events Module

**Purpose**: Simulate faction dynamics and political intrigue

**Key Functions**:
- Process faction relationships and conflicts
- Generate political events (elections, coups, treaties)
- Update faction influence based on actions
- Create political NPC motivations
- Generate succession crises and power struggles

### 3. Natural Events Module

**Purpose**: Create environmental context and challenges

**Key Functions**:
- Process seasonal changes
- Generate weather patterns
- Create natural disasters (with appropriate rarity)
- Update agricultural cycles
- Generate environmental narrative hooks

### 4. Enhanced NPC Agency

**Purpose**: Give NPCs goals and motivations beyond basic needs

**Key Functions**:
- Generate personal goals for important NPCs
- Process NPC relationships and rivalries
- Create NPC-driven subplot events
- Generate NPC backstories dynamically
- Track NPC achievements and failures

### 5. Narrative Hooks Generator

**Purpose**: Transform simulation events into story opportunities

**Key Functions**:
- Analyze event patterns for interesting narratives
- Generate "what if" scenarios for player choices
- Create tension through conflicting interests
- Identify dramatic moments worth highlighting
- Generate mysteries and unresolved questions

## Integration Architecture

### Event Queue System

```rust
// Reducer to generate narrative events
#[spacetimedb::reducer]
pub fn generate_narrative_events(ctx: &ReducerContext, world_id: u32) -> Result<(), String> {
    // Collect significant events from the last simulation cycle
    let political_events = analyze_political_changes(ctx, world_id)?;
    let economic_events = analyze_economic_changes(ctx, world_id)?;
    let social_events = analyze_social_changes(ctx, world_id)?;
    let natural_events = get_active_natural_events(ctx, world_id)?;

    // Generate narrative events with appropriate importance
    for event in political_events {
        create_narrative_event(ctx, world_id, event)?;
    }

    // Create narrative hooks
    generate_narrative_hooks(ctx, world_id)?;

    Ok(())
}

// API endpoint for narrative consumption
#[spacetimedb::reducer]
pub fn consume_narrative_events(
    ctx: &ReducerContext,
    game_id: u32,
    max_events: u32
) -> Result<Vec<NarrativeEvent>, String> {
    // Get unconsumed events for this game
    let events = ctx.db.narrative_event()
        .game_id().find(&game_id)
        .filter(|e| !e.consumed)
        .take(max_events as usize)
        .collect();

    // Mark as consumed
    for event in &events {
        // Update consumed flag
    }

    Ok(events)
}
```

### Multi-World Scheduling

```rust
#[spacetimedb::reducer]
pub fn process_world_batch(ctx: &ReducerContext) -> Result<(), String> {
    // Get worlds that need updating
    let worlds_to_update = ctx.db.game_world()
        .iter()
        .filter(|w| needs_update(w))
        .take(10) // Process in batches
        .collect();

    for world in worlds_to_update {
        // Run simulation tick for this world
        tick_world(ctx, world.id)?;

        // Generate narrative events
        generate_narrative_events(ctx, world.id)?;

        // Update last_update timestamp
        update_world_timestamp(ctx, world.id)?;
    }

    Ok(())
}
```

## Performance Optimization Strategies

### 1. Batch Processing
- Process worlds in groups of 10
- Use parallel processing where possible
- Cache frequently accessed data

### 2. Event Aggregation
- Combine similar events before narrative generation
- Summarize repetitive actions
- Focus on significant changes

### 3. Lazy Evaluation
- Only simulate active game worlds
- Skip detailed simulation for inactive areas
- Use level-of-detail based on player focus

### 4. Data Pruning
- Archive old events periodically
- Aggregate historical data
- Remove consumed narrative events

## Integration Points

### 1. Enonomics Integration
- Pull economic data for realistic markets
- Use demographic data for population dynamics
- Integrate trade flow information

### 2. Game Frontend Integration
- REST API for narrative event retrieval
- WebSocket for real-time important events
- Batch endpoints for efficiency

### 3. AI Narrative Pipeline
- Structured event format for LLM consumption
- Context windows with relevant history
- Narrative hooks as prompt seeds

## Implementation Phases

### Phase 1: Core Extensions (Week 1)
- [ ] Create new SpacetimeDB tables
- [ ] Implement basic world and faction entities
- [ ] Add narrative event queue
- [ ] Create consumption API

### Phase 2: Economic & Political (Week 2)
- [ ] Implement market dynamics
- [ ] Add faction relationships
- [ ] Create political event generation
- [ ] Integrate basic Enonomics data

### Phase 3: Natural & NPC Events (Week 3)
- [ ] Add seasonal cycles
- [ ] Implement weather patterns
- [ ] Enhance NPC goal system
- [ ] Create NPC subplot generation

### Phase 4: Narrative Hooks (Week 4)
- [ ] Build hook generation algorithms
- [ ] Create tension identification
- [ ] Add mystery generation
- [ ] Implement consequence chains

### Phase 5: Multi-World & Optimization (Week 5)
- [ ] Build batch processing system
- [ ] Implement world scheduling
- [ ] Add performance monitoring
- [ ] Optimize for 100+ worlds

### Phase 6: Integration & Testing (Week 6)
- [ ] Connect to game frontend
- [ ] Test with multiple active games
- [ ] Performance benchmarking
- [ ] Narrative quality assessment

## Success Metrics

1. **Performance**
   - Process 100+ worlds in under 10 minutes
   - Generate events within 100ms per world
   - Maintain <1GB memory per 10 worlds

2. **Narrative Quality**
   - 5-10 meaningful events per cycle
   - Coherent cause-effect chains
   - Diverse event types per session
   - Player engagement metrics

3. **System Reliability**
   - 99.9% uptime for scheduler
   - No data loss on crashes
   - Graceful degradation under load

## Next Steps

1. Review and approve design
2. Set up development environment
3. Create database migrations
4. Begin Phase 1 implementation
5. Establish testing framework