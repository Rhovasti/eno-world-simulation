// Game world entity and management

use spacetimedb::{ReducerContext, Table, SpacetimeType};
use log;
use crate::world::{Season, ClimateZone, NarrativeSpeed, calculate_season};

// Main game world entity
#[spacetimedb::table(name = game_world)]
pub struct GameWorld {
    #[primary_key]
    pub id: u32,
    pub name: String,
    pub current_cycle: i32,     // Game time in cycles (360 days each)
    pub current_day: u16,        // Day within cycle (1-360)
    pub total_hours: u64,        // Total simulation hours elapsed
    pub season: Season,
    pub climate_zone: ClimateZone,
    pub active_players: u32,
    pub total_population: u32,
    pub narrative_speed: NarrativeSpeed,
    pub last_update_ms: i64,     // Unix timestamp in milliseconds
    pub next_update_ms: i64,     // When next update should occur
    pub is_active: bool,
}

// World statistics tracking
#[spacetimedb::table(name = world_stats)]
pub struct WorldStats {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub total_births: u64,
    pub total_deaths: u64,
    pub total_trades: u64,
    pub total_conflicts: u32,
    pub total_disasters: u32,
    pub total_narrative_events: u64,
    pub average_happiness: f32,
    pub average_prosperity: f32,
    pub political_stability: f32,
    pub last_calculated_hour: u64,
}

// Initialize a new game world
#[spacetimedb::reducer]
pub fn create_game_world(
    ctx: &ReducerContext,
    name: String,
    climate_zone: ClimateZone,
    initial_population: u32,
) -> Result<u32, String> {
    // Generate new world ID
    let world_id = ctx.db.game_world().iter().count() as u32 + 1;

    // Get current time
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| format!("Time error: {}", e))?
        .as_millis() as i64;

    // Create world
    let world = GameWorld {
        id: world_id,
        name: name.clone(),
        current_cycle: 0,
        current_day: 1,
        total_hours: 0,
        season: Season::Spring,
        climate_zone,
        active_players: 0,
        total_population: initial_population,
        narrative_speed: NarrativeSpeed::Normal,
        last_update_ms: now,
        next_update_ms: now + 60000, // Next update in 1 minute
        is_active: true,
    };

    ctx.db.game_world().insert(world);

    // Initialize world stats
    let stats = WorldStats {
        id: world_id,
        world_id,
        total_births: 0,
        total_deaths: 0,
        total_trades: 0,
        total_conflicts: 0,
        total_disasters: 0,
        total_narrative_events: 0,
        average_happiness: 50.0,
        average_prosperity: 50.0,
        political_stability: 75.0,
        last_calculated_hour: 0,
    };

    ctx.db.world_stats().insert(stats);

    log::info!("Created new game world '{}' (ID: {}) with {} population",
        name, world_id, initial_population);

    Ok(world_id)
}

// Advance world time
#[spacetimedb::reducer]
pub fn advance_world_time(
    ctx: &ReducerContext,
    world_id: u32,
    hours: u32,
) -> Result<(), String> {
    let mut world = ctx.db.game_world()
        .id()
        .find(&world_id)
        .ok_or("World not found")?;

    // Update total hours
    world.total_hours += hours as u64;

    // Update days and cycles
    let total_days = world.total_hours / 24;
    world.current_cycle = (total_days / 360) as i32;
    world.current_day = ((total_days % 360) + 1) as u16;

    // Update season
    world.season = calculate_season(world.current_day);

    // Update timestamp
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| format!("Time error: {}", e))?
        .as_millis() as i64;

    world.last_update_ms = now;

    // Calculate next update based on narrative speed
    let next_update_interval_ms = match world.narrative_speed {
        NarrativeSpeed::Paused => i64::MAX,
        NarrativeSpeed::Slow => 3600000,    // 1 hour real time = 1 day game time
        NarrativeSpeed::Normal => 3600000/7, // 1 hour real time = 1 week game time
        NarrativeSpeed::Fast => 3600000/30,  // 1 hour real time = 1 month game time
    };

    world.next_update_ms = now + next_update_interval_ms;

    // Update the world
    ctx.db.game_world().id().update(world_id, world);

    log::info!("World {} advanced {} hours to Cycle {} Day {} ({})",
        world_id, hours, world.current_cycle, world.current_day, world.season);

    Ok(())
}

// Get worlds that need updating
#[spacetimedb::reducer]
pub fn get_worlds_needing_update(ctx: &ReducerContext) -> Result<Vec<u32>, String> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| format!("Time error: {}", e))?
        .as_millis() as i64;

    let world_ids: Vec<u32> = ctx.db.game_world()
        .iter()
        .filter(|w| w.is_active && w.narrative_speed != NarrativeSpeed::Paused && w.next_update_ms <= now)
        .map(|w| w.id)
        .collect();

    if !world_ids.is_empty() {
        log::info!("Found {} worlds needing update", world_ids.len());
    }

    Ok(world_ids)
}

// Update world statistics
#[spacetimedb::reducer]
pub fn update_world_stats(
    ctx: &ReducerContext,
    world_id: u32,
) -> Result<(), String> {
    let world = ctx.db.game_world()
        .id()
        .find(&world_id)
        .ok_or("World not found")?;

    let mut stats = ctx.db.world_stats()
        .world_id()
        .find(&world_id)
        .ok_or("World stats not found")?;

    // TODO: Calculate actual statistics from simulation data
    // For now, using placeholder calculations

    // Update happiness based on various factors
    stats.average_happiness = calculate_world_happiness(ctx, world_id)?;

    // Update prosperity based on economic factors
    stats.average_prosperity = calculate_world_prosperity(ctx, world_id)?;

    // Update political stability
    stats.political_stability = calculate_political_stability(ctx, world_id)?;

    stats.last_calculated_hour = world.total_hours;

    // Update the stats
    ctx.db.world_stats().id().update(stats.id, stats);

    Ok(())
}

// Helper functions for statistics calculation
fn calculate_world_happiness(ctx: &ReducerContext, world_id: u32) -> Result<f32, String> {
    // TODO: Implement actual happiness calculation
    // Should consider: individual happiness, social events, political stability
    Ok(65.0)
}

fn calculate_world_prosperity(ctx: &ReducerContext, world_id: u32) -> Result<f32, String> {
    // TODO: Implement actual prosperity calculation
    // Should consider: economic indicators, trade volume, employment
    Ok(55.0)
}

fn calculate_political_stability(ctx: &ReducerContext, world_id: u32) -> Result<f32, String> {
    // TODO: Implement actual stability calculation
    // Should consider: faction relationships, conflicts, leadership changes
    Ok(70.0)
}