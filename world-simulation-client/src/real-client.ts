/**
 * Real SpacetimeDB Client for World Simulation
 * Connects to actual SpacetimeDB server (not mock data)
 */

import WebSocket from 'ws';

interface ConnectionConfig {
  url: string;
  database: string;
  auth_token?: string;
}

interface Message {
  type: string;
  data?: any;
}

export class RealSpacetimeClient {
  private ws: WebSocket | null = null;
  private config: ConnectionConfig;
  private connected: boolean = false;
  private messageId: number = 0;
  private pendingRequests: Map<number, { resolve: Function; reject: Function }> = new Map();

  constructor(config: Partial<ConnectionConfig> = {}) {
    this.config = {
      url: 'ws://localhost:3001',
      database: 'worldsim',
      ...config
    };
  }

  async connect(): Promise<void> {
    return new Promise((resolve, reject) => {
      try {
        // Connect to SpacetimeDB WebSocket
        const wsUrl = `${this.config.url}/database/${this.config.database}/ws`;
        console.log(`Connecting to: ${wsUrl}`);
        
        this.ws = new WebSocket(wsUrl);
        
        this.ws.on('open', () => {
          console.log('WebSocket connected!');
          this.connected = true;
          
          // Send initial subscription message
          this.send({
            type: 'subscribe',
            queries: ['*'] // Subscribe to all tables
          });
          
          resolve();
        });
        
        this.ws.on('message', (data: Buffer) => {
          try {
            const message = JSON.parse(data.toString());
            console.log('Received:', message.type);
            this.handleMessage(message);
          } catch (error) {
            console.error('Failed to parse message:', error);
          }
        });
        
        this.ws.on('error', (error: Error) => {
          console.error('WebSocket error:', error);
          reject(error);
        });
        
        this.ws.on('close', () => {
          console.log('WebSocket disconnected');
          this.connected = false;
        });
        
      } catch (error) {
        reject(error);
      }
    });
  }

  disconnect(): void {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
    this.connected = false;
  }

  isConnected(): boolean {
    return this.connected && this.ws !== null && this.ws.readyState === WebSocket.OPEN;
  }

  private send(message: any): void {
    if (!this.ws || !this.connected) {
      throw new Error('Not connected to SpacetimeDB');
    }
    
    this.ws.send(JSON.stringify(message));
  }

  private handleMessage(message: Message): void {
    // Handle different message types
    switch (message.type) {
      case 'subscription_update':
        console.log('Subscription update received');
        break;
      case 'reducer_result':
        console.log('Reducer result:', message.data);
        break;
      case 'error':
        console.error('Server error:', message.data);
        break;
      default:
        console.log('Unknown message type:', message.type);
    }
  }

  // Call a reducer
  async callReducer(name: string, args: any[] = []): Promise<any> {
    const id = ++this.messageId;
    
    return new Promise((resolve, reject) => {
      this.pendingRequests.set(id, { resolve, reject });
      
      this.send({
        type: 'call_reducer',
        id,
        reducer: name,
        args
      });
      
      // Timeout after 5 seconds
      setTimeout(() => {
        if (this.pendingRequests.has(id)) {
          this.pendingRequests.delete(id);
          reject(new Error('Request timeout'));
        }
      }, 5000);
    });
  }

  // Query tables
  async query(sql: string): Promise<any> {
    const id = ++this.messageId;
    
    return new Promise((resolve, reject) => {
      this.pendingRequests.set(id, { resolve, reject });
      
      this.send({
        type: 'query',
        id,
        sql
      });
      
      setTimeout(() => {
        if (this.pendingRequests.has(id)) {
          this.pendingRequests.delete(id);
          reject(new Error('Query timeout'));
        }
      }, 5000);
    });
  }
}

// Test the real client
export async function testRealClient() {
  console.log('🌍 Testing Real SpacetimeDB Connection\n');
  
  const client = new RealSpacetimeClient({
    url: 'ws://localhost:3001',
    database: 'worldsim'
  });
  
  try {
    await client.connect();
    console.log('✅ Connected to real SpacetimeDB!\n');
    
    // Test reducer call
    console.log('Testing get_current_hour reducer...');
    const result = await client.callReducer('get_current_hour');
    console.log('Result:', result);
    
    // Test query
    console.log('\nTesting SQL query...');
    const queryResult = await client.query('SELECT * FROM simulation_time LIMIT 1');
    console.log('Query result:', queryResult);
    
    client.disconnect();
    
  } catch (error) {
    console.error('❌ Error:', error);
  }
}

// Export for use
export default RealSpacetimeClient;