/**
 * Core SpacetimeDB client wrapper for World Simulation
 * Handles connection management, subscriptions, and basic queries
 */

import { DbConnectionBuilder, DbConnectionImpl, Identity } from '@clockworklabs/spacetimedb-sdk';
import {
  ConnectionConfig,
  ClientStatus,
  SimulationClientError,
  SimulationTime,
  Individual,
  Building,
  City,
  MovementEvent,
  WorkEvent,
  SocialEvent,
  BuildingEvent,
  CityEvent,
  NeedFulfillmentEvent,
  AutotickerConfig
} from './types.js';

/**
 * Core SpacetimeDB client for World Simulation
 * Manages connections, subscriptions, and provides low-level query methods
 */
export class SpacetimeClient {
  private client: DbConnectionImpl | null = null;
  private config: ConnectionConfig;
  private status: ClientStatus;
  private eventHandlers: Map<string, Function[]> = new Map();

  constructor(config: Partial<ConnectionConfig> = {}) {
    this.config = {
      url: 'ws://localhost:3001',
      auto_reconnect: true,
      reconnect_delay: 2000,
      max_reconnect_attempts: 5,
      timeout: 10000,
      ...config
    };

    this.status = {
      connected: false,
      server_url: this.config.url,
      last_ping: 0,
      subscription_count: 0,
      query_count: 0
    };
  }

  /**
   * Connect to the SpacetimeDB server
   */
  async connect(): Promise<void> {
    try {
      this.client = new SpacetimeDBClient();
      
      // Set up event handlers
      this.setupEventHandlers();
      
      // Connect to the server
      await this.client.connect(this.config.url);
      
      this.status.connected = true;
      this.status.last_ping = Date.now();
      
      this.emit('connected', { url: this.config.url });
      
      console.log(`Connected to SpacetimeDB at ${this.config.url}`);
    } catch (error) {
      this.status.connected = false;
      const message = error instanceof Error ? error.message : 'Unknown connection error';
      throw new SimulationClientError(
        `Failed to connect to SpacetimeDB: ${message}`,
        'CONNECTION_FAILED',
        { url: this.config.url, error }
      );
    }
  }

  /**
   * Disconnect from the SpacetimeDB server
   */
  disconnect(): void {
    if (this.client) {
      this.client.disconnect();
      this.client = null;
    }
    
    this.status.connected = false;
    this.emit('disconnected', {});
    
    console.log('Disconnected from SpacetimeDB');
  }

  /**
   * Check if client is connected
   */
  isConnected(): boolean {
    return this.status.connected && this.client !== null;
  }

  /**
   * Get current client status
   */
  getStatus(): ClientStatus {
    return { ...this.status };
  }

  /**
   * Call a reducer function on the SpacetimeDB server
   */
  async callReducer(name: string, args: any[] = []): Promise<any> {
    if (!this.isConnected() || !this.client) {
      throw new SimulationClientError(
        'Client not connected',
        'NOT_CONNECTED'
      );
    }

    try {
      this.status.query_count++;
      const result = await this.client.call(name, args);
      return result;
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Unknown reducer error';
      throw new SimulationClientError(
        `Reducer call failed: ${message}`,
        'REDUCER_FAILED',
        { reducer: name, args, error }
      );
    }
  }

  /**
   * Subscribe to a table for real-time updates
   */
  subscribe<T>(tableName: string, callback: (rows: T[]) => void): () => void {
    if (!this.isConnected() || !this.client) {
      throw new SimulationClientError(
        'Client not connected',
        'NOT_CONNECTED'
      );
    }

    try {
      // Note: Actual SpacetimeDB SDK subscription syntax may vary
      // This is a placeholder for the expected interface
      const subscription = this.client.subscribe(tableName, callback);
      this.status.subscription_count++;
      
      // Return unsubscribe function
      return () => {
        subscription.unsubscribe();
        this.status.subscription_count--;
      };
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Unknown subscription error';
      throw new SimulationClientError(
        `Subscription failed: ${message}`,
        'SUBSCRIPTION_FAILED',
        { table: tableName, error }
      );
    }
  }

  /**
   * Query a table directly
   */
  async queryTable<T>(tableName: string, filter?: any): Promise<T[]> {
    if (!this.isConnected() || !this.client) {
      throw new SimulationClientError(
        'Client not connected',
        'NOT_CONNECTED'
      );
    }

    try {
      this.status.query_count++;
      // Note: Actual query syntax depends on SpacetimeDB SDK implementation
      const result = await this.client.query(tableName, filter);
      return result as T[];
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Unknown query error';
      throw new SimulationClientError(
        `Table query failed: ${message}`,
        'QUERY_FAILED',
        { table: tableName, filter, error }
      );
    }
  }

  // ===== BASIC SIMULATION QUERIES =====

  /**
   * Get current simulation time
   */
  async getSimulationTime(): Promise<SimulationTime | null> {
    const results = await this.queryTable<SimulationTime>('simulation_time');
    return results.length > 0 ? results[0] : null;
  }

  /**
   * Get autoticker configuration
   */
  async getAutotickerConfig(): Promise<AutotickerConfig | null> {
    const results = await this.queryTable<AutotickerConfig>('autoticker_config');
    return results.length > 0 ? results[0] : null;
  }

  /**
   * Get all individuals
   */
  async getIndividuals(): Promise<Individual[]> {
    return await this.queryTable<Individual>('individual');
  }

  /**
   * Get all buildings
   */
  async getBuildings(): Promise<Building[]> {
    return await this.queryTable<Building>('building');
  }

  /**
   * Get all cities
   */
  async getCities(): Promise<City[]> {
    return await this.queryTable<City>('city');
  }

  /**
   * Get movement events
   */
  async getMovementEvents(limit?: number): Promise<MovementEvent[]> {
    const events = await this.queryTable<MovementEvent>('movement_event');
    return limit ? events.slice(-limit) : events;
  }

  /**
   * Get work events
   */
  async getWorkEvents(limit?: number): Promise<WorkEvent[]> {
    const events = await this.queryTable<WorkEvent>('work_event');
    return limit ? events.slice(-limit) : events;
  }

  /**
   * Get social events
   */
  async getSocialEvents(limit?: number): Promise<SocialEvent[]> {
    const events = await this.queryTable<SocialEvent>('social_event');
    return limit ? events.slice(-limit) : events;
  }

  /**
   * Get building events
   */
  async getBuildingEvents(limit?: number): Promise<BuildingEvent[]> {
    const events = await this.queryTable<BuildingEvent>('building_event');
    return limit ? events.slice(-limit) : events;
  }

  /**
   * Get city events
   */
  async getCityEvents(limit?: number): Promise<CityEvent[]> {
    const events = await this.queryTable<CityEvent>('city_event');
    return limit ? events.slice(-limit) : events;
  }

  /**
   * Get need fulfillment events
   */
  async getNeedFulfillmentEvents(limit?: number): Promise<NeedFulfillmentEvent[]> {
    const events = await this.queryTable<NeedFulfillmentEvent>('need_fulfillment_event');
    return limit ? events.slice(-limit) : events;
  }

  // ===== REDUCER CALLS =====

  /**
   * Get current simulation hour
   */
  async getCurrentHour(): Promise<number> {
    await this.callReducer('get_current_hour');
    const simTime = await this.getSimulationTime();
    return simTime?.current_hour ?? 0;
  }

  /**
   * Check autoticker status
   */
  async checkAutoTick(): Promise<void> {
    await this.callReducer('check_autotick');
  }

  /**
   * Get autoticker status
   */
  async getAutotickerStatus(): Promise<void> {
    await this.callReducer('get_autoticker_status');
  }

  /**
   * Start autoticker
   */
  async startAutoticker(): Promise<void> {
    await this.callReducer('start_autoticker');
  }

  /**
   * Stop autoticker
   */
  async stopAutoticker(): Promise<void> {
    await this.callReducer('stop_autoticker');
  }

  /**
   * Set tick rate
   */
  async setTickRate(rate: string): Promise<void> {
    await this.callReducer('set_tick_rate', [rate]);
  }

  /**
   * Get individual story
   */
  async getIndividualStory(individualId: number, hoursBack: number): Promise<void> {
    await this.callReducer('get_individual_story', [individualId.toString(), hoursBack.toString()]);
  }

  /**
   * Get building story
   */
  async getBuildingStory(buildingId: number, hoursBack: number): Promise<void> {
    await this.callReducer('get_building_story', [buildingId.toString(), hoursBack.toString()]);
  }

  /**
   * Get city summary
   */
  async getCitySummary(cityId: number): Promise<void> {
    await this.callReducer('get_city_summary', [cityId.toString()]);
  }

  // ===== EVENT HANDLING =====

  /**
   * Add event listener
   */
  on(event: string, handler: Function): void {
    if (!this.eventHandlers.has(event)) {
      this.eventHandlers.set(event, []);
    }
    this.eventHandlers.get(event)!.push(handler);
  }

  /**
   * Remove event listener
   */
  off(event: string, handler: Function): void {
    const handlers = this.eventHandlers.get(event);
    if (handlers) {
      const index = handlers.indexOf(handler);
      if (index > -1) {
        handlers.splice(index, 1);
      }
    }
  }

  /**
   * Emit event
   */
  private emit(event: string, data: any): void {
    const handlers = this.eventHandlers.get(event);
    if (handlers) {
      handlers.forEach(handler => {
        try {
          handler(data);
        } catch (error) {
          console.error(`Error in event handler for ${event}:`, error);
        }
      });
    }
  }

  /**
   * Set up internal event handlers for SpacetimeDB client
   */
  private setupEventHandlers(): void {
    if (!this.client) return;

    // Handle connection events
    this.client.on('connected', () => {
      this.status.connected = true;
      this.status.last_ping = Date.now();
      this.emit('connected', {});
    });

    this.client.on('disconnected', () => {
      this.status.connected = false;
      this.emit('disconnected', {});
      
      // Auto-reconnect if enabled
      if (this.config.auto_reconnect) {
        setTimeout(() => {
          this.reconnect();
        }, this.config.reconnect_delay);
      }
    });

    // Handle errors
    this.client.on('error', (error: any) => {
      this.emit('error', error);
    });
  }

  /**
   * Attempt to reconnect
   */
  private async reconnect(): Promise<void> {
    if (this.status.connected) return;

    try {
      await this.connect();
    } catch (error) {
      console.error('Reconnection failed:', error);
      // Retry logic would go here
    }
  }
}