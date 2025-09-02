use serde::{Deserialize, Serialize};
use spacetimedb::SpacetimeType;

// Individual Status Modifiers
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub struct StatusData {
    pub until_hour: u64,
    pub target_location: Option<u32>,
    pub target_building: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum IndividualStatus {
    Working(StatusData),
    Sleeping(StatusData),
    Eating(StatusData),
    Socializing(StatusData),
    InTransit(StatusData),
    Maintaining(StatusData),
    UsingFacilities(StatusData),
    Idle,
}

// Building Types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub struct HomeConfig {
    pub capacity: u32,
    pub rent: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub struct WorkplaceConfig {
    pub job_type: JobType,
    pub positions: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum BuildingType {
    Home(HomeConfig),
    Workplace(WorkplaceConfig),
    Restaurant,
    Park,
    Hospital,
    PoliceStation,
    School,
    ResearchLab,
    CultureCenter,
    CityHall,
}

// Job Types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum JobType {
    Factory,
    Office,
    Retail,
    Healthcare,
    Education,
    Research,
    Culture,
    Utilities,
    Government,
}

// Individual specialized roles (Level 5 self-actualization)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum SpecializedRole {
    None,
    Artist,
    Scientist,
    Leader,
    Educator,
    Healer,
}

// Resource types for production/consumption
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum ResourceType {
    Food,
    Goods,
    Services,
    Culture,
    Science,
    Healthcare,
}

// Actions individuals can take
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum IndividualAction {
    Move { to_location: u32 },
    Work,
    Sleep,
    Eat,
    Socialize,
    UseFacilities,
    MaintainBuilding,
    CleanBuilding,
    PayRent,
    AttendEvent,
    CreateArt,
    Research,
}

// Event types for narrative generation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum EventType {
    Movement,
    NeedFulfilled,
    WorkCompleted,
    SocialInteraction,
    Achievement,
    BuildingUpgrade,
    CityMilestone,
    Emergency,
}

// Achievement types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum AchievementType {
    FirstJob,
    HomePurchase,
    RelationshipFormed,
    SkillMastery,
    CommunityLeader,
    CulturalContribution,
    ScientificBreakthrough,
    WealthAccumulated,
    HealthOptimized,
}

// Unified need types (mapped to 5 fundamental needs)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum FundamentalNeed {
    Environment,    // Safety, comfort, livability
    Consumption,    // Resource intake and usage
    Connection,     // Social bonds and networks
    Rest,           // Recovery and maintenance
    Waste,          // Byproduct management
}