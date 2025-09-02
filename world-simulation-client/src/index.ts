/**
 * World Simulation Client
 * TypeScript client library for accessing SpacetimeDB World Simulation
 * 
 * @example Basic Usage
 * ```typescript
 * import { WorldSimulationClient } from 'world-simulation-client';
 * 
 * const client = new WorldSimulationClient();
 * await client.connect('ws://localhost:3001');
 * 
 * const currentHour = await client.getCurrentHour();
 * const tsinState = await client.getLocationState('Tsin');
 * ```
 * 
 * @example AI Integration
 * ```typescript
 * import { NarrativeAPI } from 'world-simulation-client';
 * 
 * const narrative = new NarrativeAPI(client);
 * const context = await narrative.getContextForStory('Citadel of Utaia');
 * const story = await narrative.generateLocationNarrative('Tsin');
 * ```
 */

// ===== MAIN CLIENT EXPORTS =====
export { WorldSimulationClient } from './lib/query-interface';
export { SpacetimeClient } from './lib/spacetime-client-simple';

// ===== SPECIALIZED APIS =====
export { NarrativeAPI } from './api/narrative-api';

// ===== TYPE DEFINITIONS =====
export * from './lib/types';

// ===== UTILITY FUNCTIONS =====
export * from './utils/location-mapper';
export * from './utils/time-helpers';
export * from './utils/data-formatter';

// ===== CONVENIENCE CLASSES =====

/**
 * All-in-one client that combines query interface with narrative API
 * Perfect for AI agents and narrative games
 */
export class WorldSimulation {
  public client: WorldSimulationClient;
  public narrative: NarrativeAPI;

  constructor(serverUrl: string = 'ws://localhost:3001') {
    this.client = new WorldSimulationClient({ url: serverUrl });
    this.narrative = new NarrativeAPI(this.client);
  }

  /**
   * Connect to the simulation server
   */
  async connect(): Promise<void> {
    await this.client.connect();
  }

  /**
   * Disconnect from the simulation server
   */
  disconnect(): void {
    this.client.disconnect();
  }

  /**
   * Check if connected
   */
  isConnected(): boolean {
    return this.client.isConnected();
  }
}

// ===== DEFAULT EXPORT =====
export default WorldSimulation;