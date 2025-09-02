/**
 * World Simulation Client - Simplified Version
 * TypeScript client library for accessing SpacetimeDB World Simulation
 */
export { WorldSimulationClient } from './lib/query-interface.js';
export { SpacetimeClient } from './lib/spacetime-client-simple.js';
export * from './lib/types.js';
export * from './utils/location-mapper.js';
export * from './utils/time-helpers.js';
export * from './utils/data-formatter.js';
/**
 * Simplified client that works with basic operations
 */
export declare class WorldSimulation {
    client: any;
    constructor(serverUrl?: string);
    connect(): Promise<void>;
    disconnect(): void;
    isConnected(): boolean;
}
export default WorldSimulation;
//# sourceMappingURL=index-simple.d.ts.map