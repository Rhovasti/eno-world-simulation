/**
 * Simplified Build for World Simulation Client
 * Basic working version for demo purposes
 */

// Basic types
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

// Simple mock client
export class WorldSimulation {
  private serverUrl: string;
  private connected: boolean = false;

  constructor(serverUrl: string = 'ws://localhost:3001') {
    this.serverUrl = serverUrl;
  }

  async connect(): Promise<void> {
    console.log(`Connecting to ${this.serverUrl}...`);
    // For demo, simulate connection
    await new Promise(resolve => setTimeout(resolve, 100));
    this.connected = true;
  }

  disconnect(): void {
    this.connected = false;
  }

  isConnected(): boolean {
    return this.connected;
  }

  // Mock simulation status
  async getSimulationStatus(): Promise<SimulationStatus> {
    if (!this.connected) throw new Error('Not connected');
    
    return {
      current_hour: Math.floor(Math.random() * 1000) + 100,
      auto_tick_enabled: Math.random() > 0.5,
      tick_interval_ms: 1000,
      total_days: Math.floor(Math.random() * 50) + 10
    };
  }

  // Mock city information
  async getLocationState(cityName: string): Promise<LocationState> {
    if (!this.connected) throw new Error('Not connected');
    
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

  // Mock events
  async getLocationHistory(cityName: string, options: any = {}): Promise<HistoricalEvent[]> {
    if (!this.connected) throw new Error('Not connected');
    
    const events = [
      {
        description: `Market activity in ${cityName}`,
        participants: ['Trader John', 'Merchant Alice'],
        hour: 40,
        impact: { magnitude: 0.3, description: 'Economic boost' }
      },
      {
        description: `Festival celebration in ${cityName}`,
        participants: ['Mayor Smith', 'Citizens'],
        hour: 38,
        impact: { magnitude: 0.5, description: 'Cultural enrichment' }
      },
      {
        description: `Construction project in ${cityName}`,
        participants: ['Builder Bob', 'Architect Carol'],
        hour: 35,
        impact: { magnitude: 0.4, description: 'Infrastructure improvement' }
      }
    ];
    
    return events.slice(0, Math.floor(Math.random() * 3) + 1);
  }

  // Mock world statistics
  getWorldStats() {
    return {
      total_cities: 160,
      valley_distribution: {
        'Dawn': 43,
        'Day': 37,
        'Dusk': 43,
        'Night': 29
      },
      capital_cities: ['Citadel of Utaia', 'Citadel of Almo', 'Citadel of the Pass']
    };
  }

  // Mock narrative generation
  async generateNarrative(cityName: string): Promise<string> {
    const cityState = await this.getLocationState(cityName);
    const events = await this.getLocationHistory(cityName);
    
    let narrative = `${cityState.city} lies in the ${cityState.valley} Valley. `;
    narrative += `Currently, it is ${cityState.time_of_day}, and the city's ${cityState.population.toLocaleString()} residents are going about their daily lives.`;
    
    if (events.length > 0) {
      narrative += '\n\nRecent happenings in the city:\n';
      narrative += events.map(event => `• ${event.description}`).join('\n');
    } else {
      narrative += ' The city has been quiet recently, with residents following their usual routines.';
    }
    
    return narrative;
  }
}

// Export as default
export default WorldSimulation;