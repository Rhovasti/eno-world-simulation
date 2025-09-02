use spacetimedb::{ReducerContext, Table, SpacetimeType};
use serde::{Deserialize, Serialize};
use crate::types::*;

#[spacetimedb::table(name = building)]
pub struct Building {
    #[primary_key]
    pub id: u32,
    pub name: String,
    pub city_id: u32,
    pub building_type: BuildingType,
    pub location_x: f32,
    pub location_y: f32,
    
    // Common metrics
    pub maintenance: f32,       // 0-100, affects environment
    pub cleanliness: f32,       // 0-100, affects environment and waste
    pub efficiency_level: u8,   // 0-5, upgrade stages
    pub prestige_level: u8,     // 0-5, upgrade stages
    
    // Occupancy
    pub current_occupants: u32,
    pub max_capacity: u32,
    
    // Economic
    pub operating_cost: f32,
    pub revenue: f32,
    pub last_payment_hour: u64,
}

// Home-specific data
#[spacetimedb::table(name = home_data)]
pub struct HomeData {
    #[primary_key]
    pub building_id: u32,
    pub rent_amount: f32,
    pub rent_paid: f32,
    pub utilities_quality: f32,
}

// Workplace-specific data
#[spacetimedb::table(name = workplace_data)]
pub struct WorkplaceData {
    #[primary_key]
    pub building_id: u32,
    pub resource_type: ResourceType,
    pub consumption_rate: f32,
    pub production_rate: f32,
    pub inventory: f32,
    pub stockpile: f32,
    pub max_inventory: f32,
    pub max_stockpile: f32,
    pub base_wage: f32,
}

// Building upgrades in progress
#[spacetimedb::table(name = building_upgrade)]
pub struct BuildingUpgrade {
    #[primary_key]
    pub id: u32,
    pub building_id: u32,
    pub upgrade_type: UpgradeType,
    pub work_hours_needed: f32,
    pub work_hours_completed: f32,
    pub started_hour: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum UpgradeType {
    Efficiency,
    Prestige,
    Capacity,
    Maintenance,
}

// Location capabilities (what needs can be fulfilled here)
#[spacetimedb::table(name = location_capability)]
pub struct LocationCapability {
    #[primary_key]
    pub id: u32,
    pub building_id: u32,
    pub provides_food: bool,
    pub provides_rest: bool,
    pub provides_social: bool,
    pub provides_facilities: bool,
    pub provides_healthcare: bool,
    pub provides_culture: bool,
    pub provides_education: bool,
    pub provides_work: bool,
    pub environmental_quality: f32,  // -3.0 to +2.0 modifier
}