/**
 * Core SpacetimeDB client wrapper for World Simulation
 * Handles connection management, subscriptions, and basic queries
 */
import { SimulationClientError } from './types.js';
/**
 * Core SpacetimeDB client for World Simulation
 * Manages connections, subscriptions, and provides low-level query methods
 */
export class SpacetimeClient {
    constructor(config = {}) {
        this.client = null;
        this.eventHandlers = new Map();
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
    async connect() {
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
        }
        catch (error) {
            this.status.connected = false;
            const message = error instanceof Error ? error.message : 'Unknown connection error';
            throw new SimulationClientError(`Failed to connect to SpacetimeDB: ${message}`, 'CONNECTION_FAILED', { url: this.config.url, error });
        }
    }
    /**
     * Disconnect from the SpacetimeDB server
     */
    disconnect() {
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
    isConnected() {
        return this.status.connected && this.client !== null;
    }
    /**
     * Get current client status
     */
    getStatus() {
        return { ...this.status };
    }
    /**
     * Call a reducer function on the SpacetimeDB server
     */
    async callReducer(name, args = []) {
        if (!this.isConnected() || !this.client) {
            throw new SimulationClientError('Client not connected', 'NOT_CONNECTED');
        }
        try {
            this.status.query_count++;
            const result = await this.client.call(name, args);
            return result;
        }
        catch (error) {
            const message = error instanceof Error ? error.message : 'Unknown reducer error';
            throw new SimulationClientError(`Reducer call failed: ${message}`, 'REDUCER_FAILED', { reducer: name, args, error });
        }
    }
    /**
     * Subscribe to a table for real-time updates
     */
    subscribe(tableName, callback) {
        if (!this.isConnected() || !this.client) {
            throw new SimulationClientError('Client not connected', 'NOT_CONNECTED');
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
        }
        catch (error) {
            const message = error instanceof Error ? error.message : 'Unknown subscription error';
            throw new SimulationClientError(`Subscription failed: ${message}`, 'SUBSCRIPTION_FAILED', { table: tableName, error });
        }
    }
    /**
     * Query a table directly
     */
    async queryTable(tableName, filter) {
        if (!this.isConnected() || !this.client) {
            throw new SimulationClientError('Client not connected', 'NOT_CONNECTED');
        }
        try {
            this.status.query_count++;
            // Note: Actual query syntax depends on SpacetimeDB SDK implementation
            const result = await this.client.query(tableName, filter);
            return result;
        }
        catch (error) {
            const message = error instanceof Error ? error.message : 'Unknown query error';
            throw new SimulationClientError(`Table query failed: ${message}`, 'QUERY_FAILED', { table: tableName, filter, error });
        }
    }
    // ===== BASIC SIMULATION QUERIES =====
    /**
     * Get current simulation time
     */
    async getSimulationTime() {
        const results = await this.queryTable('simulation_time');
        return results.length > 0 ? results[0] : null;
    }
    /**
     * Get autoticker configuration
     */
    async getAutotickerConfig() {
        const results = await this.queryTable('autoticker_config');
        return results.length > 0 ? results[0] : null;
    }
    /**
     * Get all individuals
     */
    async getIndividuals() {
        return await this.queryTable('individual');
    }
    /**
     * Get all buildings
     */
    async getBuildings() {
        return await this.queryTable('building');
    }
    /**
     * Get all cities
     */
    async getCities() {
        return await this.queryTable('city');
    }
    /**
     * Get movement events
     */
    async getMovementEvents(limit) {
        const events = await this.queryTable('movement_event');
        return limit ? events.slice(-limit) : events;
    }
    /**
     * Get work events
     */
    async getWorkEvents(limit) {
        const events = await this.queryTable('work_event');
        return limit ? events.slice(-limit) : events;
    }
    /**
     * Get social events
     */
    async getSocialEvents(limit) {
        const events = await this.queryTable('social_event');
        return limit ? events.slice(-limit) : events;
    }
    /**
     * Get building events
     */
    async getBuildingEvents(limit) {
        const events = await this.queryTable('building_event');
        return limit ? events.slice(-limit) : events;
    }
    /**
     * Get city events
     */
    async getCityEvents(limit) {
        const events = await this.queryTable('city_event');
        return limit ? events.slice(-limit) : events;
    }
    /**
     * Get need fulfillment events
     */
    async getNeedFulfillmentEvents(limit) {
        const events = await this.queryTable('need_fulfillment_event');
        return limit ? events.slice(-limit) : events;
    }
    // ===== REDUCER CALLS =====
    /**
     * Get current simulation hour
     */
    async getCurrentHour() {
        await this.callReducer('get_current_hour');
        const simTime = await this.getSimulationTime();
        return simTime?.current_hour ?? 0;
    }
    /**
     * Check autoticker status
     */
    async checkAutoTick() {
        await this.callReducer('check_autotick');
    }
    /**
     * Get autoticker status
     */
    async getAutotickerStatus() {
        await this.callReducer('get_autoticker_status');
    }
    /**
     * Start autoticker
     */
    async startAutoticker() {
        await this.callReducer('start_autoticker');
    }
    /**
     * Stop autoticker
     */
    async stopAutoticker() {
        await this.callReducer('stop_autoticker');
    }
    /**
     * Set tick rate
     */
    async setTickRate(rate) {
        await this.callReducer('set_tick_rate', [rate]);
    }
    /**
     * Get individual story
     */
    async getIndividualStory(individualId, hoursBack) {
        await this.callReducer('get_individual_story', [individualId.toString(), hoursBack.toString()]);
    }
    /**
     * Get building story
     */
    async getBuildingStory(buildingId, hoursBack) {
        await this.callReducer('get_building_story', [buildingId.toString(), hoursBack.toString()]);
    }
    /**
     * Get city summary
     */
    async getCitySummary(cityId) {
        await this.callReducer('get_city_summary', [cityId.toString()]);
    }
    // ===== EVENT HANDLING =====
    /**
     * Add event listener
     */
    on(event, handler) {
        if (!this.eventHandlers.has(event)) {
            this.eventHandlers.set(event, []);
        }
        this.eventHandlers.get(event).push(handler);
    }
    /**
     * Remove event listener
     */
    off(event, handler) {
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
    emit(event, data) {
        const handlers = this.eventHandlers.get(event);
        if (handlers) {
            handlers.forEach(handler => {
                try {
                    handler(data);
                }
                catch (error) {
                    console.error(`Error in event handler for ${event}:`, error);
                }
            });
        }
    }
    /**
     * Set up internal event handlers for SpacetimeDB client
     */
    setupEventHandlers() {
        if (!this.client)
            return;
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
        this.client.on('error', (error) => {
            this.emit('error', error);
        });
    }
    /**
     * Attempt to reconnect
     */
    async reconnect() {
        if (this.status.connected)
            return;
        try {
            await this.connect();
        }
        catch (error) {
            console.error('Reconnection failed:', error);
            // Retry logic would go here
        }
    }
}
//# sourceMappingURL=spacetime-client.js.map