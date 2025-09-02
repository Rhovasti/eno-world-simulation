use spacetimedb::{ReducerContext, Table};
use log;
use crate::tables::*;
use crate::types::*;
use crate::tables::individual::individual;
use crate::tables::building::building;
use crate::tables::city::city;
use crate::tables::events::{simulation_time, movement_event, work_event, social_event, building_event, city_event};
use crate::tables::individual::individual_achievement;
use crate::tables::city::city_achievement;

/// Generate a narrative summary for a specific hour
#[spacetimedb::reducer]
pub fn generate_hourly_narrative(ctx: &ReducerContext, hour: u64) -> Result<(), String> {
    let movements: Vec<MovementEvent> = ctx.db.movement_event().iter()
        .filter(|e| e.hour == hour)
        .collect();
    
    let work_events: Vec<WorkEvent> = ctx.db.work_event().iter()
        .filter(|e| e.hour == hour)
        .collect();
    
    let social_events: Vec<SocialEvent> = ctx.db.social_event().iter()
        .filter(|e| e.hour == hour)
        .collect();
    
    let mut narrative = format!("Hour {}: ", hour);
    
    // Movement summary
    if !movements.is_empty() {
        let food_seekers = movements.iter()
            .filter(|e| e.reason == FundamentalNeed::Consumption)
            .count();
        let rest_seekers = movements.iter()
            .filter(|e| e.reason == FundamentalNeed::Rest)
            .count();
        
        narrative.push_str(&format!("{} people moved locations. ", movements.len()));
        if food_seekers > 0 {
            narrative.push_str(&format!("{} sought food. ", food_seekers));
        }
        if rest_seekers > 0 {
            narrative.push_str(&format!("{} went home to rest. ", rest_seekers));
        }
    }
    
    // Work summary
    if !work_events.is_empty() {
        let total_production: f32 = work_events.iter()
            .map(|e| e.resources_produced)
            .sum();
        narrative.push_str(&format!("{} people worked, producing {} resources. ", 
            work_events.len(), total_production as i32));
    }
    
    // Social summary
    if !social_events.is_empty() {
        narrative.push_str(&format!("{} social interactions occurred. ", social_events.len()));
    }
    
    log::info!("{}", narrative);
    Ok(())
}

/// Get the story of a specific individual
#[spacetimedb::reducer]
pub fn get_individual_story(ctx: &ReducerContext, individual_id: u32, hours_back: u64) -> Result<(), String> {
    let individual = ctx.db.individual().id().find(&individual_id)
        .ok_or("Individual not found")?;
    
    let current_hour = ctx.db.simulation_time().id().find(&1)
        .ok_or("Simulation not initialized")?
        .current_hour;
    
    let start_hour = current_hour.saturating_sub(hours_back);
    
    // Get movements
    let movements: Vec<MovementEvent> = ctx.db.movement_event().iter()
        .filter(|e| e.individual_id == individual_id && e.hour >= start_hour)
        .collect();
    
    // Get work events
    let work_events: Vec<WorkEvent> = ctx.db.work_event().iter()
        .filter(|e| e.individual_id == individual_id && e.hour >= start_hour)
        .collect();
    
    // Get social events
    let social_events: Vec<SocialEvent> = ctx.db.social_event().iter()
        .filter(|e| (e.individual1_id == individual_id || e.individual2_id == individual_id) && e.hour >= start_hour)
        .collect();
    
    let mut story = format!("{}'s story over the last {} hours:\n", individual.name, hours_back);
    
    // Current status
    story.push_str(&format!("Current needs: Food {:.0}%, Rest {:.0}%, Stress {:.0}%\n", 
        individual.food_water, individual.rest, individual.stress));
    
    // Movement summary
    if !movements.is_empty() {
        story.push_str(&format!("Traveled to {} different locations\n", movements.len()));
    }
    
    // Work summary
    if !work_events.is_empty() {
        let total_hours: f32 = work_events.iter().map(|e| e.hours_worked).sum();
        let total_wage: f32 = work_events.iter().map(|e| e.wage_earned).sum();
        story.push_str(&format!("Worked {:.0} hours and earned ${:.0}\n", total_hours, total_wage));
    }
    
    // Social summary
    if !social_events.is_empty() {
        story.push_str(&format!("Had {} social interactions\n", social_events.len()));
    }
    
    // Achievements
    let achievements: Vec<IndividualAchievement> = ctx.db.individual_achievement().iter()
        .filter(|a| a.individual_id == individual_id && a.achieved_hour >= start_hour)
        .collect();
    
    if !achievements.is_empty() {
        story.push_str("Recent achievements:\n");
        for achievement in achievements {
            story.push_str(&format!("- {}\n", achievement.description));
        }
    }
    
    log::info!("{}", story); Ok(())
}

/// Get the story of a specific building
#[spacetimedb::reducer]
pub fn get_building_story(ctx: &ReducerContext, building_id: u32, hours_back: u64) -> Result<(), String> {
    let building = ctx.db.building().id().find(&building_id)
        .ok_or("Building not found")?;
    
    let current_hour = ctx.db.simulation_time().id().find(&1)
        .ok_or("Simulation not initialized")?
        .current_hour;
    
    let start_hour = current_hour.saturating_sub(hours_back);
    
    // Get all movements to/from this building
    let arrivals: Vec<MovementEvent> = ctx.db.movement_event().iter()
        .filter(|e| e.to_location_id == building_id && e.hour >= start_hour)
        .collect();
    
    let departures: Vec<MovementEvent> = ctx.db.movement_event().iter()
        .filter(|e| e.from_location_id == building_id && e.hour >= start_hour)
        .collect();
    
    // Get work events at this building
    let work_events: Vec<WorkEvent> = ctx.db.work_event().iter()
        .filter(|e| e.building_id == building_id && e.hour >= start_hour)
        .collect();
    
    // Get building events
    let building_events: Vec<BuildingEvent> = ctx.db.building_event().iter()
        .filter(|e| e.building_id == building_id && e.hour >= start_hour)
        .collect();
    
    let mut story = format!("{} - Activity Report (last {} hours):\n", building.name, hours_back);
    
    // Current status
    story.push_str(&format!("Type: {:?}\n", building.building_type));
    story.push_str(&format!("Occupancy: {}/{}\n", building.current_occupants, building.max_capacity));
    story.push_str(&format!("Maintenance: {:.0}%, Cleanliness: {:.0}%\n", 
        building.maintenance, building.cleanliness));
    
    // Traffic
    story.push_str(&format!("\nVisitor traffic: {} arrivals, {} departures\n", 
        arrivals.len(), departures.len()));
    
    // Work activity
    if !work_events.is_empty() {
        let total_production: f32 = work_events.iter()
            .map(|e| e.resources_produced)
            .sum();
        story.push_str(&format!("Production: {:.0} resources produced\n", total_production));
    }
    
    // Notable events
    if !building_events.is_empty() {
        story.push_str("\nNotable events:\n");
        for event in building_events {
            story.push_str(&format!("- {}\n", event.description));
        }
    }
    
    log::info!("{}", story); Ok(())
}

/// Get city-wide summary
#[spacetimedb::reducer]
pub fn get_city_summary(ctx: &ReducerContext, city_id: u32) -> Result<(), String> {
    let city = ctx.db.city().id().find(&city_id)
        .ok_or("City not found")?;
    
    let mut summary = format!("{} - City Report\n", city.name);
    summary.push_str(&format!("Population: {}\n", city.population));
    summary.push_str(&format!("Days since founding: {}\n\n", 
        (ctx.db.simulation_time().id().find(&1).unwrap().current_hour - city.founded_hour) / 24));
    
    // Economic summary
    summary.push_str("ECONOMY:\n");
    summary.push_str(&format!("- Tax Reserve: ${:.0}\n", city.tax_reserve));
    summary.push_str(&format!("- Unemployment: {:.1}%\n", city.unemployment_rate));
    summary.push_str(&format!("- Import/Export: {:.0}/{:.0} per hour\n\n", 
        city.import_rate, city.export_rate));
    
    // Social summary
    summary.push_str("SOCIAL:\n");
    summary.push_str(&format!("- Stability: {:.0}%\n", city.stability));
    summary.push_str(&format!("- Average Health: {:.0}%\n", city.health));
    summary.push_str(&format!("- Safety: {:.0}%\n", city.safety));
    summary.push_str(&format!("- Happiness: {:.0}%\n\n", city.average_happiness));
    
    // Development summary
    summary.push_str("DEVELOPMENT:\n");
    summary.push_str(&format!("- Culture Points: {:.0}\n", city.culture));
    summary.push_str(&format!("- Science Points: {:.0}\n", city.science));
    summary.push_str(&format!("- Prestige: {:.0}\n", city.prestige));
    
    // Recent achievements
    let recent_achievements: Vec<CityAchievement> = ctx.db.city_achievement().iter()
        .filter(|a| a.city_id == city_id)
        .take(3)
        .collect();
    
    if !recent_achievements.is_empty() {
        summary.push_str("\nRECENT ACHIEVEMENTS:\n");
        for achievement in recent_achievements {
            summary.push_str(&format!("- {}\n", achievement.description));
        }
    }
    
    log::info!("{}", summary); Ok(())
}

/// Log a narrative event
#[spacetimedb::reducer]
pub fn log_narrative_event(
    ctx: &ReducerContext,
    description: String,
    event_type: EventType,
) -> Result<(), String> {
    let current_hour = ctx.db.simulation_time().id().find(&1)
        .ok_or("Simulation not initialized")?
        .current_hour;
    
    log::info!("[Hour {}] {}: {}", current_hour, 
        match event_type {
            EventType::Movement => "MOVE",
            EventType::NeedFulfilled => "NEED",
            EventType::WorkCompleted => "WORK",
            EventType::SocialInteraction => "SOCIAL",
            EventType::Achievement => "ACHIEVE",
            EventType::BuildingUpgrade => "UPGRADE",
            EventType::CityMilestone => "MILESTONE",
            EventType::Emergency => "EMERGENCY",
        },
        description
    );
    
    Ok(())
}