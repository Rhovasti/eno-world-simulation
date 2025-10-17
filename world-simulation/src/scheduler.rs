// Multi-world simulation scheduler and coordinator

use spacetimedb::{ReducerContext, Table};
use log;
use crate::world::game_world::GameWorld;
use crate::narrative::{create_narrative_event, EventCategory};
use crate::economics::{update_market_prices, process_trade_routes, generate_economic_events};
use crate::political::{update_faction_status, generate_political_events, process_political_events};
use crate::natural::{update_climate_conditions, generate_natural_events, process_natural_events};

// Scheduler configuration
#[spacetimedb::table(name = scheduler_config)]
pub struct SchedulerConfig {
    #[primary_key]
    pub id: u32,
    pub enabled: bool,
    pub batch_size: u32,           // How many worlds to process per batch
    pub max_processing_time_ms: u64, // Maximum time to spend processing
    pub last_run_ms: i64,          // Last execution timestamp
    pub next_run_ms: i64,          // Next scheduled execution
    pub run_interval_ms: u64,      // How often to run (e.g., every 5 minutes)
    pub performance_stats: String, // JSON of performance metrics
}

// Processing statistics
#[derive(Debug, Clone)]
pub struct ProcessingStats {
    pub worlds_processed: u32,
    pub events_generated: u32,
    pub processing_time_ms: u64,
    pub errors_encountered: u32,
    pub narrative_events: u32,
    pub economic_events: u32,
    pub political_events: u32,
    pub natural_events: u32,
}

impl Default for ProcessingStats {
    fn default() -> Self {
        Self {
            worlds_processed: 0,
            events_generated: 0,
            processing_time_ms: 0,
            errors_encountered: 0,
            narrative_events: 0,
            economic_events: 0,
            political_events: 0,
            natural_events: 0,
        }
    }
}

// Initialize the scheduler
#[spacetimedb::reducer]
pub fn initialize_scheduler(ctx: &ReducerContext) -> Result<(), String> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| format!("Time error: {}", e))?
        .as_millis() as i64;

    let config = SchedulerConfig {
        id: 1,
        enabled: true,
        batch_size: 10,                // Process 10 worlds at a time
        max_processing_time_ms: 30000, // 30 seconds max
        last_run_ms: now,
        next_run_ms: now + 300000,     // Next run in 5 minutes
        run_interval_ms: 300000,       // Run every 5 minutes
        performance_stats: "{}".to_string(),
    };

    ctx.db.scheduler_config().insert(config);

    log::info!("Initialized simulation scheduler");
    Ok(())
}

// Main scheduler function - processes all worlds needing updates
#[spacetimedb::reducer]
pub fn run_world_simulation_batch(ctx: &ReducerContext) -> Result<(), String> {
    let start_time = std::time::Instant::now();
    let mut stats = ProcessingStats::default();

    // Check if scheduler is enabled
    let config = ctx.db.scheduler_config()
        .id()
        .find(&1)
        .ok_or("Scheduler not initialized")?;

    if !config.enabled {
        log::debug!("Scheduler is disabled, skipping batch");
        return Ok(());
    }

    // Get worlds that need updating
    let worlds_to_update = get_worlds_needing_update(ctx)?;
    let batch_size = config.batch_size as usize;
    let worlds_batch: Vec<GameWorld> = worlds_to_update
        .into_iter()
        .take(batch_size)
        .collect();

    log::info!("Processing batch of {} worlds", worlds_batch.len());

    // Process each world in the batch
    for world in worlds_batch {
        if start_time.elapsed().as_millis() > config.max_processing_time_ms as u128 {
            log::warn!("Batch processing timeout reached, stopping early");
            break;
        }

        match process_single_world(ctx, &world) {
            Ok(world_stats) => {
                stats.worlds_processed += 1;
                stats.events_generated += world_stats.events_generated;
                stats.narrative_events += world_stats.narrative_events;
                stats.economic_events += world_stats.economic_events;
                stats.political_events += world_stats.political_events;
                stats.natural_events += world_stats.natural_events;
            },
            Err(e) => {
                log::error!("Failed to process world {}: {}", world.id, e);
                stats.errors_encountered += 1;
            }
        }
    }

    stats.processing_time_ms = start_time.elapsed().as_millis() as u64;

    // Update scheduler statistics
    update_scheduler_stats(ctx, &stats)?;

    log::info!("Batch complete: {} worlds processed, {} events generated in {}ms",
        stats.worlds_processed, stats.events_generated, stats.processing_time_ms);

    Ok(())
}

// Process a single world through all simulation systems
fn process_single_world(
    ctx: &ReducerContext,
    world: &GameWorld,
) -> Result<ProcessingStats, String> {
    let mut stats = ProcessingStats::default();
    let world_id = world.id;

    log::debug!("Processing world {} ({})", world_id, world.name);

    // 1. Advance world time based on narrative speed
    let hours_to_advance = match world.narrative_speed {
        crate::world::NarrativeSpeed::Paused => 0,
        crate::world::NarrativeSpeed::Slow => 1,    // 1 hour per tick
        crate::world::NarrativeSpeed::Normal => 24,  // 1 day per tick
        crate::world::NarrativeSpeed::Fast => 168,   // 1 week per tick
    };

    if hours_to_advance > 0 {
        crate::world::game_world::advance_world_time(ctx, world_id, hours_to_advance)?;
    }

    let current_hour = world.total_hours + hours_to_advance as u64;

    // 2. Update economic systems
    if let Err(e) = update_market_prices(ctx, world_id, current_hour) {
        log::warn!("Failed to update market prices for world {}: {}", world_id, e);
    }

    if let Err(e) = process_trade_routes(ctx, world_id, current_hour) {
        log::warn!("Failed to process trade routes for world {}: {}", world_id, e);
    }

    // Generate economic events
    match generate_economic_events(ctx, world_id, current_hour) {
        Ok(event_ids) => {
            stats.economic_events += event_ids.len() as u32;
            // Convert economic events to narrative events
            for _event_id in event_ids {
                match create_narrative_event(
                    ctx,
                    world_id,
                    1, // Default game ID - TODO: map to actual games
                    EventCategory::Economic,
                    "Economic development".to_string(),
                    "Market conditions have changed".to_string(),
                    3,
                ) {
                    Ok(_) => stats.narrative_events += 1,
                    Err(e) => log::warn!("Failed to create economic narrative event: {}", e),
                }
            }
        },
        Err(e) => log::warn!("Failed to generate economic events for world {}: {}", world_id, e),
    }

    // 3. Update political systems
    if let Err(e) = update_faction_status(ctx, world_id, current_hour) {
        log::warn!("Failed to update faction status for world {}: {}", world_id, e);
    }

    // Generate political events
    match generate_political_events(ctx, world_id, current_hour) {
        Ok(event_ids) => {
            stats.political_events += event_ids.len() as u32;
            // Convert political events to narrative events
            for _event_id in event_ids {
                match create_narrative_event(
                    ctx,
                    world_id,
                    1, // Default game ID
                    EventCategory::Political,
                    "Political development".to_string(),
                    "Political landscape is shifting".to_string(),
                    4,
                ) {
                    Ok(_) => stats.narrative_events += 1,
                    Err(e) => log::warn!("Failed to create political narrative event: {}", e),
                }
            }
        },
        Err(e) => log::warn!("Failed to generate political events for world {}: {}", world_id, e),
    }

    // Process ongoing political events
    if let Err(e) = process_political_events(ctx, world_id, current_hour) {
        log::warn!("Failed to process political events for world {}: {}", world_id, e);
    }

    // 4. Update natural systems
    if let Err(e) = update_climate_conditions(ctx, world_id, current_hour) {
        log::warn!("Failed to update climate conditions for world {}: {}", world_id, e);
    }

    // Generate natural events
    match generate_natural_events(ctx, world_id, current_hour) {
        Ok(event_ids) => {
            stats.natural_events += event_ids.len() as u32;
        },
        Err(e) => log::warn!("Failed to generate natural events for world {}: {}", world_id, e),
    }

    // Process ongoing natural events
    if let Err(e) = process_natural_events(ctx, world_id, current_hour) {
        log::warn!("Failed to process natural events for world {}: {}", world_id, e);
    }

    // Update seasonal activities and phenology (daily)
    if current_hour % 24 == 0 {
        if let Err(e) = crate::natural::seasonal_cycles::update_seasonal_activities(ctx, world_id, current_hour) {
            log::warn!("Failed to update seasonal activities for world {}: {}", world_id, e);
        }

        if let Err(e) = crate::natural::seasonal_cycles::update_phenological_phases(ctx, world_id, current_hour) {
            log::warn!("Failed to update phenological phases for world {}: {}", world_id, e);
        }
    }

    // 5. Update world statistics
    if let Err(e) = crate::world::game_world::update_world_stats(ctx, world_id) {
        log::warn!("Failed to update world stats for world {}: {}", world_id, e);
    }

    // 6. Periodic Enonomics sync (every day)
    if current_hour % 24 == 0 {
        if let Err(e) = crate::economics::enonomics_integration::periodic_enonomics_sync(ctx, world_id) {
            log::warn!("Failed Enonomics sync for world {}: {}", world_id, e);
        }
    }

    stats.events_generated = stats.narrative_events + stats.economic_events + stats.political_events + stats.natural_events;

    Ok(stats)
}

// Get worlds that need updating
fn get_worlds_needing_update(ctx: &ReducerContext) -> Result<Vec<GameWorld>, String> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| format!("Time error: {}", e))?
        .as_millis() as i64;

    let worlds: Vec<GameWorld> = ctx.db.game_world()
        .iter()
        .filter(|w| w.is_active)
        .filter(|w| w.narrative_speed != crate::world::NarrativeSpeed::Paused)
        .filter(|w| w.next_update_ms <= now)
        .cloned()
        .collect();

    Ok(worlds)
}

// Update scheduler performance statistics
fn update_scheduler_stats(
    ctx: &ReducerContext,
    stats: &ProcessingStats,
) -> Result<(), String> {
    let mut config = ctx.db.scheduler_config()
        .id()
        .find(&1)
        .ok_or("Scheduler config not found")?;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| format!("Time error: {}", e))?
        .as_millis() as i64;

    config.last_run_ms = now;
    config.next_run_ms = now + config.run_interval_ms as i64;

    // Update performance stats
    let stats_json = serde_json::json!({
        "last_run": {
            "worlds_processed": stats.worlds_processed,
            "events_generated": stats.events_generated,
            "processing_time_ms": stats.processing_time_ms,
            "errors": stats.errors_encountered,
            "narrative_events": stats.narrative_events,
            "economic_events": stats.economic_events,
            "political_events": stats.political_events,
            "natural_events": stats.natural_events,
        },
        "timestamp": now
    });

    config.performance_stats = stats_json.to_string();

    ctx.db.scheduler_config().id().update(1, config);

    Ok(())
}

// Get scheduler status and performance metrics
#[spacetimedb::reducer]
pub fn get_scheduler_status(ctx: &ReducerContext) -> Result<(), String> {
    let config = ctx.db.scheduler_config()
        .id()
        .find(&1)
        .ok_or("Scheduler not initialized")?;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| format!("Time error: {}", e))?
        .as_millis() as i64;

    let time_until_next = if config.next_run_ms > now {
        config.next_run_ms - now
    } else {
        0
    };

    log::info!("Scheduler Status:");
    log::info!("  Enabled: {}", config.enabled);
    log::info!("  Batch Size: {}", config.batch_size);
    log::info!("  Run Interval: {}ms", config.run_interval_ms);
    log::info!("  Time Until Next Run: {}ms", time_until_next);
    log::info!("  Last Performance: {}", config.performance_stats);

    Ok(())
}

// Enable or disable the scheduler
#[spacetimedb::reducer]
pub fn set_scheduler_enabled(
    ctx: &ReducerContext,
    enabled: bool,
) -> Result<(), String> {
    let mut config = ctx.db.scheduler_config()
        .id()
        .find(&1)
        .ok_or("Scheduler not initialized")?;

    config.enabled = enabled;

    if enabled {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_millis() as i64;
        config.next_run_ms = now + config.run_interval_ms as i64;
    }

    ctx.db.scheduler_config().id().update(1, config);

    log::info!("Scheduler {}", if enabled { "enabled" } else { "disabled" });
    Ok(())
}

// Configure scheduler parameters
#[spacetimedb::reducer]
pub fn configure_scheduler(
    ctx: &ReducerContext,
    batch_size: u32,
    run_interval_ms: u64,
    max_processing_time_ms: u64,
) -> Result<(), String> {
    let mut config = ctx.db.scheduler_config()
        .id()
        .find(&1)
        .ok_or("Scheduler not initialized")?;

    config.batch_size = batch_size;
    config.run_interval_ms = run_interval_ms;
    config.max_processing_time_ms = max_processing_time_ms;

    ctx.db.scheduler_config().id().update(1, config);

    log::info!("Scheduler configured: batch_size={}, interval={}ms, max_time={}ms",
        batch_size, run_interval_ms, max_processing_time_ms);

    Ok(())
}