// Natural events and environmental systems for world simulation

use spacetimedb::{ReducerContext, Table, SpacetimeType};
use serde::{Serialize, Deserialize};
use log;
use rand::Rng;
use crate::world::{Season, ClimateZone};
use crate::narrative::{create_narrative_event, EventCategory};

pub mod weather;
pub mod disasters;
pub mod seasonal_cycles;

// Natural event types
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum NaturalEventType {
    WeatherChange,
    Storm,
    Flood,
    Drought,
    Earthquake,
    Fire,
    Plague,
    Migration,
    Harvest,
    ResourceDiscovery,
    ClimateShift,
    EcosystemChange,
}

// Severity levels for natural events
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum EventSeverity {
    Minor,      // 1-2 importance
    Moderate,   // 3-4 importance
    Major,      // 5-6 importance
    Catastrophic, // 7 importance
}

// Natural event record
#[spacetimedb::table(name = natural_event)]
pub struct NaturalEvent {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub event_type: NaturalEventType,
    pub severity: EventSeverity,
    pub affected_region: String, // JSON array of city IDs
    pub start_hour: u64,
    pub duration_hours: u32,
    pub description: String,
    pub environmental_effects: String, // JSON of effects on environment
    pub economic_impact: f32,
    pub population_impact: f32,
    pub resolved: bool,
    pub resolution_description: String,
}

// Climate state tracking
#[spacetimedb::table(name = climate_state)]
pub struct ClimateState {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub region_id: u32,
    pub current_temperature: f32, // Celsius
    pub humidity: f32,           // 0-100%
    pub precipitation: f32,      // mm/hour
    pub wind_speed: f32,         // km/h
    pub atmospheric_pressure: f32, // hPa
    pub air_quality: f32,        // 0-100 index
    pub last_updated_hour: u64,
    pub weather_pattern: WeatherPattern,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum WeatherPattern {
    Clear,
    Cloudy,
    Rainy,
    Stormy,
    Foggy,
    Windy,
    Hot,
    Cold,
}

// Seasonal effects on different aspects
#[spacetimedb::table(name = seasonal_effect)]
pub struct SeasonalEffect {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub season: Season,
    pub climate_zone: ClimateZone,
    pub agriculture_modifier: f32,   // Crop yield multiplier
    pub trade_modifier: f32,         // Trade efficiency
    pub population_health: f32,      // Health effects
    pub migration_tendency: f32,     // Population movement
    pub resource_availability: String, // JSON of resource modifiers
    pub event_probabilities: String,   // JSON of event type probabilities
}

// Initialize natural systems for a world
#[spacetimedb::reducer]
pub fn initialize_natural_systems(
    ctx: &ReducerContext,
    world_id: u32,
    climate_zone: ClimateZone,
) -> Result<(), String> {
    let world = ctx.db.game_world()
        .id()
        .find(&world_id)
        .ok_or("World not found")?;

    // Initialize climate states for regions (using city data as proxy)
    let cities: Vec<u32> = ctx.db.market()
        .iter()
        .filter(|m| m.world_id == world_id)
        .map(|m| m.city_id)
        .collect::<std::collections::HashSet<u32>>()
        .into_iter()
        .collect();

    for city_id in cities {
        let climate_id = ctx.db.climate_state().iter().count() as u32 + 1;

        // Base climate values based on climate zone
        let (base_temp, base_humidity, base_precipitation) = match climate_zone {
            ClimateZone::Arctic => (-20.0, 60.0, 0.5),
            ClimateZone::Temperate => (15.0, 70.0, 1.5),
            ClimateZone::Tropical => (28.0, 85.0, 3.0),
            ClimateZone::Arid => (25.0, 30.0, 0.2),
            ClimateZone::Mediterranean => (20.0, 65.0, 1.0),
        };

        let climate_state = ClimateState {
            id: climate_id,
            world_id,
            region_id: city_id,
            current_temperature: base_temp,
            humidity: base_humidity,
            precipitation: base_precipitation,
            wind_speed: 10.0,
            atmospheric_pressure: 1013.25,
            air_quality: 80.0,
            last_updated_hour: world.total_hours,
            weather_pattern: WeatherPattern::Clear,
        };

        ctx.db.climate_state().insert(climate_state);
    }

    // Initialize seasonal effects for all seasons
    for season in [Season::Spring, Season::Summer, Season::Autumn, Season::Winter] {
        let effect_id = ctx.db.seasonal_effect().iter().count() as u32 + 1;

        let (agri_mod, trade_mod, health_mod, migration_mod) =
            calculate_seasonal_modifiers(season, climate_zone);

        let seasonal_effect = SeasonalEffect {
            id: effect_id,
            world_id,
            season,
            climate_zone,
            agriculture_modifier: agri_mod,
            trade_modifier: trade_mod,
            population_health: health_mod,
            migration_tendency: migration_mod,
            resource_availability: generate_resource_modifiers_json(season, climate_zone),
            event_probabilities: generate_event_probabilities_json(season, climate_zone),
        };

        ctx.db.seasonal_effect().insert(seasonal_effect);
    }

    log::info!("Initialized natural systems for world {} with climate {:?}",
        world_id, climate_zone);
    Ok(())
}

// Calculate seasonal modifiers based on season and climate
fn calculate_seasonal_modifiers(
    season: Season,
    climate_zone: ClimateZone,
) -> (f32, f32, f32, f32) {
    match (season, climate_zone) {
        // Spring modifiers
        (Season::Spring, ClimateZone::Temperate) => (1.2, 1.1, 1.0, 0.8),
        (Season::Spring, ClimateZone::Arctic) => (0.8, 0.9, 0.9, 1.2),
        (Season::Spring, ClimateZone::Tropical) => (1.1, 1.0, 1.0, 0.9),
        (Season::Spring, ClimateZone::Arid) => (1.3, 1.0, 1.1, 0.7),
        (Season::Spring, ClimateZone::Mediterranean) => (1.4, 1.1, 1.1, 0.8),

        // Summer modifiers
        (Season::Summer, ClimateZone::Temperate) => (1.3, 1.2, 1.1, 0.9),
        (Season::Summer, ClimateZone::Arctic) => (1.0, 1.1, 1.0, 1.0),
        (Season::Summer, ClimateZone::Tropical) => (0.9, 0.8, 0.8, 1.1),
        (Season::Summer, ClimateZone::Arid) => (0.6, 0.7, 0.7, 1.3),
        (Season::Summer, ClimateZone::Mediterranean) => (1.0, 1.2, 1.0, 1.0),

        // Autumn modifiers
        (Season::Autumn, ClimateZone::Temperate) => (1.1, 1.0, 1.0, 1.0),
        (Season::Autumn, ClimateZone::Arctic) => (0.7, 0.8, 0.8, 1.1),
        (Season::Autumn, ClimateZone::Tropical) => (1.0, 1.0, 1.0, 1.0),
        (Season::Autumn, ClimateZone::Arid) => (1.2, 1.1, 1.0, 0.9),
        (Season::Autumn, ClimateZone::Mediterranean) => (1.2, 1.0, 1.0, 0.9),

        // Winter modifiers
        (Season::Winter, ClimateZone::Temperate) => (0.8, 0.9, 0.9, 1.1),
        (Season::Winter, ClimateZone::Arctic) => (0.3, 0.6, 0.6, 1.4),
        (Season::Winter, ClimateZone::Tropical) => (1.0, 1.0, 1.0, 1.0),
        (Season::Winter, ClimateZone::Arid) => (1.0, 1.0, 1.0, 1.0),
        (Season::Winter, ClimateZone::Mediterranean) => (0.9, 0.9, 0.9, 1.0),
    }
}

// Generate resource availability modifiers for season/climate
fn generate_resource_modifiers_json(season: Season, climate_zone: ClimateZone) -> String {
    let modifiers = match (season, climate_zone) {
        (Season::Spring, _) => serde_json::json!({
            "food": 1.1,
            "raw_materials": 1.2,
            "water": 1.3,
            "luxury": 0.9
        }),
        (Season::Summer, ClimateZone::Arid) => serde_json::json!({
            "food": 0.7,
            "raw_materials": 1.0,
            "water": 0.5,
            "luxury": 1.1
        }),
        (Season::Summer, _) => serde_json::json!({
            "food": 1.3,
            "raw_materials": 1.1,
            "water": 1.0,
            "luxury": 1.2
        }),
        (Season::Autumn, _) => serde_json::json!({
            "food": 1.4,
            "raw_materials": 1.0,
            "water": 1.1,
            "luxury": 1.0
        }),
        (Season::Winter, ClimateZone::Arctic) => serde_json::json!({
            "food": 0.5,
            "raw_materials": 0.7,
            "water": 0.8,
            "luxury": 0.6
        }),
        (Season::Winter, _) => serde_json::json!({
            "food": 0.8,
            "raw_materials": 0.9,
            "water": 1.0,
            "luxury": 0.8
        }),
    };

    modifiers.to_string()
}

// Generate event probability modifiers for season/climate
fn generate_event_probabilities_json(season: Season, climate_zone: ClimateZone) -> String {
    let probabilities = match (season, climate_zone) {
        (Season::Spring, _) => serde_json::json!({
            "flood": 1.5,
            "storm": 1.2,
            "plague": 0.8,
            "migration": 1.3,
            "resource_discovery": 1.2
        }),
        (Season::Summer, ClimateZone::Arid) => serde_json::json!({
            "drought": 2.0,
            "fire": 1.8,
            "storm": 0.5,
            "migration": 1.5
        }),
        (Season::Summer, _) => serde_json::json!({
            "storm": 1.4,
            "fire": 1.2,
            "drought": 0.8,
            "harvest": 1.5
        }),
        (Season::Autumn, _) => serde_json::json!({
            "harvest": 2.0,
            "migration": 1.1,
            "storm": 1.0,
            "resource_discovery": 1.1
        }),
        (Season::Winter, ClimateZone::Arctic) => serde_json::json!({
            "storm": 1.8,
            "cold_snap": 2.0,
            "migration": 0.5,
            "plague": 1.3
        }),
        (Season::Winter, _) => serde_json::json!({
            "storm": 1.3,
            "flood": 1.2,
            "plague": 1.1,
            "migration": 0.8
        }),
    };

    probabilities.to_string()
}

// Update climate conditions based on time and season
#[spacetimedb::reducer]
pub fn update_climate_conditions(
    ctx: &ReducerContext,
    world_id: u32,
    current_hour: u64,
) -> Result<(), String> {
    let world = ctx.db.game_world()
        .id()
        .find(&world_id)
        .ok_or("World not found")?;

    let current_season = crate::world::calculate_season_from_hour(current_hour);

    let climate_states: Vec<ClimateState> = ctx.db.climate_state()
        .iter()
        .filter(|c| c.world_id == world_id)
        .cloned()
        .collect();

    for mut climate in climate_states {
        // Apply seasonal changes
        let seasonal_effect = ctx.db.seasonal_effect()
            .iter()
            .find(|e| e.world_id == world_id &&
                     e.season == current_season &&
                     e.climate_zone == world.climate_zone);

        if let Some(effect) = seasonal_effect {
            // Update temperature based on season
            climate.current_temperature = apply_seasonal_temperature_change(
                climate.current_temperature,
                current_season,
                world.climate_zone,
            );

            // Update other weather parameters
            update_weather_parameters(&mut climate, current_season, current_hour);
        }

        climate.last_updated_hour = current_hour;
        ctx.db.climate_state().id().update(climate.id, climate);
    }

    Ok(())
}

// Apply seasonal temperature changes
fn apply_seasonal_temperature_change(
    base_temp: f32,
    season: Season,
    climate_zone: ClimateZone,
) -> f32 {
    let seasonal_adjustment = match (season, climate_zone) {
        (Season::Summer, ClimateZone::Arctic) => 15.0,
        (Season::Summer, ClimateZone::Temperate) => 10.0,
        (Season::Summer, ClimateZone::Tropical) => 2.0,
        (Season::Summer, ClimateZone::Arid) => 8.0,
        (Season::Summer, ClimateZone::Mediterranean) => 8.0,

        (Season::Winter, ClimateZone::Arctic) => -25.0,
        (Season::Winter, ClimateZone::Temperate) => -15.0,
        (Season::Winter, ClimateZone::Tropical) => -2.0,
        (Season::Winter, ClimateZone::Arid) => -5.0,
        (Season::Winter, ClimateZone::Mediterranean) => -8.0,

        (Season::Spring, _) => 0.0,
        (Season::Autumn, _) => -5.0,
    };

    base_temp + seasonal_adjustment
}

// Update weather parameters based on season and time
fn update_weather_parameters(
    climate: &mut ClimateState,
    season: Season,
    hour: u64,
) {
    let mut rng = rand::thread_rng();

    // Daily temperature variation
    let hour_of_day = hour % 24;
    let daily_temp_variation = ((hour_of_day as f32 - 12.0) / 24.0 * std::f32::consts::PI).sin() * 5.0;
    climate.current_temperature += daily_temp_variation;

    // Random weather changes
    if rng.gen::<f32>() < 0.1 { // 10% chance of weather change
        climate.weather_pattern = match rng.gen_range(0..8) {
            0 => WeatherPattern::Clear,
            1 => WeatherPattern::Cloudy,
            2 => WeatherPattern::Rainy,
            3 => WeatherPattern::Stormy,
            4 => WeatherPattern::Foggy,
            5 => WeatherPattern::Windy,
            6 => WeatherPattern::Hot,
            7 => WeatherPattern::Cold,
            _ => WeatherPattern::Clear,
        };
    }

    // Update based on weather pattern
    match climate.weather_pattern {
        WeatherPattern::Rainy => {
            climate.precipitation = (climate.precipitation + 2.0).min(10.0);
            climate.humidity = (climate.humidity + 10.0).min(100.0);
        },
        WeatherPattern::Stormy => {
            climate.precipitation = (climate.precipitation + 5.0).min(20.0);
            climate.wind_speed = (climate.wind_speed + 20.0).min(100.0);
            climate.humidity = (climate.humidity + 15.0).min(100.0);
        },
        WeatherPattern::Clear => {
            climate.precipitation = (climate.precipitation - 1.0).max(0.0);
            climate.humidity = (climate.humidity - 5.0).max(20.0);
        },
        _ => {} // Other patterns have minimal immediate effects
    }
}

// Generate natural events based on season and climate
#[spacetimedb::reducer]
pub fn generate_natural_events(
    ctx: &ReducerContext,
    world_id: u32,
    current_hour: u64,
) -> Result<Vec<u32>, String> {
    let mut event_ids = Vec::new();
    let mut rng = rand::thread_rng();

    let world = ctx.db.game_world()
        .id()
        .find(&world_id)
        .ok_or("World not found")?;

    let current_season = crate::world::calculate_season_from_hour(current_hour);

    // Get seasonal effect probabilities
    let seasonal_effect = ctx.db.seasonal_effect()
        .iter()
        .find(|e| e.world_id == world_id &&
                 e.season == current_season &&
                 e.climate_zone == world.climate_zone);

    if let Some(effect) = seasonal_effect {
        let probabilities: serde_json::Value = serde_json::from_str(&effect.event_probabilities)
            .unwrap_or_else(|_| serde_json::json!({}));

        // Check for various natural events
        for (event_type_str, base_probability) in probabilities.as_object().unwrap_or(&serde_json::Map::new()) {
            let prob_multiplier = base_probability.as_f64().unwrap_or(1.0) as f32;
            let base_chance = 0.01; // 1% base chance per hour
            let adjusted_chance = base_chance * prob_multiplier;

            if rng.gen::<f32>() < adjusted_chance {
                let event_type = match event_type_str.as_str() {
                    "flood" => NaturalEventType::Flood,
                    "drought" => NaturalEventType::Drought,
                    "storm" => NaturalEventType::Storm,
                    "fire" => NaturalEventType::Fire,
                    "plague" => NaturalEventType::Plague,
                    "migration" => NaturalEventType::Migration,
                    "harvest" => NaturalEventType::Harvest,
                    "resource_discovery" => NaturalEventType::ResourceDiscovery,
                    _ => continue,
                };

                let event_id = create_natural_event(
                    ctx,
                    world_id,
                    event_type,
                    current_hour,
                )?;

                event_ids.push(event_id);
            }
        }
    }

    // Generate climate-based events
    let climate_states: Vec<ClimateState> = ctx.db.climate_state()
        .iter()
        .filter(|c| c.world_id == world_id)
        .cloned()
        .collect();

    for climate in climate_states {
        // Extreme weather events
        if climate.current_temperature > 40.0 && rng.gen::<f32>() < 0.02 {
            let event_id = create_natural_event(
                ctx,
                world_id,
                NaturalEventType::Fire,
                current_hour,
            )?;
            event_ids.push(event_id);
        }

        if climate.precipitation > 15.0 && rng.gen::<f32>() < 0.05 {
            let event_id = create_natural_event(
                ctx,
                world_id,
                NaturalEventType::Flood,
                current_hour,
            )?;
            event_ids.push(event_id);
        }

        if climate.wind_speed > 80.0 && rng.gen::<f32>() < 0.03 {
            let event_id = create_natural_event(
                ctx,
                world_id,
                NaturalEventType::Storm,
                current_hour,
            )?;
            event_ids.push(event_id);
        }
    }

    if !event_ids.is_empty() {
        log::info!("Generated {} natural events for world {}", event_ids.len(), world_id);
    }

    Ok(event_ids)
}

// Create a specific natural event
fn create_natural_event(
    ctx: &ReducerContext,
    world_id: u32,
    event_type: NaturalEventType,
    hour: u64,
) -> Result<u32, String> {
    let event_id = ctx.db.natural_event().iter().count() as u32 + 1;

    let (severity, duration, description, economic_impact, population_impact) =
        generate_event_details(event_type);

    let natural_event = NaturalEvent {
        id: event_id,
        world_id,
        event_type,
        severity,
        affected_region: "[]".to_string(), // TODO: Determine affected regions
        start_hour: hour,
        duration_hours: duration,
        description,
        environmental_effects: generate_environmental_effects_json(event_type, severity),
        economic_impact,
        population_impact,
        resolved: false,
        resolution_description: String::new(),
    };

    ctx.db.natural_event().insert(natural_event);

    // Create corresponding narrative event
    let importance = match severity {
        EventSeverity::Minor => 2,
        EventSeverity::Moderate => 4,
        EventSeverity::Major => 6,
        EventSeverity::Catastrophic => 7,
    };

    let narrative_title = format!("{:?} Event", event_type);
    create_narrative_event(
        ctx,
        world_id,
        1, // Default game ID
        EventCategory::Natural,
        narrative_title,
        description.clone(),
        importance,
    )?;

    Ok(event_id)
}

// Generate event details based on type
fn generate_event_details(event_type: NaturalEventType) -> (EventSeverity, u32, String, f32, f32) {
    let mut rng = rand::thread_rng();

    match event_type {
        NaturalEventType::Storm => {
            let severity = if rng.gen::<f32>() < 0.7 { EventSeverity::Minor } else { EventSeverity::Moderate };
            let duration = rng.gen_range(6..24);
            let description = "A powerful storm system moves through the region, bringing strong winds and heavy rain.".to_string();
            (severity, duration, description, -0.1, -0.05)
        },
        NaturalEventType::Flood => {
            let severity = if rng.gen::<f32>() < 0.6 { EventSeverity::Moderate } else { EventSeverity::Major };
            let duration = rng.gen_range(48..168);
            let description = "Rising water levels threaten settlements and disrupt transportation networks.".to_string();
            (severity, duration, description, -0.3, -0.15)
        },
        NaturalEventType::Drought => {
            let severity = EventSeverity::Major;
            let duration = rng.gen_range(168..720); // 1 week to 1 month
            let description = "Extended period without rainfall threatens crops and water supplies.".to_string();
            (severity, duration, description, -0.4, -0.2)
        },
        NaturalEventType::Fire => {
            let severity = if rng.gen::<f32>() < 0.5 { EventSeverity::Moderate } else { EventSeverity::Major };
            let duration = rng.gen_range(24..72);
            let description = "Wildfires spread rapidly through dry vegetation, threatening structures and lives.".to_string();
            (severity, duration, description, -0.2, -0.1)
        },
        NaturalEventType::Harvest => {
            let severity = EventSeverity::Minor;
            let duration = rng.gen_range(168..336); // 1-2 weeks
            let description = "Abundant harvest yields provide surplus food and economic prosperity.".to_string();
            (severity, duration, description, 0.3, 0.1)
        },
        NaturalEventType::ResourceDiscovery => {
            let severity = EventSeverity::Moderate;
            let duration = 24;
            let description = "New natural resource deposits have been discovered in the region.".to_string();
            (severity, duration, description, 0.2, 0.05)
        },
        NaturalEventType::Migration => {
            let severity = EventSeverity::Minor;
            let duration = rng.gen_range(168..720);
            let description = "Seasonal migration patterns bring changes to local population dynamics.".to_string();
            (severity, duration, description, 0.1, 0.15)
        },
        _ => {
            let severity = EventSeverity::Minor;
            let duration = 24;
            let description = format!("{:?} event affects the region", event_type);
            (severity, duration, description, 0.0, 0.0)
        }
    }
}

// Generate environmental effects JSON
fn generate_environmental_effects_json(event_type: NaturalEventType, severity: EventSeverity) -> String {
    let severity_multiplier = match severity {
        EventSeverity::Minor => 1.0,
        EventSeverity::Moderate => 2.0,
        EventSeverity::Major => 3.0,
        EventSeverity::Catastrophic => 5.0,
    };

    let effects = match event_type {
        NaturalEventType::Flood => serde_json::json!({
            "water_level": 2.0 * severity_multiplier,
            "soil_fertility": -0.3 * severity_multiplier,
            "transportation": -0.5 * severity_multiplier,
            "air_quality": -0.2 * severity_multiplier
        }),
        NaturalEventType::Drought => serde_json::json!({
            "water_availability": -0.6 * severity_multiplier,
            "vegetation": -0.4 * severity_multiplier,
            "fire_risk": 0.8 * severity_multiplier,
            "crop_yield": -0.7 * severity_multiplier
        }),
        NaturalEventType::Fire => serde_json::json!({
            "air_quality": -0.8 * severity_multiplier,
            "vegetation": -0.9 * severity_multiplier,
            "soil_fertility": -0.2 * severity_multiplier,
            "wildlife": -0.6 * severity_multiplier
        }),
        NaturalEventType::Storm => serde_json::json!({
            "air_quality": -0.3 * severity_multiplier,
            "transportation": -0.4 * severity_multiplier,
            "infrastructure": -0.3 * severity_multiplier,
            "visibility": -0.7 * severity_multiplier
        }),
        NaturalEventType::Harvest => serde_json::json!({
            "food_availability": 0.8 * severity_multiplier,
            "economic_activity": 0.5 * severity_multiplier,
            "population_satisfaction": 0.6 * severity_multiplier
        }),
        _ => serde_json::json!({
            "general_impact": 0.1 * severity_multiplier
        })
    };

    effects.to_string()
}

// Process ongoing natural events
#[spacetimedb::reducer]
pub fn process_natural_events(
    ctx: &ReducerContext,
    world_id: u32,
    current_hour: u64,
) -> Result<Vec<u32>, String> {
    let mut resolved_events = Vec::new();

    let ongoing_events: Vec<NaturalEvent> = ctx.db.natural_event()
        .iter()
        .filter(|e| e.world_id == world_id && !e.resolved)
        .filter(|e| current_hour >= e.start_hour + e.duration_hours as u64)
        .cloned()
        .collect();

    for mut event in ongoing_events {
        // Resolve the event
        event.resolved = true;
        event.resolution_description = format!("The {:?} event has concluded after {} hours",
            event.event_type, event.duration_hours);

        // Apply lasting effects (if any)
        apply_event_resolution_effects(ctx, &event)?;

        ctx.db.natural_event().id().update(event.id, event);
        resolved_events.push(event.id);

        log::info!("Resolved natural event {} ({})", event.id, event.event_type);
    }

    Ok(resolved_events)
}

// Apply effects when an event resolves
fn apply_event_resolution_effects(
    ctx: &ReducerContext,
    event: &NaturalEvent,
) -> Result<(), String> {
    // Apply economic effects to markets
    if event.economic_impact != 0.0 {
        let markets: Vec<crate::economics::Market> = ctx.db.market()
            .iter()
            .filter(|m| m.world_id == event.world_id)
            .cloned()
            .collect();

        for mut market in markets {
            match event.event_type {
                NaturalEventType::Drought => {
                    if market.resource_type == crate::economics::ResourceType::Food {
                        market.supply *= 0.7; // Reduce food supply
                    }
                },
                NaturalEventType::Flood => {
                    market.supply *= 0.9; // General supply reduction
                },
                NaturalEventType::Harvest => {
                    if market.resource_type == crate::economics::ResourceType::Food {
                        market.supply *= 1.5; // Increase food supply
                    }
                },
                NaturalEventType::ResourceDiscovery => {
                    if market.resource_type == crate::economics::ResourceType::RawMaterials {
                        market.supply *= 1.3; // Increase raw materials
                    }
                },
                _ => {}
            }

            ctx.db.market().id().update(market.id, market);
        }
    }

    Ok(())
}