// World-level simulation components

use spacetimedb::{ReducerContext, Table, SpacetimeType};
use serde::{Serialize, Deserialize};

pub mod game_world;
pub mod factions;
pub mod natural_events;

// Re-export world structures
pub use game_world::*;

// Season enumeration
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

// Climate zones
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum ClimateZone {
    Tropical,
    Temperate,
    Arid,
    Arctic,
    Mediterranean,
}

// Narrative speed settings
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum NarrativeSpeed {
    Paused,
    Slow,    // 1 day per real hour
    Normal,  // 1 week per real hour
    Fast,    // 1 month per real hour
}

// Weather event types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum WeatherEvent {
    Clear,
    Rain,
    Storm,
    Snow,
    Fog,
    Heatwave,
    Blizzard,
}

// Disaster types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum DisasterType {
    Earthquake,
    Flood,
    Drought,
    Fire,
    Plague,
    Famine,
}

// Seasonal events
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum SeasonalEvent {
    Harvest,
    Planting,
    Migration,
    Festival,
}

// Natural event types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum NaturalEventType {
    Weather(WeatherEvent),
    Disaster(DisasterType),
    Seasonal(SeasonalEvent),
}

// Calculate current season based on day of year
pub fn calculate_season(day_of_year: u16) -> Season {
    match day_of_year {
        1..=90 => Season::Spring,
        91..=180 => Season::Summer,
        181..=270 => Season::Autumn,
        271..=360 => Season::Winter,
        _ => Season::Spring, // Default fallback
    }
}

// Helper function to calculate season from hour
pub fn calculate_season_from_hour(hour: u64) -> Season {
    let day_of_year = ((hour / 24) % 365) as u16 + 1;
    calculate_season(day_of_year)
}

// Get seasonal modifiers for various systems
pub fn get_seasonal_modifiers(season: Season, climate: ClimateZone) -> SeasonalModifiers {
    match (season, climate) {
        (Season::Winter, ClimateZone::Arctic) => SeasonalModifiers {
            food_production: 0.1,
            movement_speed: 0.5,
            heating_cost: 3.0,
            disease_risk: 1.2,
        },
        (Season::Summer, ClimateZone::Arid) => SeasonalModifiers {
            food_production: 0.3,
            movement_speed: 0.7,
            water_consumption: 2.5,
            disease_risk: 1.3,
        },
        (Season::Spring, _) => SeasonalModifiers {
            food_production: 1.2,
            movement_speed: 1.0,
            fertility_bonus: 1.3,
            mood_bonus: 1.1,
        },
        _ => SeasonalModifiers::default(),
    }
}

#[derive(Debug, Clone)]
pub struct SeasonalModifiers {
    pub food_production: f32,
    pub movement_speed: f32,
    pub heating_cost: f32,
    pub water_consumption: f32,
    pub disease_risk: f32,
    pub fertility_bonus: f32,
    pub mood_bonus: f32,
}

impl Default for SeasonalModifiers {
    fn default() -> Self {
        Self {
            food_production: 1.0,
            movement_speed: 1.0,
            heating_cost: 1.0,
            water_consumption: 1.0,
            disease_risk: 1.0,
            fertility_bonus: 1.0,
            mood_bonus: 1.0,
        }
    }
}