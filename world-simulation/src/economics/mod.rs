// Economic simulation module with market dynamics and trade

use spacetimedb::{ReducerContext, Table, SpacetimeType};
use serde::{Serialize, Deserialize};
use log;

pub mod markets;
pub mod trade_routes;
pub mod enonomics_integration;

// Resource types in the economy
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum ResourceType {
    Food,
    RawMaterials,
    ProcessedGoods,
    Luxury,
    Knowledge,
    Energy,
    Military,
}

// Market state for a resource in a city
#[spacetimedb::table(name = market)]
pub struct Market {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub city_id: u32,
    pub resource_type: ResourceType,
    pub supply: f32,
    pub demand: f32,
    pub price: f32,
    pub price_volatility: f32,
    pub price_history: String, // JSON array of recent prices
    pub last_update_hour: u64,
}

// Trade route between cities
#[spacetimedb::table(name = trade_route)]
pub struct TradeRoute {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub from_city_id: u32,
    pub to_city_id: u32,
    pub resource_type: ResourceType,
    pub volume: f32,
    pub frequency_hours: u32,
    pub profitability: f32,
    pub safety: f32, // 0-100, affected by political stability
    pub merchant_count: u32,
    pub last_trade_hour: u64,
    pub is_active: bool,
}

// Merchant NPCs that conduct trade
#[spacetimedb::table(name = merchant)]
pub struct Merchant {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub individual_id: u32, // Links to Individual table
    pub home_city_id: u32,
    pub current_city_id: u32,
    pub capital: f32,
    pub reputation: f32,
    pub specialization: ResourceType,
    pub trade_route_id: Option<u32>,
    pub goods_carried: String, // JSON of resource quantities
    pub profit_this_cycle: f32,
}

// Economic event types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, SpacetimeType)]
pub enum EconomicEventType {
    MarketCrash,
    MarketBoom,
    TradeDisruption,
    NewTradeRoute,
    ResourceShortage,
    ResourceSurplus,
    MerchantArrival,
    PriceManipulation,
}

// Economic events that affect markets
#[spacetimedb::table(name = economic_event)]
pub struct EconomicEvent {
    #[primary_key]
    pub id: u32,
    pub world_id: u32,
    pub event_type: EconomicEventType,
    pub affected_cities: String, // JSON array of city IDs
    pub affected_resource: Option<ResourceType>,
    pub impact_magnitude: f32,
    pub start_hour: u64,
    pub duration_hours: u32,
    pub description: String,
}

// Calculate supply and demand based on population and production
pub fn calculate_supply_demand(
    population: u32,
    production_capacity: f32,
    resource_type: ResourceType,
) -> (f32, f32) {
    let base_demand = match resource_type {
        ResourceType::Food => population as f32 * 1.5,
        ResourceType::RawMaterials => population as f32 * 0.8,
        ResourceType::ProcessedGoods => population as f32 * 0.6,
        ResourceType::Luxury => population as f32 * 0.2,
        ResourceType::Knowledge => population as f32 * 0.1,
        ResourceType::Energy => population as f32 * 1.0,
        ResourceType::Military => population as f32 * 0.05,
    };

    let base_supply = production_capacity * match resource_type {
        ResourceType::Food => 1.0,
        ResourceType::RawMaterials => 0.8,
        ResourceType::ProcessedGoods => 0.6,
        ResourceType::Luxury => 0.3,
        ResourceType::Knowledge => 0.2,
        ResourceType::Energy => 0.9,
        ResourceType::Military => 0.1,
    };

    (base_supply, base_demand)
}

// Calculate price based on supply and demand
pub fn calculate_price(supply: f32, demand: f32, base_price: f32, volatility: f32) -> f32 {
    if supply <= 0.0 {
        return base_price * 10.0; // Extreme scarcity
    }

    let ratio = demand / supply;
    let price_multiplier = ratio.powf(volatility);

    (base_price * price_multiplier).clamp(base_price * 0.1, base_price * 10.0)
}

// Initialize markets for a city
#[spacetimedb::reducer]
pub fn initialize_city_markets(
    ctx: &ReducerContext,
    world_id: u32,
    city_id: u32,
    population: u32,
) -> Result<(), String> {
    let resource_types = [
        ResourceType::Food,
        ResourceType::RawMaterials,
        ResourceType::ProcessedGoods,
        ResourceType::Luxury,
        ResourceType::Knowledge,
        ResourceType::Energy,
        ResourceType::Military,
    ];

    for resource_type in resource_types {
        let market_id = ctx.db.market().iter().count() as u32 + 1;

        let (supply, demand) = calculate_supply_demand(
            population,
            100.0, // Base production capacity
            resource_type,
        );

        let base_price = match resource_type {
            ResourceType::Food => 10.0,
            ResourceType::RawMaterials => 20.0,
            ResourceType::ProcessedGoods => 50.0,
            ResourceType::Luxury => 200.0,
            ResourceType::Knowledge => 100.0,
            ResourceType::Energy => 30.0,
            ResourceType::Military => 500.0,
        };

        let market = Market {
            id: market_id,
            world_id,
            city_id,
            resource_type,
            supply,
            demand,
            price: base_price,
            price_volatility: 0.5,
            price_history: format!("[{}]", base_price),
            last_update_hour: 0,
        };

        ctx.db.market().insert(market);
    }

    log::info!("Initialized markets for city {} in world {}", city_id, world_id);
    Ok(())
}

// Update market prices based on supply and demand
#[spacetimedb::reducer]
pub fn update_market_prices(
    ctx: &ReducerContext,
    world_id: u32,
    hour: u64,
) -> Result<(), String> {
    let markets: Vec<Market> = ctx.db.market()
        .iter()
        .filter(|m| m.world_id == world_id)
        .cloned()
        .collect();

    for mut market in markets {
        // Calculate new price
        let base_price = match market.resource_type {
            ResourceType::Food => 10.0,
            ResourceType::RawMaterials => 20.0,
            ResourceType::ProcessedGoods => 50.0,
            ResourceType::Luxury => 200.0,
            ResourceType::Knowledge => 100.0,
            ResourceType::Energy => 30.0,
            ResourceType::Military => 500.0,
        };

        let new_price = calculate_price(
            market.supply,
            market.demand,
            base_price,
            market.price_volatility,
        );

        // Update price history (keep last 10 prices)
        let mut price_history: Vec<f32> = serde_json::from_str(&market.price_history)
            .unwrap_or_else(|_| vec![market.price]);

        price_history.push(new_price);
        if price_history.len() > 10 {
            price_history.remove(0);
        }

        market.price = new_price;
        market.price_history = serde_json::to_string(&price_history).unwrap();
        market.last_update_hour = hour;

        // Update the market
        ctx.db.market().id().update(market.id, market);
    }

    Ok(())
}

// Process trade between cities
#[spacetimedb::reducer]
pub fn process_trade_routes(
    ctx: &ReducerContext,
    world_id: u32,
    hour: u64,
) -> Result<(), String> {
    let trade_routes: Vec<TradeRoute> = ctx.db.trade_route()
        .iter()
        .filter(|tr| tr.world_id == world_id && tr.is_active)
        .filter(|tr| hour - tr.last_trade_hour >= tr.frequency_hours as u64)
        .cloned()
        .collect();

    for mut route in trade_routes {
        // Get source and destination markets
        let source_market = ctx.db.market()
            .iter()
            .find(|m| m.city_id == route.from_city_id && m.resource_type == route.resource_type);

        let dest_market = ctx.db.market()
            .iter()
            .find(|m| m.city_id == route.to_city_id && m.resource_type == route.resource_type);

        if let (Some(mut source), Some(mut dest)) = (source_market, dest_market) {
            // Calculate trade volume based on price differential
            let price_diff = dest.price - source.price;
            let trade_volume = if price_diff > 0.0 {
                (route.volume * (price_diff / source.price).min(2.0)).min(source.supply * 0.1)
            } else {
                0.0
            };

            if trade_volume > 0.0 {
                // Execute trade
                source.supply -= trade_volume;
                dest.supply += trade_volume * (route.safety / 100.0); // Some loss due to safety

                // Update markets
                ctx.db.market().id().update(source.id, source);
                ctx.db.market().id().update(dest.id, dest);

                // Calculate profitability
                route.profitability = (price_diff * trade_volume) / route.volume;

                log::info!("Trade route {} moved {} units of {:?} from city {} to {}",
                    route.id, trade_volume, route.resource_type,
                    route.from_city_id, route.to_city_id);
            }
        }

        route.last_trade_hour = hour;
        ctx.db.trade_route().id().update(route.id, route);
    }

    Ok(())
}

// Generate economic events based on market conditions
#[spacetimedb::reducer]
pub fn generate_economic_events(
    ctx: &ReducerContext,
    world_id: u32,
    hour: u64,
) -> Result<Vec<u32>, String> {
    let mut event_ids = Vec::new();

    // Check for market crashes or booms
    let markets: Vec<Market> = ctx.db.market()
        .iter()
        .filter(|m| m.world_id == world_id)
        .cloned()
        .collect();

    for market in markets {
        let price_history: Vec<f32> = serde_json::from_str(&market.price_history)
            .unwrap_or_else(|_| vec![market.price]);

        if price_history.len() >= 3 {
            let recent_avg = price_history.iter().rev().take(3).sum::<f32>() / 3.0;
            let older_avg = price_history.iter().take(3).sum::<f32>() / 3.0;

            // Check for significant price changes
            let change_ratio = recent_avg / older_avg;

            if change_ratio > 2.0 {
                // Market boom
                let event_id = create_economic_event(
                    ctx,
                    world_id,
                    EconomicEventType::MarketBoom,
                    Some(market.resource_type),
                    vec![market.city_id],
                    hour,
                    format!("{:?} prices soar in city {}!", market.resource_type, market.city_id),
                )?;
                event_ids.push(event_id);
            } else if change_ratio < 0.5 {
                // Market crash
                let event_id = create_economic_event(
                    ctx,
                    world_id,
                    EconomicEventType::MarketCrash,
                    Some(market.resource_type),
                    vec![market.city_id],
                    hour,
                    format!("{:?} market crashes in city {}!", market.resource_type, market.city_id),
                )?;
                event_ids.push(event_id);
            }

            // Check for shortages
            if market.supply < market.demand * 0.5 {
                let event_id = create_economic_event(
                    ctx,
                    world_id,
                    EconomicEventType::ResourceShortage,
                    Some(market.resource_type),
                    vec![market.city_id],
                    hour,
                    format!("Critical shortage of {:?} in city {}", market.resource_type, market.city_id),
                )?;
                event_ids.push(event_id);
            }
        }
    }

    Ok(event_ids)
}

// Helper to create economic events
fn create_economic_event(
    ctx: &ReducerContext,
    world_id: u32,
    event_type: EconomicEventType,
    resource: Option<ResourceType>,
    affected_cities: Vec<u32>,
    hour: u64,
    description: String,
) -> Result<u32, String> {
    let event_id = ctx.db.economic_event().iter().count() as u32 + 1;

    let event = EconomicEvent {
        id: event_id,
        world_id,
        event_type,
        affected_cities: serde_json::to_string(&affected_cities).unwrap(),
        affected_resource: resource,
        impact_magnitude: 1.0,
        start_hour: hour,
        duration_hours: 24,
        description,
    };

    ctx.db.economic_event().insert(event);

    Ok(event_id)
}