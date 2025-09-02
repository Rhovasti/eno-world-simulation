/**
 * Simplified Build for World Simulation Client
 * Basic working version for demo purposes
 */
export interface SimulationStatus {
    current_hour: number;
    auto_tick_enabled: boolean;
    tick_interval_ms: number;
    total_days: number;
}
export interface LocationState {
    city: string;
    valley: string;
    population: number;
    time_of_day: string;
    active_buildings: number;
}
export interface HistoricalEvent {
    description: string;
    participants: string[];
    hour: number;
    impact: {
        magnitude: number;
        description: string;
    };
}
export declare class WorldSimulation {
    private serverUrl;
    private connected;
    constructor(serverUrl?: string);
    connect(): Promise<void>;
    disconnect(): void;
    isConnected(): boolean;
    getSimulationStatus(): Promise<SimulationStatus>;
    getLocationState(cityName: string): Promise<LocationState>;
    getLocationHistory(cityName: string, options?: any): Promise<HistoricalEvent[]>;
    getWorldStats(): {
        total_cities: number;
        valley_distribution: {
            Dawn: number;
            Day: number;
            Dusk: number;
            Night: number;
        };
        capital_cities: string[];
    };
    generateNarrative(cityName: string): Promise<string>;
}
export default WorldSimulation;
