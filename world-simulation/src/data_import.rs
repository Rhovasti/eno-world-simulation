use spacetimedb::{ReducerContext, Table};
use log;
use serde::{Deserialize, Serialize};
use crate::types::{BuildingType, JobType, HomeConfig, WorkplaceConfig};
use crate::tables::*;
use std::collections::HashMap;
use crate::tables::city::city;
use crate::tables::building::building;
use crate::tables::events::simulation_time;

// Data structures to match the Eno JSON files
#[derive(Debug, Deserialize)]
pub struct EnoCity {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub population: u32,
    pub elevation: u32,
    pub capital: bool,
    pub port: bool,
    pub citadel: bool,
    pub walls: bool,
    pub plaza: bool,
    pub temple: bool,
    pub shanty_town: bool,
    pub valley: Option<String>,
    pub founded_in: u32,
    pub roads_count: u32,
    pub walls_count: u32,
    pub rivers_count: u32,
    pub planks_count: u32,
    pub building_count: u32,
    pub prisms_count: u32,
    pub squares_count: u32,
    pub greens_count: u32,
    pub fields_count: u32,
    pub trees_count: u32,
    pub districts: Vec<EnoDistrict>,
    pub district_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct EnoDistrict {
    pub name: String,
    pub building_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct EnoBuilding {
    pub id: String,
    pub age: String,
    pub generation: String,
    #[serde(rename = "type")]
    pub building_type: String,
    pub specific_type: String,
    pub occupants: String,
    pub bodies: Option<String>,
    pub souls: Option<String>,
    pub floors: String,
    pub subterrain_level: String,
    pub jobs: Option<String>,
    pub employees: Option<String>,
}

/// Import a manageable subset of Eno cities for simulation testing
#[spacetimedb::reducer]
pub fn import_eno_cities_subset(
    ctx: &ReducerContext,
    max_cities: u32,
    max_population_per_city: u32,
) -> Result<(), String> {
    let current_hour = ctx.db.simulation_time().id().find(&1)
        .ok_or("Simulation not initialized")?
        .current_hour;
    
    // For now, create sample data based on the Eno structure
    // In production, this would parse the actual JSON files
    let sample_cities = create_sample_eno_cities(max_cities, max_population_per_city);
    
    let mut imported_cities = 0;
    let mut imported_buildings = 0;
    let mut imported_individuals = 0;
    
    for eno_city in sample_cities {
        if imported_cities >= max_cities {
            break;
        }
        
        // Create the city
        let city_id = create_city_from_eno(ctx, &eno_city, current_hour)?;
        imported_cities += 1;
        
        // Create districts as building clusters
        let mut district_buildings = Vec::new();
        for (district_idx, district) in eno_city.districts.iter().enumerate() {
            let buildings = create_district_buildings(
                ctx,
                city_id, 
                district_idx as u32, 
                &district.name, 
                district.building_count.min(100), // Cap buildings per district
                eno_city.latitude,
                eno_city.longitude,
            )?;
            let building_count = buildings.len();
            district_buildings.extend(buildings);
            imported_buildings += building_count;
        }
        
        // Create population based on city size
        let target_population = eno_city.population.min(max_population_per_city);
        let individuals = create_city_population(
            ctx,
            city_id,
            &district_buildings,
            target_population,
            current_hour,
        )?;
        imported_individuals += individuals;
        
        log::info!("Imported city: {} (pop: {}, buildings: {})", 
                 eno_city.name, target_population, district_buildings.len());
    }
    
    log::info!("Successfully imported {} cities, {} buildings, {} individuals",
        imported_cities, imported_buildings, imported_individuals);
    Ok(())
}

/// Create sample cities based on Eno data patterns
fn create_sample_eno_cities(max_cities: u32, max_pop: u32) -> Vec<EnoCity> {
    let cities = vec![
        ("Guild", 79193, 5, true, true),    // Major trade hub
        ("Mahyapak", 71912, 4, false, true),  // Large port city
        ("Chingsan", 57543, 3, false, false), // Industrial center
        ("Pranos", 56744, 4, false, true),    // Coastal city
        ("Jeong", 50393, 3, false, false),    // Mountain city
        ("Aira", 1848, 5, true, false),       // Small capital (Night Valley)
        ("Palwede", 47137, 4, false, true),   // River port
        ("Zadardelen", 44324, 3, false, false), // Trade town
        ("Engar", 42312, 4, false, false),    // Agricultural center
        ("Alebuo", 38623, 3, false, true),    // Coastal town
    ];
    
    cities.into_iter()
        .take(max_cities as usize)
        .enumerate()
        .map(|(i, (name, pop, districts, capital, port))| {
            let capped_pop = pop.min(max_pop);
            EnoCity {
                name: name.to_string(),
                latitude: 30.0 + (i as f64 * 2.0), // Spread cities across latitude
                longitude: 20.0 + (i as f64 * 3.0),
                population: capped_pop,
                elevation: 100 + (i as u32 * 200),
                capital,
                port,
                citadel: capital || pop > 40000,
                walls: pop > 10000,
                plaza: pop > 5000,
                temple: pop > 15000,
                shanty_town: pop > 30000,
                valley: if capital { Some("Trade".to_string()) } else { None },
                founded_in: 50 + (i as u32 * 20),
                roads_count: (pop / 3000).max(3),
                walls_count: if pop > 10000 { 2 } else { 0 },
                rivers_count: if port { 1 } else { 0 },
                planks_count: if port { pop / 5000 } else { 0 },
                building_count: estimate_building_count(capped_pop),
                prisms_count: if pop > 50000 { 2 } else { 1 },
                squares_count: (pop / 20000).max(1),
                greens_count: pop / 15000,
                fields_count: pop / 2000,
                trees_count: pop / 1000,
                districts: create_sample_districts(districts, capped_pop),
                district_count: districts,
            }
        })
        .collect()
}

fn create_sample_districts(count: u32, total_pop: u32) -> Vec<EnoDistrict> {
    let district_names = vec![
        "Old Town", "Market District", "Noble Quarter", "Artisan Ward", 
        "Port District", "Temple Quarter", "Trade Quarter", "Residential Area",
        "Castle Ward", "Merchant District", "Industrial Zone", "Harbor Front"
    ];
    
    let total_buildings = estimate_building_count(total_pop);
    
    (0..count)
        .map(|i| {
            let base_buildings = total_buildings / count;
            let variation = if i == 0 { total_buildings % count } else { 0 };
            
            EnoDistrict {
                name: district_names[i as usize % district_names.len()].to_string(),
                building_count: base_buildings + variation,
            }
        })
        .collect()
}

fn estimate_building_count(population: u32) -> u32 {
    // Estimate based on Eno data: roughly 4-6 people per building
    (population / 5).max(10)
}

fn create_city_from_eno(
    ctx: &ReducerContext,
    eno_city: &EnoCity,
    current_hour: u64,
) -> Result<u32, String> {
    let id = (ctx.db.city().iter().count() + 1) as u32;
    
    // Calculate initial city metrics based on Eno data
    let base_infrastructure = if eno_city.capital { 90.0 } else { 70.0 };
    let infrastructure = base_infrastructure + 
        (eno_city.roads_count as f32 * 2.0) + 
        (eno_city.walls_count as f32 * 5.0);
    
    let starting_funds = (eno_city.population as f32 * 10.0) + 
        if eno_city.port { 5000.0 } else { 0.0 } +
        if eno_city.capital { 10000.0 } else { 0.0 };
    
    let city = City {
        id,
        name: eno_city.name.clone(),
        founded_hour: current_hour - (eno_city.founded_in as u64 * 24 * 365), // Rough conversion
        population: 0, // Will be set when individuals are created
        
        // Infrastructure based on Eno characteristics
        public_works: infrastructure.min(100.0),
        tax_base: 0.0,
        tax_reserve: starting_funds,
        import_rate: if eno_city.port { 50.0 } else { 10.0 },
        export_rate: if eno_city.port { 40.0 } else { 5.0 },
        
        // Social metrics based on city features
        stability: if eno_city.walls { 80.0 } else { 70.0 },
        health: if eno_city.temple { 80.0 } else { 70.0 },
        safety: if eno_city.citadel { 85.0 } else { 75.0 },
        
        // Cultural development
        culture: (eno_city.plaza as u32 as f32 * 20.0) + 
                 (eno_city.temple as u32 as f32 * 30.0),
        science: if eno_city.capital { 50.0 } else { 10.0 },
        prestige: (eno_city.capital as u32 as f32 * 100.0) + 
                  (eno_city.citadel as u32 as f32 * 50.0),
        
        unemployment_rate: 5.0,
        average_happiness: 70.0,
        crime_rate: if eno_city.shanty_town { 15.0 } else { 5.0 },
        last_update_hour: current_hour,
    };
    
    ctx.db.city().insert(city);
    Ok(id)
}

fn create_district_buildings(
    ctx: &ReducerContext,
    city_id: u32,
    district_id: u32,
    district_name: &str,
    building_count: u32,
    base_lat: f64,
    base_lon: f64,
) -> Result<Vec<u32>, String> {
    let mut building_ids = Vec::new();
    
    // Determine district type and building mix
    let (residential_ratio, workplace_ratio, amenity_ratio) = match district_name {
        name if name.contains("Residential") || name.contains("Noble") => (0.8, 0.1, 0.1),
        name if name.contains("Market") || name.contains("Trade") || name.contains("Industrial") => (0.3, 0.6, 0.1),
        name if name.contains("Port") || name.contains("Harbor") => (0.4, 0.5, 0.1),
        _ => (0.6, 0.3, 0.1), // Mixed district
    };
    
    let residential_count = (building_count as f32 * residential_ratio) as u32;
    let workplace_count = (building_count as f32 * workplace_ratio) as u32;
    let amenity_count = building_count - residential_count - workplace_count;
    
    // Create residential buildings
    for i in 0..residential_count {
        let building_id = create_residential_building(
            ctx,
            city_id, 
            district_id, 
            i,
            base_lat, 
            base_lon
        )?;
        building_ids.push(building_id);
    }
    
    // Create workplace buildings
    for i in 0..workplace_count {
        let building_id = create_workplace_building(
            ctx,
            city_id, 
            district_id, 
            residential_count + i,
            base_lat, 
            base_lon,
            district_name,
        )?;
        building_ids.push(building_id);
    }
    
    // Create amenity buildings
    for i in 0..amenity_count {
        let building_id = create_amenity_building(
            ctx,
            city_id, 
            district_id, 
            residential_count + workplace_count + i,
            base_lat, 
            base_lon,
        )?;
        building_ids.push(building_id);
    }
    
    Ok(building_ids)
}

fn create_residential_building(
    ctx: &ReducerContext,
    city_id: u32,
    district_id: u32,
    building_idx: u32,
    base_lat: f64,
    base_lon: f64,
) -> Result<u32, String> {
    use crate::reducers::building::create_building;
    
    let capacity = match building_idx % 4 {
        0 => 2,  // Small cottage
        1 => 4,  // Townhouse
        2 => 6,  // Large house
        _ => 8,  // Manor/apartment
    };
    
    let rent = match capacity {
        2 => 300.0,
        4 => 500.0,
        6 => 700.0,
        _ => 900.0,
    };
    
    // Spread buildings around the district
    let offset_x = (building_idx % 10) as f32 * 0.01;
    let offset_y = (building_idx / 10) as f32 * 0.01;
    
    let building_id = (ctx.db.building().iter().count() + 1) as u32;
    create_building(
        ctx,
        format!("House {}-{}", district_id, building_idx),
        city_id,
        BuildingType::Home(HomeConfig { capacity, rent }),
        base_lat as f32 + offset_x,
        base_lon as f32 + offset_y,
    )?;
    Ok(building_id)
}

fn create_workplace_building(
    ctx: &ReducerContext,
    city_id: u32,
    district_id: u32,
    building_idx: u32,
    base_lat: f64,
    base_lon: f64,
    district_name: &str,
) -> Result<u32, String> {
    use crate::reducers::building::create_building;
    
    let (job_type, positions) = if district_name.contains("Industrial") {
        (JobType::Factory, 20)
    } else if district_name.contains("Market") || district_name.contains("Trade") {
        (JobType::Retail, 10)
    } else if district_name.contains("Port") || district_name.contains("Harbor") {
        (JobType::Factory, 15) // Docks/shipping
    } else {
        (JobType::Office, 12)
    };
    
    let offset_x = (building_idx % 10) as f32 * 0.01;
    let offset_y = (building_idx / 10) as f32 * 0.01;
    
    let building_id = (ctx.db.building().iter().count() + 1) as u32;
    create_building(
        ctx,
        format!("Work {}-{}", district_id, building_idx),
        city_id,
        BuildingType::Workplace(WorkplaceConfig { job_type, positions }),
        base_lat as f32 + offset_x,
        base_lon as f32 + offset_y,
    )?;
    Ok(building_id)
}

fn create_amenity_building(
    ctx: &ReducerContext,
    city_id: u32,
    district_id: u32,
    building_idx: u32,
    base_lat: f64,
    base_lon: f64,
) -> Result<u32, String> {
    use crate::reducers::building::create_building;
    
    let building_type = match building_idx % 5 {
        0 => BuildingType::Restaurant,
        1 => BuildingType::Park,
        2 => BuildingType::Hospital,
        3 => BuildingType::School,
        _ => BuildingType::CultureCenter,
    };
    
    let offset_x = (building_idx % 10) as f32 * 0.01;
    let offset_y = (building_idx / 10) as f32 * 0.01;
    
    let building_id = (ctx.db.building().iter().count() + 1) as u32;
    create_building(
        ctx,
        format!("Amenity {}-{}", district_id, building_idx),
        city_id,
        building_type,
        base_lat as f32 + offset_x,
        base_lon as f32 + offset_y,
    )?;
    Ok(building_id)
}

fn create_city_population(
    ctx: &ReducerContext,
    city_id: u32,
    building_ids: &[u32],
    target_population: u32,
    current_hour: u64,
) -> Result<u32, String> {
    use crate::reducers::individual::create_individual;
    
    // Get residential and workplace buildings
    let residential_buildings: Vec<Building> = ctx.db.building().iter()
        .filter(|b| {
            b.city_id == city_id && 
            matches!(b.building_type, BuildingType::Home(_))
        })
        .collect();
    
    let workplace_buildings: Vec<Building> = ctx.db.building().iter()
        .filter(|b| {
            b.city_id == city_id && 
            matches!(b.building_type, BuildingType::Workplace(_))
        })
        .collect();
    
    let mut created_individuals = 0;
    let mut current_workplace_idx = 0;
    
    // Create individuals for each residential building
    for building in &residential_buildings {
        if let BuildingType::Home(home_data) = &building.building_type {
            let occupants = (target_population / residential_buildings.len() as u32)
                .min(home_data.capacity)
                .max(1);
            
            for i in 0..occupants {
                if created_individuals >= target_population {
                    break;
                }
                
                // Assign workplace (not everyone has a job)
                let workplace_id = if current_workplace_idx < workplace_buildings.len() {
                    let workplace = &workplace_buildings[current_workplace_idx];
                    current_workplace_idx = (current_workplace_idx + 1) % workplace_buildings.len();
                    Some(workplace.id)
                } else {
                    None
                };
                
                let name = generate_random_name(created_individuals);
                create_individual(
                    ctx,
                    name,
                    Some(building.id),
                    workplace_id,
                )?;
                
                created_individuals += 1;
            }
        }
    }
    
    Ok(created_individuals)
}

fn generate_random_name(index: u32) -> String {
    let first_names = vec![
        "Aerin", "Brix", "Cala", "Dero", "Elyn", "Fynn", "Gira", "Hale",
        "Iska", "Jeth", "Kira", "Lann", "Mira", "Noel", "Oren", "Peri",
        "Quin", "Rava", "Senn", "Tara", "Ulix", "Vera", "Wynn", "Xara",
        "Ysel", "Zara", "Alec", "Bren", "Cora", "Dain", "Ella", "Fren",
    ];
    
    let last_names = vec![
        "Ashford", "Blake", "Cross", "Dorne", "Ember", "Flint", "Gray", "Hunt",
        "Iron", "Kane", "Lane", "Moon", "North", "Oak", "Pike", "Quinn",
        "Reed", "Stone", "Thorne", "Vale", "Ward", "York", "Ash", "Bell",
        "Clay", "Dale", "Fox", "Glen", "Hill", "Marsh", "Rivers", "Woods",
    ];
    
    let first = &first_names[index as usize % first_names.len()];
    let last = &last_names[(index / first_names.len() as u32) as usize % last_names.len()];
    
    format!("{} {}", first, last)
}

/// Import a single test city for development
#[spacetimedb::reducer]
pub fn import_test_city(ctx: &ReducerContext) -> Result<(), String> {
    import_eno_cities_subset(ctx, 1, 1000)
}

/// Import a small set of cities for testing
#[spacetimedb::reducer]
pub fn import_small_dataset(ctx: &ReducerContext) -> Result<(), String> {
    import_eno_cities_subset(ctx, 3, 2000)
}

/// Import a medium dataset for simulation
#[spacetimedb::reducer]
pub fn import_medium_dataset(ctx: &ReducerContext) -> Result<(), String> {
    import_eno_cities_subset(ctx, 5, 5000)
}