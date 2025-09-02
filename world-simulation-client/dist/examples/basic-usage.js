/**
 * Basic Usage Example for World Simulation Client
 * Demonstrates fundamental client operations
 */
import { WorldSimulation, formatSimulationDate } from '../index.js';
async function basicUsageExample() {
    console.log('🌍 World Simulation Client - Basic Usage Example\n');
    // Initialize the client
    const simulation = new WorldSimulation('ws://localhost:3001');
    try {
        // Connect to the simulation
        console.log('📡 Connecting to simulation...');
        await simulation.connect();
        console.log('✅ Connected successfully!\n');
        // Get current simulation status
        console.log('⏰ Current Simulation Status:');
        const status = await simulation.client.getSimulationStatus();
        console.log(`- Current Hour: ${status.current_hour}`);
        console.log(`- Formatted Date: ${formatSimulationDate(status.current_hour)}`);
        console.log(`- Auto-tick: ${status.auto_tick_enabled ? 'Enabled' : 'Disabled'}`);
        console.log(`- Tick Interval: ${status.tick_interval_ms}ms\n`);
        // Query a specific city
        console.log('🏙️  City Information:');
        const cityState = await simulation.client.getLocationState('Tsin');
        console.log(`- ${cityState.city} in the ${cityState.valley} Valley`);
        console.log(`- Population: ${cityState.population.toLocaleString()}`);
        console.log(`- Active Buildings: ${cityState.active_buildings}`);
        console.log(`- Time of Day: ${cityState.time_of_day}\n`);
        // Get recent events
        console.log('📰 Recent Events:');
        const recentEvents = await simulation.client.getLocationHistory('Tsin', { hours_back: 6 });
        if (recentEvents.length > 0) {
            recentEvents.slice(0, 3).forEach((event, index) => {
                console.log(`${index + 1}. ${event.description} (${event.participants.join(', ')})`);
            });
        }
        else {
            console.log('No recent events found.');
        }
    }
    catch (error) {
        console.error('❌ Error:', error);
    }
    finally {
        // Disconnect
        simulation.disconnect();
        console.log('\n📴 Disconnected from simulation');
    }
}
// Run the example
if (import.meta.url === `file://${process.argv[1]}`) {
    basicUsageExample().catch(console.error);
}
export { basicUsageExample };
//# sourceMappingURL=basic-usage.js.map