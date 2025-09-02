use spacetimedb::{ReducerContext, Table};
use crate::tables::*;
use crate::types::*;
use crate::systems::*;
// Import table traits for SpacetimeDB 1.1.2
use crate::tables::individual::individual;
use crate::tables::building::building;
use crate::tables::events::{simulation_time, movement_event, need_fulfillment_event, work_event};
use crate::tables::individual::employment;

/// Create a new individual
#[spacetimedb::reducer]
pub fn create_individual(
    ctx: &ReducerContext,
    name: String,
    home_id: Option<u32>,
    workplace_id: Option<u32>,
) -> Result<(), String> {
    let current_hour = ctx.db.simulation_time().id().find(&1)
        .ok_or("Simulation not initialized")?
        .current_hour;
    
    // Generate ID (in production, use proper ID generation)
    let id = (ctx.db.individual().iter().count() + 1) as u32;
    
    // Get starting location
    let location_id = home_id.unwrap_or(1); // Default to location 1 if no home
    
    let individual = Individual {
        id,
        name,
        age: 25, // Default starting age
        current_location_id: location_id,
        home_id,
        workplace_id,
        
        // Start with moderate needs
        food_water: 70.0,
        environment: 80.0,
        intimacy: 50.0,
        rest: 80.0,
        waste: 20.0,
        
        // Level 2 needs
        threat: 20.0,
        income: 50.0,
        stress: 30.0,
        safety: 70.0,
        
        // Level 3 needs
        relationship: 0.0,
        social_interaction: 0.0,
        community: 20.0,
        
        // Level 4-5
        achievements: 0.0,
        progression: 0.0,
        specialized_role: SpecializedRole::None,
        
        status: IndividualStatus::Idle,
        last_update_hour: current_hour,
        birth_hour: current_hour,
    };
    
    ctx.db.individual().insert(individual);
    
    // If has workplace, create employment record
    if let Some(workplace_id) = workplace_id {
        let employment_id = (ctx.db.employment().iter().count() + 1) as u32;
        ctx.db.employment().insert(Employment {
            id: employment_id,
            individual_id: id,
            building_id: workplace_id,
            job_type: JobType::Office,
            wage: 5.0,
            started_hour: current_hour,
            ended_hour: None,
            is_active: true,
        });
    }
    
    Ok(())
}

/// Update individual needs based on time and status
#[spacetimedb::reducer]
pub fn update_individual_needs(ctx: &ReducerContext, individual_id: u32) -> Result<(), String> {
    let current_hour = ctx.db.simulation_time().id().find(&1)
        .ok_or("Simulation not initialized")?
        .current_hour;
    
    let mut individual = ctx.db.individual().id().find(&individual_id)
        .ok_or("Individual not found")?;
    
    // Calculate hours passed
    let hours_passed = current_hour - individual.last_update_hour;
    if hours_passed == 0 {
        return Ok(());
    }
    
    // Get current location capabilities
    let location = get_location_for_building(ctx, individual.current_location_id)?;
    
    // Update needs based on time and location
    individual.update_needs(hours_passed, &location);
    
    // Check if any status has expired
    match &individual.status {
        IndividualStatus::Working(status_data) |
        IndividualStatus::Sleeping(status_data) |
        IndividualStatus::Eating(status_data) |
        IndividualStatus::Socializing(status_data) |
        IndividualStatus::Maintaining(status_data) |
        IndividualStatus::UsingFacilities(status_data) => {
            if status_data.until_hour <= current_hour {
                individual.status = IndividualStatus::Idle;
            }
        },
        IndividualStatus::InTransit(status_data) => {
            if status_data.until_hour <= current_hour {
                if let Some(target_location) = status_data.target_location {
                    individual.current_location_id = target_location;
                    individual.status = IndividualStatus::Idle;
                    
                    // Log movement
                    log_movement(ctx, individual_id, individual.current_location_id, target_location, current_hour);
                }
            }
        },
        _ => {},
    }
    
    // If idle, check for pressing needs
    if matches!(individual.status, IndividualStatus::Idle) {
        if let Some((need, _priority)) = individual.get_most_pressing_need() {
            handle_pressing_need(ctx, &mut individual, need, current_hour)?;
        }
    }
    
    individual.last_update_hour = current_hour;
    ctx.db.individual().id().update(individual);
    
    Ok(())
}

/// Handle a pressing need by finding a location and taking action
fn handle_pressing_need(
    ctx: &ReducerContext,
    individual: &mut Individual,
    need: FundamentalNeed,
    current_hour: u64,
) -> Result<(), String> {
    // Get all buildings and their locations
    let buildings: Vec<Building> = ctx.db.building().iter().collect();
    let locations: Vec<LocationCapability> = ctx.db.location_capability().iter().collect();
    
    // Find current building location
    let current_building = buildings.iter()
        .find(|b| b.id == individual.current_location_id)
        .ok_or("Current building not found")?;
    
    // Find best location for need
    if let Some((target_building_id, _score)) = find_best_location_for_need(
        individual,
        &need,
        &buildings,
        &locations,
        current_building.location_x,
        current_building.location_y,
    ) {
        // If at target location, perform action
        if target_building_id == individual.current_location_id {
            if let Some(action) = determine_action_for_need(individual, &need, target_building_id) {
                perform_action(ctx, individual, action, current_hour)?;
            }
        } else {
            // Need to move to target location
            let target_building = buildings.iter()
                .find(|b| b.id == target_building_id)
                .unwrap();
            
            let distance = calculate_distance(
                current_building.location_x,
                current_building.location_y,
                target_building.location_x,
                target_building.location_y,
            );
            
            let travel_time = calculate_travel_time(distance);
            
            individual.status = IndividualStatus::InTransit(StatusData {
                until_hour: current_hour + travel_time,
                target_location: Some(target_building_id),
                target_building: None,
            });
            
            // Apply movement costs
            individual.rest += actions::MOVE_REST_COST * travel_time as f32;
        }
    }
    
    Ok(())
}

/// Perform an action
fn perform_action(
    ctx: &ReducerContext,
    individual: &mut Individual,
    action: IndividualAction,
    current_hour: u64,
) -> Result<(), String> {
    match action {
        IndividualAction::Work => {
            individual.status = IndividualStatus::Working(StatusData {
                until_hour: current_hour + actions::WORK_DURATION,
                target_location: None,
                target_building: individual.workplace_id,
            });
            individual.rest += actions::WORK_REST_COST;
            individual.stress += actions::WORK_STRESS_GAIN;
            individual.income += actions::WORK_INCOME_GAIN;
            
            // Log work event
            if let Some(workplace_id) = individual.workplace_id {
                log_work_event(ctx, individual.id, workplace_id, current_hour, actions::WORK_DURATION as f32);
            }
        },
        IndividualAction::Sleep => {
            individual.status = IndividualStatus::Sleeping(StatusData {
                until_hour: current_hour + actions::SLEEP_DURATION,
                target_location: None,
                target_building: individual.home_id,
            });
            individual.rest += actions::SLEEP_REST_GAIN;
        },
        IndividualAction::Eat => {
            individual.status = IndividualStatus::Eating(StatusData {
                until_hour: current_hour + actions::EAT_DURATION,
                target_location: None,
                target_building: None,
            });
            individual.food_water += actions::EAT_FOOD_GAIN;
            individual.income -= 5.0; // Cost of meal
        },
        IndividualAction::Socialize => {
            individual.status = IndividualStatus::Socializing(StatusData {
                until_hour: current_hour + actions::SOCIALIZE_DURATION,
                target_location: None,
                target_building: None,
            });
            individual.social_interaction = (individual.social_interaction + actions::SOCIALIZE_SOCIAL_GAIN)
                .min(33.3);
            individual.stress += actions::SOCIALIZE_STRESS_LOSS;
        },
        IndividualAction::UseFacilities => {
            individual.status = IndividualStatus::UsingFacilities(StatusData {
                until_hour: current_hour + 1,
                target_location: None,
                target_building: Some(individual.current_location_id),
            });
            individual.waste += individual_depletion::WASTE_FACILITIES;
        },
        IndividualAction::MaintainBuilding => {
            individual.status = IndividualStatus::Maintaining(StatusData {
                until_hour: current_hour + actions::MAINTAIN_DURATION,
                target_location: None,
                target_building: Some(individual.current_location_id),
            });
            // Building maintenance will be updated when status completes
        },
        IndividualAction::CleanBuilding => {
            individual.status = IndividualStatus::Maintaining(StatusData {
                until_hour: current_hour + actions::CLEAN_DURATION,
                target_location: None,
                target_building: Some(individual.current_location_id),
            });
            // Building cleanliness will be updated when status completes
        },
        IndividualAction::PayRent => {
            if let Some(home_id) = individual.home_id {
                pay_rent(ctx, individual.id, home_id)?;
            }
        },
        _ => {}, // Other actions not implemented yet
    }
    
    // Log need fulfillment
    log_need_fulfillment(ctx, individual.id, individual.current_location_id, current_hour, need_from_action(&action));
    
    Ok(())
}

/// Helper functions
fn get_location_for_building(ctx: &ReducerContext, building_id: u32) -> Result<LocationCapability, String> {
    ctx.db.location_capability().iter()
        .find(|l| l.building_id == building_id)
        .ok_or("Location capability not found".to_string())
}

fn need_from_action(action: &IndividualAction) -> FundamentalNeed {
    match action {
        IndividualAction::Eat => FundamentalNeed::Consumption,
        IndividualAction::Sleep | IndividualAction::Work => FundamentalNeed::Rest,
        IndividualAction::Socialize => FundamentalNeed::Connection,
        IndividualAction::UseFacilities => FundamentalNeed::Waste,
        _ => FundamentalNeed::Environment,
    }
}

fn log_movement(ctx: &ReducerContext, individual_id: u32, from: u32, to: u32, hour: u64) {
    let id = (ctx.db.movement_event().iter().count() + 1) as u32;
    ctx.db.movement_event().insert(MovementEvent {
        id,
        individual_id,
        from_location_id: from,
        to_location_id: to,
        hour,
        reason: FundamentalNeed::Environment, // Simplified
        travel_time: 1,
    });
}

fn log_need_fulfillment(ctx: &ReducerContext, individual_id: u32, location_id: u32, hour: u64, need: FundamentalNeed) {
    let id = (ctx.db.need_fulfillment_event().iter().count() + 1) as u32;
    ctx.db.need_fulfillment_event().insert(NeedFulfillmentEvent {
        id,
        individual_id,
        location_id,
        hour,
        need_type: need,
        amount_fulfilled: 20.0, // Simplified
        action_taken: IndividualAction::Work, // Simplified
    });
}

fn log_work_event(ctx: &ReducerContext, individual_id: u32, building_id: u32, hour: u64, hours: f32) {
    let id = (ctx.db.work_event().iter().count() + 1) as u32;
    ctx.db.work_event().insert(WorkEvent {
        id,
        individual_id,
        building_id,
        hour,
        hours_worked: hours,
        wage_earned: hours * 5.0,
        productivity: 1.0,
        resources_consumed: 5.0,
        resources_produced: 10.0,
    });
}


fn pay_rent(ctx: &ReducerContext, individual_id: u32, home_id: u32) -> Result<(), String> {
    // Implementation simplified for now
    Ok(())
}