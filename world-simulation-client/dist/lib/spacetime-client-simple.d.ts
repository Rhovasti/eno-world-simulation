/**
 * Simplified SpacetimeDB client wrapper for World Simulation
 * Basic implementation for demo purposes
 */
import { ConnectionConfig, ClientStatus, SimulationTime, Individual, Building, City, MovementEvent, WorkEvent, SocialEvent, BuildingEvent, CityEvent, NeedFulfillmentEvent, AutotickerConfig } from './types';
/**
 * Simplified SpacetimeDB client for World Simulation
 */
export declare class SpacetimeClient {
    private client;
    private config;
    private status;
    constructor(config?: Partial<ConnectionConfig>);
    connect(): Promise<void>;
    disconnect(): void;
    isConnected(): boolean;
    getStatus(): ClientStatus;
    callReducer(name: string, args?: any[]): Promise<any>;
    getSimulationTime(): Promise<SimulationTime[]>;
    getIndividuals(): Promise<Individual[]>;
    getBuildings(): Promise<Building[]>;
    getCities(): Promise<City[]>;
    getMovementEvents(): Promise<MovementEvent[]>;
    getWorkEvents(): Promise<WorkEvent[]>;
    getSocialEvents(): Promise<SocialEvent[]>;
    getBuildingEvents(): Promise<BuildingEvent[]>;
    getCityEvents(): Promise<CityEvent[]>;
    getNeedFulfillmentEvents(): Promise<NeedFulfillmentEvent[]>;
    getAutotickerConfig(): Promise<AutotickerConfig[]>;
    subscribe<T>(tableName: string, callback: (rows: T[]) => void): () => void;
    on(event: string, handler: Function): void;
    off(event: string, handler?: Function): void;
    private eventHandlers;
    private emit;
}
//# sourceMappingURL=spacetime-client-simple.d.ts.map