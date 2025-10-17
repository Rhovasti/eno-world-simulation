use spacetimedb::{spacetimedb, ReducerContext, Table};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NeedType {
    Food,
    Sleep,
    Work,
    Social,
    Entertainment,
    Shopping,
    Medical,
    Education,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatusModifier {
    AtWork { until_hour: u64 },
    Sleeping { until_hour: u64 },
    Traveling { to_location: u32, arrives_hour: u64 },
    InMeeting { until_hour: u64 },
    Hospitalized { until_hour: u64 },
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LocationCapability {
    ProvidesFood,
    ProvidesWork,
    ProvidesEntertainment,
    ProvidesShopping,
    ProvidesHousing,
    ProvidesMedical,
    ProvidesEducation,
    ProvidesSocial,
}

#[spacetimedb(table)]
pub struct Entity {
    #[primarykey]
    pub id: u32,
    pub current_location_id: u32,
    pub home_location_id: u32,
    pub work_location_id: Option<u32>,
    
    // Need levels (0-100, higher = more urgent)
    pub food_need: u8,
    pub sleep_need: u8,
    pub work_need: u8,
    pub social_need: u8,
    pub entertainment_need: u8,
    pub shopping_need: u8,
    pub medical_need: u8,
    pub education_need: u8,
    
    // Status that might override need-based movement
    pub status: StatusModifier,
    pub last_update_hour: u64,
}

#[spacetimedb(table)]
pub struct Location {
    #[primarykey]
    pub id: u32,
    pub name: String,
    pub location_type: String,
    pub parent_location_id: Option<u32>, // For buildings in cities
    pub capabilities: Vec<LocationCapability>,
    pub capacity: u32,
    pub current_occupancy: u32,
}

#[spacetimedb(table)]
pub struct MovementEvent {
    #[primarykey]
    pub id: u32,
    pub entity_id: u32,
    pub from_location_id: u32,
    pub to_location_id: u32,
    pub hour: u64,
    pub reason: NeedType,
    pub need_satisfaction: u8, // How much the need was reduced
}

impl Entity {
    // Find the most pressing need that isn't being overridden
    pub fn get_most_pressing_need(&self, current_hour: u64) -> Option<NeedType> {
        // Check if status modifier prevents movement
        match &self.status {
            StatusModifier::AtWork { until_hour } if *until_hour > current_hour => return None,
            StatusModifier::Sleeping { until_hour } if *until_hour > current_hour => return None,
            StatusModifier::Traveling { arrives_hour, .. } if *arrives_hour > current_hour => return None,
            StatusModifier::InMeeting { until_hour } if *until_hour > current_hour => return None,
            StatusModifier::Hospitalized { until_hour } if *until_hour > current_hour => return None,
            _ => {}
        }

        // Find highest need level
        let needs = vec![
            (NeedType::Medical, self.medical_need),
            (NeedType::Food, self.food_need),
            (NeedType::Sleep, self.sleep_need),
            (NeedType::Work, self.work_need),
            (NeedType::Social, self.social_need),
            (NeedType::Entertainment, self.entertainment_need),
            (NeedType::Shopping, self.shopping_need),
            (NeedType::Education, self.education_need),
        ];

        needs.into_iter()
            .filter(|(_, level)| *level > 60) // Only act on urgent needs
            .max_by_key(|(_, level)| *level)
            .map(|(need_type, _)| need_type)
    }

    // Update need levels based on time passage and current activity
    pub fn update_needs(&mut self, hours_passed: u64) {
        // Basic need decay over time
        self.food_need = (self.food_need + (hours_passed * 8) as u8).min(100);
        self.sleep_need = (self.sleep_need + (hours_passed * 6) as u8).min(100);
        self.social_need = (self.social_need + (hours_passed * 3) as u8).min(100);
        self.entertainment_need = (self.entertainment_need + (hours_passed * 4) as u8).min(100);
        self.shopping_need = (self.shopping_need + (hours_passed * 2) as u8).min(100);
        self.education_need = (self.education_need + (hours_passed * 1) as u8).min(100);

        // Work need increases during work hours, decreases otherwise
        let is_work_hours = (hours_passed % 24) >= 8 && (hours_passed % 24) <= 17;
        if is_work_hours && self.work_location_id.is_some() {
            self.work_need = (self.work_need + (hours_passed * 10) as u8).min(100);
        } else {
            self.work_need = self.work_need.saturating_sub((hours_passed * 5) as u8);
        }

        // Medical need is usually low but can spike randomly
        if rand::random::<f32>() < 0.001 { // 0.1% chance per hour
            self.medical_need = (self.medical_need + 50).min(100);
        }
    }

    // Satisfy a need and update gauges
    pub fn satisfy_need(&mut self, need_type: &NeedType, satisfaction_amount: u8) -> u8 {
        let actual_satisfaction = match need_type {
            NeedType::Food => {
                let old_level = self.food_need;
                self.food_need = self.food_need.saturating_sub(satisfaction_amount);
                old_level - self.food_need
            },
            NeedType::Sleep => {
                let old_level = self.sleep_need;
                self.sleep_need = self.sleep_need.saturating_sub(satisfaction_amount);
                old_level - self.sleep_need
            },
            NeedType::Work => {
                let old_level = self.work_need;
                self.work_need = self.work_need.saturating_sub(satisfaction_amount);
                old_level - self.work_need
            },
            NeedType::Social => {
                let old_level = self.social_need;
                self.social_need = self.social_need.saturating_sub(satisfaction_amount);
                old_level - self.social_need
            },
            NeedType::Entertainment => {
                let old_level = self.entertainment_need;
                self.entertainment_need = self.entertainment_need.saturating_sub(satisfaction_amount);
                old_level - self.entertainment_need
            },
            NeedType::Shopping => {
                let old_level = self.shopping_need;
                self.shopping_need = self.shopping_need.saturating_sub(satisfaction_amount);
                old_level - self.shopping_need
            },
            NeedType::Medical => {
                let old_level = self.medical_need;
                self.medical_need = self.medical_need.saturating_sub(satisfaction_amount);
                old_level - self.medical_need
            },
            NeedType::Education => {
                let old_level = self.education_need;
                self.education_need = self.education_need.saturating_sub(satisfaction_amount);
                old_level - self.education_need
            },
        };
        actual_satisfaction
    }
}

// Find locations that can satisfy a specific need
pub fn find_locations_for_need(need_type: &NeedType) -> Vec<Location> {
    let required_capability = match need_type {
        NeedType::Food => LocationCapability::ProvidesFood,
        NeedType::Sleep => LocationCapability::ProvidesHousing,
        NeedType::Work => LocationCapability::ProvidesWork,
        NeedType::Social => LocationCapability::ProvidesSocial,
        NeedType::Entertainment => LocationCapability::ProvidesEntertainment,
        NeedType::Shopping => LocationCapability::ProvidesShopping,
        NeedType::Medical => LocationCapability::ProvidesMedical,
        NeedType::Education => LocationCapability::ProvidesEducation,
    };

    Location::iter()
        .filter(|loc| {
            loc.capabilities.contains(&required_capability) && 
            loc.current_occupancy < loc.capacity
        })
        .collect()
}

#[spacetimedb(reducer)]
pub fn hourly_simulation_update() {
    let current_hour = get_current_hour();
    
    // Process all entities
    let mut movement_events = Vec::new();
    
    for entity in Entity::iter() {
        if entity.last_update_hour >= current_hour {
            continue; // Already processed this hour
        }
        
        let hours_passed = current_hour - entity.last_update_hour;
        let mut updated_entity = entity.clone();
        
        // Update needs based on time passage
        updated_entity.update_needs(hours_passed);
        
        // Check if entity can move (not blocked by status)
        if let Some(pressing_need) = updated_entity.get_most_pressing_need(current_hour) {
            // Find suitable locations for this need
            let suitable_locations = find_locations_for_need(&pressing_need);
            
            if !suitable_locations.is_empty() {
                // Choose closest or best location (simplified: pick first available)
                let target_location = &suitable_locations[0];
                
                // Move entity
                let old_location = updated_entity.current_location_id;
                updated_entity.current_location_id = target_location.id;
                
                // Satisfy the need
                let satisfaction = 40; // Base satisfaction amount
                let actual_satisfaction = updated_entity.satisfy_need(&pressing_need, satisfaction);
                
                // Update location occupancy
                Location::update_by_id(old_location, |loc| loc.current_occupancy -= 1);
                Location::update_by_id(target_location.id, |loc| loc.current_occupancy += 1);
                
                // Record movement event
                let movement_event = MovementEvent {
                    id: generate_movement_id(),
                    entity_id: entity.id,
                    from_location_id: old_location,
                    to_location_id: target_location.id,
                    hour: current_hour,
                    reason: pressing_need,
                    need_satisfaction: actual_satisfaction,
                };
                movement_events.push(movement_event);
                
                // Set appropriate status modifier
                updated_entity.status = match pressing_need {
                    NeedType::Work => StatusModifier::AtWork { until_hour: current_hour + 8 },
                    NeedType::Sleep => StatusModifier::Sleeping { until_hour: current_hour + 8 },
                    NeedType::Medical => StatusModifier::InMeeting { until_hour: current_hour + 2 },
                    _ => StatusModifier::None,
                };
            }
        }
        
        updated_entity.last_update_hour = current_hour;
        Entity::update_by_id(entity.id, |e| *e = updated_entity);
    }
    
    // Insert all movement events
    for event in movement_events {
        MovementEvent::insert(event);
    }
    
    // Generate narrative summaries for clients
    generate_hourly_narrative(current_hour);
}

#[spacetimedb(reducer)]
pub fn generate_hourly_narrative(hour: u64) {
    // Query recent movements for interesting patterns
    let recent_movements: Vec<MovementEvent> = MovementEvent::iter()
        .filter(|event| event.hour == hour)
        .collect();
    
    // Example narrative generation logic
    let food_seekers = recent_movements.iter()
        .filter(|event| event.reason == NeedType::Food)
        .count();
    
    let workers = recent_movements.iter()
        .filter(|event| event.reason == NeedType::Work)
        .count();
    
    // Clients could subscribe to these narrative events
    println!("Hour {}: {} entities sought food, {} went to work", 
             hour, food_seekers, workers);
}

// Helper functions (would need proper implementation)
fn get_current_hour() -> u64 {
    // Implementation to get current simulation hour
    0
}

fn generate_movement_id() -> u32 {
    // Generate unique ID for movement events
    0
}

// Example of how clients might query for narratives
#[spacetimedb(reducer)]
pub fn get_location_story(location_id: u32, hours_back: u64) -> String {
    let current_hour = get_current_hour();
    let start_hour = current_hour.saturating_sub(hours_back);
    
    let movements: Vec<MovementEvent> = MovementEvent::iter()
        .filter(|event| {
            (event.to_location_id == location_id || event.from_location_id == location_id) &&
            event.hour >= start_hour
        })
        .collect();
    
    let arrivals = movements.iter().filter(|e| e.to_location_id == location_id).count();
    let departures = movements.iter().filter(|e| e.from_location_id == location_id).count();
    
    format!("In the last {} hours, {} entities arrived and {} departed", 
            hours_back, arrivals, departures)
}