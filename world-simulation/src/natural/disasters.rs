// Natural disaster system for catastrophic events

use spacetimedb::{ReducerContext, Table, SpacetimeType};
use serde::{Serialize, Deserialize};
use log;
use rand::Rng;
use crate::world::ClimateZone;
use crate::natural::{NaturalEventType, EventSeverity};
use crate::narrative::{create_narrative_event, EventCategory};

// Disaster risk assessment
#[spacetimedb::table(name = disaster_risk)]
pub struct DisasterRisk {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub region_id: u32,
    pub disaster_type: DisasterType,
    pub base_probability: f32,    // Annual probability (0-1)
    pub current_risk_level: f32,  // Current risk multiplier
    pub last_occurrence: u64,     // Last time this disaster occurred
    pub severity_distribution: String, // JSON of severity probabilities
    pub warning_time: u32,        // Hours of advance warning possible
    pub mitigation_level: f32,    // 0-100% disaster preparedness
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum DisasterType {
    Earthquake,
    Volcano,
    Tsunami,
    Hurricane,
    Tornado,
    Wildfire,
    Flood,
    Landslide,
    Blizzard,
    Heatwave,
    Drought,
    Plague,
    Meteor,
}

// Disaster warning system
#[spacetimedb::table(name = disaster_warning)]
pub struct DisasterWarning {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub disaster_type: DisasterType,
    pub affected_regions: String, // JSON array of region IDs
    pub warning_level: WarningLevel,
    pub estimated_severity: EventSeverity,
    pub estimated_impact_hour: u64,
    pub confidence: f32,          // 0-100% forecast confidence
    pub warning_issued_hour: u64,
    pub evacuation_recommended: bool,
    pub preparation_actions: String, // JSON array of recommended actions
    pub is_active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum WarningLevel {
    Watch,      // Conditions favorable
    Advisory,   // Minor impact expected
    Warning,    // Significant impact expected
    Emergency,  // Severe/catastrophic impact expected
}

// Disaster preparation and response
#[spacetimedb::table(name = disaster_response)]
pub struct DisasterResponse {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub region_id: u32,
    pub disaster_event_id: u32,   // Links to natural_event
    pub response_type: ResponseType,
    pub resources_allocated: f32,
    pub personnel_count: u32,
    pub effectiveness: f32,       // 0-100% response effectiveness
    pub start_hour: u64,
    pub duration_hours: u32,
    pub status: ResponseStatus,
    pub results: String,          // JSON of response outcomes
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum ResponseType {
    Evacuation,
    EmergencyServices,
    MedicalResponse,
    SearchAndRescue,
    FireSuppression,
    FloodControl,
    Relief,
    Reconstruction,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum ResponseStatus {
    Preparing,
    Active,
    Completed,
    Failed,
}

// Initialize disaster risk assessment for a world
#[spacetimedb::reducer]
pub fn initialize_disaster_risks(
    ctx: &ReducerContext,
    world_id: u32,
    climate_zone: ClimateZone,
) -> Result<Vec<u32>, String> {
    let mut risk_ids = Vec::new();

    // Get all regions in the world
    let regions: Vec<u32> = ctx.db.climate_state()
        .iter()
        .filter(|c| c.world_id == world_id)
        .map(|c| c.region_id)
        .collect::<std::collections::HashSet<u32>>()
        .into_iter()
        .collect();

    // Initialize risks for each disaster type per region
    for region_id in regions {
        let disaster_types = get_relevant_disasters(climate_zone);

        for disaster_type in disaster_types {
            let risk_id = ctx.db.disaster_risk().iter().count() as u32 + 1;

            let (base_prob, warning_time, mitigation) = get_disaster_characteristics(disaster_type, climate_zone);

            let disaster_risk = DisasterRisk {
                id: risk_id,
                world_id,
                region_id,
                disaster_type,
                base_probability: base_prob,
                current_risk_level: 1.0,
                last_occurrence: 0,
                severity_distribution: generate_severity_distribution_json(disaster_type),
                warning_time,
                mitigation_level: mitigation,
            };

            ctx.db.disaster_risk().insert(disaster_risk);
            risk_ids.push(risk_id);
        }
    }

    log::info!("Initialized disaster risk assessments for {} regions in world {}",
        regions.len(), world_id);
    Ok(risk_ids)
}

// Get relevant disaster types for climate zone
fn get_relevant_disasters(climate_zone: ClimateZone) -> Vec<DisasterType> {
    match climate_zone {
        ClimateZone::Arctic => vec![
            DisasterType::Blizzard,
            DisasterType::Earthquake,
            DisasterType::Flood,
        ],
        ClimateZone::Temperate => vec![
            DisasterType::Earthquake,
            DisasterType::Flood,
            DisasterType::Tornado,
            DisasterType::Wildfire,
            DisasterType::Blizzard,
            DisasterType::Drought,
        ],
        ClimateZone::Tropical => vec![
            DisasterType::Hurricane,
            DisasterType::Flood,
            DisasterType::Wildfire,
            DisasterType::Volcano,
            DisasterType::Earthquake,
            DisasterType::Landslide,
        ],
        ClimateZone::Arid => vec![
            DisasterType::Drought,
            DisasterType::Wildfire,
            DisasterType::Heatwave,
            DisasterType::Earthquake,
            DisasterType::Flood, // Flash floods
        ],
        ClimateZone::Mediterranean => vec![
            DisasterType::Wildfire,
            DisasterType::Earthquake,
            DisasterType::Drought,
            DisasterType::Flood,
            DisasterType::Heatwave,
        ],
    }
}

// Get disaster characteristics (probability, warning time, base mitigation)
fn get_disaster_characteristics(disaster_type: DisasterType, climate_zone: ClimateZone) -> (f32, u32, f32) {
    let base_characteristics = match disaster_type {
        DisasterType::Earthquake => (0.02, 0, 30.0),      // 2% annual, no warning, 30% mitigation
        DisasterType::Volcano => (0.001, 168, 50.0),      // 0.1% annual, 1 week warning, 50% mitigation
        DisasterType::Hurricane => (0.1, 72, 70.0),       // 10% annual, 3 days warning, 70% mitigation
        DisasterType::Tornado => (0.05, 2, 40.0),         // 5% annual, 2 hours warning, 40% mitigation
        DisasterType::Wildfire => (0.2, 12, 60.0),        // 20% annual, 12 hours warning, 60% mitigation
        DisasterType::Flood => (0.15, 24, 55.0),          // 15% annual, 1 day warning, 55% mitigation
        DisasterType::Drought => (0.08, 720, 40.0),       // 8% annual, 1 month warning, 40% mitigation
        DisasterType::Blizzard => (0.3, 48, 80.0),        // 30% annual, 2 days warning, 80% mitigation
        DisasterType::Heatwave => (0.25, 72, 50.0),       // 25% annual, 3 days warning, 50% mitigation
        DisasterType::Landslide => (0.03, 6, 35.0),       // 3% annual, 6 hours warning, 35% mitigation
        DisasterType::Tsunami => (0.005, 3, 60.0),        // 0.5% annual, 3 hours warning, 60% mitigation
        DisasterType::Plague => (0.01, 168, 70.0),        // 1% annual, 1 week warning, 70% mitigation
        DisasterType::Meteor => (0.0001, 0, 0.0),         // 0.01% annual, no warning, no mitigation
    };

    // Adjust for climate zone
    let climate_modifier = match (disaster_type, climate_zone) {
        (DisasterType::Hurricane, ClimateZone::Tropical) => 3.0,
        (DisasterType::Wildfire, ClimateZone::Arid) => 2.5,
        (DisasterType::Blizzard, ClimateZone::Arctic) => 4.0,
        (DisasterType::Drought, ClimateZone::Arid) => 3.0,
        (DisasterType::Flood, ClimateZone::Tropical) => 2.0,
        _ => 1.0,
    };

    let (mut prob, warning, mitigation) = base_characteristics;
    prob *= climate_modifier;

    (prob, warning, mitigation)
}

// Generate severity distribution JSON for disaster type
fn generate_severity_distribution_json(disaster_type: DisasterType) -> String {
    let distribution = match disaster_type {
        DisasterType::Earthquake => serde_json::json!({
            "minor": 0.7,
            "moderate": 0.2,
            "major": 0.08,
            "catastrophic": 0.02
        }),
        DisasterType::Hurricane => serde_json::json!({
            "minor": 0.4,
            "moderate": 0.4,
            "major": 0.15,
            "catastrophic": 0.05
        }),
        DisasterType::Wildfire => serde_json::json!({
            "minor": 0.6,
            "moderate": 0.25,
            "major": 0.12,
            "catastrophic": 0.03
        }),
        DisasterType::Meteor => serde_json::json!({
            "minor": 0.0,
            "moderate": 0.0,
            "major": 0.3,
            "catastrophic": 0.7
        }),
        _ => serde_json::json!({
            "minor": 0.5,
            "moderate": 0.3,
            "major": 0.15,
            "catastrophic": 0.05
        })
    };

    distribution.to_string()
}

// Check for potential disasters and issue warnings
#[spacetimedb::reducer]
pub fn assess_disaster_risks(
    ctx: &ReducerContext,
    world_id: u32,
    current_hour: u64,
) -> Result<Vec<u32>, String> {
    let mut warning_ids = Vec::new();
    let mut rng = rand::thread_rng();

    let disaster_risks: Vec<DisasterRisk> = ctx.db.disaster_risk()
        .iter()
        .filter(|r| r.world_id == world_id)
        .cloned()
        .collect();

    for risk in disaster_risks {
        // Calculate current risk based on various factors
        let mut current_risk = risk.base_probability;

        // Increase risk based on time since last occurrence
        let hours_since_last = if risk.last_occurrence > 0 {
            current_hour - risk.last_occurrence
        } else {
            8760 // 1 year default
        };

        let recurrence_factor = (hours_since_last as f32 / 8760.0).min(2.0); // Max 2x increase
        current_risk *= recurrence_factor;

        // Adjust risk based on current climate conditions
        current_risk *= calculate_climate_risk_modifier(ctx, world_id, risk.region_id, risk.disaster_type)?;

        // Check if disaster should occur (convert annual probability to hourly)
        let hourly_probability = current_risk / 8760.0;

        if rng.gen::<f32>() < hourly_probability {
            // Generate disaster warning
            let warning_id = issue_disaster_warning(
                ctx,
                world_id,
                risk.disaster_type,
                risk.region_id,
                current_hour,
                risk.warning_time,
            )?;

            warning_ids.push(warning_id);
        }
    }

    Ok(warning_ids)
}

// Calculate climate-based risk modifier
fn calculate_climate_risk_modifier(
    ctx: &ReducerContext,
    world_id: u32,
    region_id: u32,
    disaster_type: DisasterType,
) -> Result<f32, String> {
    let climate = ctx.db.climate_state()
        .iter()
        .find(|c| c.world_id == world_id && c.region_id == region_id)
        .ok_or("Climate state not found")?;

    let modifier = match disaster_type {
        DisasterType::Wildfire => {
            if climate.current_temperature > 30.0 && climate.humidity < 30.0 {
                3.0 // High fire risk
            } else if climate.current_temperature > 25.0 && climate.humidity < 50.0 {
                1.5
            } else {
                0.5
            }
        },
        DisasterType::Flood => {
            if climate.precipitation > 10.0 {
                4.0 // Heavy precipitation
            } else if climate.precipitation > 5.0 {
                2.0
            } else {
                0.3
            }
        },
        DisasterType::Tornado => {
            if climate.wind_speed > 50.0 && climate.current_temperature > 20.0 {
                2.5
            } else {
                1.0
            }
        },
        DisasterType::Heatwave => {
            if climate.current_temperature > 35.0 {
                3.0
            } else if climate.current_temperature > 30.0 {
                1.8
            } else {
                0.2
            }
        },
        DisasterType::Blizzard => {
            if climate.current_temperature < -10.0 && climate.wind_speed > 30.0 {
                2.5
            } else {
                1.0
            }
        },
        _ => 1.0, // No climate modifier for earthquakes, meteors, etc.
    };

    Ok(modifier)
}

// Issue disaster warning
fn issue_disaster_warning(
    ctx: &ReducerContext,
    world_id: u32,
    disaster_type: DisasterType,
    region_id: u32,
    current_hour: u64,
    warning_time: u32,
) -> Result<u32, String> {
    let warning_id = ctx.db.disaster_warning().iter().count() as u32 + 1;

    // Determine severity
    let severity = determine_disaster_severity(disaster_type);

    // Calculate impact time
    let impact_hour = current_hour + warning_time as u64;

    // Determine warning level
    let warning_level = match (severity, warning_time) {
        (EventSeverity::Catastrophic, _) => WarningLevel::Emergency,
        (EventSeverity::Major, _) => WarningLevel::Warning,
        (EventSeverity::Moderate, _) => WarningLevel::Advisory,
        (EventSeverity::Minor, _) => WarningLevel::Watch,
    };

    let warning = DisasterWarning {
        id: warning_id,
        world_id,
        disaster_type,
        affected_regions: format!("[{}]", region_id),
        warning_level,
        estimated_severity: severity,
        estimated_impact_hour: impact_hour,
        confidence: calculate_warning_confidence(disaster_type, warning_time),
        warning_issued_hour: current_hour,
        evacuation_recommended: matches!(severity, EventSeverity::Major | EventSeverity::Catastrophic),
        preparation_actions: generate_preparation_actions_json(disaster_type, severity),
        is_active: true,
    };

    ctx.db.disaster_warning().insert(warning);

    // Create narrative event for the warning
    let importance = match warning_level {
        WarningLevel::Watch => 3,
        WarningLevel::Advisory => 4,
        WarningLevel::Warning => 6,
        WarningLevel::Emergency => 7,
    };

    let title = format!("{:?} {:?} Issued", warning_level, disaster_type);
    let description = format!("Authorities have issued a {:?} for potential {:?} impact. Impact expected in {} hours.",
        warning_level, disaster_type, warning_time);

    create_narrative_event(
        ctx,
        world_id,
        1, // Default game ID
        EventCategory::Natural,
        title,
        description,
        importance,
    )?;

    log::info!("Issued {:?} warning for {:?} in region {} (ID: {})",
        warning_level, disaster_type, region_id, warning_id);

    Ok(warning_id)
}

// Determine disaster severity using probability distribution
fn determine_disaster_severity(disaster_type: DisasterType) -> EventSeverity {
    let mut rng = rand::thread_rng();
    let distribution_json = generate_severity_distribution_json(disaster_type);
    let distribution: serde_json::Value = serde_json::from_str(&distribution_json)
        .unwrap_or_else(|_| serde_json::json!({}));

    let random_value = rng.gen::<f32>();
    let mut cumulative = 0.0;

    let minor_prob = distribution.get("minor").and_then(|v| v.as_f64()).unwrap_or(0.5) as f32;
    cumulative += minor_prob;
    if random_value < cumulative {
        return EventSeverity::Minor;
    }

    let moderate_prob = distribution.get("moderate").and_then(|v| v.as_f64()).unwrap_or(0.3) as f32;
    cumulative += moderate_prob;
    if random_value < cumulative {
        return EventSeverity::Moderate;
    }

    let major_prob = distribution.get("major").and_then(|v| v.as_f64()).unwrap_or(0.15) as f32;
    cumulative += major_prob;
    if random_value < cumulative {
        return EventSeverity::Major;
    }

    EventSeverity::Catastrophic
}

// Calculate warning confidence based on disaster type and warning time
fn calculate_warning_confidence(disaster_type: DisasterType, warning_time: u32) -> f32 {
    let base_confidence = match disaster_type {
        DisasterType::Hurricane => 85.0,
        DisasterType::Blizzard => 90.0,
        DisasterType::Flood => 80.0,
        DisasterType::Wildfire => 70.0,
        DisasterType::Heatwave => 85.0,
        DisasterType::Volcano => 75.0,
        DisasterType::Tornado => 60.0,
        DisasterType::Earthquake => 0.0,  // No reliable prediction
        DisasterType::Meteor => 95.0,    // If detected, very accurate
        _ => 50.0,
    };

    // Confidence decreases with longer warning times
    let time_factor = if warning_time > 0 {
        (1.0 - (warning_time as f32 / 168.0).min(0.5)).max(0.3)
    } else {
        1.0
    };

    (base_confidence * time_factor).clamp(0.0, 100.0)
}

// Generate preparation actions JSON
fn generate_preparation_actions_json(disaster_type: DisasterType, severity: EventSeverity) -> String {
    let is_severe = matches!(severity, EventSeverity::Major | EventSeverity::Catastrophic);

    let actions = match disaster_type {
        DisasterType::Hurricane => {
            if is_severe {
                vec!["evacuate_coastal_areas", "secure_buildings", "emergency_supplies", "shelter_preparation"]
            } else {
                vec!["monitor_conditions", "secure_loose_objects", "check_emergency_kit"]
            }
        },
        DisasterType::Wildfire => {
            if is_severe {
                vec!["evacuate_fire_zones", "create_firebreaks", "wet_down_structures", "livestock_evacuation"]
            } else {
                vec!["clear_vegetation", "water_availability", "monitor_air_quality"]
            }
        },
        DisasterType::Flood => {
            if is_severe {
                vec!["evacuate_low_areas", "sandbagging", "move_valuables_up", "emergency_boat_prep"]
            } else {
                vec!["monitor_water_levels", "clear_drainage", "emergency_supplies"]
            }
        },
        DisasterType::Earthquake => {
            vec!["secure_heavy_objects", "identify_safe_spots", "emergency_kit_check", "evacuation_plan_review"]
        },
        DisasterType::Blizzard => {
            if is_severe {
                vec!["stock_food_water", "heating_fuel", "avoid_travel", "insulate_pipes"]
            } else {
                vec!["winter_supplies", "vehicle_preparation", "heating_check"]
            }
        },
        _ => vec!["monitor_conditions", "emergency_supplies", "evacuation_plan_review"],
    };

    serde_json::to_string(&actions).unwrap_or_else(|_| "[]".to_string())
}

// Execute disaster based on warning
#[spacetimedb::reducer]
pub fn execute_disaster_events(
    ctx: &ReducerContext,
    world_id: u32,
    current_hour: u64,
) -> Result<Vec<u32>, String> {
    let mut disaster_event_ids = Vec::new();

    // Find warnings that should trigger now
    let triggering_warnings: Vec<DisasterWarning> = ctx.db.disaster_warning()
        .iter()
        .filter(|w| w.world_id == world_id && w.is_active)
        .filter(|w| current_hour >= w.estimated_impact_hour)
        .cloned()
        .collect();

    for mut warning in triggering_warnings {
        // Create the actual disaster event
        let disaster_id = create_disaster_event(ctx, &warning, current_hour)?;
        disaster_event_ids.push(disaster_id);

        // Deactivate the warning
        warning.is_active = false;
        ctx.db.disaster_warning().id().update(warning.id, warning);

        // Initiate disaster response
        initiate_disaster_response(ctx, world_id, &warning, disaster_id, current_hour)?;
    }

    Ok(disaster_event_ids)
}

// Create actual disaster event
fn create_disaster_event(
    ctx: &ReducerContext,
    warning: &DisasterWarning,
    current_hour: u64,
) -> Result<u32, String> {
    let event_id = ctx.db.natural_event().iter().count() as u32 + 1;

    let natural_event_type = match warning.disaster_type {
        DisasterType::Earthquake => NaturalEventType::Earthquake,
        DisasterType::Wildfire => NaturalEventType::Fire,
        DisasterType::Flood => NaturalEventType::Flood,
        DisasterType::Drought => NaturalEventType::Drought,
        _ => NaturalEventType::WeatherChange, // Generic fallback
    };

    let (duration, economic_impact, population_impact) = calculate_disaster_impacts(warning.disaster_type, warning.estimated_severity);

    let natural_event = crate::natural::NaturalEvent {
        id: event_id,
        world_id: warning.world_id,
        event_type: natural_event_type,
        severity: warning.estimated_severity,
        affected_region: warning.affected_regions.clone(),
        start_hour: current_hour,
        duration_hours: duration,
        description: format!("{:?} {:?} strikes the region", warning.estimated_severity, warning.disaster_type),
        environmental_effects: generate_disaster_effects_json(warning.disaster_type, warning.estimated_severity),
        economic_impact,
        population_impact,
        resolved: false,
        resolution_description: String::new(),
    };

    ctx.db.natural_event().insert(natural_event);

    // Create high-importance narrative event
    let importance = match warning.estimated_severity {
        EventSeverity::Minor => 4,
        EventSeverity::Moderate => 5,
        EventSeverity::Major => 6,
        EventSeverity::Catastrophic => 7,
    };

    let title = format!("{:?} Disaster", warning.disaster_type);
    let description = format!("A {:?} {:?} has struck the region, causing significant impact",
        warning.estimated_severity, warning.disaster_type);

    create_narrative_event(
        ctx,
        warning.world_id,
        1, // Default game ID
        EventCategory::Natural,
        title,
        description,
        importance,
    )?;

    Ok(event_id)
}

// Calculate disaster impacts
fn calculate_disaster_impacts(disaster_type: DisasterType, severity: EventSeverity) -> (u32, f32, f32) {
    let severity_multiplier = match severity {
        EventSeverity::Minor => 1.0,
        EventSeverity::Moderate => 3.0,
        EventSeverity::Major => 8.0,
        EventSeverity::Catastrophic => 20.0,
    };

    let (base_duration, base_economic, base_population) = match disaster_type {
        DisasterType::Earthquake => (1, -0.5, -0.3),
        DisasterType::Hurricane => (24, -0.4, -0.2),
        DisasterType::Wildfire => (72, -0.3, -0.15),
        DisasterType::Flood => (168, -0.35, -0.25),
        DisasterType::Drought => (720, -0.6, -0.4),
        DisasterType::Volcano => (168, -0.8, -0.5),
        DisasterType::Tsunami => (12, -0.9, -0.7),
        DisasterType::Meteor => (1, -0.95, -0.8),
        _ => (24, -0.2, -0.1),
    };

    let duration = (base_duration as f32 * severity_multiplier.sqrt()) as u32;
    let economic_impact = base_economic * severity_multiplier;
    let population_impact = base_population * severity_multiplier;

    (duration, economic_impact, population_impact)
}

// Generate disaster-specific environmental effects
fn generate_disaster_effects_json(disaster_type: DisasterType, severity: EventSeverity) -> String {
    let severity_multiplier = match severity {
        EventSeverity::Minor => 1.0,
        EventSeverity::Moderate => 2.5,
        EventSeverity::Major => 6.0,
        EventSeverity::Catastrophic => 15.0,
    };

    let effects = match disaster_type {
        DisasterType::Earthquake => serde_json::json!({
            "infrastructure_damage": 0.8 * severity_multiplier,
            "ground_stability": -0.6 * severity_multiplier,
            "building_collapse": 0.3 * severity_multiplier,
            "aftershock_risk": 0.7 * severity_multiplier
        }),
        DisasterType::Wildfire => serde_json::json!({
            "air_quality": -0.9 * severity_multiplier,
            "vegetation_loss": 0.95 * severity_multiplier,
            "wildlife_displacement": 0.8 * severity_multiplier,
            "soil_damage": 0.4 * severity_multiplier
        }),
        DisasterType::Flood => serde_json::json!({
            "water_contamination": 0.7 * severity_multiplier,
            "infrastructure_damage": 0.6 * severity_multiplier,
            "agricultural_loss": 0.8 * severity_multiplier,
            "transportation_disruption": 0.9 * severity_multiplier
        }),
        DisasterType::Hurricane => serde_json::json!({
            "wind_damage": 0.9 * severity_multiplier,
            "flooding": 0.7 * severity_multiplier,
            "power_outages": 0.8 * severity_multiplier,
            "coastal_erosion": 0.5 * severity_multiplier
        }),
        _ => serde_json::json!({
            "general_destruction": 0.5 * severity_multiplier
        })
    };

    effects.to_string()
}

// Initiate disaster response
fn initiate_disaster_response(
    ctx: &ReducerContext,
    world_id: u32,
    warning: &DisasterWarning,
    disaster_event_id: u32,
    current_hour: u64,
) -> Result<u32, String> {
    let response_id = ctx.db.disaster_response().iter().count() as u32 + 1;

    // Parse affected regions
    let affected_regions: Vec<u32> = serde_json::from_str(&warning.affected_regions)
        .unwrap_or_else(|_| vec![]);

    if let Some(&region_id) = affected_regions.first() {
        let response_type = match warning.disaster_type {
            DisasterType::Wildfire => ResponseType::FireSuppression,
            DisasterType::Flood => ResponseType::FloodControl,
            DisasterType::Earthquake => ResponseType::SearchAndRescue,
            DisasterType::Hurricane => ResponseType::EmergencyServices,
            _ => ResponseType::Relief,
        };

        let disaster_response = DisasterResponse {
            id: response_id,
            world_id,
            region_id,
            disaster_event_id,
            response_type,
            resources_allocated: 100000.0, // Base resource allocation
            personnel_count: 50,
            effectiveness: 70.0, // Base effectiveness
            start_hour: current_hour,
            duration_hours: 72, // 3 days default response
            status: ResponseStatus::Active,
            results: "{}".to_string(),
        };

        ctx.db.disaster_response().insert(disaster_response);

        log::info!("Initiated {:?} response for disaster {} in region {}",
            response_type, disaster_event_id, region_id);
    }

    Ok(response_id)
}