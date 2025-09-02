use spacetimedb::{ReducerContext, Table, SpacetimeType};
use serde::{Deserialize, Serialize};

#[spacetimedb::table(name = city)]
pub struct City {
    #[primary_key]
    pub id: u32,
    pub name: String,
    pub founded_hour: u64,
    pub population: u32,
    
    // Level 1: Infrastructure & Economy
    pub public_works: f32,      // 0-100, infrastructure health
    pub tax_base: f32,          // Total taxable amount
    pub tax_reserve: f32,       // Current funds
    pub import_rate: f32,       // Resources/hour imported
    pub export_rate: f32,       // Resources/hour exported
    
    // Level 2: Safety & Social Cohesion
    pub stability: f32,         // 0-100, social order
    pub health: f32,            // 0-100, average population health
    pub safety: f32,            // 0-100, inverse of threat
    
    // Level 3: Culture & Development
    pub culture: f32,           // Cumulative culture points
    pub science: f32,           // Cumulative science points
    pub prestige: f32,          // Cumulative prestige points
    
    // Metrics
    pub unemployment_rate: f32,
    pub average_happiness: f32,
    pub crime_rate: f32,
    pub last_update_hour: u64,
}

// City services and infrastructure
#[spacetimedb::table(name = city_service)]
pub struct CityService {
    #[primary_key]
    pub id: u32,
    pub city_id: u32,
    pub service_type: ServiceType,
    pub coverage: f32,          // 0-100% of population covered
    pub quality: f32,           // 0-100 service quality
    pub cost_per_hour: f32,
    pub workers_needed: u32,
    pub workers_assigned: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum ServiceType {
    Police,
    Fire,
    Hospital,
    Education,
    Utilities,
    Culture,
    Research,
    Sanitation,
}

// Trade relationships
#[spacetimedb::table(name = trade_route)]
pub struct TradeRoute {
    #[primary_key]
    pub id: u32,
    pub city_id: u32,
    pub partner_name: String,
    pub resource_type: crate::types::ResourceType,
    pub is_import: bool,
    pub rate_per_hour: f32,
    pub price_per_unit: f32,
    pub established_hour: u64,
}

// City achievements and milestones
#[spacetimedb::table(name = city_achievement)]
pub struct CityAchievement {
    #[primary_key]
    pub id: u32,
    pub city_id: u32,
    pub achievement_type: CityAchievementType,
    pub achieved_hour: u64,
    pub description: String,
    pub prestige_bonus: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum CityAchievementType {
    PopulationMilestone,
    EconomicProsperity,
    CulturalHub,
    ScientificBreakthrough,
    ZeroCrime,
    FullEmployment,
    EnvironmentalExcellence,
    EducationExcellence,
}

// City policies that affect modifiers
#[spacetimedb::table(name = city_policy)]
pub struct CityPolicy {
    #[primary_key]
    pub id: u32,
    pub city_id: u32,
    pub policy_type: PolicyType,
    pub intensity: f32,         // 0.0-1.0, how strongly applied
    pub cost_multiplier: f32,   // Effect on city costs
    pub enacted_hour: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum PolicyType {
    TaxRate,
    PublicSpending,
    EnvironmentalProtection,
    CulturalInvestment,
    EducationFunding,
    HealthcareFunding,
    SecurityFunding,
}