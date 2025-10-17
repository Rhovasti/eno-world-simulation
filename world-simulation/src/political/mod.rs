// Political faction system and dynamics

use spacetimedb::{ReducerContext, Table, SpacetimeType};
use serde::{Serialize, Deserialize};
use log;
use rand::Rng;

pub mod faction_relationships;
pub mod political_events;

// Faction types and ideologies
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum FactionType {
    Political,
    Religious,
    Economic,
    Military,
    Cultural,
    Criminal,
}

// Political ideologies
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum Ideology {
    Authoritarian,
    Democratic,
    Theocratic,
    Mercantile,
    Militaristic,
    Scholarly,
    Anarchist,
}

// Political faction entity
#[spacetimedb::table(name = faction)]
pub struct Faction {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub name: String,
    pub faction_type: FactionType,
    pub ideology: Ideology,
    pub leader_id: u32,        // Links to Individual
    pub base_city_id: u32,     // Home base
    pub influence: f32,        // 0-100 scale
    pub treasury: f32,
    pub stability: f32,        // Internal cohesion
    pub public_support: f32,   // Popular backing
    pub member_count: u32,
    pub founding_hour: u64,
    pub goals: String,         // JSON array of faction objectives
    pub recent_actions: String, // JSON array of recent events
    pub is_active: bool,
}

// Relationships between factions
#[spacetimedb::table(name = faction_relationship)]
pub struct FactionRelationship {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub faction1_id: u32,
    pub faction2_id: u32,
    pub relationship: f32,     // -100 (war) to +100 (alliance)
    pub relationship_type: RelationshipType,
    pub trade_volume: f32,
    pub recent_interactions: String, // JSON array of events
    pub last_interaction_hour: u64,
    pub treaty_status: TreatyStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum RelationshipType {
    Allied,
    Friendly,
    Neutral,
    Rival,
    Hostile,
    AtWar,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum TreatyStatus {
    None,
    TradeAgreement,
    NonAggressionPact,
    MutualDefense,
    FullAlliance,
}

// Political events
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum PoliticalEventType {
    Election,
    Coup,
    Revolution,
    Treaty,
    War,
    Assassination,
    Reform,
    Scandal,
    Succession,
    Rebellion,
}

#[spacetimedb::table(name = political_event)]
pub struct PoliticalEvent {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub event_type: PoliticalEventType,
    pub primary_faction_id: u32,
    pub secondary_faction_id: Option<u32>,
    pub affected_cities: String, // JSON array of city IDs
    pub start_hour: u64,
    pub duration_hours: u32,
    pub success_chance: f32,
    pub impact_magnitude: f32,
    pub description: String,
    pub consequences: String,    // JSON of effects
    pub resolved: bool,
}

// Political office/position
#[spacetimedb::table(name = political_office)]
pub struct PoliticalOffice {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub city_id: u32,
    pub office_name: String,   // "Mayor", "Governor", "High Priest", etc.
    pub holder_id: u32,       // Individual ID
    pub faction_id: Option<u32>,
    pub power_level: f32,     // 0-100 influence
    pub term_start_hour: u64,
    pub term_length_hours: u32,
    pub election_method: ElectionMethod,
    pub approval_rating: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum ElectionMethod {
    Democratic,
    Hereditary,
    Appointed,
    MightMakesRight,
    Religious,
    Lottery,
}

// Create a new faction
#[spacetimedb::reducer]
pub fn create_faction(
    ctx: &ReducerContext,
    world_id: u32,
    name: String,
    faction_type: FactionType,
    ideology: Ideology,
    leader_id: u32,
    base_city_id: u32,
) -> Result<u32, String> {
    let faction_id = ctx.db.faction().iter().count() as u32 + 1;

    let world = ctx.db.game_world()
        .id()
        .find(&world_id)
        .ok_or("World not found")?;

    let faction = Faction {
        id: faction_id,
        world_id,
        name: name.clone(),
        faction_type,
        ideology,
        leader_id,
        base_city_id,
        influence: 25.0, // Starting influence
        treasury: 10000.0,
        stability: 75.0,
        public_support: 50.0,
        member_count: 10,
        founding_hour: world.total_hours,
        goals: "[]".to_string(),
        recent_actions: "[]".to_string(),
        is_active: true,
    };

    ctx.db.faction().insert(faction);

    log::info!("Created new faction '{}' (ID: {}) of type {:?} in world {}",
        name, faction_id, faction_type, world_id);

    Ok(faction_id)
}

// Update faction influence and status
#[spacetimedb::reducer]
pub fn update_faction_status(
    ctx: &ReducerContext,
    world_id: u32,
    hour: u64,
) -> Result<(), String> {
    let factions: Vec<Faction> = ctx.db.faction()
        .iter()
        .filter(|f| f.world_id == world_id && f.is_active)
        .cloned()
        .collect();

    for mut faction in factions {
        // Natural decay/growth of influence
        let base_change = match faction.faction_type {
            FactionType::Political => faction.public_support * 0.001,
            FactionType::Religious => faction.stability * 0.0005,
            FactionType::Economic => faction.treasury * 0.00001,
            FactionType::Military => faction.member_count as f32 * 0.01,
            FactionType::Cultural => faction.influence * 0.0008,
            FactionType::Criminal => -faction.public_support * 0.002,
        };

        faction.influence = (faction.influence + base_change).clamp(0.0, 100.0);

        // Treasury changes based on influence and type
        let treasury_change = match faction.faction_type {
            FactionType::Economic => faction.influence * 100.0,
            FactionType::Political => faction.public_support * 50.0,
            FactionType::Religious => faction.member_count as f32 * 10.0,
            _ => faction.influence * 25.0,
        };

        faction.treasury = (faction.treasury + treasury_change).max(0.0);

        // Stability factors
        let stability_change = if faction.influence > 50.0 {
            1.0
        } else if faction.influence < 25.0 {
            -2.0
        } else {
            0.0
        };

        faction.stability = (faction.stability + stability_change).clamp(0.0, 100.0);

        // Update the faction
        ctx.db.faction().id().update(faction.id, faction);
    }

    Ok(())
}

// Generate political events based on faction dynamics
#[spacetimedb::reducer]
pub fn generate_political_events(
    ctx: &ReducerContext,
    world_id: u32,
    hour: u64,
) -> Result<Vec<u32>, String> {
    let mut event_ids = Vec::new();
    let mut rng = rand::thread_rng();

    let factions: Vec<Faction> = ctx.db.faction()
        .iter()
        .filter(|f| f.world_id == world_id && f.is_active)
        .cloned()
        .collect();

    // Check for various political events
    for faction in &factions {
        let random_chance: f32 = rng.gen();

        // Election events for political factions
        if faction.faction_type == FactionType::Political && random_chance < 0.02 {
            let event_id = create_political_event(
                ctx,
                world_id,
                PoliticalEventType::Election,
                faction.id,
                None,
                hour,
                format!("{} calls for new elections!", faction.name),
            )?;
            event_ids.push(event_id);
        }

        // Coup attempts for low stability factions
        if faction.stability < 30.0 && random_chance < 0.01 {
            let event_id = create_political_event(
                ctx,
                world_id,
                PoliticalEventType::Coup,
                faction.id,
                None,
                hour,
                format!("Internal power struggle threatens {} leadership!", faction.name),
            )?;
            event_ids.push(event_id);
        }

        // Scandals for high-influence factions
        if faction.influence > 70.0 && random_chance < 0.015 {
            let event_id = create_political_event(
                ctx,
                world_id,
                PoliticalEventType::Scandal,
                faction.id,
                None,
                hour,
                format!("Corruption allegations surface against {}!", faction.name),
            )?;
            event_ids.push(event_id);
        }

        // Reforms for democratic ideologies
        if faction.ideology == Ideology::Democratic && faction.public_support > 60.0 && random_chance < 0.02 {
            let event_id = create_political_event(
                ctx,
                world_id,
                PoliticalEventType::Reform,
                faction.id,
                None,
                hour,
                format!("{} proposes democratic reforms!", faction.name),
            )?;
            event_ids.push(event_id);
        }
    }

    // Check for inter-faction conflicts
    let relationships: Vec<FactionRelationship> = ctx.db.faction_relationship()
        .iter()
        .filter(|r| r.world_id == world_id)
        .cloned()
        .collect();

    for relationship in relationships {
        let random_chance: f32 = rng.gen();

        // War declarations for hostile relationships
        if relationship.relationship < -70.0 &&
           relationship.relationship_type != RelationshipType::AtWar &&
           random_chance < 0.005 {
            let event_id = create_political_event(
                ctx,
                world_id,
                PoliticalEventType::War,
                relationship.faction1_id,
                Some(relationship.faction2_id),
                hour,
                "War declared between rival factions!".to_string(),
            )?;
            event_ids.push(event_id);
        }

        // Treaty negotiations for improving relationships
        if relationship.relationship > 50.0 &&
           relationship.treaty_status == TreatyStatus::None &&
           random_chance < 0.01 {
            let event_id = create_political_event(
                ctx,
                world_id,
                PoliticalEventType::Treaty,
                relationship.faction1_id,
                Some(relationship.faction2_id),
                hour,
                "Diplomatic negotiations begin between allies!".to_string(),
            )?;
            event_ids.push(event_id);
        }
    }

    if !event_ids.is_empty() {
        log::info!("Generated {} political events for world {}", event_ids.len(), world_id);
    }

    Ok(event_ids)
}

// Helper function to create political events
fn create_political_event(
    ctx: &ReducerContext,
    world_id: u32,
    event_type: PoliticalEventType,
    primary_faction_id: u32,
    secondary_faction_id: Option<u32>,
    hour: u64,
    description: String,
) -> Result<u32, String> {
    let event_id = ctx.db.political_event().iter().count() as u32 + 1;

    let duration = match event_type {
        PoliticalEventType::Election => 168, // 1 week
        PoliticalEventType::War => 720,      // 1 month
        PoliticalEventType::Treaty => 72,    // 3 days
        PoliticalEventType::Coup => 24,      // 1 day
        _ => 48, // 2 days default
    };

    let event = PoliticalEvent {
        id: event_id,
        world_id,
        event_type,
        primary_faction_id,
        secondary_faction_id,
        affected_cities: "[]".to_string(), // TODO: Determine affected cities
        start_hour: hour,
        duration_hours: duration,
        success_chance: 0.5,
        impact_magnitude: 1.0,
        description,
        consequences: "[]".to_string(),
        resolved: false,
    };

    ctx.db.political_event().insert(event);

    Ok(event_id)
}

// Process ongoing political events
#[spacetimedb::reducer]
pub fn process_political_events(
    ctx: &ReducerContext,
    world_id: u32,
    hour: u64,
) -> Result<Vec<u32>, String> {
    let mut completed_events = Vec::new();

    let ongoing_events: Vec<PoliticalEvent> = ctx.db.political_event()
        .iter()
        .filter(|e| e.world_id == world_id && !e.resolved)
        .filter(|e| hour >= e.start_hour + e.duration_hours as u64)
        .cloned()
        .collect();

    for mut event in ongoing_events {
        // Resolve the event
        let success = resolve_political_event(ctx, &event)?;

        event.resolved = true;
        event.consequences = if success {
            format!(r#"{{"success": true, "description": "Event completed successfully"}}"#)
        } else {
            format!(r#"{{"success": false, "description": "Event failed to achieve goals"}}"#)
        };

        ctx.db.political_event().id().update(event.id, event);
        completed_events.push(event.id);

        log::info!("Resolved political event {} with success: {}", event.id, success);
    }

    Ok(completed_events)
}

// Resolve a political event and apply consequences
fn resolve_political_event(
    ctx: &ReducerContext,
    event: &PoliticalEvent,
) -> Result<bool, String> {
    let mut rng = rand::thread_rng();
    let random_outcome: f32 = rng.gen();
    let success = random_outcome < event.success_chance;

    // Apply consequences based on event type and success
    match event.event_type {
        PoliticalEventType::Election => {
            if success {
                // Increase public support for winning faction
                if let Some(mut faction) = ctx.db.faction().id().find(&event.primary_faction_id) {
                    faction.public_support = (faction.public_support + 20.0).min(100.0);
                    faction.influence = (faction.influence + 10.0).min(100.0);
                    ctx.db.faction().id().update(faction.id, faction);
                }
            }
        },
        PoliticalEventType::Coup => {
            if let Some(mut faction) = ctx.db.faction().id().find(&event.primary_faction_id) {
                if success {
                    // Successful coup - gain power but lose stability
                    faction.influence = (faction.influence + 30.0).min(100.0);
                    faction.stability = (faction.stability - 40.0).max(0.0);
                    faction.public_support = (faction.public_support - 25.0).max(0.0);
                } else {
                    // Failed coup - lose everything
                    faction.influence = (faction.influence - 20.0).max(0.0);
                    faction.stability = (faction.stability - 20.0).max(0.0);
                    faction.public_support = (faction.public_support - 30.0).max(0.0);
                }
                ctx.db.faction().id().update(faction.id, faction);
            }
        },
        PoliticalEventType::War => {
            // War outcomes affect both factions
            if let (Some(mut faction1), Some(mut faction2)) = (
                ctx.db.faction().id().find(&event.primary_faction_id),
                event.secondary_faction_id.and_then(|id| ctx.db.faction().id().find(&id))
            ) {
                if success {
                    // Faction 1 wins
                    faction1.influence = (faction1.influence + 25.0).min(100.0);
                    faction1.treasury = (faction1.treasury - 5000.0).max(0.0);
                    faction2.influence = (faction2.influence - 30.0).max(0.0);
                    faction2.treasury = (faction2.treasury - 10000.0).max(0.0);
                } else {
                    // Faction 2 wins
                    faction2.influence = (faction2.influence + 25.0).min(100.0);
                    faction2.treasury = (faction2.treasury - 5000.0).max(0.0);
                    faction1.influence = (faction1.influence - 30.0).max(0.0);
                    faction1.treasury = (faction1.treasury - 10000.0).max(0.0);
                }
                ctx.db.faction().id().update(faction1.id, faction1);
                ctx.db.faction().id().update(faction2.id, faction2);
            }
        },
        _ => {
            // Default outcome handling
            if let Some(mut faction) = ctx.db.faction().id().find(&event.primary_faction_id) {
                if success {
                    faction.influence = (faction.influence + 5.0).min(100.0);
                } else {
                    faction.influence = (faction.influence - 3.0).max(0.0);
                }
                ctx.db.faction().id().update(faction.id, faction);
            }
        }
    }

    Ok(success)
}