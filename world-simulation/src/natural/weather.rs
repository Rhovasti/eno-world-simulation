// Weather system for detailed atmospheric simulation

use spacetimedb::{ReducerContext, Table, SpacetimeType};
use serde::{Serialize, Deserialize};
use log;
use rand::Rng;
use crate::world::{Season, ClimateZone};
use crate::natural::{ClimateState, WeatherPattern};

// Weather forecast data
#[spacetimedb::table(name = weather_forecast)]
pub struct WeatherForecast {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub region_id: u32,
    pub forecast_hour: u64,      // Hour this forecast is for
    pub temperature: f32,
    pub precipitation_chance: f32, // 0-100%
    pub wind_speed: f32,
    pub weather_pattern: WeatherPattern,
    pub confidence: f32,         // Forecast accuracy 0-100%
    pub created_hour: u64,       // When forecast was made
}

// Weather front system
#[spacetimedb::table(name = weather_front)]
pub struct WeatherFront {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub front_type: FrontType,
    pub origin_region: u32,
    pub current_region: u32,
    pub target_region: u32,
    pub movement_speed: f32,     // regions per hour
    pub intensity: f32,          // 0-100
    pub size: f32,              // affected radius
    pub created_hour: u64,
    pub expected_arrival: u64,
    pub weather_effects: String, // JSON of effects
    pub is_active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum FrontType {
    ColdFront,
    WarmFront,
    OccludedFront,
    StormSystem,
    HighPressure,
    LowPressure,
}

// Microclimate variations within regions
#[spacetimedb::table(name = microclimate)]
pub struct Microclimate {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub region_id: u32,
    pub location_type: MicroclimateType,
    pub temperature_modifier: f32, // +/- degrees from regional average
    pub humidity_modifier: f32,    // +/- % from regional average
    pub wind_modifier: f32,        // multiplier for wind speed
    pub precipitation_modifier: f32, // multiplier for precipitation
    pub elevation: f32,            // meters above sea level
    pub vegetation_density: f32,   // 0-100%
    pub urban_heat_island: f32,    // urban warming effect
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum MicroclimateType {
    Urban,
    Forest,
    Mountain,
    Coastal,
    Desert,
    River,
    Lake,
    Agricultural,
}

// Generate weather forecast for regions
#[spacetimedb::reducer]
pub fn generate_weather_forecast(
    ctx: &ReducerContext,
    world_id: u32,
    current_hour: u64,
    forecast_hours: u32,
) -> Result<Vec<u32>, String> {
    let mut forecast_ids = Vec::new();

    let climate_states: Vec<ClimateState> = ctx.db.climate_state()
        .iter()
        .filter(|c| c.world_id == world_id)
        .cloned()
        .collect();

    for climate in climate_states {
        // Generate forecasts for next N hours
        for hour_offset in 1..=forecast_hours {
            let forecast_hour = current_hour + hour_offset as u64;
            let forecast_id = ctx.db.weather_forecast().iter().count() as u32 + 1;

            let (temp, precip_chance, wind, pattern, confidence) =
                predict_weather_conditions(&climate, hour_offset, world_id, ctx)?;

            let forecast = WeatherForecast {
                id: forecast_id,
                world_id,
                region_id: climate.region_id,
                forecast_hour,
                temperature: temp,
                precipitation_chance: precip_chance,
                wind_speed: wind,
                weather_pattern: pattern,
                confidence,
                created_hour: current_hour,
            };

            ctx.db.weather_forecast().insert(forecast);
            forecast_ids.push(forecast_id);
        }
    }

    log::info!("Generated {} weather forecasts for {} regions",
        forecast_hours, climate_states.len());
    Ok(forecast_ids)
}

// Predict weather conditions based on current state and patterns
fn predict_weather_conditions(
    climate: &ClimateState,
    hours_ahead: u32,
    world_id: u32,
    ctx: &ReducerContext,
) -> Result<(f32, f32, f32, WeatherPattern, f32), String> {
    let mut rng = rand::thread_rng();

    // Base prediction on current conditions
    let mut predicted_temp = climate.current_temperature;
    let mut predicted_wind = climate.wind_speed;
    let mut predicted_pattern = climate.weather_pattern;

    // Account for daily temperature cycles
    let hour_of_day = (climate.last_updated_hour + hours_ahead as u64) % 24;
    let daily_temp_variation = ((hour_of_day as f32 - 12.0) / 24.0 * std::f32::consts::PI).sin() * 5.0;
    predicted_temp += daily_temp_variation;

    // Check for incoming weather fronts
    let incoming_fronts: Vec<crate::natural::weather::WeatherFront> = ctx.db.weather_front()
        .iter()
        .filter(|f| f.world_id == world_id && f.is_active)
        .filter(|f| f.target_region == climate.region_id)
        .filter(|f| f.expected_arrival <= climate.last_updated_hour + hours_ahead as u64)
        .cloned()
        .collect();

    if let Some(front) = incoming_fronts.first() {
        // Modify predictions based on incoming front
        match front.front_type {
            FrontType::ColdFront => {
                predicted_temp -= 8.0;
                predicted_wind *= 1.5;
                predicted_pattern = WeatherPattern::Stormy;
            },
            FrontType::WarmFront => {
                predicted_temp += 5.0;
                predicted_pattern = WeatherPattern::Cloudy;
            },
            FrontType::StormSystem => {
                predicted_wind *= 2.0;
                predicted_pattern = WeatherPattern::Stormy;
            },
            _ => {}
        }
    }

    // Calculate precipitation chance
    let precip_chance = match predicted_pattern {
        WeatherPattern::Clear => 5.0,
        WeatherPattern::Cloudy => 20.0,
        WeatherPattern::Rainy => 70.0,
        WeatherPattern::Stormy => 90.0,
        WeatherPattern::Foggy => 30.0,
        _ => 15.0,
    };

    // Forecast confidence decreases with time
    let confidence = (100.0 - (hours_ahead as f32 * 3.0)).max(20.0);

    // Add some randomness for weather unpredictability
    let temp_noise = rng.gen_range(-2.0..2.0);
    predicted_temp += temp_noise;

    Ok((predicted_temp, precip_chance, predicted_wind, predicted_pattern, confidence))
}

// Create weather fronts that move between regions
#[spacetimedb::reducer]
pub fn generate_weather_fronts(
    ctx: &ReducerContext,
    world_id: u32,
    current_hour: u64,
) -> Result<Vec<u32>, String> {
    let mut front_ids = Vec::new();
    let mut rng = rand::thread_rng();

    // Get all regions in the world
    let regions: Vec<u32> = ctx.db.climate_state()
        .iter()
        .filter(|c| c.world_id == world_id)
        .map(|c| c.region_id)
        .collect::<std::collections::HashSet<u32>>()
        .into_iter()
        .collect();

    // Generate fronts with low probability
    if rng.gen::<f32>() < 0.05 { // 5% chance per hour
        if regions.len() >= 2 {
            let origin = regions[rng.gen_range(0..regions.len())];
            let target = regions[rng.gen_range(0..regions.len())];

            if origin != target {
                let front_id = ctx.db.weather_front().iter().count() as u32 + 1;
                let front_type = match rng.gen_range(0..6) {
                    0 => FrontType::ColdFront,
                    1 => FrontType::WarmFront,
                    2 => FrontType::StormSystem,
                    3 => FrontType::HighPressure,
                    4 => FrontType::LowPressure,
                    _ => FrontType::OccludedFront,
                };

                let movement_speed = rng.gen_range(0.1..0.5); // regions per hour
                let travel_time = (1.0 / movement_speed) as u64;

                let front = WeatherFront {
                    id: front_id,
                    world_id,
                    front_type,
                    origin_region: origin,
                    current_region: origin,
                    target_region: target,
                    movement_speed,
                    intensity: rng.gen_range(30.0..90.0),
                    size: rng.gen_range(1.0..3.0),
                    created_hour: current_hour,
                    expected_arrival: current_hour + travel_time,
                    weather_effects: generate_front_effects_json(front_type),
                    is_active: true,
                };

                ctx.db.weather_front().insert(front);
                front_ids.push(front_id);

                log::info!("Created weather front {} moving from region {} to {}",
                    front_id, origin, target);
            }
        }
    }

    Ok(front_ids)
}

// Generate JSON effects for weather fronts
fn generate_front_effects_json(front_type: FrontType) -> String {
    let effects = match front_type {
        FrontType::ColdFront => serde_json::json!({
            "temperature_change": -8.0,
            "wind_increase": 1.5,
            "precipitation_chance": 80.0,
            "duration_hours": 12
        }),
        FrontType::WarmFront => serde_json::json!({
            "temperature_change": 5.0,
            "wind_increase": 1.1,
            "precipitation_chance": 40.0,
            "duration_hours": 24
        }),
        FrontType::StormSystem => serde_json::json!({
            "temperature_change": -3.0,
            "wind_increase": 2.5,
            "precipitation_chance": 95.0,
            "duration_hours": 8
        }),
        FrontType::HighPressure => serde_json::json!({
            "temperature_change": 2.0,
            "wind_increase": 0.8,
            "precipitation_chance": 5.0,
            "duration_hours": 48
        }),
        FrontType::LowPressure => serde_json::json!({
            "temperature_change": -1.0,
            "wind_increase": 1.3,
            "precipitation_chance": 60.0,
            "duration_hours": 36
        }),
        FrontType::OccludedFront => serde_json::json!({
            "temperature_change": -4.0,
            "wind_increase": 1.4,
            "precipitation_chance": 70.0,
            "duration_hours": 18
        }),
    };

    effects.to_string()
}

// Update weather front positions and apply effects
#[spacetimedb::reducer]
pub fn update_weather_fronts(
    ctx: &ReducerContext,
    world_id: u32,
    current_hour: u64,
) -> Result<Vec<u32>, String> {
    let mut updated_fronts = Vec::new();

    let active_fronts: Vec<WeatherFront> = ctx.db.weather_front()
        .iter()
        .filter(|f| f.world_id == world_id && f.is_active)
        .cloned()
        .collect();

    for mut front in active_fronts {
        // Check if front has reached its destination
        if current_hour >= front.expected_arrival {
            // Apply front effects to target region
            apply_front_effects(ctx, &front, current_hour)?;

            // Deactivate the front
            front.is_active = false;
            ctx.db.weather_front().id().update(front.id, front);
            updated_fronts.push(front.id);

            log::info!("Weather front {} reached region {} and dissipated",
                front.id, front.target_region);
        }
    }

    Ok(updated_fronts)
}

// Apply weather front effects to climate state
fn apply_front_effects(
    ctx: &ReducerContext,
    front: &WeatherFront,
    current_hour: u64,
) -> Result<(), String> {
    if let Some(mut climate) = ctx.db.climate_state()
        .iter()
        .find(|c| c.world_id == front.world_id && c.region_id == front.target_region)
        .cloned() {

        let effects: serde_json::Value = serde_json::from_str(&front.weather_effects)
            .unwrap_or_else(|_| serde_json::json!({}));

        // Apply temperature change
        if let Some(temp_change) = effects.get("temperature_change") {
            climate.current_temperature += temp_change.as_f64().unwrap_or(0.0) as f32;
        }

        // Apply wind change
        if let Some(wind_mult) = effects.get("wind_increase") {
            climate.wind_speed *= wind_mult.as_f64().unwrap_or(1.0) as f32;
        }

        // Update weather pattern based on front type
        climate.weather_pattern = match front.front_type {
            FrontType::StormSystem => WeatherPattern::Stormy,
            FrontType::ColdFront => WeatherPattern::Rainy,
            FrontType::WarmFront => WeatherPattern::Cloudy,
            FrontType::HighPressure => WeatherPattern::Clear,
            _ => climate.weather_pattern,
        };

        climate.last_updated_hour = current_hour;
        ctx.db.climate_state().id().update(climate.id, climate);
    }

    Ok(())
}

// Initialize microclimates for a region
#[spacetimedb::reducer]
pub fn initialize_microclimates(
    ctx: &ReducerContext,
    world_id: u32,
    region_id: u32,
) -> Result<Vec<u32>, String> {
    let mut microclimate_ids = Vec::new();

    // Create different microclimate zones
    let microclimate_types = [
        MicroclimateType::Urban,
        MicroclimateType::Forest,
        MicroclimateType::Agricultural,
        MicroclimateType::Coastal,
    ];

    for microclimate_type in microclimate_types {
        let micro_id = ctx.db.microclimate().iter().count() as u32 + 1;

        let (temp_mod, humidity_mod, wind_mod, precip_mod, elevation, vegetation, urban_heat) =
            get_microclimate_modifiers(microclimate_type);

        let microclimate = Microclimate {
            id: micro_id,
            world_id,
            region_id,
            location_type: microclimate_type,
            temperature_modifier: temp_mod,
            humidity_modifier: humidity_mod,
            wind_modifier: wind_mod,
            precipitation_modifier: precip_mod,
            elevation,
            vegetation_density: vegetation,
            urban_heat_island: urban_heat,
        };

        ctx.db.microclimate().insert(microclimate);
        microclimate_ids.push(micro_id);
    }

    log::info!("Initialized {} microclimates for region {}",
        microclimate_types.len(), region_id);
    Ok(microclimate_ids)
}

// Get modifiers for different microclimate types
fn get_microclimate_modifiers(
    microclimate_type: MicroclimateType,
) -> (f32, f32, f32, f32, f32, f32, f32) {
    match microclimate_type {
        MicroclimateType::Urban => (2.0, -5.0, 0.8, 0.9, 100.0, 20.0, 3.0),
        MicroclimateType::Forest => (-1.0, 10.0, 0.6, 1.2, 200.0, 90.0, 0.0),
        MicroclimateType::Mountain => (-5.0, -10.0, 1.5, 1.3, 1000.0, 40.0, 0.0),
        MicroclimateType::Coastal => (0.0, 15.0, 1.3, 1.1, 10.0, 30.0, 0.0),
        MicroclimateType::Desert => (8.0, -20.0, 1.2, 0.3, 300.0, 5.0, 0.0),
        MicroclimateType::River => (-2.0, 20.0, 0.9, 1.4, 50.0, 70.0, 0.0),
        MicroclimateType::Lake => (-1.0, 25.0, 0.8, 1.2, 50.0, 60.0, 0.0),
        MicroclimateType::Agricultural => (1.0, 0.0, 1.0, 1.0, 150.0, 60.0, 0.0),
    }
}

// Calculate weather conditions for specific microclimate
#[spacetimedb::reducer]
pub fn get_microclimate_conditions(
    ctx: &ReducerContext,
    world_id: u32,
    region_id: u32,
    microclimate_type: MicroclimateType,
) -> Result<(), String> {
    // Get regional climate
    let regional_climate = ctx.db.climate_state()
        .iter()
        .find(|c| c.world_id == world_id && c.region_id == region_id)
        .ok_or("Regional climate not found")?;

    // Get microclimate modifiers
    let microclimate = ctx.db.microclimate()
        .iter()
        .find(|m| m.world_id == world_id &&
                  m.region_id == region_id &&
                  m.location_type == microclimate_type)
        .ok_or("Microclimate not found")?;

    // Calculate modified conditions
    let modified_temp = regional_climate.current_temperature + microclimate.temperature_modifier;
    let modified_humidity = (regional_climate.humidity + microclimate.humidity_modifier).clamp(0.0, 100.0);
    let modified_wind = regional_climate.wind_speed * microclimate.wind_modifier;
    let modified_precipitation = regional_climate.precipitation * microclimate.precipitation_modifier;

    log::info!("Microclimate {:?} in region {}: {}°C, {}% humidity, {} km/h wind",
        microclimate_type, region_id, modified_temp, modified_humidity, modified_wind);

    Ok(())
}