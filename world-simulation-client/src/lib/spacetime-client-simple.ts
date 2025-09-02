/**
 * Simplified SpacetimeDB client wrapper for World Simulation
 * Basic implementation for demo purposes
 */

import { DbConnectionBuilder } from '@clockworklabs/spacetimedb-sdk';
import {
  ConnectionConfig,
  ClientStatus,
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
} from './types';

/**
 * Simplified SpacetimeDB client for World Simulation
 */
export class SpacetimeClient {
  private client: any = null;
  private config: ConnectionConfig;
  private status: ClientStatus;

  constructor(config: Partial<ConnectionConfig> = {}) {
    this.config = {
      url: 'ws://localhost:3001',
      database: 'worldsim',
      reconnect: true,
      maxReconnectAttempts: 5,
      reconnectInterval: 1000,
      ...config
    };

    this.status = {
      connected: false,
      authenticated: false,
      subscriptions: [],
      lastError: null,
      reconnectAttempts: 0
    };
  }

  // ===== CONNECTION MANAGEMENT =====

  async connect(): Promise<void> {
    try {
      // Create connection using SpacetimeDB SDK
      this.client = await DbConnectionBuilder.create()
        .uri(this.config.url)
        .dbName(this.config.database)
        .build();

      this.status.connected = true;
      this.status.lastError = null;
    } catch (error) {
      this.status.connected = false;
      this.status.lastError = error instanceof Error ? error.message : 'Connection failed';
      throw new Error(`Failed to connect to SpacetimeDB: ${this.status.lastError}`);
    }
  }

  disconnect(): void {
    if (this.client) {
      this.client.close();
      this.client = null;
    }
    this.status.connected = false;
    this.status.authenticated = false;
  }

  isConnected(): boolean {
    return this.status.connected && this.client !== null;
  }

  getStatus(): ClientStatus {
    return { ...this.status };
  }

  // ===== BASIC QUERY METHODS =====

  async callReducer(name: string, args: any[] = []): Promise<any> {
    if (!this.isConnected()) {
      throw new Error('Not connected to SpacetimeDB');
    }

    try {
      return await this.client.call(name, args);
    } catch (error) {
      throw new Error(`Reducer call failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }

  // ===== SIMULATION DATA METHODS =====

  async getSimulationTime(): Promise<SimulationTime[]> {
    return this.callReducer('get_simulation_time');
  }

  async getIndividuals(): Promise<Individual[]> {
    return this.callReducer('get_individuals');
  }

  async getBuildings(): Promise<Building[]> {
    return this.callReducer('get_buildings');
  }

  async getCities(): Promise<City[]> {
    return this.callReducer('get_cities');
  }

  async getMovementEvents(): Promise<MovementEvent[]> {
    return this.callReducer('get_movement_events');
  }

  async getWorkEvents(): Promise<WorkEvent[]> {
    return this.callReducer('get_work_events');
  }

  async getSocialEvents(): Promise<SocialEvent[]> {
    return this.callReducer('get_social_events');
  }

  async getBuildingEvents(): Promise<BuildingEvent[]> {
    return this.callReducer('get_building_events');
  }

  async getCityEvents(): Promise<CityEvent[]> {
    return this.callReducer('get_city_events');
  }

  async getNeedFulfillmentEvents(): Promise<NeedFulfillmentEvent[]> {
    return this.callReducer('get_need_fulfillment_events');
  }

  async getAutotickerConfig(): Promise<AutotickerConfig[]> {
    return this.callReducer('get_autoticker_config');
  }

  // ===== SUBSCRIPTION METHODS (Simplified) =====

  subscribe<T>(tableName: string, callback: (rows: T[]) => void): () => void {
    // Simplified subscription - in a real implementation this would use SpacetimeDB subscriptions
    console.log(`Subscribing to table: ${tableName}`);
    
    // Return unsubscribe function
    return () => {
      console.log(`Unsubscribing from table: ${tableName}`);
    };
  }

  // ===== EVENT HANDLING =====

  on(event: string, handler: Function): void {
    if (!this.eventHandlers.has(event)) {
      this.eventHandlers.set(event, []);
    }
    this.eventHandlers.get(event)!.push(handler);
  }

  off(event: string, handler?: Function): void {
    if (!handler) {
      this.eventHandlers.delete(event);
      return;
    }

    const handlers = this.eventHandlers.get(event);
    if (handlers) {
      const index = handlers.indexOf(handler);
      if (index > -1) {
        handlers.splice(index, 1);
      }
    }
  }

  private eventHandlers: Map<string, Function[]> = new Map();

  private emit(event: string, ...args: any[]): void {
    const handlers = this.eventHandlers.get(event);
    if (handlers) {
      handlers.forEach(handler => {
        try {
          handler(...args);
        } catch (error) {
          console.error(`Error in event handler for ${event}:`, error);
        }
      });
    }
  }
}