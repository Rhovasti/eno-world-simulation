use spacetimedb::{ReducerContext, Table};
use log;
use crate::tables::*;
use crate::types::{BuildingType, ResourceType};
use crate::systems::modifiers::*;
use crate::tables::building::{building, home_data, workplace_data, location_capability};
use crate::tables::city::city;

/// Create a new building
#[spacetimedb::reducer]
pub fn create_building(
    ctx: &ReducerContext,
    name: String,
    city_id: u32,
    building_type: BuildingType,
    x: f32,
    y: f32,
) -> Result<(), String> {
    let id = (ctx.db.building().iter().count() + 1) as u32;
    
    let (max_capacity, base_cost) = match &building_type {
        BuildingType::Home(home_data) => (home_data.capacity, 100.0),
        BuildingType::Workplace(workplace_data) => (workplace_data.positions, 500.0),
        _ => (20, 200.0),
    };
    
    let building = Building {
        id,
        name: name.clone(),
        city_id,
        building_type: building_type.clone(),
        location_x: x,
        location_y: y,
        maintenance: 100.0,
        cleanliness: 100.0,
        efficiency_level: 1,
        prestige_level: 1,
        current_occupants: 0,
        max_capacity,
        operating_cost: base_cost,
        revenue: 0.0,
        last_payment_hour: 0,
    };
    
    ctx.db.building().insert(building);
    
    // Create location capabilities
    create_location_capabilities(ctx, id, &building_type)?;
    
    // Create type-specific data
    match building_type {
        BuildingType::Home(home_data) => {
            ctx.db.home_data().insert(HomeData {
                building_id: id,
                rent_amount: home_data.rent,
                rent_paid: 0.0,
                utilities_quality: 80.0,
            });
        },
        BuildingType::Workplace(workplace_data) => {
            ctx.db.workplace_data().insert(WorkplaceData {
                building_id: id,
                resource_type: ResourceType::Goods,
                consumption_rate: building_depletion::RESOURCE_CONSUMPTION_BASE,
                production_rate: building_depletion::RESOURCE_PRODUCTION_BASE,
                inventory: 0.0,
                stockpile: 100.0,
                max_inventory: 1000.0,
                max_stockpile: 1000.0,
                base_wage: 5.0,
            });
        },
        _ => {},
    }
    
    log::info!("Created building {} with ID {}", name, id);
    Ok(())
}

/// Daily building update
#[spacetimedb::reducer]
pub fn update_building_daily(ctx: &ReducerContext, building_id: u32) -> Result<(), String> {
    let mut building = ctx.db.building().id().find(&building_id)
        .ok_or("Building not found")?;
    
    let city = ctx.db.city().id().find(&building.city_id)
        .ok_or("City not found")?;
    
    // Update maintenance
    let maintenance_decay = building_depletion::MAINTENANCE_BASE + 
        (building.current_occupants as f32 * building_depletion::MAINTENANCE_PER_OCCUPANT);
    
    // Poor city infrastructure increases decay
    if city.public_works < 30.0 {
        building.maintenance += building_depletion::MAINTENANCE_POOR_INFRASTRUCTURE;
    }
    
    building.maintenance = (building.maintenance + maintenance_decay).clamp(0.0, 100.0);
    
    // Update cleanliness
    let cleanliness_decay = building_depletion::CLEANLINESS_BASE + 
        (building.current_occupants as f32 * building_depletion::CLEANLINESS_PER_OCCUPANT);
    
    building.cleanliness = (building.cleanliness + cleanliness_decay).clamp(0.0, 100.0);
    
    // Process type-specific updates
    match &building.building_type {
        BuildingType::Home(_) => update_home_daily(ctx, &mut building)?,
        BuildingType::Workplace(_) => update_workplace_daily(ctx, &mut building)?,
        _ => {},
    }
    
    ctx.db.building().id().update(building);
    
    Ok(())
}

fn update_home_daily(ctx: &ReducerContext, building: &mut Building) -> Result<(), String> {
    if let Some(mut home_data) = ctx.db.home_data().building_id().find(&building.id) {
        // Deplete rent
        home_data.rent_paid -= building_depletion::RENT_BASE;
        
        // Check if rent is overdue
        if home_data.rent_paid < 0.0 {
            // TODO: Eviction logic
        }
        
        ctx.db.home_data().building_id().update(home_data);
    }
    
    Ok(())
}

fn update_workplace_daily(ctx: &ReducerContext, building: &mut Building) -> Result<(), String> {
    if let Some(mut workplace) = ctx.db.workplace_data().building_id().find(&building.id) {
        // Calculate efficiency
        let efficiency_factor = 1.0 + (building.efficiency_level as f32 * upgrades::EFFICIENCY_PRODUCTION_BONUS);
        let consumption_reduction = 1.0 - (building.efficiency_level as f32 * upgrades::EFFICIENCY_CONSUMPTION_REDUCTION);
        
        // Consume resources
        let consumption = (building_depletion::RESOURCE_CONSUMPTION_BASE + 
            building.current_occupants as f32 * building_depletion::RESOURCE_CONSUMPTION_PER_WORKER) * 
            consumption_reduction;
        
        workplace.stockpile -= consumption;
        
        // Produce resources if have materials
        if workplace.stockpile > 0.0 {
            let production = (building_depletion::RESOURCE_PRODUCTION_BASE + 
                building.current_occupants as f32 * building_depletion::RESOURCE_PRODUCTION_PER_WORKER) * 
                efficiency_factor;
            
            workplace.inventory += production;
            
            // Cap at max inventory
            workplace.inventory = workplace.inventory.min(workplace.max_inventory);
        }
        
        // Update costs and revenue
        building.operating_cost = building_depletion::OPERATIONAL_COST_BASE + 
            building.current_occupants as f32 * building_depletion::OPERATIONAL_COST_PER_WORKER;
        
        ctx.db.workplace_data().building_id().update(workplace);
    }
    
    Ok(())
}

fn create_location_capabilities(ctx: &ReducerContext, building_id: u32, building_type: &BuildingType) -> Result<(), String> {
    let capabilities = match building_type {
        BuildingType::Home(_) => LocationCapability {
            id: (ctx.db.location_capability().iter().count() + 1) as u32,
            building_id,
            provides_food: true,
            provides_rest: true,
            provides_social: false,
            provides_facilities: true,
            provides_healthcare: false,
            provides_culture: false,
            provides_education: false,
            provides_work: false,
            environmental_quality: 0.5,
        },
        BuildingType::Workplace(_) => LocationCapability {
            id: (ctx.db.location_capability().iter().count() + 1) as u32,
            building_id,
            provides_food: false,
            provides_rest: false,
            provides_social: true,
            provides_facilities: true,
            provides_healthcare: false,
            provides_culture: false,
            provides_education: false,
            provides_work: true,
            environmental_quality: -0.5,
        },
        BuildingType::Restaurant => LocationCapability {
            id: (ctx.db.location_capability().iter().count() + 1) as u32,
            building_id,
            provides_food: true,
            provides_rest: false,
            provides_social: true,
            provides_facilities: true,
            provides_healthcare: false,
            provides_culture: false,
            provides_education: false,
            provides_work: false,
            environmental_quality: 0.0,
        },
        BuildingType::Park => LocationCapability {
            id: (ctx.db.location_capability().iter().count() + 1) as u32,
            building_id,
            provides_food: false,
            provides_rest: true,
            provides_social: true,
            provides_facilities: false,
            provides_healthcare: false,
            provides_culture: true,
            provides_education: false,
            provides_work: false,
            environmental_quality: 1.5,
        },
        BuildingType::Hospital => LocationCapability {
            id: (ctx.db.location_capability().iter().count() + 1) as u32,
            building_id,
            provides_food: false,
            provides_rest: true,
            provides_social: false,
            provides_facilities: true,
            provides_healthcare: true,
            provides_culture: false,
            provides_education: false,
            provides_work: false,
            environmental_quality: 2.0,
        },
        _ => LocationCapability {
            id: (ctx.db.location_capability().iter().count() + 1) as u32,
            building_id,
            provides_food: false,
            provides_rest: false,
            provides_social: false,
            provides_facilities: true,
            provides_healthcare: false,
            provides_culture: false,
            provides_education: false,
            provides_work: false,
            environmental_quality: 0.0,
        },
    };
    
    ctx.db.location_capability().insert(capabilities);
    Ok(())
}