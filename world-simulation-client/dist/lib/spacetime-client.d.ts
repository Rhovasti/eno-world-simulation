/**
 * Core SpacetimeDB client wrapper for World Simulation
 * Handles connection management, subscriptions, and basic queries
 */
import { ConnectionConfig, ClientStatus, SimulationTime, Individual, Building, City, MovementEvent, WorkEvent, SocialEvent, BuildingEvent, CityEvent, NeedFulfillmentEvent, AutotickerConfig } from './types.js';
/**
 * Core SpacetimeDB client for World Simulation
 * Manages connections, subscriptions, and provides low-level query methods
 */
export declare class SpacetimeClient {
    private client;
    private config;
    private status;
    private eventHandlers;
    constructor(config?: Partial<ConnectionConfig>);
    /**
     * Connect to the SpacetimeDB server
     */
    connect(): Promise<void>;
    /**
     * Disconnect from the SpacetimeDB server
     */
    disconnect(): void;
    /**
     * Check if client is connected
     */
    isConnected(): boolean;
    /**
     * Get current client status
     */
    getStatus(): ClientStatus;
    /**
     * Call a reducer function on the SpacetimeDB server
     */
    callReducer(name: string, args?: any[]): Promise<any>;
    /**
     * Subscribe to a table for real-time updates
     */
    subscribe<T>(tableName: string, callback: (rows: T[]) => void): () => void;
    /**
     * Query a table directly
     */
    queryTable<T>(tableName: string, filter?: any): Promise<T[]>;
    /**
     * Get current simulation time
     */
    getSimulationTime(): Promise<SimulationTime | null>;
    /**
     * Get autoticker configuration
     */
    getAutotickerConfig(): Promise<AutotickerConfig | null>;
    /**
     * Get all individuals
     */
    getIndividuals(): Promise<Individual[]>;
    /**
     * Get all buildings
     */
    getBuildings(): Promise<Building[]>;
    /**
     * Get all cities
     */
    getCities(): Promise<City[]>;
    /**
     * Get movement events
     */
    getMovementEvents(limit?: number): Promise<MovementEvent[]>;
    /**
     * Get work events
     */
    getWorkEvents(limit?: number): Promise<WorkEvent[]>;
    /**
     * Get social events
     */
    getSocialEvents(limit?: number): Promise<SocialEvent[]>;
    /**
     * Get building events
     */
    getBuildingEvents(limit?: number): Promise<BuildingEvent[]>;
    /**
     * Get city events
     */
    getCityEvents(limit?: number): Promise<CityEvent[]>;
    /**
     * Get need fulfillment events
     */
    getNeedFulfillmentEvents(limit?: number): Promise<NeedFulfillmentEvent[]>;
    /**
     * Get current simulation hour
     */
    getCurrentHour(): Promise<number>;
    /**
     * Check autoticker status
     */
    checkAutoTick(): Promise<void>;
    /**
     * Get autoticker status
     */
    getAutotickerStatus(): Promise<void>;
    /**
     * Start autoticker
     */
    startAutoticker(): Promise<void>;
    /**
     * Stop autoticker
     */
    stopAutoticker(): Promise<void>;
    /**
     * Set tick rate
     */
    setTickRate(rate: string): Promise<void>;
    /**
     * Get individual story
     */
    getIndividualStory(individualId: number, hoursBack: number): Promise<void>;
    /**
     * Get building story
     */
    getBuildingStory(buildingId: number, hoursBack: number): Promise<void>;
    /**
     * Get city summary
     */
    getCitySummary(cityId: number): Promise<void>;
    /**
     * Add event listener
     */
    on(event: string, handler: Function): void;
    /**
     * Remove event listener
     */
    off(event: string, handler: Function): void;
    /**
     * Emit event
     */
    private emit;
    /**
     * Set up internal event handlers for SpacetimeDB client
     */
    private setupEventHandlers;
    /**
     * Attempt to reconnect
     */
    private reconnect;
}
//# sourceMappingURL=spacetime-client.d.ts.map