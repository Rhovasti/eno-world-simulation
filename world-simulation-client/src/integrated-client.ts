/**
 * Integrated World Simulation Client
 * Works with both real SpacetimeDB server and mock data
 */

import { SpacetimeDBConnection } from './spacetimedb-connection.js';

export interface SimulationConfig {
  url?: string;
  database?: string;
  useMockFallback?: boolean;
}

export class IntegratedWorldSimulation {
  private connection: SpacetimeDBConnection;
  
  constructor(config: SimulationConfig = {}) {
    this.connection = new SpacetimeDBConnection({
      url: config.url || 'http://localhost:3001',
      database: config.database || 'worldsim',
      useMockFallback: config.useMockFallback !== false
    });
  }

  async connect(): Promise<void> {
    await this.connection.connect();
    
    if (this.connection.isUsingMockData()) {
      console.log('ℹ️  Using mock data mode (real server unavailable)');
    } else {
      console.log('✅ Connected to real SpacetimeDB server');
    }
  }

  disconnect(): void {
    this.connection.disconnect();
  }

  isConnected(): boolean {
    return this.connection.isConnected();
  }

  isUsingRealServer(): boolean {
    return this.connection.isConnected() && !this.connection.isUsingMockData();
  }

  // Simulation queries with real/mock hybrid approach
  async getSimulationStatus(): Promise<any> {
    // Try real reducer first
    if (this.isUsingRealServer()) {
      try {
        const result = await this.connection.callReducer('get_simulation_status');
        if (result) return result;
      } catch (error) {
        console.log('⚠️  Real server query failed, using mock data');
      }
    }

    // Fallback to mock
    return {
      current_hour: Math.floor(Math.random() * 1000) + 100,
      auto_tick_enabled: true,
      tick_interval_ms: 1000,
      total_days: Math.floor(Math.random() * 50) + 10,
      is_running: true
    };
  }

  async getLocationState(cityName: string): Promise<any> {
    // For now, always use mock data for complex queries
    // In a real implementation, this would query tables from SpacetimeDB
    
    const valleys = ['Dawn', 'Day', 'Dusk', 'Night'];
    const timesOfDay = ['Dawn', 'Day', 'Dusk', 'Night'];
    
    return {
      city: cityName,
      valley: valleys[Math.floor(Math.random() * valleys.length)],
      population: Math.floor(Math.random() * 50000) + 5000,
      time_of_day: timesOfDay[Math.floor(Math.random() * timesOfDay.length)],
      active_buildings: Math.floor(Math.random() * 200) + 50
    };
  }

  async getLocationHistory(cityName: string, options: any = {}): Promise<any[]> {
    // Mock event data
    const events = [
      {
        description: `Market activity in ${cityName}`,
        participants: ['Trader John', 'Merchant Alice'],
        hour: Math.floor(Math.random() * 100),
        impact: { magnitude: 0.3, description: 'Economic boost' }
      },
      {
        description: `Festival celebration in ${cityName}`,
        participants: ['Mayor Smith', 'Citizens'],
        hour: Math.floor(Math.random() * 100),
        impact: { magnitude: 0.5, description: 'Cultural enrichment' }
      }
    ];
    
    return events.slice(0, Math.floor(Math.random() * events.length) + 1);
  }

  async generateNarrative(cityName: string): Promise<string> {
    const cityState = await this.getLocationState(cityName);
    const events = await this.getLocationHistory(cityName);
    
    let narrative = `${cityState.city} lies in the ${cityState.valley} Valley. `;
    narrative += `Currently, it is ${cityState.time_of_day}, and the city's ${cityState.population.toLocaleString()} residents are going about their daily lives.`;
    
    if (events.length > 0) {
      narrative += '\n\nRecent happenings in the city:\n';
      narrative += events.map(event => `• ${event.description}`).join('\n');
    }
    
    const serverStatus = this.isUsingRealServer() ? ' (Connected to real server)' : ' (Using mock data)';
    narrative += `\n\n[${serverStatus}]`;
    
    return narrative;
  }

  getWorldStats() {
    return {
      total_cities: 160,
      valley_distribution: {
        'Dawn': 43,
        'Day': 37,
        'Dusk': 43,
        'Night': 29
      },
      capital_cities: ['Citadel of Utaia', 'Citadel of Almo', 'Citadel of the Pass'],
      server_connected: this.isUsingRealServer()
    };
  }
}

// Test function
export async function testIntegratedClient() {
  console.log('🌍 Testing Integrated World Simulation Client\n');

  const simulation = new IntegratedWorldSimulation({
    url: 'http://localhost:3001',
    database: 'worldsim',
    useMockFallback: true
  });

  try {
    await simulation.connect();
    
    console.log('\n📊 Simulation Status:');
    const status = await simulation.getSimulationStatus();
    console.log(`   Current Hour: ${status.current_hour}`);
    console.log(`   Auto-tick: ${status.auto_tick_enabled}`);
    console.log(`   Server Mode: ${simulation.isUsingRealServer() ? 'Real SpacetimeDB' : 'Mock Data'}`);

    console.log('\n🏙️ City Information (Tsin):');
    const cityState = await simulation.getLocationState('Tsin');
    console.log(`   Valley: ${cityState.valley}`);
    console.log(`   Population: ${cityState.population.toLocaleString()}`);
    console.log(`   Time of Day: ${cityState.time_of_day}`);

    console.log('\n📖 Generated Narrative:');
    const narrative = await simulation.generateNarrative('Citadel of Utaia');
    console.log(narrative);

    console.log('\n🌍 World Statistics:');
    const stats = simulation.getWorldStats();
    console.log(`   Total Cities: ${stats.total_cities}`);
    console.log(`   Server Connected: ${stats.server_connected ? 'Yes' : 'No (Mock Mode)'}`);

    simulation.disconnect();
    console.log('\n✅ Test completed successfully!');

  } catch (error) {
    console.error('❌ Error:', error);
  }
}

export default IntegratedWorldSimulation;