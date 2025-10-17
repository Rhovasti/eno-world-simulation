// Narrative event generation and queue management

use spacetimedb::{ReducerContext, Table, SpacetimeType};
use serde::{Serialize, Deserialize};
use log;

pub mod event_queue;
pub mod hooks_generator;

// Event categories for narrative classification
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum EventCategory {
    Political,
    Economic,
    Social,
    Military,
    Natural,
    Personal,
    Mystery,
    Cultural,
}

// Event importance scale
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum EventImportance {
    Trivial = 1,
    Minor = 2,
    Notable = 3,
    Significant = 4,
    Major = 5,
    Critical = 6,
    WorldChanging = 7,
}

// Narrative hooks that can be used by AI
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum NarrativeHook {
    Conflict(String),           // Tension between parties
    Mystery(String),            // Unexplained event
    Opportunity(String),        // Chance for player action
    Consequence(String),        // Result of previous action
    Discovery(String),          // New information revealed
    Relationship(String),       // Character connection
    Challenge(String),          // Obstacle to overcome
    Transformation(String),     // Major change occurring
}

// Main narrative event structure
#[spacetimedb::table(name = narrative_event)]
pub struct NarrativeEvent {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub game_id: u32,           // Specific game session
    pub event_category: EventCategory,
    pub importance: u8,          // 1-7 scale
    pub title: String,
    pub description: String,
    pub long_description: String, // Detailed context for AI
    pub participants: String,    // JSON array of character/faction IDs
    pub location_context: String, // JSON with city/building info
    pub temporal_context: String, // JSON with time/season info
    pub consequences: String,    // JSON of potential impacts
    pub narrative_hooks: String, // JSON array of NarrativeHook
    pub related_events: String,  // JSON array of related event IDs
    pub created_hour: u64,
    pub game_cycle: i32,        // Game time when created
    pub game_day: u16,
    pub consumed: bool,
    pub consumed_at_ms: i64,    // Unix timestamp when consumed
    pub ai_processed: bool,     // Whether AI has processed this
}

// Narrative event template for common event types
#[spacetimedb::table(name = narrative_template)]
pub struct NarrativeTemplate {
    #[primary_key]
    pub id: u32,
    pub template_name: String,
    pub event_category: EventCategory,
    pub base_importance: u8,
    pub title_template: String,     // With placeholders like {actor}, {location}
    pub description_template: String,
    pub required_conditions: String, // JSON of conditions
    pub possible_hooks: String,      // JSON array of hook types
}

// Track narrative arcs across multiple events
#[spacetimedb::table(name = narrative_arc)]
pub struct NarrativeArc {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub arc_name: String,
    pub arc_type: String,           // "political_intrigue", "economic_crisis", etc.
    pub status: ArcStatus,
    pub key_events: String,         // JSON array of event IDs
    pub key_participants: String,   // JSON array of character/faction IDs
    pub start_hour: u64,
    pub expected_duration: u32,     // In game hours
    pub tension_level: f32,         // 0-100
    pub resolution_state: String,   // JSON of current state
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum ArcStatus {
    Building,    // Tension rising
    Climax,      // Peak conflict
    Resolving,   // Moving toward resolution
    Resolved,    // Completed
    Abandoned,   // Player didn't engage
}

// Helper function to calculate event importance
pub fn calculate_event_importance(
    base_importance: u8,
    participant_count: usize,
    world_impact: f32,
    player_proximity: f32,
) -> u8 {
    let mut importance = base_importance as f32;

    // More participants generally means more important
    if participant_count > 10 {
        importance += 1.0;
    }
    if participant_count > 50 {
        importance += 1.0;
    }

    // World impact modifier
    importance *= (1.0 + world_impact);

    // Player proximity makes events more important
    importance *= (1.0 + player_proximity);

    // Clamp to valid range
    importance.clamp(1.0, 7.0) as u8
}

// Create a narrative event from simulation data
#[spacetimedb::reducer]
pub fn create_narrative_event(
    ctx: &ReducerContext,
    world_id: u32,
    game_id: u32,
    category: EventCategory,
    title: String,
    description: String,
    importance: u8,
) -> Result<u32, String> {
    let event_id = ctx.db.narrative_event().iter().count() as u32 + 1;

    let world = ctx.db.game_world()
        .id()
        .find(&world_id)
        .ok_or("World not found")?;

    let event = NarrativeEvent {
        id: event_id,
        world_id,
        game_id,
        event_category: category,
        importance: importance.clamp(1, 7),
        title: title.clone(),
        description: description.clone(),
        long_description: description.clone(), // TODO: Generate more detailed description
        participants: "[]".to_string(),
        location_context: "{}".to_string(),
        temporal_context: format!(r#"{{"cycle": {}, "day": {}, "season": "{:?}"}}"#,
            world.current_cycle, world.current_day, world.season),
        consequences: "[]".to_string(),
        narrative_hooks: "[]".to_string(),
        related_events: "[]".to_string(),
        created_hour: world.total_hours,
        game_cycle: world.current_cycle,
        game_day: world.current_day,
        consumed: false,
        consumed_at_ms: 0,
        ai_processed: false,
    };

    ctx.db.narrative_event().insert(event);

    log::info!("Created narrative event: '{}' (ID: {}, Importance: {})",
        title, event_id, importance);

    Ok(event_id)
}

// Get unconsumed events for a game
#[spacetimedb::reducer]
pub fn get_unconsumed_events(
    ctx: &ReducerContext,
    game_id: u32,
    max_events: u32,
    min_importance: u8,
) -> Result<Vec<NarrativeEvent>, String> {
    let events: Vec<NarrativeEvent> = ctx.db.narrative_event()
        .iter()
        .filter(|e| e.game_id == game_id && !e.consumed && e.importance >= min_importance)
        .take(max_events as usize)
        .cloned()
        .collect();

    log::info!("Retrieved {} unconsumed events for game {}",
        events.len(), game_id);

    Ok(events)
}

// Mark events as consumed by the narrative pipeline
#[spacetimedb::reducer]
pub fn consume_narrative_events(
    ctx: &ReducerContext,
    event_ids: Vec<u32>,
) -> Result<(), String> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| format!("Time error: {}", e))?
        .as_millis() as i64;

    for event_id in event_ids {
        if let Some(mut event) = ctx.db.narrative_event().id().find(&event_id) {
            event.consumed = true;
            event.consumed_at_ms = now;
            ctx.db.narrative_event().id().update(event_id, event);
        }
    }

    Ok(())
}

// Generate summary of recent events for context
#[spacetimedb::reducer]
pub fn generate_event_summary(
    ctx: &ReducerContext,
    world_id: u32,
    hours_back: u64,
) -> Result<String, String> {
    let world = ctx.db.game_world()
        .id()
        .find(&world_id)
        .ok_or("World not found")?;

    let cutoff_hour = world.total_hours.saturating_sub(hours_back);

    let recent_events: Vec<NarrativeEvent> = ctx.db.narrative_event()
        .iter()
        .filter(|e| e.world_id == world_id && e.created_hour >= cutoff_hour)
        .cloned()
        .collect();

    let mut summary = format!("World {} - Recent Events Summary\n", world.name);
    summary.push_str(&format!("Time Period: Last {} hours\n", hours_back));
    summary.push_str(&format!("Total Events: {}\n\n", recent_events.len()));

    // Group by category
    let mut by_category: std::collections::HashMap<EventCategory, Vec<&NarrativeEvent>> =
        std::collections::HashMap::new();

    for event in &recent_events {
        by_category.entry(event.event_category).or_default().push(event);
    }

    for (category, events) in by_category {
        summary.push_str(&format!("{:?} Events ({}): \n", category, events.len()));
        for event in events.iter().take(3) {
            summary.push_str(&format!("  - {}\n", event.title));
        }
    }

    Ok(summary)
}