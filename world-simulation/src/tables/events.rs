use spacetimedb::{ReducerContext, Table, SpacetimeType};
use crate::types::*;

// Movement tracking
#[spacetimedb::table(name = movement_event)]
pub struct MovementEvent {
    #[primary_key]
    pub id: u32,
    pub individual_id: u32,
    pub from_location_id: u32,
    pub to_location_id: u32,
    pub hour: u64,
    pub reason: FundamentalNeed,
    pub travel_time: u32,
}

// Need fulfillment tracking
#[spacetimedb::table(name = need_fulfillment_event)]
pub struct NeedFulfillmentEvent {
    #[primary_key]
    pub id: u32,
    pub individual_id: u32,
    pub location_id: u32,
    pub hour: u64,
    pub need_type: FundamentalNeed,
    pub amount_fulfilled: f32,
    pub action_taken: IndividualAction,
}

// Work tracking
#[spacetimedb::table(name = work_event)]
pub struct WorkEvent {
    #[primary_key]
    pub id: u32,
    pub individual_id: u32,
    pub building_id: u32,
    pub hour: u64,
    pub hours_worked: f32,
    pub wage_earned: f32,
    pub productivity: f32,
    pub resources_consumed: f32,
    pub resources_produced: f32,
}

// Social interactions
#[spacetimedb::table(name = social_event)]
pub struct SocialEvent {
    #[primary_key]
    pub id: u32,
    pub individual1_id: u32,
    pub individual2_id: u32,
    pub location_id: u32,
    pub hour: u64,
    pub interaction_type: SocialInteractionType,
    pub relationship_change: f32,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, SpacetimeType)]
pub enum SocialInteractionType {
    Conversation,
    SharedMeal,
    Collaboration,
    Romance,
    Conflict,
    CommunityEvent,
}

// Building events
#[spacetimedb::table(name = building_event)]
pub struct BuildingEvent {
    #[primary_key]
    pub id: u32,
    pub building_id: u32,
    pub hour: u64,
    pub event_type: BuildingEventType,
    pub description: String,
    pub impact_value: f32,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, SpacetimeType)]
pub enum BuildingEventType {
    Upgraded,
    MaintenancePerformed,
    Cleaned,
    CapacityReached,
    ResourceShortage,
    ProductionCompleted,
    RentCollected,
}

// City events
#[spacetimedb::table(name = city_event)]
pub struct CityEvent {
    #[primary_key]
    pub id: u32,
    pub city_id: u32,
    pub hour: u64,
    pub event_type: CityEventType,
    pub description: String,
    pub participants: u32,
    pub impact_stability: f32,
    pub impact_culture: f32,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, SpacetimeType)]
pub enum CityEventType {
    Festival,
    Election,
    Emergency,
    PolicyChange,
    MilestoneReached,
    TradeAgreement,
    InfrastructureProject,
}

// Global simulation time
#[spacetimedb::table(name = simulation_time)]
#[derive(Clone)]
pub struct SimulationTime {
    #[primary_key]
    pub id: u32,  // Always 1
    pub current_hour: u64,
    pub day_of_week: u8,     // 0-6
    pub hour_of_day: u8,     // 0-23
    pub total_days: u64,
    pub is_running: bool,
    pub auto_tick_enabled: bool,
    pub tick_interval_ms: u64,  // Milliseconds between auto-ticks
}

// Auto-ticker configuration table (manual scheduling approach)
#[spacetimedb::table(name = autoticker_config)]
pub struct AutotickerConfig {
    #[primary_key]
    pub id: u32,  // Always 1
    pub last_tick_time: i64,  // Timestamp of last tick (microseconds since unix epoch / 1000)
    pub next_tick_time: i64,  // Timestamp of next scheduled tick (microseconds since unix epoch / 1000)
}