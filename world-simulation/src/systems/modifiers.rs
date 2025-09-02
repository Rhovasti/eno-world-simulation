// All modifier constants from modifiers.md

// Individual Level Depletion Rates (per hour)
pub mod individual_depletion {
    // Level 1: Physiological
    pub const FOOD_WATER_BASE: f32 = -2.0;
    pub const FOOD_WATER_WORKING: f32 = -3.0;
    pub const FOOD_WATER_RESTING: f32 = -1.5;
    
    pub const ENVIRONMENT_BASE: f32 = -1.0;
    pub const ENVIRONMENT_HAZARDOUS: f32 = -3.0;
    pub const ENVIRONMENT_NEUTRAL: f32 = -1.0;
    pub const ENVIRONMENT_HEALING: f32 = 0.5;
    
    pub const INTIMACY_BASE: f32 = -0.5;
    pub const INTIMACY_WITH_PARTNER: f32 = 10.0;
    
    pub const REST_BASE: f32 = -1.5;
    pub const REST_SLEEPING: f32 = 8.0;
    pub const REST_RESTING: f32 = 2.0;
    pub const REST_WORKING: f32 = -2.5;
    
    pub const WASTE_BASE: f32 = 2.0;
    pub const WASTE_FACILITIES: f32 = -50.0;
    pub const WASTE_EMERGENCY: f32 = -100.0;
    pub const WASTE_EMERGENCY_ENV_PENALTY: f32 = -20.0;
    
    // Level 2: Safety & Security
    pub const THREAT_BASE: f32 = -0.5;
    pub const THREAT_DANGEROUS: f32 = -2.0;
    pub const THREAT_SAFE_BUILDING: f32 = 0.5;
    pub const THREAT_WITH_SECURITY: f32 = 1.0;
    
    pub const INCOME_LIVING_COST: f32 = -0.2;
    pub const INCOME_WORKING: f32 = 5.0;
    pub const INCOME_UNEMPLOYED: f32 = -0.5;
    
    pub const STRESS_BASE: f32 = -0.3;
    pub const STRESS_HIGH_WORKLOAD: f32 = -1.0;
    pub const STRESS_RECREATION: f32 = 2.0;
    pub const STRESS_LOW_INCOME: f32 = -0.5;
    pub const STRESS_TO_REST_FACTOR: f32 = -0.1; // Per 10 stress
    
    pub const SAFETY_BASE: f32 = -0.2;
    pub const SAFETY_AT_HOME: f32 = 1.0;
    pub const SAFETY_SAFE_LOCATION: f32 = 0.5;
    pub const SAFETY_UNSAFE_AREA: f32 = -2.0;
    
    // Level 3: Love & Belonging
    pub const COMMUNITY_BASE: f32 = -0.3;
    pub const COMMUNITY_PROJECT: f32 = 5.0;
    pub const COMMUNITY_EVENT: f32 = 3.0;
    pub const COMMUNITY_ISOLATION: f32 = -0.5;
    
    // Level 5: Self-Actualization
    pub const PROGRESSION_MEANINGFUL_WORK: f32 = 0.5;
    pub const PROGRESSION_ACHIEVEMENT: f32 = 10.0;
    pub const PROGRESSION_MILESTONE: f32 = 20.0;
}

// Building Level Depletion Rates (per day)
pub mod building_depletion {
    // Home
    pub const RENT_BASE: f32 = -10.0;
    pub const MAINTENANCE_BASE: f32 = -2.0;
    pub const MAINTENANCE_PER_OCCUPANT: f32 = -0.5;
    pub const MAINTENANCE_POOR_INFRASTRUCTURE: f32 = -1.0;
    pub const CLEANLINESS_BASE: f32 = -3.0;
    pub const CLEANLINESS_PER_OCCUPANT: f32 = -1.0;
    
    // Workplace
    pub const OPERATIONAL_COST_BASE: f32 = -50.0;
    pub const OPERATIONAL_COST_PER_WORKER: f32 = -5.0;
    pub const RESOURCE_CONSUMPTION_BASE: f32 = 10.0;
    pub const RESOURCE_CONSUMPTION_PER_WORKER: f32 = 5.0;
    pub const RESOURCE_PRODUCTION_BASE: f32 = 5.0;
    pub const RESOURCE_PRODUCTION_PER_WORKER: f32 = 10.0;
}

// City Level Depletion Rates (per week)
pub mod city_depletion {
    pub const PUBLIC_WORKS_PER_CITIZEN: f32 = -0.01;
    pub const PUBLIC_SERVICE_COST_PER_100: f32 = -1.0;
    pub const IMPORT_COST: f32 = -10.0;
    pub const EXPORT_REVENUE: f32 = 15.0;
    pub const STABILITY_PER_STRESSED: f32 = -0.1;
    pub const ARTIST_CULTURE_RATE: f32 = 0.5;
    pub const SCIENTIST_SCIENCE_RATE: f32 = 0.3;
}

// Action costs and durations
pub mod actions {
    pub const MOVE_DURATION: u64 = 1;
    pub const MOVE_REST_COST: f32 = -2.0;
    
    pub const WORK_DURATION: u64 = 8;
    pub const WORK_REST_COST: f32 = -16.0;
    pub const WORK_STRESS_GAIN: f32 = 5.0;
    pub const WORK_INCOME_GAIN: f32 = 40.0;
    
    pub const SLEEP_DURATION: u64 = 8;
    pub const SLEEP_REST_GAIN: f32 = 64.0;
    
    pub const EAT_DURATION: u64 = 1;
    pub const EAT_FOOD_GAIN: f32 = 25.0;
    
    pub const SOCIALIZE_DURATION: u64 = 2;
    pub const SOCIALIZE_SOCIAL_GAIN: f32 = 10.0;
    pub const SOCIALIZE_STRESS_LOSS: f32 = -5.0;
    
    pub const MAINTAIN_DURATION: u64 = 4;
    pub const MAINTAIN_BUILDING_GAIN: f32 = 20.0;
    
    pub const CLEAN_DURATION: u64 = 2;
    pub const CLEAN_BUILDING_GAIN: f32 = 30.0;
}

// Need thresholds
pub mod thresholds {
    pub const NEED_MAX: f32 = 100.0;
    pub const NEED_CRITICAL_LOW: f32 = 20.0;
    pub const NEED_CRITICAL_HIGH: f32 = 80.0;
    pub const NEED_ADEQUATE: f32 = 50.0;
    pub const NEED_URGENT: f32 = 60.0;
    
    pub const INCOME_MAX: f32 = 1000.0;
    pub const INCOME_CRITICAL: f32 = 10.0;
    
    pub const WASTE_CRITICAL: f32 = 80.0;
    pub const STRESS_CRITICAL: f32 = 70.0;
}

// Priority weights
pub mod priority_weights {
    pub const WASTE_HIGH: f32 = 10.0;
    pub const FOOD_CRITICAL: f32 = 8.0;
    pub const REST_CRITICAL: f32 = 7.0;
    pub const SAFETY_LOW: f32 = 6.0;
    pub const INCOME_CRITICAL: f32 = 5.0;
    pub const ENVIRONMENT_LOW: f32 = 4.0;
    pub const STRESS_HIGH: f32 = 3.0;
    pub const SOCIAL_NEEDS: f32 = 2.0;
    pub const HIGHER_NEEDS: f32 = 1.0;
}

// Efficiency and prestige modifiers
pub mod upgrades {
    pub const EFFICIENCY_PRODUCTION_BONUS: f32 = 0.2;  // Per level
    pub const EFFICIENCY_CONSUMPTION_REDUCTION: f32 = 0.1;  // Per level
    pub const PRESTIGE_RENT_MULTIPLIER: f32 = 1.2;  // Per level
    pub const UPGRADE_WORK_HOURS_EFFICIENCY: f32 = 100.0;
    pub const UPGRADE_WORK_HOURS_PRESTIGE: f32 = 200.0;
}

// Location modifiers
pub mod location {
    pub const HOME_SAFETY_BONUS: f32 = 1.0;
    pub const HOME_STRESS_REDUCTION: f32 = -0.2;
    pub const HOME_REST_BONUS: f32 = 0.5;
    
    pub const WORKPLACE_STRESS_INCREASE: f32 = 0.3;
    
    pub const PARK_ENVIRONMENT_BONUS: f32 = 0.5;
    pub const PARK_STRESS_REDUCTION: f32 = -0.5;
    
    pub const HOSPITAL_ENVIRONMENT_BONUS: f32 = 2.0;
    
    pub const DANGEROUS_THREAT_PENALTY: f32 = -2.0;
    pub const DANGEROUS_STRESS_INCREASE: f32 = 1.0;
}