use spacetimedb::{ReducerContext, Table, Timestamp};
use log;
use crate::tables::*;
use crate::tables::events::{simulation_time, autoticker_config};

/// Initialize the simulation time
#[spacetimedb::reducer]
pub fn init_simulation(ctx: &ReducerContext) -> Result<(), String> {
    // Check if already initialized
    if ctx.db.simulation_time().iter().count() > 0 {
        return Err("Simulation already initialized".to_string());
    }
    
    // Create the time tracker
    ctx.db.simulation_time().insert(SimulationTime {
        id: 1,
        current_hour: 0,
        day_of_week: 0,
        hour_of_day: 0,
        total_days: 0,
        is_running: false,
        auto_tick_enabled: false,
        tick_interval_ms: 3600000, // Default: 1 hour = 3,600,000 ms
    });
    
    Ok(())
}

/// Start or stop the simulation
#[spacetimedb::reducer]
pub fn toggle_simulation(ctx: &ReducerContext) -> Result<(), String> {
    let mut time = ctx.db.simulation_time().id().find(&1)
        .ok_or("Simulation not initialized")?;
    
    time.is_running = !time.is_running;
    ctx.db.simulation_time().id().update(time.clone());
    
    if time.is_running {
        log::info!("Simulation started at hour {}", time.current_hour);
    } else {
        log::info!("Simulation paused at hour {}", time.current_hour);
    }
    
    Ok(())
}

/// Main time ticker - advances simulation by one hour
#[spacetimedb::reducer]
pub fn tick_hour(ctx: &ReducerContext) -> Result<(), String> {
    let mut time = ctx.db.simulation_time().id().find(&1)
        .ok_or("Simulation not initialized")?;
    
    if !time.is_running {
        return Ok(()); // Don't tick if paused
    }
    
    // Advance time
    time.current_hour += 1;
    time.hour_of_day = (time.current_hour % 24) as u8;
    time.day_of_week = ((time.current_hour / 24) % 7) as u8;
    time.total_days = time.current_hour / 24;
    
    ctx.db.simulation_time().id().update(time.clone());
    
    // For now, we'll just log when updates should happen
    // In production, these would be triggered by separate scheduled tasks
    
    // Log hourly updates for individuals
    log::info!("Hour {}: Individual updates triggered", time.current_hour);
    
    // Log daily updates for buildings (every 24 hours)
    if time.hour_of_day == 0 {
        log::info!("Day {}: Building updates triggered", time.total_days);
    }
    
    // Log weekly updates for cities (every 168 hours)
    if time.current_hour % 168 == 0 {
        log::info!("Week {}: City updates triggered", time.current_hour / 168);
    }
    
    Ok(())
}


/// Get current simulation hour
#[spacetimedb::reducer]
pub fn get_current_hour(ctx: &ReducerContext) -> Result<(), String> {
    let time = ctx.db.simulation_time().id().find(&1)
        .ok_or("Simulation not initialized")?;
    log::info!("Current simulation hour: {}", time.current_hour);
    Ok(())
}

/// Skip time forward (for testing)
#[spacetimedb::reducer]
pub fn skip_hours(ctx: &ReducerContext, hours: u64) -> Result<(), String> {
    for _ in 0..hours {
        advance_time_by_one_hour(ctx)?;
    }
    Ok(())
}

/// Helper function to advance time by one hour (for internal use)
fn advance_time_by_one_hour(ctx: &ReducerContext) -> Result<(), String> {
    let mut time = ctx.db.simulation_time().id().find(&1)
        .ok_or("Simulation not initialized")?;
    
    if !time.is_running {
        return Err("Simulation is not running".to_string());
    }
    
    // Advance time
    time.current_hour += 1;
    time.hour_of_day = (time.current_hour % 24) as u8;
    time.day_of_week = ((time.current_hour / 24) % 7) as u8;
    time.total_days = time.current_hour / 24;
    
    let current_hour = time.current_hour;
    let hour_of_day = time.hour_of_day;
    let total_days = time.total_days;
    
    ctx.db.simulation_time().id().update(time);
    
    // For now, we'll just log when updates should happen
    // In production, these would be triggered by separate scheduled tasks
    
    // Log hourly updates for individuals
    log::info!("Hour {}: Individual updates triggered", current_hour);
    
    // Log daily updates for buildings (every 24 hours)
    if hour_of_day == 0 {
        log::info!("Day {}: Building updates triggered", total_days);
    }
    
    // Log weekly updates for cities (every 168 hours)
    if current_hour % 168 == 0 {
        log::info!("Week {}: City updates triggered", current_hour / 168);
    }
    
    Ok(())
}

// =============================================================================
// AUTOTICKER FUNCTIONALITY
// =============================================================================

/// Check if it's time for an auto-tick and execute if needed
/// This reducer should be called periodically to check for auto-ticks
#[spacetimedb::reducer]
pub fn check_autotick(ctx: &ReducerContext) -> Result<(), String> {
    let time = ctx.db.simulation_time().id().find(&1)
        .ok_or("Simulation not initialized")?;
    
    // Only proceed if auto-tick is enabled and simulation is running
    if !time.auto_tick_enabled || !time.is_running {
        return Ok(());
    }
    
    // Check if we have an autoticker config
    let config = ctx.db.autoticker_config().id().find(&1);
    let current_time = ctx.timestamp.to_micros_since_unix_epoch() / 1000; // Convert to milliseconds
    
    let should_tick = if let Some(config) = config {
        current_time >= config.next_tick_time
    } else {
        // First time running, create config and tick immediately
        true
    };
    
    if should_tick {
        // Execute the tick
        tick_hour(ctx)?;
        
        // Update the autoticker config for next tick
        let next_tick_time = current_time + time.tick_interval_ms as i64;
        
        if let Some(mut config) = ctx.db.autoticker_config().id().find(&1) {
            config.last_tick_time = current_time;
            config.next_tick_time = next_tick_time;
            ctx.db.autoticker_config().id().update(config);
        } else {
            ctx.db.autoticker_config().insert(AutotickerConfig {
                id: 1,
                last_tick_time: current_time,
                next_tick_time,
            });
        }
        
        log::info!("Auto-tick executed at time {}, next tick at {}", current_time, next_tick_time);
    }
    
    Ok(())
}

/// Start the autoticker with current tick interval
#[spacetimedb::reducer]
pub fn start_autoticker(ctx: &ReducerContext) -> Result<(), String> {
    let mut time = ctx.db.simulation_time().id().find(&1)
        .ok_or("Simulation not initialized")?;
    
    if time.auto_tick_enabled {
        return Err("Autoticker is already running".to_string());
    }
    
    // Enable auto-tick
    time.auto_tick_enabled = true;
    ctx.db.simulation_time().id().update(time.clone());
    
    // Initialize autoticker config for immediate first tick
    let current_time = ctx.timestamp.to_micros_since_unix_epoch() / 1000; // Convert to milliseconds
    
    // Remove any existing config
    if let Some(config) = ctx.db.autoticker_config().id().find(&1) {
        ctx.db.autoticker_config().id().delete(&config.id);
    }
    
    // Create new config that will trigger immediately on next check
    ctx.db.autoticker_config().insert(AutotickerConfig {
        id: 1,
        last_tick_time: 0,
        next_tick_time: current_time, // Trigger immediately
    });
    
    log::info!("Autoticker started with interval: {}ms", time.tick_interval_ms);
    log::info!("Call 'check_autotick' periodically to advance time automatically");
    Ok(())
}

/// Stop the autoticker
#[spacetimedb::reducer]
pub fn stop_autoticker(ctx: &ReducerContext) -> Result<(), String> {
    let mut time = ctx.db.simulation_time().id().find(&1)
        .ok_or("Simulation not initialized")?;
    
    if !time.auto_tick_enabled {
        return Err("Autoticker is not running".to_string());
    }
    
    // Disable auto-tick
    time.auto_tick_enabled = false;
    ctx.db.simulation_time().id().update(time);
    
    // Clear autoticker config
    if let Some(config) = ctx.db.autoticker_config().id().find(&1) {
        ctx.db.autoticker_config().id().delete(&config.id);
    }
    
    log::info!("Autoticker stopped");
    Ok(())
}

/// Set the autoticker interval (in milliseconds)
#[spacetimedb::reducer]
pub fn set_tick_interval(ctx: &ReducerContext, interval_ms: u64) -> Result<(), String> {
    if interval_ms == 0 {
        return Err("Tick interval must be greater than 0".to_string());
    }
    
    if interval_ms < 1000 {
        return Err("Minimum tick interval is 1000ms (1 second)".to_string());
    }
    
    let mut time = ctx.db.simulation_time().id().find(&1)
        .ok_or("Simulation not initialized")?;
    
    let was_running = time.auto_tick_enabled;
    
    // Stop autoticker if running
    if was_running {
        stop_autoticker(ctx)?;
        // Refresh time record after stopping
        time = ctx.db.simulation_time().id().find(&1)
            .ok_or("Simulation not initialized")?;
    }
    
    // Update interval
    time.tick_interval_ms = interval_ms;
    ctx.db.simulation_time().id().update(time);
    
    // Restart autoticker if it was running
    if was_running {
        start_autoticker(ctx)?;
    }
    
    log::info!("Tick interval set to {}ms", interval_ms);
    Ok(())
}

/// Set a predefined tick rate
#[spacetimedb::reducer]
pub fn set_tick_rate(ctx: &ReducerContext, rate: String) -> Result<(), String> {
    let interval_ms = match rate.as_str() {
        "realtime" => 3600000,    // 1 hour = 1 hour real time
        "fast" => 60000,          // 1 hour = 1 minute real time
        "very_fast" => 10000,     // 1 hour = 10 seconds real time
        "test" => 1000,           // 1 hour = 1 second real time
        "slow" => 300000,         // 1 hour = 5 minutes real time
        _ => return Err("Invalid rate. Use: realtime, fast, very_fast, test, or slow".to_string()),
    };
    
    set_tick_interval(ctx, interval_ms)
}

/// Get autoticker status
#[spacetimedb::reducer]
pub fn get_autoticker_status(ctx: &ReducerContext) -> Result<(), String> {
    let time = ctx.db.simulation_time().id().find(&1)
        .ok_or("Simulation not initialized")?;
    
    log::info!("Autoticker Status:");
    log::info!("  Enabled: {}", time.auto_tick_enabled);
    log::info!("  Interval: {}ms", time.tick_interval_ms);
    log::info!("  Simulation Running: {}", time.is_running);
    
    if let Some(config) = ctx.db.autoticker_config().id().find(&1) {
        let current_time = ctx.timestamp.to_micros_since_unix_epoch() / 1000;
        let time_until_next = if config.next_tick_time > current_time {
            config.next_tick_time - current_time
        } else {
            0
        };
        log::info!("  Last Tick: {} ms ago", current_time - config.last_tick_time);
        log::info!("  Next Tick: in {} ms", time_until_next);
    }
    
    if time.auto_tick_enabled {
        let rate_name = match time.tick_interval_ms {
            3600000 => "realtime",
            60000 => "fast",
            10000 => "very_fast",
            1000 => "test",
            300000 => "slow",
            _ => "custom",
        };
        log::info!("  Rate: {} ({}ms)", rate_name, time.tick_interval_ms);
        log::info!("  Note: Call 'check_autotick' periodically to trigger automatic progression");
    }
    
    Ok(())
}

