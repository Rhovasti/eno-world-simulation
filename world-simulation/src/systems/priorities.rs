use crate::tables::*;
use crate::types::*;

/// Calculate distance between two locations
pub fn calculate_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

/// Calculate travel time based on distance (1 hour per 10 units)
pub fn calculate_travel_time(distance: f32) -> u64 {
    ((distance / 10.0).ceil() as u64).max(1)
}

/// Find the best location for fulfilling a specific need
pub fn find_best_location_for_need(
    individual: &Individual,
    need: &FundamentalNeed,
    buildings: &[Building],
    locations: &[LocationCapability],
    current_x: f32,
    current_y: f32,
) -> Option<(u32, f32)> { // Returns (building_id, score)
    let mut candidates = Vec::new();
    
    for (building, location) in buildings.iter().zip(locations.iter()) {
        // Check if building has capacity
        if building.current_occupants >= building.max_capacity {
            continue;
        }
        
        // Check if location can fulfill the need
        let can_fulfill = match need {
            FundamentalNeed::Environment => {
                location.environmental_quality > 0.0 || 
                location.provides_healthcare ||
                (individual.home_id == Some(building.id) && location.provides_rest)
            },
            FundamentalNeed::Consumption => location.provides_food,
            FundamentalNeed::Connection => location.provides_social || location.provides_culture,
            FundamentalNeed::Rest => location.provides_rest && individual.home_id == Some(building.id),
            FundamentalNeed::Waste => location.provides_facilities,
        };
        
        if !can_fulfill {
            continue;
        }
        
        // Calculate score based on distance and quality
        let distance = calculate_distance(current_x, current_y, building.location_x, building.location_y);
        let travel_time = calculate_travel_time(distance);
        
        // Base score from location quality and building prestige
        let quality_score = location.environmental_quality + (building.prestige_level as f32 * 0.2);
        
        // Distance penalty (closer is better)
        let distance_penalty = distance * 0.1;
        
        // Special bonuses
        let home_bonus = if individual.home_id == Some(building.id) { 2.0 } else { 0.0 };
        let work_bonus = if individual.workplace_id == Some(building.id) { 1.0 } else { 0.0 };
        
        let total_score = quality_score + home_bonus + work_bonus - distance_penalty;
        
        candidates.push((building.id, total_score, travel_time));
    }
    
    // Return the best scoring location
    candidates.into_iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .map(|(id, score, _)| (id, score))
}

/// Determine what action to take at a location
pub fn determine_action_for_need(
    individual: &Individual,
    need: &FundamentalNeed,
    building_id: u32,
) -> Option<IndividualAction> {
    match need {
        FundamentalNeed::Environment => {
            if individual.home_id == Some(building_id) {
                Some(IndividualAction::Sleep)
            } else {
                None // Just being in a good environment helps
            }
        },
        FundamentalNeed::Consumption => Some(IndividualAction::Eat),
        FundamentalNeed::Connection => Some(IndividualAction::Socialize),
        FundamentalNeed::Rest => Some(IndividualAction::Sleep),
        FundamentalNeed::Waste => Some(IndividualAction::UseFacilities),
    }
}

/// Check if individual can afford an action
pub fn can_afford_action(individual: &Individual, action: &IndividualAction) -> bool {
    match action {
        IndividualAction::Eat => individual.income >= 5.0, // Cost of a meal
        IndividualAction::PayRent => individual.income >= 10.0, // Minimum rent payment
        _ => true, // Most actions are free
    }
}

/// Calculate productivity based on individual's needs
pub fn calculate_productivity(individual: &Individual) -> f32 {
    // Base productivity
    let mut productivity: f32 = 1.0;
    
    // Level 1 needs affect productivity
    if individual.food_water < 30.0 {
        productivity *= 0.5;
    }
    if individual.rest < 30.0 {
        productivity *= 0.6;
    }
    if individual.environment < 40.0 {
        productivity *= 0.8;
    }
    
    // Level 2 needs affect productivity
    if individual.is_need_level_active(2) {
        if individual.stress > 70.0 {
            productivity *= 0.7;
        }
        if individual.safety < 40.0 {
            productivity *= 0.8;
        }
    }
    
    // Higher level needs provide bonuses
    if individual.is_need_level_active(3) && individual.community > 20.0 {
        productivity *= 1.1;
    }
    
    if individual.is_need_level_active(4) && individual.achievements > 60.0 {
        productivity *= 1.2;
    }
    
    if individual.is_need_level_active(5) && individual.progression > 50.0 {
        productivity *= 1.3;
    }
    
    productivity.clamp(0.1, 2.0)
}

/// Calculate building efficiency based on workers and maintenance
pub fn calculate_building_efficiency(
    building: &Building,
    worker_count: u32,
    average_worker_productivity: f32,
) -> f32 {
    let base_efficiency = 1.0 + (building.efficiency_level as f32 * 0.2);
    
    // Maintenance affects efficiency
    let maintenance_factor = building.maintenance / 100.0;
    
    // Worker productivity affects efficiency
    let worker_factor = average_worker_productivity;
    
    // Occupancy rate affects efficiency
    let occupancy_rate = worker_count as f32 / building.max_capacity as f32;
    let occupancy_factor = if occupancy_rate > 0.8 {
        0.9 // Overcrowding reduces efficiency
    } else if occupancy_rate < 0.2 {
        0.7 // Too few workers reduces efficiency
    } else {
        1.0
    };
    
    base_efficiency * maintenance_factor * worker_factor * occupancy_factor
}