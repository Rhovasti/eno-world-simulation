// Integration with Enonomics data for realistic economic simulation

use spacetimedb::{ReducerContext, Table};
use serde::{Serialize, Deserialize};
use log;
use crate::economics::{ResourceType, Market};

// Enonomics data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnonomicsCity {
    pub id: String,
    pub name: String,
    pub population: u32,
    pub gdp: f64,
    pub unemployment_rate: f64,
    pub trade_volume: f64,
    pub primary_industries: Vec<String>,
    pub trade_partners: Vec<String>,
    pub wealth_index: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnonomicsRegion {
    pub id: String,
    pub name: String,
    pub cities: Vec<String>,
    pub total_population: u32,
    pub climate: String,
    pub natural_resources: Vec<String>,
    pub trade_routes: Vec<TradeConnection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeConnection {
    pub from: String,
    pub to: String,
    pub resource_type: String,
    pub volume: f64,
    pub importance: f64,
}

// Cache for Enonomics data
#[spacetimedb::table(name = enonomics_cache)]
pub struct EnonomicsCache {
    #[primary_key]
    pub id: u32,
    pub cache_key: String, // City ID, region ID, or "global"
    pub data_type: String, // "city", "region", "trade_routes"
    pub data_json: String,
    pub last_updated_ms: i64,
    pub expires_ms: i64,
}

// Fetch Enonomics data (simulated - in real implementation would call API)
#[spacetimedb::reducer]
pub fn fetch_enonomics_data(
    ctx: &ReducerContext,
    data_type: String,
    identifier: String,
) -> Result<String, String> {
    // Check cache first
    if let Some(cached) = ctx.db.enonomics_cache()
        .iter()
        .find(|c| c.cache_key == identifier && c.data_type == data_type) {

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_millis() as i64;

        if cached.expires_ms > now {
            log::info!("Retrieved cached Enonomics data for {} {}", data_type, identifier);
            return Ok(cached.data_json.clone());
        }
    }

    // Fetch fresh data (simulated)
    let data = match data_type.as_str() {
        "city" => fetch_city_data(&identifier)?,
        "region" => fetch_region_data(&identifier)?,
        "trade_routes" => fetch_trade_routes_data()?,
        _ => return Err("Unknown data type".to_string()),
    };

    // Cache the data
    cache_enonomics_data(ctx, data_type, identifier, data.clone())?;

    Ok(data)
}

// Simulated city data fetch
fn fetch_city_data(city_id: &str) -> Result<String, String> {
    // In real implementation, this would call Enonomics API
    let city = EnonomicsCity {
        id: city_id.to_string(),
        name: format!("City {}", city_id),
        population: 10000 + (city_id.len() as u32 * 5000),
        gdp: 50000000.0 + (city_id.len() as f64 * 10000000.0),
        unemployment_rate: 5.0 + (city_id.len() as f64 % 10.0),
        trade_volume: 1000000.0 + (city_id.len() as f64 * 500000.0),
        primary_industries: vec!["Manufacturing".to_string(), "Agriculture".to_string()],
        trade_partners: vec!["partner1".to_string(), "partner2".to_string()],
        wealth_index: 0.6 + (city_id.len() as f64 % 5.0) / 10.0,
    };

    serde_json::to_string(&city).map_err(|e| e.to_string())
}

// Simulated region data fetch
fn fetch_region_data(region_id: &str) -> Result<String, String> {
    let region = EnonomicsRegion {
        id: region_id.to_string(),
        name: format!("Region {}", region_id),
        cities: vec!["city1".to_string(), "city2".to_string(), "city3".to_string()],
        total_population: 100000,
        climate: "Temperate".to_string(),
        natural_resources: vec!["Iron".to_string(), "Coal".to_string(), "Timber".to_string()],
        trade_routes: vec![
            TradeConnection {
                from: "city1".to_string(),
                to: "city2".to_string(),
                resource_type: "food".to_string(),
                volume: 1000.0,
                importance: 0.8,
            },
        ],
    };

    serde_json::to_string(&region).map_err(|e| e.to_string())
}

// Simulated trade routes data fetch
fn fetch_trade_routes_data() -> Result<String, String> {
    let routes = vec![
        TradeConnection {
            from: "city1".to_string(),
            to: "city2".to_string(),
            resource_type: "food".to_string(),
            volume: 1000.0,
            importance: 0.8,
        },
        TradeConnection {
            from: "city2".to_string(),
            to: "city3".to_string(),
            resource_type: "materials".to_string(),
            volume: 500.0,
            importance: 0.6,
        },
    ];

    serde_json::to_string(&routes).map_err(|e| e.to_string())
}

// Cache Enonomics data
fn cache_enonomics_data(
    ctx: &ReducerContext,
    data_type: String,
    identifier: String,
    data: String,
) -> Result<(), String> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| format!("Time error: {}", e))?
        .as_millis() as i64;

    let cache_id = ctx.db.enonomics_cache().iter().count() as u32 + 1;
    let expires_ms = now + (60 * 60 * 1000); // Cache for 1 hour

    // Remove old cache entry if exists
    if let Some(old_cache) = ctx.db.enonomics_cache()
        .iter()
        .find(|c| c.cache_key == identifier && c.data_type == data_type) {
        // In a real implementation, you'd delete the old entry
        log::info!("Updating existing cache for {} {}", data_type, identifier);
    }

    let cache_entry = EnonomicsCache {
        id: cache_id,
        cache_key: identifier,
        data_type,
        data_json: data,
        last_updated_ms: now,
        expires_ms,
    };

    ctx.db.enonomics_cache().insert(cache_entry);

    Ok(())
}

// Update market data based on Enonomics information
#[spacetimedb::reducer]
pub fn sync_market_with_enonomics(
    ctx: &ReducerContext,
    world_id: u32,
    city_id: u32,
) -> Result<(), String> {
    // Fetch city data from Enonomics
    let city_data_json = fetch_enonomics_data(
        ctx,
        "city".to_string(),
        city_id.to_string(),
    )?;

    let city_data: EnonomicsCity = serde_json::from_str(&city_data_json)
        .map_err(|e| format!("Failed to parse city data: {}", e))?;

    // Update markets based on Enonomics data
    let markets: Vec<Market> = ctx.db.market()
        .iter()
        .filter(|m| m.world_id == world_id && m.city_id == city_id)
        .cloned()
        .collect();

    for mut market in markets {
        // Adjust supply and demand based on Enonomics indicators
        let gdp_factor = (city_data.gdp / 100000000.0) as f32;
        let unemployment_factor = (100.0 - city_data.unemployment_rate) / 100.0;
        let wealth_factor = city_data.wealth_index as f32;

        // Update demand based on wealth and population
        market.demand = match market.resource_type {
            ResourceType::Food => city_data.population as f32 * 1.5,
            ResourceType::Luxury => city_data.population as f32 * wealth_factor * 0.3,
            ResourceType::ProcessedGoods => city_data.population as f32 * gdp_factor,
            _ => market.demand, // Keep existing demand for others
        };

        // Update supply based on industries and unemployment
        if city_data.primary_industries.contains(&"Manufacturing".to_string()) {
            if market.resource_type == ResourceType::ProcessedGoods {
                market.supply *= unemployment_factor;
            }
        }

        if city_data.primary_industries.contains(&"Agriculture".to_string()) {
            if market.resource_type == ResourceType::Food {
                market.supply *= 1.2; // Agricultural bonus
            }
        }

        // Update price volatility based on trade volume
        let trade_factor = (city_data.trade_volume / 1000000.0) as f32;
        market.price_volatility = (0.3 + trade_factor * 0.4).clamp(0.1, 1.0);

        // Update the market
        ctx.db.market().id().update(market.id, market);
    }

    log::info!("Synced markets for city {} with Enonomics data", city_id);
    Ok(())
}

// Generate trade routes based on Enonomics data
#[spacetimedb::reducer]
pub fn generate_trade_routes_from_enonomics(
    ctx: &ReducerContext,
    world_id: u32,
) -> Result<Vec<u32>, String> {
    let trade_data_json = fetch_enonomics_data(
        ctx,
        "trade_routes".to_string(),
        "global".to_string(),
    )?;

    let trade_connections: Vec<TradeConnection> = serde_json::from_str(&trade_data_json)
        .map_err(|e| format!("Failed to parse trade data: {}", e))?;

    let mut route_ids = Vec::new();

    for connection in trade_connections {
        let from_city_id: u32 = connection.from.parse()
            .unwrap_or_else(|_| connection.from.len() as u32);
        let to_city_id: u32 = connection.to.parse()
            .unwrap_or_else(|_| connection.to.len() as u32);

        let resource_type = match connection.resource_type.as_str() {
            "food" => ResourceType::Food,
            "materials" => ResourceType::RawMaterials,
            "luxury" => ResourceType::Luxury,
            _ => ResourceType::ProcessedGoods,
        };

        let route_id = ctx.db.trade_route().iter().count() as u32 + 1;

        let trade_route = crate::economics::TradeRoute {
            id: route_id,
            world_id,
            from_city_id,
            to_city_id,
            resource_type,
            volume: connection.volume as f32,
            frequency_hours: 24, // Daily trade
            profitability: connection.importance as f32,
            safety: 80.0, // Base safety level
            merchant_count: (connection.volume / 100.0) as u32,
            last_trade_hour: 0,
            is_active: true,
        };

        ctx.db.trade_route().insert(trade_route);
        route_ids.push(route_id);
    }

    log::info!("Generated {} trade routes from Enonomics data", route_ids.len());
    Ok(route_ids)
}

// Periodic sync with Enonomics (should be called regularly)
#[spacetimedb::reducer]
pub fn periodic_enonomics_sync(
    ctx: &ReducerContext,
    world_id: u32,
) -> Result<(), String> {
    // Get all cities in the world
    let cities: Vec<u32> = ctx.db.market()
        .iter()
        .filter(|m| m.world_id == world_id)
        .map(|m| m.city_id)
        .collect::<std::collections::HashSet<u32>>()
        .into_iter()
        .collect();

    // Sync each city with Enonomics data
    for city_id in cities {
        if let Err(e) = sync_market_with_enonomics(ctx, world_id, city_id) {
            log::warn!("Failed to sync city {} with Enonomics: {}", city_id, e);
        }
    }

    log::info!("Completed periodic Enonomics sync for world {}", world_id);
    Ok(())
}