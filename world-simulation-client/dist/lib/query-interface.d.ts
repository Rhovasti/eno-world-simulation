/**
 * High-level query interface for World Simulation
 * Provides convenient methods for common simulation queries
 */
import { SpacetimeClient } from './spacetime-client-simple';
import { ConnectionConfig, ClientStatus, LocationState, HistoricalEvent, TimeContext, TimeRange, QueryOptions, Valley, Individual, Building, City } from './types';
/**
 * Main query interface for World Simulation
 * Provides high-level methods for querying simulation data
 */
export declare class WorldSimulationClient {
    spacetimeClient: SpacetimeClient;
    private cache;
    private readonly DEFAULT_CACHE_TTL;
    constructor(config?: Partial<ConnectionConfig>);
    /**
     * Connect to the simulation server
     */
    connect(serverUrl?: string): Promise<void>;
    /**
     * Disconnect from the simulation server
     */
    disconnect(): void;
    /**
     * Check if connected to the simulation server
     */
    isConnected(): boolean;
    /**
     * Get client status
     */
    getStatus(): ClientStatus;
    /**
     * Get current simulation hour
     */
    getCurrentHour(): Promise<number>;
    /**
     * Get current simulation status
     */
    getSimulationStatus(): Promise<TimeContext>;
    /**
     * Get current state of a specific location
     */
    getLocationState(cityName: string, valley?: Valley): Promise<LocationState>;
    /**
     * Get historical events for a location
     */
    getLocationHistory(cityName: string, timeRange: TimeRange, options?: QueryOptions): Promise<HistoricalEvent[]>;
    /**
     * Get story for an individual
     */
    getIndividualStory(individualId: number, hoursBack?: number): Promise<HistoricalEvent[]>;
    /**
     * Get story for a building
     */
    getBuildingStory(buildingId: number, hoursBack?: number): Promise<HistoricalEvent[]>;
    /**
     * Get summary for a city
     */
    getCitySummary(cityId: number): Promise<City | null>;
    /**
     * Get individuals at a specific location
     */
    getIndividualsAtLocation(cityName: string): Promise<Individual[]>;
    /**
     * Get buildings in a city
     */
    getBuildingsInCity(cityName: string): Promise<Building[]>;
    /**
     * Calculate time of day based on hour
     */
    private calculateTimeOfDay;
    /**
     * Calculate time of day for a specific valley
     */
    private calculateTimeOfDayForValley;
    /**
     * Get recent events for a city
     */
    private getRecentEventsForCity;
    /**
     * Get all recent events
     */
    private getAllRecentEvents;
    /**
     * Get data from cache
     */
    private getFromCache;
    /**
     * Set data in cache
     */
    private setCache;
}
//# sourceMappingURL=query-interface.d.ts.map