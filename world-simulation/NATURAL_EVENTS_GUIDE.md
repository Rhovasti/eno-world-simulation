# Natural Events and Seasonal Cycles Guide

## Overview

The Natural Events system provides realistic environmental simulation with weather patterns, seasonal cycles, natural disasters, and ecological changes. This system creates dynamic environmental storytelling that affects economic, political, and social systems.

## Core Components

### 1. Climate System (`natural/mod.rs`)

#### Climate States
- **Temperature**: Real-time temperature tracking with seasonal variation
- **Humidity**: 0-100% humidity levels affecting weather patterns
- **Precipitation**: Rainfall intensity affecting agriculture and flooding
- **Wind Speed**: Wind patterns affecting storms and transportation
- **Weather Patterns**: Clear, Cloudy, Rainy, Stormy, Foggy, Windy, Hot, Cold

#### Natural Events
- **Event Types**: WeatherChange, Storm, Flood, Drought, Earthquake, Fire, Plague, Migration, Harvest, ResourceDiscovery
- **Severity Levels**: Minor (1-2 importance), Moderate (3-4), Major (5-6), Catastrophic (7)
- **Duration**: Varies by event type (hours to weeks)
- **Environmental Effects**: JSON-encoded impact on air quality, soil fertility, water levels

### 2. Weather System (`natural/weather.rs`)

#### Weather Forecasting
- **Forecast Generation**: Predictive weather modeling up to 168 hours ahead
- **Confidence Levels**: Accuracy decreases with forecast distance
- **Temperature Prediction**: Daily cycles with seasonal trends
- **Precipitation Chances**: Based on current conditions and weather patterns

#### Weather Fronts
- **Front Types**: ColdFront, WarmFront, OccludedFront, StormSystem, HighPressure, LowPressure
- **Movement Simulation**: Fronts travel between regions with realistic speed
- **Effect Application**: Temperature, wind, and precipitation changes
- **Duration**: Variable impact timing based on front intensity

#### Microclimates
- **Location Types**: Urban, Forest, Mountain, Coastal, Desert, River, Lake, Agricultural
- **Local Modifiers**: Temperature, humidity, wind, and precipitation adjustments
- **Urban Heat Islands**: City warming effects
- **Elevation Effects**: Mountain altitude impacts on weather

### 3. Disaster System (`natural/disasters.rs`)

#### Risk Assessment
- **Probability Calculation**: Climate-zone specific disaster risks
- **Risk Factors**: Temperature extremes, precipitation levels, wind speed
- **Recurrence Intervals**: Time-based risk accumulation
- **Mitigation Levels**: Preparedness affecting impact severity

#### Warning System
- **Warning Levels**: Watch, Advisory, Warning, Emergency
- **Lead Time**: Variable warning periods by disaster type
- **Confidence Ratings**: Forecast accuracy for different disasters
- **Preparation Actions**: Recommended response measures

#### Disaster Types
- **Geological**: Earthquake, Volcano, Landslide, Tsunami
- **Meteorological**: Hurricane, Tornado, Blizzard, Heatwave
- **Hydrological**: Flood, Drought
- **Biological**: Plague, Wildfire
- **Astronomical**: Meteor (rare catastrophic events)

#### Response System
- **Response Types**: Evacuation, EmergencyServices, MedicalResponse, SearchAndRescue, FireSuppression, FloodControl, Relief, Reconstruction
- **Resource Allocation**: Personnel and materials for disaster response
- **Effectiveness Measurement**: Success rates and outcomes tracking

### 4. Seasonal Cycles (`natural/seasonal_cycles.rs`)

#### Seasonal Activities
- **Activity Types**: Planting, Harvest, Fishing, Hunting, Festival, Migration, Construction, Trading, Preparation, Celebration, Mourning, Worship
- **Participation Rates**: Population engagement levels
- **Economic Impact**: Resource generation and consumption effects
- **Cultural Significance**: Community importance ratings

#### Phenological Tracking
- **Species Types**: Trees, Crops, Wildflowers, Migratory_Birds, Fish, Insects, Large_Mammals, Small_Mammals
- **Phenological Phases**: BudBurst, FirstLeaf, Flowering, Fruiting, LeafFall, Migration_Arrival, Migration_Departure, Breeding, Hibernation, Emergence
- **Temperature Thresholds**: Climate triggers for biological events
- **Climate Sensitivity**: How much weather affects timing

#### Seasonal Transitions
- **Transition Periods**: Gradual seasonal changes over time
- **Progress Tracking**: 0-100% completion of seasonal shift
- **Effect Application**: Gradual implementation of seasonal modifiers
- **Event Generation**: Transition-specific environmental changes

## Integration with Other Systems

### Economic Integration
- **Market Effects**: Natural events affect supply/demand
- **Resource Availability**: Seasonal modifiers for different resources
- **Trade Impact**: Weather affecting transportation and commerce
- **Agricultural Cycles**: Harvest timing and crop yields

### Political Integration
- **Disaster Response**: Government effectiveness during crises
- **Resource Competition**: Natural scarcity creating political tension
- **Migration Pressure**: Environmental refugees and population movement
- **Emergency Powers**: Political responses to natural disasters

### Narrative Integration
- **Event Importance**: Natural events generate narrative hooks
- **Environmental Storytelling**: Weather and seasons create atmosphere
- **Crisis Narratives**: Disasters create dramatic story moments
- **Cultural Events**: Seasonal festivals and traditions

## Configuration

### Climate Zone Setup
```rust
// Initialize for different climate zones
ClimateZone::Arctic     // Cold, limited growing season
ClimateZone::Temperate  // Four distinct seasons
ClimateZone::Tropical   // Hot, wet/dry seasons
ClimateZone::Arid       // Hot, dry, limited precipitation
ClimateZone::Mediterranean // Mild, wet winters, dry summers
```

### Event Probability Tuning
- **Base Probabilities**: Annual occurrence rates (0.0-1.0)
- **Climate Modifiers**: Zone-specific risk multipliers
- **Seasonal Adjustments**: Time-of-year probability changes
- **Recurrence Factors**: Risk accumulation over time

### Severity Distribution
- **Minor Events**: 50-70% of occurrences
- **Moderate Events**: 20-30% of occurrences
- **Major Events**: 10-15% of occurrences
- **Catastrophic Events**: 2-5% of occurrences

## API Usage

### Core Initialization
```rust
// Initialize natural systems for a world
initialize_natural_systems(ctx, world_id, ClimateZone::Temperate)?;

// Initialize seasonal cycles
initialize_seasonal_cycles(ctx, world_id, ClimateZone::Temperate)?;

// Initialize disaster risk assessment
initialize_disaster_risks(ctx, world_id, ClimateZone::Temperate)?;
```

### Regular Updates (Called by Scheduler)
```rust
// Every hour: Update climate conditions
update_climate_conditions(ctx, world_id, current_hour)?;

// Every hour: Generate natural events
let events = generate_natural_events(ctx, world_id, current_hour)?;

// Every hour: Process ongoing events
process_natural_events(ctx, world_id, current_hour)?;

// Daily: Update seasonal activities and phenology
update_seasonal_activities(ctx, world_id, current_hour)?;
update_phenological_phases(ctx, world_id, current_hour)?;
```

### Weather System Usage
```rust
// Generate weather forecasts
generate_weather_forecast(ctx, world_id, current_hour, 72)?; // 3-day forecast

// Create weather fronts
generate_weather_fronts(ctx, world_id, current_hour)?;

// Update front positions
update_weather_fronts(ctx, world_id, current_hour)?;
```

### Disaster Management
```rust
// Assess current disaster risks
assess_disaster_risks(ctx, world_id, current_hour)?;

// Execute disasters when warnings trigger
execute_disaster_events(ctx, world_id, current_hour)?;
```

## Performance Considerations

### Optimization Strategies
- **Event Batching**: Process multiple events together
- **Probability Caching**: Pre-calculate seasonal probabilities
- **Region Clustering**: Group nearby areas for weather processing
- **Update Frequency**: Different systems update at different intervals

### Memory Management
- **Event Cleanup**: Remove resolved events after impact period
- **Forecast Pruning**: Delete old weather forecasts
- **Cache Expiration**: Automatic cleanup of temporary data

### Scalability
- **Multi-World Support**: Each world maintains independent natural systems
- **Configurable Detail**: Adjust complexity based on narrative needs
- **Background Processing**: Async updates for large world counts

## Narrative Impact Examples

### Seasonal Storytelling
- **Spring Awakening**: "The first buds are appearing on the trees, and migratory birds have returned to the region"
- **Summer Abundance**: "The harvest festival brings the community together to celebrate the year's bounty"
- **Autumn Preparation**: "Merchants hurry to complete trade routes before winter storms make travel dangerous"
- **Winter Hardship**: "Food stores run low as the harsh winter drags on longer than expected"

### Weather Events
- **Storm Drama**: "A powerful storm system approaches, with winds strong enough to damage buildings and disrupt communication"
- **Drought Crisis**: "The extended drought has dried up wells and threatens the survival of crops and livestock"
- **Flood Emergency**: "Rising waters force evacuations as the river overflows its banks for the first time in decades"

### Disaster Narratives
- **Earthquake Response**: "The ground shakes violently, toppling buildings and opening fissures in the earth"
- **Wildfire Threat**: "Smoke fills the air as wildfires race toward the settlement, forcing difficult evacuation decisions"
- **Hurricane Approach**: "The massive hurricane bears down on the coast with devastating winds and storm surge"

## Future Enhancements

### Advanced Weather Modeling
- **Climate Change**: Long-term environmental shifts
- **Extreme Weather**: Increased frequency of severe events
- **Ocean Currents**: Marine weather pattern influences
- **Atmospheric Modeling**: More detailed pressure and wind systems

### Ecological Complexity
- **Species Interactions**: Predator-prey dynamics
- **Ecosystem Health**: Biodiversity and habitat quality
- **Invasive Species**: Non-native species impacts
- **Conservation**: Protected areas and wildlife management

### Human Adaptation
- **Technology Development**: Weather prediction improvements
- **Infrastructure**: Disaster-resistant construction
- **Agricultural Innovation**: Drought-resistant crops
- **Migration Patterns**: Environmental adaptation strategies

This natural events system provides rich environmental storytelling opportunities while maintaining realistic climate and weather patterns that integrate seamlessly with economic, political, and social simulation systems.