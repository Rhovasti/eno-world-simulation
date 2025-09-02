/**
 * World Simulation Client - Simplified Version
 * TypeScript client library for accessing SpacetimeDB World Simulation
 */

// ===== MAIN CLIENT EXPORTS =====
export { WorldSimulationClient } from './lib/query-interface.js';
export { SpacetimeClient } from './lib/spacetime-client-simple.js';

// ===== TYPE DEFINITIONS =====
export * from './lib/types.js';

// ===== UTILITY FUNCTIONS =====
export * from './utils/location-mapper.js';
export * from './utils/time-helpers.js';
export * from './utils/data-formatter.js';

// ===== CONVENIENCE CLASS =====

/**
 * Simplified client that works with basic operations
 */
export class WorldSimulation {
  public client: any;

  constructor(serverUrl: string = 'ws://localhost:3001') {
    // For now, create a basic mock client for the demo
    this.client = {
      connect: async () => {
        console.log(`Connecting to ${serverUrl}...`);
        // Mock connection for demo
        return Promise.resolve();
      },
      disconnect: () => {
        console.log('Disconnected');
      },
      isConnected: () => true,
      getSimulationStatus: async () => ({
        current_hour: 42,
        auto_tick_enabled: true,
        tick_interval_ms: 1000,
        total_days: 2
      }),
      getLocationState: async (cityName: string) => ({
        city: cityName,
        valley: 'Dawn',
        population: Math.floor(Math.random() * 10000) + 1000,
        time_of_day: 'Day',
        active_buildings: Math.floor(Math.random() * 50) + 10
      }),
      getLocationHistory: async (cityName: string, options: any) => [
        {
          description: `Market activity in ${cityName}`,
          participants: ['Trader John', 'Merchant Alice'],
          hour: 40,
          impact: { magnitude: 0.3, description: 'Economic boost' }
        }
      ]
    };
  }

  async connect(): Promise<void> {
    await this.client.connect();
  }

  disconnect(): void {
    this.client.disconnect();
  }

  isConnected(): boolean {
    return this.client.isConnected();
  }
}

// ===== DEFAULT EXPORT =====
export default WorldSimulation;