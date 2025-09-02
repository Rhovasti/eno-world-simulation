use spacetimedb::{ReducerContext, Table};
use log;
use crate::tables::*;
use crate::types::*;
use crate::systems::modifiers::*;
use crate::tables::city::{city, city_service};
use crate::tables::individual::individual;
use crate::tables::building::building;
use crate::tables::events::simulation_time;

/// Create a new city
#[spacetimedb::reducer]
pub fn create_city(ctx: &ReducerContext, name: String) -> Result<(), String> {
    let current_hour = ctx.db.simulation_time().id().find(&1)
        .ok_or("Simulation not initialized")?
        .current_hour;
    
    let id = (ctx.db.city().iter().count() + 1) as u32;
    
    let city = City {
        id,
        name: name.clone(),
        founded_hour: current_hour,
        population: 0,
        
        // Infrastructure & Economy
        public_works: 100.0,
        tax_base: 0.0,
        tax_reserve: 1000.0, // Starting funds
        import_rate: 0.0,
        export_rate: 0.0,
        
        // Safety & Social
        stability: 100.0,
        health: 100.0,
        safety: 100.0,
        
        // Culture & Development
        culture: 0.0,
        science: 0.0,
        prestige: 0.0,
        
        // Metrics
        unemployment_rate: 0.0,
        average_happiness: 70.0,
        crime_rate: 0.0,
        last_update_hour: current_hour,
    };
    
    ctx.db.city().insert(city);
    
    // Create basic city services
    create_basic_services(ctx, id)?;
    
    log::info!("Created city {} with ID {}", name, id);
    Ok(())
}

/// Weekly city update
#[spacetimedb::reducer]
pub fn update_city_weekly(ctx: &ReducerContext, city_id: u32) -> Result<(), String> {
    let mut city = ctx.db.city().id().find(&city_id)
        .ok_or("City not found")?;
    
    // Update population count
    city.population = ctx.db.individual().iter()
        .filter(|i| {
            if let Some(home_id) = i.home_id {
                ctx.db.building().id().find(&home_id)
                    .map(|b| b.city_id == city_id)
                    .unwrap_or(false)
            } else {
                false
            }
        })
        .count() as u32;
    
    // Update infrastructure
    update_infrastructure(&mut city)?;
    
    // Update economy
    update_economy(&ctx, &mut city)?;
    
    // Update social metrics
    update_social_metrics(&ctx, &mut city)?;
    
    // Update culture and development
    update_culture_development(&ctx, &mut city)?;
    
    ctx.db.city().id().update(city);
    
    Ok(())
}

fn update_infrastructure(city: &mut City) -> Result<(), String> {
    // Public works decay based on population
    let decay = city.population as f32 * city_depletion::PUBLIC_WORKS_PER_CITIZEN;
    city.public_works = (city.public_works + decay).clamp(0.0, 100.0);
    
    // Tax collection and spending
    let service_cost = (city.population as f32 / 100.0) * city_depletion::PUBLIC_SERVICE_COST_PER_100;
    city.tax_reserve -= service_cost;
    
    // Calculate imports/exports based on production/consumption balance
    // Simplified for now
    if city.tax_reserve < 0.0 {
        city.import_rate += 10.0;
        city.tax_reserve -= city_depletion::IMPORT_COST * city.import_rate;
    }
    
    Ok(())
}

fn update_economy(ctx: &ReducerContext, city: &mut City) -> Result<(), String> {
    // Calculate tax base from working individuals
    let total_income: f32 = ctx.db.individual().iter()
        .filter(|i| i.workplace_id.is_some())
        .map(|i| i.income * 0.2) // 20% tax rate
        .sum();
    
    city.tax_base = total_income;
    city.tax_reserve += total_income;
    
    // Calculate unemployment
    let workforce = ctx.db.individual().iter()
        .filter(|i| i.age >= 18 && i.age <= 65)
        .count();
    
    let employed = ctx.db.individual().iter()
        .filter(|i| i.workplace_id.is_some())
        .count();
    
    city.unemployment_rate = if workforce > 0 {
        ((workforce - employed) as f32 / workforce as f32) * 100.0
    } else {
        0.0
    };
    
    Ok(())
}

fn update_social_metrics(ctx: &ReducerContext, city: &mut City) -> Result<(), String> {
    // Calculate stability based on stressed individuals
    let stressed_count = ctx.db.individual().iter()
        .filter(|i| i.stress > thresholds::STRESS_CRITICAL)
        .count() as f32;
    
    let stability_loss = stressed_count * city_depletion::STABILITY_PER_STRESSED;
    city.stability = (city.stability - stability_loss).clamp(0.0, 100.0);
    
    // Low unemployment increases stability
    if city.unemployment_rate < 5.0 {
        city.stability = (city.stability + 10.0).min(100.0);
    }
    
    // Calculate average health
    let total_environment: f32 = ctx.db.individual().iter()
        .map(|i| i.environment)
        .sum();
    
    city.health = if city.population > 0 {
        total_environment / city.population as f32
    } else {
        100.0
    };
    
    // Calculate safety (inverse of average threat)
    let total_threat: f32 = ctx.db.individual().iter()
        .map(|i| i.threat)
        .sum();
    
    city.safety = if city.population > 0 {
        100.0 - (total_threat / city.population as f32)
    } else {
        100.0
    };
    
    // Calculate average happiness
    let total_happiness: f32 = ctx.db.individual().iter()
        .map(|i| {
            // Simplified happiness calculation
            let basic_needs = (i.food_water + i.rest + i.safety) / 3.0;
            let social_needs = i.community;
            (basic_needs + social_needs) / 2.0
        })
        .sum();
    
    city.average_happiness = if city.population > 0 {
        total_happiness / city.population as f32
    } else {
        70.0
    };
    
    Ok(())
}

fn update_culture_development(ctx: &ReducerContext, city: &mut City) -> Result<(), String> {
    // Culture from artists and cultural buildings
    let artists = ctx.db.individual().iter()
        .filter(|i| matches!(i.specialized_role, SpecializedRole::Artist))
        .count() as f32;
    
    city.culture += artists * city_depletion::ARTIST_CULTURE_RATE * 168.0; // Weekly hours
    
    // Science from scientists
    let scientists = ctx.db.individual().iter()
        .filter(|i| matches!(i.specialized_role, SpecializedRole::Scientist))
        .count() as f32;
    
    city.science += scientists * city_depletion::SCIENTIST_SCIENCE_RATE * 168.0;
    
    // Prestige from various sources
    let prestige_buildings = ctx.db.building().iter()
        .filter(|b| b.city_id == city.id && b.prestige_level >= 5)
        .count() as f32;
    
    let self_actualized = ctx.db.individual().iter()
        .filter(|i| i.progression > 80.0)
        .count() as f32;
    
    city.prestige += prestige_buildings + (self_actualized * 5.0);
    
    Ok(())
}

fn create_basic_services(ctx: &ReducerContext, city_id: u32) -> Result<(), String> {
    let services = vec![
        (ServiceType::Police, 50.0),
        (ServiceType::Fire, 50.0),
        (ServiceType::Hospital, 30.0),
        (ServiceType::Education, 40.0),
        (ServiceType::Utilities, 100.0),
    ];
    
    for (service_type, coverage) in services {
        let id = (ctx.db.city_service().iter().count() + 1) as u32;
        ctx.db.city_service().insert(CityService {
            id,
            city_id,
            service_type,
            coverage,
            quality: 50.0,
            cost_per_hour: 10.0,
            workers_needed: 5,
            workers_assigned: 0,
        });
    }
    
    Ok(())
}