// Seasonal cycles and their effects on world simulation

use spacetimedb::{ReducerContext, Table, SpacetimeType};
use serde::{Serialize, Deserialize};
use log;
use crate::world::{Season, ClimateZone};
use crate::natural::SeasonalEffect;

// Seasonal transition tracking
#[spacetimedb::table(name = seasonal_transition)]
pub struct SeasonalTransition {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub from_season: Season,
    pub to_season: Season,
    pub transition_start_hour: u64,
    pub transition_duration_hours: u32,
    pub current_progress: f32,     // 0.0 to 1.0
    pub effects_applied: bool,
    pub transition_events: String, // JSON array of events during transition
}

// Seasonal activities and behaviors
#[spacetimedb::table(name = seasonal_activity)]
pub struct SeasonalActivity {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub region_id: u32,
    pub season: Season,
    pub activity_type: ActivityType,
    pub participation_rate: f32,   // % of population participating
    pub economic_impact: f32,      // Economic multiplier
    pub cultural_significance: f32, // Cultural importance 0-100
    pub resource_requirements: String, // JSON of resources needed
    pub duration_weeks: u32,
    pub is_active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum ActivityType {
    Planting,
    Harvest,
    Fishing,
    Hunting,
    Festival,
    Migration,
    Construction,
    Trading,
    Preparation,
    Celebration,
    Mourning,
    Worship,
}

// Phenological phases (natural timing of biological events)
#[spacetimedb::table(name = phenology)]
pub struct Phenology {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub region_id: u32,
    pub species_type: SpeciesType,
    pub phase: PhenologicalPhase,
    pub typical_start_day: u32,    // Day of year (1-365)
    pub current_year_start: u32,   // Actual start this year
    pub duration_days: u32,
    pub temperature_threshold: f32,
    pub climate_sensitivity: f32,  // How much climate affects timing
    pub ecological_impact: String, // JSON of impacts on ecosystem
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum SpeciesType {
    Trees,
    Crops,
    Wildflowers,
    Migratory_Birds,
    Fish,
    Insects,
    Large_Mammals,
    Small_Mammals,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum PhenologicalPhase {
    BudBurst,
    FirstLeaf,
    Flowering,
    Fruiting,
    LeafFall,
    Migration_Arrival,
    Migration_Departure,
    Breeding,
    Hibernation,
    Emergence,
}

// Initialize seasonal cycles for a world
#[spacetimedb::reducer]
pub fn initialize_seasonal_cycles(
    ctx: &ReducerContext,
    world_id: u32,
    climate_zone: ClimateZone,
) -> Result<(), String> {
    // Initialize seasonal activities for all regions
    let regions: Vec<u32> = ctx.db.climate_state()
        .iter()
        .filter(|c| c.world_id == world_id)
        .map(|c| c.region_id)
        .collect::<std::collections::HashSet<u32>>()
        .into_iter()
        .collect();

    for region_id in regions {
        // Create seasonal activities for each season
        for season in [Season::Spring, Season::Summer, Season::Autumn, Season::Winter] {
            let activities = get_seasonal_activities(season, climate_zone);

            for activity_type in activities {
                let activity_id = ctx.db.seasonal_activity().iter().count() as u32 + 1;

                let (participation, economic_impact, cultural_sig, duration) =
                    get_activity_characteristics(activity_type, season, climate_zone);

                let seasonal_activity = SeasonalActivity {
                    id: activity_id,
                    world_id,
                    region_id,
                    season,
                    activity_type,
                    participation_rate: participation,
                    economic_impact,
                    cultural_significance: cultural_sig,
                    resource_requirements: generate_resource_requirements_json(activity_type),
                    duration_weeks: duration,
                    is_active: false, // Will be activated when season starts
                };

                ctx.db.seasonal_activity().insert(seasonal_activity);
            }
        }

        // Initialize phenological data
        initialize_phenology_for_region(ctx, world_id, region_id, climate_zone)?;
    }

    log::info!("Initialized seasonal cycles for world {} with climate {:?}",
        world_id, climate_zone);
    Ok(())
}

// Get relevant activities for season and climate
fn get_seasonal_activities(season: Season, climate_zone: ClimateZone) -> Vec<ActivityType> {
    match (season, climate_zone) {
        (Season::Spring, _) => vec![
            ActivityType::Planting,
            ActivityType::Festival,
            ActivityType::Construction,
            ActivityType::Preparation,
        ],
        (Season::Summer, ClimateZone::Temperate) => vec![
            ActivityType::Fishing,
            ActivityType::Trading,
            ActivityType::Festival,
            ActivityType::Construction,
        ],
        (Season::Summer, ClimateZone::Tropical) => vec![
            ActivityType::Fishing,
            ActivityType::Trading,
            ActivityType::Worship,
        ],
        (Season::Summer, ClimateZone::Arid) => vec![
            ActivityType::Migration,
            ActivityType::Trading,
        ],
        (Season::Autumn, _) => vec![
            ActivityType::Harvest,
            ActivityType::Preparation,
            ActivityType::Trading,
            ActivityType::Celebration,
        ],
        (Season::Winter, ClimateZone::Arctic) => vec![
            ActivityType::Hunting,
            ActivityType::Worship,
            ActivityType::Mourning,
        ],
        (Season::Winter, ClimateZone::Temperate) => vec![
            ActivityType::Hunting,
            ActivityType::Festival,
            ActivityType::Preparation,
        ],
        (Season::Winter, _) => vec![
            ActivityType::Trading,
            ActivityType::Festival,
            ActivityType::Worship,
        ],
    }
}

// Get activity characteristics
fn get_activity_characteristics(
    activity_type: ActivityType,
    season: Season,
    climate_zone: ClimateZone,
) -> (f32, f32, f32, u32) {
    let base_characteristics = match activity_type {
        ActivityType::Planting => (80.0, 1.2, 70.0, 4),
        ActivityType::Harvest => (90.0, 1.5, 85.0, 6),
        ActivityType::Fishing => (40.0, 1.1, 50.0, 8),
        ActivityType::Hunting => (30.0, 1.0, 60.0, 12),
        ActivityType::Festival => (70.0, 0.8, 90.0, 1),
        ActivityType::Migration => (15.0, 0.9, 40.0, 2),
        ActivityType::Construction => (50.0, 1.3, 30.0, 8),
        ActivityType::Trading => (60.0, 1.4, 70.0, 4),
        ActivityType::Preparation => (85.0, 1.0, 60.0, 2),
        ActivityType::Celebration => (80.0, 0.9, 95.0, 1),
        ActivityType::Mourning => (20.0, 0.7, 80.0, 1),
        ActivityType::Worship => (60.0, 0.8, 85.0, 2),
    };

    // Adjust for climate and season
    let climate_modifier = match (activity_type, climate_zone) {
        (ActivityType::Fishing, ClimateZone::Coastal) => 1.5,
        (ActivityType::Hunting, ClimateZone::Arctic) => 2.0,
        (ActivityType::Planting, ClimateZone::Arid) => 0.5,
        (ActivityType::Construction, ClimateZone::Temperate) => 1.2,
        _ => 1.0,
    };

    let (mut participation, mut economic, cultural, duration) = base_characteristics;
    participation *= climate_modifier;
    economic *= climate_modifier;

    (participation, economic, cultural, duration)
}

// Generate resource requirements JSON
fn generate_resource_requirements_json(activity_type: ActivityType) -> String {
    let requirements = match activity_type {
        ActivityType::Planting => serde_json::json!({
            "seeds": 100,
            "tools": 50,
            "labor_hours": 200,
            "water": 500
        }),
        ActivityType::Harvest => serde_json::json!({
            "tools": 80,
            "labor_hours": 300,
            "storage": 150,
            "transport": 100
        }),
        ActivityType::Fishing => serde_json::json!({
            "boats": 20,
            "nets": 50,
            "labor_hours": 150,
            "fuel": 30
        }),
        ActivityType::Hunting => serde_json::json!({
            "weapons": 30,
            "provisions": 50,
            "labor_hours": 100
        }),
        ActivityType::Festival => serde_json::json!({
            "food": 200,
            "decorations": 50,
            "entertainment": 100,
            "labor_hours": 80
        }),
        ActivityType::Construction => serde_json::json!({
            "materials": 500,
            "tools": 100,
            "labor_hours": 400,
            "transport": 50
        }),
        ActivityType::Trading => serde_json::json!({
            "goods": 300,
            "transport": 100,
            "labor_hours": 120,
            "security": 50
        }),
        _ => serde_json::json!({
            "labor_hours": 50,
            "resources": 100
        })
    };

    requirements.to_string()
}

// Initialize phenological data for a region
fn initialize_phenology_for_region(
    ctx: &ReducerContext,
    world_id: u32,
    region_id: u32,
    climate_zone: ClimateZone,
) -> Result<(), String> {
    let species_phases = get_phenological_schedule(climate_zone);

    for (species, phase, start_day, duration, temp_threshold) in species_phases {
        let pheno_id = ctx.db.phenology().iter().count() as u32 + 1;

        let phenology = Phenology {
            id: pheno_id,
            world_id,
            region_id,
            species_type: species,
            phase,
            typical_start_day: start_day,
            current_year_start: start_day, // Will be adjusted by climate
            duration_days: duration,
            temperature_threshold: temp_threshold,
            climate_sensitivity: get_climate_sensitivity(species, phase),
            ecological_impact: generate_ecological_impact_json(species, phase),
        };

        ctx.db.phenology().insert(phenology);
    }

    Ok(())
}

// Get phenological schedule for climate zone
fn get_phenological_schedule(climate_zone: ClimateZone) -> Vec<(SpeciesType, PhenologicalPhase, u32, u32, f32)> {
    match climate_zone {
        ClimateZone::Temperate => vec![
            (SpeciesType::Trees, PhenologicalPhase::BudBurst, 60, 14, 5.0),
            (SpeciesType::Trees, PhenologicalPhase::FirstLeaf, 75, 21, 8.0),
            (SpeciesType::Trees, PhenologicalPhase::Flowering, 90, 30, 12.0),
            (SpeciesType::Trees, PhenologicalPhase::LeafFall, 280, 45, 5.0),
            (SpeciesType::Crops, PhenologicalPhase::Flowering, 120, 60, 15.0),
            (SpeciesType::Crops, PhenologicalPhase::Fruiting, 150, 90, 18.0),
            (SpeciesType::Migratory_Birds, PhenologicalPhase::Migration_Arrival, 80, 30, 10.0),
            (SpeciesType::Migratory_Birds, PhenologicalPhase::Migration_Departure, 250, 45, 8.0),
            (SpeciesType::Insects, PhenologicalPhase::Emergence, 100, 180, 12.0),
            (SpeciesType::Large_Mammals, PhenologicalPhase::Breeding, 110, 60, 8.0),
        ],
        ClimateZone::Tropical => vec![
            (SpeciesType::Trees, PhenologicalPhase::Flowering, 1, 365, 20.0),
            (SpeciesType::Trees, PhenologicalPhase::Fruiting, 1, 365, 22.0),
            (SpeciesType::Fish, PhenologicalPhase::Migration_Arrival, 180, 60, 25.0),
            (SpeciesType::Fish, PhenologicalPhase::Breeding, 200, 90, 26.0),
            (SpeciesType::Insects, PhenologicalPhase::Emergence, 1, 365, 24.0),
            (SpeciesType::Large_Mammals, PhenologicalPhase::Breeding, 90, 120, 24.0),
        ],
        ClimateZone::Arctic => vec![
            (SpeciesType::Trees, PhenologicalPhase::BudBurst, 140, 21, -2.0),
            (SpeciesType::Trees, PhenologicalPhase::FirstLeaf, 160, 30, 2.0),
            (SpeciesType::Migratory_Birds, PhenologicalPhase::Migration_Arrival, 120, 60, 0.0),
            (SpeciesType::Migratory_Birds, PhenologicalPhase::Migration_Departure, 230, 30, -5.0),
            (SpeciesType::Large_Mammals, PhenologicalPhase::Breeding, 150, 45, 5.0),
            (SpeciesType::Fish, PhenologicalPhase::Migration_Arrival, 130, 90, 1.0),
        ],
        ClimateZone::Arid => vec![
            (SpeciesType::Trees, PhenologicalPhase::Flowering, 30, 60, 15.0),
            (SpeciesType::Wildflowers, PhenologicalPhase::Flowering, 45, 30, 18.0),
            (SpeciesType::Small_Mammals, PhenologicalPhase::Breeding, 60, 90, 20.0),
            (SpeciesType::Insects, PhenologicalPhase::Emergence, 50, 120, 22.0),
        ],
        ClimateZone::Mediterranean => vec![
            (SpeciesType::Trees, PhenologicalPhase::Flowering, 45, 45, 10.0),
            (SpeciesType::Trees, PhenologicalPhase::Fruiting, 120, 90, 18.0),
            (SpeciesType::Crops, PhenologicalPhase::Flowering, 90, 75, 16.0),
            (SpeciesType::Wildflowers, PhenologicalPhase::Flowering, 60, 120, 12.0),
            (SpeciesType::Migratory_Birds, PhenologicalPhase::Migration_Arrival, 70, 45, 8.0),
            (SpeciesType::Large_Mammals, PhenologicalPhase::Breeding, 100, 75, 15.0),
        ],
    }
}

// Get climate sensitivity for species/phase
fn get_climate_sensitivity(species: SpeciesType, phase: PhenologicalPhase) -> f32 {
    match (species, phase) {
        (SpeciesType::Trees, PhenologicalPhase::BudBurst) => 0.8,
        (SpeciesType::Trees, PhenologicalPhase::Flowering) => 0.7,
        (SpeciesType::Crops, _) => 0.9,
        (SpeciesType::Migratory_Birds, _) => 0.6,
        (SpeciesType::Insects, PhenologicalPhase::Emergence) => 0.95,
        _ => 0.5,
    }
}

// Generate ecological impact JSON
fn generate_ecological_impact_json(species: SpeciesType, phase: PhenologicalPhase) -> String {
    let impact = match (species, phase) {
        (SpeciesType::Trees, PhenologicalPhase::Flowering) => serde_json::json!({
            "pollinator_activity": 1.5,
            "aesthetic_value": 1.3,
            "air_quality": 1.2
        }),
        (SpeciesType::Trees, PhenologicalPhase::Fruiting) => serde_json::json!({
            "food_availability": 1.8,
            "wildlife_activity": 1.4,
            "economic_value": 1.6
        }),
        (SpeciesType::Crops, PhenologicalPhase::Fruiting) => serde_json::json!({
            "food_security": 2.0,
            "economic_value": 1.8,
            "employment": 1.5
        }),
        (SpeciesType::Migratory_Birds, PhenologicalPhase::Migration_Arrival) => serde_json::json!({
            "biodiversity": 1.4,
            "pest_control": 1.3,
            "cultural_significance": 1.2
        }),
        _ => serde_json::json!({
            "ecosystem_activity": 1.1
        })
    };

    impact.to_string()
}

// Update seasonal activities based on current season
#[spacetimedb::reducer]
pub fn update_seasonal_activities(
    ctx: &ReducerContext,
    world_id: u32,
    current_hour: u64,
) -> Result<Vec<u32>, String> {
    let mut updated_activities = Vec::new();

    let current_season = crate::world::calculate_season_from_hour(current_hour);

    // Activate activities for current season
    let seasonal_activities: Vec<SeasonalActivity> = ctx.db.seasonal_activity()
        .iter()
        .filter(|a| a.world_id == world_id)
        .cloned()
        .collect();

    for mut activity in seasonal_activities {
        let should_be_active = activity.season == current_season;

        if should_be_active != activity.is_active {
            activity.is_active = should_be_active;
            ctx.db.seasonal_activity().id().update(activity.id, activity);
            updated_activities.push(activity.id);

            if should_be_active {
                log::info!("Activated seasonal activity {:?} for {:?} in region {}",
                    activity.activity_type, current_season, activity.region_id);

                // Apply economic effects
                apply_seasonal_activity_effects(ctx, &activity)?;
            }
        }
    }

    Ok(updated_activities)
}

// Apply effects of seasonal activities
fn apply_seasonal_activity_effects(
    ctx: &ReducerContext,
    activity: &SeasonalActivity,
) -> Result<(), String> {
    // Apply economic effects to markets
    if activity.economic_impact != 1.0 {
        let markets: Vec<crate::economics::Market> = ctx.db.market()
            .iter()
            .filter(|m| m.world_id == activity.world_id && m.city_id == activity.region_id)
            .cloned()
            .collect();

        for mut market in markets {
            match activity.activity_type {
                ActivityType::Harvest => {
                    if market.resource_type == crate::economics::ResourceType::Food {
                        market.supply *= activity.economic_impact;
                    }
                },
                ActivityType::Trading => {
                    market.demand *= activity.economic_impact;
                },
                ActivityType::Construction => {
                    if market.resource_type == crate::economics::ResourceType::RawMaterials {
                        market.demand *= activity.economic_impact;
                    }
                },
                _ => {
                    // General economic effect
                    market.supply *= (1.0 + (activity.economic_impact - 1.0) * 0.5);
                }
            }

            ctx.db.market().id().update(market.id, market);
        }
    }

    Ok(())
}

// Update phenological phases based on climate
#[spacetimedb::reducer]
pub fn update_phenological_phases(
    ctx: &ReducerContext,
    world_id: u32,
    current_hour: u64,
) -> Result<Vec<u32>, String> {
    let mut updated_phases = Vec::new();

    let current_day_of_year = ((current_hour / 24) % 365) as u32 + 1;

    let phenology_data: Vec<Phenology> = ctx.db.phenology()
        .iter()
        .filter(|p| p.world_id == world_id)
        .cloned()
        .collect();

    for mut pheno in phenology_data {
        // Check if phase should start based on adjusted timing
        let start_day = calculate_adjusted_phenology_start(ctx, &pheno)?;

        if current_day_of_year >= start_day &&
           current_day_of_year < start_day + pheno.duration_days &&
           pheno.current_year_start != start_day {

            pheno.current_year_start = start_day;
            ctx.db.phenology().id().update(pheno.id, pheno);
            updated_phases.push(pheno.id);

            log::info!("Phenological phase {:?} started for {:?} in region {}",
                pheno.phase, pheno.species_type, pheno.region_id);

            // Create narrative event for significant phenological events
            if matches!(pheno.phase, PhenologicalPhase::Flowering | PhenologicalPhase::Migration_Arrival) {
                let title = format!("{:?} {:?} Begins", pheno.species_type, pheno.phase);
                let description = format!("The {:?} phase has begun for {:?} in the region",
                    pheno.phase, pheno.species_type);

                crate::narrative::create_narrative_event(
                    ctx,
                    world_id,
                    1, // Default game ID
                    crate::narrative::EventCategory::Natural,
                    title,
                    description,
                    2, // Low importance for natural cycles
                )?;
            }
        }
    }

    Ok(updated_phases)
}

// Calculate climate-adjusted phenology start date
fn calculate_adjusted_phenology_start(
    ctx: &ReducerContext,
    pheno: &Phenology,
) -> Result<u32, String> {
    // Get current climate conditions
    let climate = ctx.db.climate_state()
        .iter()
        .find(|c| c.world_id == pheno.world_id && c.region_id == pheno.region_id)
        .ok_or("Climate state not found")?;

    // Calculate temperature-based adjustment
    let temp_difference = climate.current_temperature - pheno.temperature_threshold;
    let temp_adjustment = temp_difference * pheno.climate_sensitivity * 2.0; // 2 days per degree

    // Ensure reasonable bounds
    let adjusted_start = (pheno.typical_start_day as f32 + temp_adjustment)
        .clamp(1.0, 365.0) as u32;

    Ok(adjusted_start)
}

// Create seasonal transition when season changes
#[spacetimedb::reducer]
pub fn initiate_seasonal_transition(
    ctx: &ReducerContext,
    world_id: u32,
    from_season: Season,
    to_season: Season,
    current_hour: u64,
) -> Result<u32, String> {
    let transition_id = ctx.db.seasonal_transition().iter().count() as u32 + 1;

    let transition = SeasonalTransition {
        id: transition_id,
        world_id,
        from_season,
        to_season,
        transition_start_hour: current_hour,
        transition_duration_hours: 168, // 1 week transition period
        current_progress: 0.0,
        effects_applied: false,
        transition_events: generate_transition_events_json(from_season, to_season),
    };

    ctx.db.seasonal_transition().insert(transition);

    // Create narrative event for season change
    let title = format!("{:?} to {:?} Transition", from_season, to_season);
    let description = format!("The season is changing from {:?} to {:?}, bringing new conditions and opportunities",
        from_season, to_season);

    crate::narrative::create_narrative_event(
        ctx,
        world_id,
        1, // Default game ID
        crate::narrative::EventCategory::Natural,
        title,
        description,
        3, // Moderate importance for season changes
    )?;

    log::info!("Initiated seasonal transition from {:?} to {:?} in world {}",
        from_season, to_season, world_id);

    Ok(transition_id)
}

// Generate transition events JSON
fn generate_transition_events_json(from_season: Season, to_season: Season) -> String {
    let events = match (from_season, to_season) {
        (Season::Winter, Season::Spring) => vec![
            "snow_melt", "ice_breakup", "bird_arrival", "plant_budding",
        ],
        (Season::Spring, Season::Summer) => vec![
            "leaf_full", "flower_bloom", "increased_daylight", "warming_temperatures",
        ],
        (Season::Summer, Season::Autumn) => vec![
            "leaf_color_change", "harvest_time", "bird_migration", "cooling_temperatures",
        ],
        (Season::Autumn, Season::Winter) => vec![
            "leaf_fall", "frost_arrival", "animal_preparation", "shorter_days",
        ],
        _ => vec!["gradual_change"],
    };

    serde_json::to_string(&events).unwrap_or_else(|_| "[]".to_string())
}

// Process ongoing seasonal transitions
#[spacetimedb::reducer]
pub fn process_seasonal_transitions(
    ctx: &ReducerContext,
    world_id: u32,
    current_hour: u64,
) -> Result<Vec<u32>, String> {
    let mut completed_transitions = Vec::new();

    let active_transitions: Vec<SeasonalTransition> = ctx.db.seasonal_transition()
        .iter()
        .filter(|t| t.world_id == world_id && t.current_progress < 1.0)
        .cloned()
        .collect();

    for mut transition in active_transitions {
        let elapsed_hours = current_hour - transition.transition_start_hour;
        let new_progress = (elapsed_hours as f32 / transition.transition_duration_hours as f32).min(1.0);

        transition.current_progress = new_progress;

        // Apply gradual effects as transition progresses
        if new_progress >= 1.0 && !transition.effects_applied {
            apply_complete_seasonal_transition_effects(ctx, &transition)?;
            transition.effects_applied = true;
            completed_transitions.push(transition.id);
        }

        ctx.db.seasonal_transition().id().update(transition.id, transition);
    }

    Ok(completed_transitions)
}

// Apply complete seasonal transition effects
fn apply_complete_seasonal_transition_effects(
    ctx: &ReducerContext,
    transition: &SeasonalTransition,
) -> Result<(), String> {
    // Update seasonal effects for the new season
    let seasonal_effect = ctx.db.seasonal_effect()
        .iter()
        .find(|e| e.world_id == transition.world_id && e.season == transition.to_season)
        .cloned();

    if let Some(effect) = seasonal_effect {
        // Apply resource availability changes
        let resource_mods: serde_json::Value = serde_json::from_str(&effect.resource_availability)
            .unwrap_or_else(|_| serde_json::json!({}));

        // Update markets with seasonal modifiers
        let markets: Vec<crate::economics::Market> = ctx.db.market()
            .iter()
            .filter(|m| m.world_id == transition.world_id)
            .cloned()
            .collect();

        for mut market in markets {
            let resource_name = match market.resource_type {
                crate::economics::ResourceType::Food => "food",
                crate::economics::ResourceType::Luxury => "luxury",
                crate::economics::ResourceType::RawMaterials => "raw_materials",
                crate::economics::ResourceType::ProcessedGoods => "processed_goods",
            };

            if let Some(modifier) = resource_mods.get(resource_name) {
                if let Some(mod_value) = modifier.as_f64() {
                    market.supply *= mod_value as f32;
                    ctx.db.market().id().update(market.id, market);
                }
            }
        }

        log::info!("Applied seasonal transition effects for {:?} in world {}",
            transition.to_season, transition.world_id);
    }

    Ok(())
}