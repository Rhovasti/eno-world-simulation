use spacetimedb::{ReducerContext, Table, SpacetimeType};
use serde::{Deserialize, Serialize};
use crate::types::*;

#[spacetimedb::table(name = individual)]
pub struct Individual {
    #[primary_key]
    pub id: u32,
    pub name: String,
    pub age: u32,
    pub current_location_id: u32,
    pub home_id: Option<u32>,
    pub workplace_id: Option<u32>,
    
    // Level 1: Physiological needs (0-100)
    pub food_water: f32,
    pub environment: f32,
    pub intimacy: f32,
    pub rest: f32,
    pub waste: f32,
    
    // Level 2: Safety & Security (0-100, only active if Level 1 > 50%)
    pub threat: f32,
    pub income: f32,  // Can go above 100 (savings)
    pub stress: f32,
    pub safety: f32,
    
    // Level 3: Love & Belonging (0-100, only active if Level 2 > 50%)
    pub relationship: f32,      // 0 or 33.3 (has relationship)
    pub social_interaction: f32, // Based on friend count
    pub community: f32,         // Depletes, filled by participation
    
    // Level 4: Self-Esteem (0-100, only active if Level 3 > 50%)
    pub achievements: f32,      // 20 per achievement, max 100
    
    // Level 5: Self-Actualization (0-100, only active if Level 4 > 50%)
    pub progression: f32,
    pub specialized_role: SpecializedRole,
    
    // Status and metadata
    pub status: IndividualStatus,
    pub last_update_hour: u64,
    pub birth_hour: u64,
}

// Relationships between individuals
#[spacetimedb::table(name = relationship)]
pub struct Relationship {
    #[primary_key]
    pub id: u32,
    pub individual1_id: u32,
    pub individual2_id: u32,
    pub relationship_type: RelationshipType,
    pub strength: f32,
    pub formed_hour: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum RelationshipType {
    Partner,
    Friend,
    Family,
    Colleague,
    Acquaintance,
}

// Individual achievements
#[spacetimedb::table(name = individual_achievement)]
pub struct IndividualAchievement {
    #[primary_key]
    pub id: u32,
    pub individual_id: u32,
    pub achievement_type: AchievementType,
    pub achieved_hour: u64,
    pub description: String,
}

// Work history
#[spacetimedb::table(name = employment)]
pub struct Employment {
    #[primary_key]
    pub id: u32,
    pub individual_id: u32,
    pub building_id: u32,
    pub job_type: JobType,
    pub wage: f32,
    pub started_hour: u64,
    pub ended_hour: Option<u64>,
    pub is_active: bool,
}