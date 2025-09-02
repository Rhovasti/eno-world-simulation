/**
 * High-level query interface for World Simulation
 * Provides convenient methods for common simulation queries
 */
import { SpacetimeClient } from './spacetime-client-simple';
import { Valley, TimeOfDay, SimulationClientError } from './types';
/**
 * Main query interface for World Simulation
 * Provides high-level methods for querying simulation data
 */
export class WorldSimulationClient {
    constructor(config = {}) {
        this.cache = new Map();
        this.DEFAULT_CACHE_TTL = 30000; // 30 seconds
        this.spacetimeClient = new SpacetimeClient(config);
    }
    // ===== CONNECTION MANAGEMENT =====
    /**
     * Connect to the simulation server
     */
    async connect(serverUrl) {
        if (serverUrl) {
            this.spacetimeClient = new SpacetimeClient({
                ...this.spacetimeClient.getStatus(),
                url: serverUrl
            });
        }
        await this.spacetimeClient.connect();
    }
    /**
     * Disconnect from the simulation server
     */
    disconnect() {
        this.spacetimeClient.disconnect();
        this.cache.clear();
    }
    /**
     * Check if connected to the simulation server
     */
    isConnected() {
        return this.spacetimeClient.isConnected();
    }
    /**
     * Get client status
     */
    getStatus() {
        return this.spacetimeClient.getStatus();
    }
    // ===== REAL-TIME QUERIES =====
    /**
     * Get current simulation hour
     */
    async getCurrentHour() {
        const cacheKey = 'current_hour';
        // Check cache first (short TTL for time data)
        const cached = this.getFromCache(cacheKey, 5000);
        if (cached !== null)
            return cached;
        const result = await this.spacetimeClient.getCurrentHour();
        this.setCache(cacheKey, result, 5000);
        return result;
    }
    /**
     * Get current simulation status
     */
    async getSimulationStatus() {
        const simTime = await this.spacetimeClient.getSimulationTime();
        if (!simTime) {
            throw new SimulationClientError('Simulation not initialized', 'SIMULATION_NOT_INITIALIZED');
        }
        return {
            current_hour: simTime.current_hour,
            time_of_day: this.calculateTimeOfDay(simTime.hour_of_day),
            day_of_week: simTime.day_of_week,
            total_days: simTime.total_days,
            simulation_running: simTime.is_running
        };
    }
    /**
     * Get current state of a specific location
     */
    async getLocationState(cityName, valley) {
        const cacheKey = `location_state_${cityName}`;
        const cached = this.getFromCache(cacheKey);
        if (cached !== null)
            return cached;
        // Get city data
        const cities = await this.spacetimeClient.getCities();
        const city = cities.find(c => c.name.toLowerCase() === cityName.toLowerCase());
        if (!city) {
            throw new SimulationClientError(`City not found: ${cityName}`, 'CITY_NOT_FOUND', { cityName });
        }
        // Get buildings in the city
        const buildings = await this.spacetimeClient.getBuildings();
        const cityBuildings = buildings.filter(b => b.city_id === city.id);
        // Get recent events
        const recentEvents = await this.getRecentEventsForCity(city.id, 6); // Last 6 hours
        // Get current simulation time
        const timeContext = await this.getSimulationStatus();
        // Determine valley and time of day
        const actualValley = valley || city.valley;
        const timeOfDay = this.calculateTimeOfDayForValley(actualValley, timeContext.hour_of_day);
        const locationState = {
            city: city.name,
            valley: actualValley,
            current_hour: timeContext.current_hour,
            time_of_day: timeOfDay,
            population: city.population,
            active_buildings: cityBuildings,
            recent_events: recentEvents,
            city_metrics: {
                stability: city.stability,
                culture: city.culture,
                prosperity: city.prosperity,
                safety: city.safety,
                sustainability: city.sustainability
            }
        };
        this.setCache(cacheKey, locationState);
        return locationState;
    }
    // ===== HISTORICAL QUERIES =====
    /**
     * Get historical events for a location
     */
    async getLocationHistory(cityName, timeRange, options = {}) {
        const cities = await this.spacetimeClient.getCities();
        const city = cities.find(c => c.name.toLowerCase() === cityName.toLowerCase());
        if (!city) {
            throw new SimulationClientError(`City not found: ${cityName}`, 'CITY_NOT_FOUND', { cityName });
        }
        const hoursBack = timeRange.hours_back ?? 24;
        const events = await this.getRecentEventsForCity(city.id, hoursBack);
        // Filter by event types if specified
        let filteredEvents = events;
        if (timeRange.event_types && timeRange.event_types.length > 0) {
            filteredEvents = events.filter(event => timeRange.event_types.includes(event.type));
        }
        // Apply limits
        const maxResults = options.max_results ?? 100;
        return filteredEvents.slice(0, maxResults);
    }
    /**
     * Get story for an individual
     */
    async getIndividualStory(individualId, hoursBack = 24) {
        // Call the reducer (note: this may not return data directly)
        await this.spacetimeClient.getIndividualStory(individualId, hoursBack);
        // Get events related to this individual
        const allEvents = await this.getAllRecentEvents(hoursBack);
        return allEvents.filter(event => event.participants.some(p => p.includes(individualId.toString())));
    }
    /**
     * Get story for a building
     */
    async getBuildingStory(buildingId, hoursBack = 24) {
        // Call the reducer
        await this.spacetimeClient.getBuildingStory(buildingId, hoursBack);
        // Get building events
        const buildingEvents = await this.spacetimeClient.getBuildingEvents();
        const recentBuildingEvents = buildingEvents
            .filter(event => event.building_id === buildingId)
            .slice(-hoursBack);
        return recentBuildingEvents.map(event => ({
            type: 'building',
            timestamp: Date.now() - ((await this.getCurrentHour() - event.hour) * 3600000),
            hour: event.hour,
            location: `Building ${buildingId}`,
            participants: [`Building ${buildingId}`],
            description: `${event.event_type}: ${event.description}`,
            impact: {
                description: `Impact value: ${event.impact_value}`,
                magnitude: Math.abs(event.impact_value),
                affected_entities: [`Building ${buildingId}`]
            },
            raw_data: event
        }));
    }
    /**
     * Get summary for a city
     */
    async getCitySummary(cityId) {
        // Call the reducer
        await this.spacetimeClient.getCitySummary(cityId);
        // Get city data
        const cities = await this.spacetimeClient.getCities();
        return cities.find(c => c.id === cityId) || null;
    }
    // ===== ENTITY QUERIES =====
    /**
     * Get individuals at a specific location
     */
    async getIndividualsAtLocation(cityName) {
        const cities = await this.spacetimeClient.getCities();
        const city = cities.find(c => c.name.toLowerCase() === cityName.toLowerCase());
        if (!city) {
            throw new SimulationClientError(`City not found: ${cityName}`, 'CITY_NOT_FOUND', { cityName });
        }
        const individuals = await this.spacetimeClient.getIndividuals();
        const buildings = await this.spacetimeClient.getBuildings();
        const cityBuildings = buildings.filter(b => b.city_id === city.id);
        const cityBuildingIds = new Set(cityBuildings.map(b => b.id));
        return individuals.filter(individual => (individual.home_id && cityBuildingIds.has(individual.home_id)) ||
            (individual.workplace_id && cityBuildingIds.has(individual.workplace_id)));
    }
    /**
     * Get buildings in a city
     */
    async getBuildingsInCity(cityName) {
        const cities = await this.spacetimeClient.getCities();
        const city = cities.find(c => c.name.toLowerCase() === cityName.toLowerCase());
        if (!city) {
            throw new SimulationClientError(`City not found: ${cityName}`, 'CITY_NOT_FOUND', { cityName });
        }
        const buildings = await this.spacetimeClient.getBuildings();
        return buildings.filter(b => b.city_id === city.id);
    }
    // ===== UTILITY METHODS =====
    /**
     * Calculate time of day based on hour
     */
    calculateTimeOfDay(hour) {
        if (hour >= 5 && hour < 12)
            return TimeOfDay.Dawn;
        if (hour >= 12 && hour < 17)
            return TimeOfDay.Day;
        if (hour >= 17 && hour < 21)
            return TimeOfDay.Dusk;
        return TimeOfDay.Night;
    }
    /**
     * Calculate time of day for a specific valley
     */
    calculateTimeOfDayForValley(valley, hour) {
        const baseTimeOfDay = this.calculateTimeOfDay(hour);
        // Valleys have different time zones
        switch (valley) {
            case Valley.Day:
                return baseTimeOfDay;
            case Valley.Night:
                // Opposite of Day valley
                switch (baseTimeOfDay) {
                    case TimeOfDay.Day: return TimeOfDay.Night;
                    case TimeOfDay.Night: return TimeOfDay.Day;
                    case TimeOfDay.Dawn: return TimeOfDay.Dusk;
                    case TimeOfDay.Dusk: return TimeOfDay.Dawn;
                }
                break;
            case Valley.Dawn:
                // 6 hours ahead of Day valley
                switch (baseTimeOfDay) {
                    case TimeOfDay.Day: return TimeOfDay.Dusk;
                    case TimeOfDay.Dusk: return TimeOfDay.Night;
                    case TimeOfDay.Night: return TimeOfDay.Dawn;
                    case TimeOfDay.Dawn: return TimeOfDay.Day;
                }
                break;
            case Valley.Dusk:
                // 6 hours behind Day valley
                switch (baseTimeOfDay) {
                    case TimeOfDay.Day: return TimeOfDay.Dawn;
                    case TimeOfDay.Dawn: return TimeOfDay.Night;
                    case TimeOfDay.Night: return TimeOfDay.Dusk;
                    case TimeOfDay.Dusk: return TimeOfDay.Day;
                }
                break;
        }
        return baseTimeOfDay;
    }
    /**
     * Get recent events for a city
     */
    async getRecentEventsForCity(cityId, hoursBack) {
        const events = [];
        const currentHour = await this.getCurrentHour();
        const cutoffHour = currentHour - hoursBack;
        // Get city events
        const cityEvents = await this.spacetimeClient.getCityEvents();
        cityEvents
            .filter(event => event.city_id === cityId && event.hour >= cutoffHour)
            .forEach(event => {
            events.push({
                type: 'city',
                timestamp: Date.now() - ((currentHour - event.hour) * 3600000),
                hour: event.hour,
                location: `City ${cityId}`,
                participants: [`${event.participants} participants`],
                description: `${event.event_type}: ${event.description}`,
                impact: {
                    description: `Stability: ${event.impact_stability}, Culture: ${event.impact_culture}`,
                    magnitude: Math.abs(event.impact_stability) + Math.abs(event.impact_culture),
                    affected_entities: [`City ${cityId}`]
                },
                raw_data: event
            });
        });
        // Get building events for buildings in this city
        const buildings = await this.spacetimeClient.getBuildings();
        const cityBuildings = buildings.filter(b => b.city_id === cityId);
        const cityBuildingIds = new Set(cityBuildings.map(b => b.id));
        const buildingEvents = await this.spacetimeClient.getBuildingEvents();
        buildingEvents
            .filter(event => cityBuildingIds.has(event.building_id) && event.hour >= cutoffHour)
            .forEach(event => {
            events.push({
                type: 'building',
                timestamp: Date.now() - ((currentHour - event.hour) * 3600000),
                hour: event.hour,
                location: `Building ${event.building_id}`,
                participants: [`Building ${event.building_id}`],
                description: `${event.event_type}: ${event.description}`,
                impact: {
                    description: `Impact value: ${event.impact_value}`,
                    magnitude: Math.abs(event.impact_value),
                    affected_entities: [`Building ${event.building_id}`]
                },
                raw_data: event
            });
        });
        // Sort by hour (most recent first)
        return events.sort((a, b) => b.hour - a.hour);
    }
    /**
     * Get all recent events
     */
    async getAllRecentEvents(hoursBack) {
        const events = [];
        const currentHour = await this.getCurrentHour();
        const cutoffHour = currentHour - hoursBack;
        // Get movement events
        const movementEvents = await this.spacetimeClient.getMovementEvents();
        movementEvents
            .filter(event => event.hour >= cutoffHour)
            .forEach(event => {
            events.push({
                type: 'movement',
                timestamp: Date.now() - ((currentHour - event.hour) * 3600000),
                hour: event.hour,
                location: `From ${event.from_location_id} to ${event.to_location_id}`,
                participants: [`Individual ${event.individual_id}`],
                description: `Travel for ${event.reason} (${event.travel_time} minutes)`,
                impact: {
                    description: `Individual movement`,
                    magnitude: 1,
                    affected_entities: [`Individual ${event.individual_id}`]
                },
                raw_data: event
            });
        });
        // Get social events
        const socialEvents = await this.spacetimeClient.getSocialEvents();
        socialEvents
            .filter(event => event.hour >= cutoffHour)
            .forEach(event => {
            events.push({
                type: 'social',
                timestamp: Date.now() - ((currentHour - event.hour) * 3600000),
                hour: event.hour,
                location: `Location ${event.location_id}`,
                participants: [`Individual ${event.individual1_id}`, `Individual ${event.individual2_id}`],
                description: `${event.interaction_type} (relationship change: ${event.relationship_change})`,
                impact: {
                    description: `Social interaction`,
                    magnitude: Math.abs(event.relationship_change),
                    affected_entities: [`Individual ${event.individual1_id}`, `Individual ${event.individual2_id}`]
                },
                raw_data: event
            });
        });
        // Get work events  
        const workEvents = await this.spacetimeClient.getWorkEvents();
        workEvents
            .filter(event => event.hour >= cutoffHour)
            .forEach(event => {
            events.push({
                type: 'work',
                timestamp: Date.now() - ((currentHour - event.hour) * 3600000),
                hour: event.hour,
                location: `Building ${event.building_id}`,
                participants: [`Individual ${event.individual_id}`],
                description: `Worked ${event.hours_worked} hours, earned ${event.wage_earned}, productivity ${event.productivity}`,
                impact: {
                    description: `Work productivity`,
                    magnitude: event.productivity,
                    affected_entities: [`Individual ${event.individual_id}`, `Building ${event.building_id}`]
                },
                raw_data: event
            });
        });
        return events.sort((a, b) => b.hour - a.hour);
    }
    // ===== CACHING =====
    /**
     * Get data from cache
     */
    getFromCache(key, ttl) {
        const cached = this.cache.get(key);
        if (!cached)
            return null;
        const effectiveTtl = ttl ?? cached.ttl;
        if (Date.now() - cached.timestamp > effectiveTtl) {
            this.cache.delete(key);
            return null;
        }
        return cached.data;
    }
    /**
     * Set data in cache
     */
    setCache(key, data, ttl) {
        this.cache.set(key, {
            data,
            timestamp: Date.now(),
            ttl: ttl ?? this.DEFAULT_CACHE_TTL
        });
    }
}
//# sourceMappingURL=query-interface.js.map