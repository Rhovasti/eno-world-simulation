/**
 * Time Management Example for World Simulation Client
 * Demonstrates time queries, calculations, and valley time zones
 */
import { WorldSimulation, Valley, formatSimulationDate, calculateTimeOfDayForValley, getValleyTimeZones, createTimeRange, getRelativeTimeDescription, TICK_RATES, calculateRealTimeDuration, formatRealTimeDuration } from '../index.js';
async function timeManagementExample() {
    console.log('⏰ World Simulation Client - Time Management Example\n');
    const simulation = new WorldSimulation('ws://localhost:3001');
    try {
        await simulation.connect();
        console.log('✅ Connected to simulation\n');
        // Get current time information
        const currentHour = await simulation.client.getCurrentHour();
        const timeStatus = await simulation.client.getSimulationStatus();
        console.log('🕐 Current Time Information:');
        console.log(`- Simulation Hour: ${currentHour}`);
        console.log(`- Formatted Date: ${formatSimulationDate(currentHour)}`);
        console.log(`- Auto-tick Status: ${timeStatus.auto_tick_enabled ? 'Running' : 'Stopped'}`);
        console.log(`- Tick Interval: ${timeStatus.tick_interval_ms}ms\n`);
        // Show time zones across all valleys
        console.log('🌍 Valley Time Zones:');
        const timeZones = getValleyTimeZones(currentHour % 24);
        timeZones.forEach(zone => {
            const sign = zone.offset_hours >= 0 ? '+' : '';
            console.log(`${zone.valley} Valley: ${zone.current_time_of_day} (${sign}${zone.offset_hours}h) - ${zone.description}`);
        });
        console.log('');
        // Demonstrate time calculations
        console.log('🧮 Time Calculations:');
        // Show real-time equivalents for different tick rates
        console.log('Real-time equivalents for 1 simulation day (24 hours):');
        Object.entries(TICK_RATES).forEach(([rateName, config]) => {
            const duration = calculateRealTimeDuration(24, rateName);
            const formatted = formatRealTimeDuration(duration);
            console.log(`- ${config.name}: ${formatted} (${config.description})`);
        });
        console.log('');
        // Time range queries
        console.log('📅 Time Range Queries:');
        const timeRange = createTimeRange(currentHour, 48); // Last 48 hours
        console.log(`Querying events from hour ${timeRange.start_hour} to ${timeRange.end_hour}`);
        console.log(`Start: ${formatSimulationDate(timeRange.start_hour)}`);
        console.log(`End: ${formatSimulationDate(timeRange.end_hour)}\n`);
        // Get historical events across different time periods
        console.log('📚 Historical Analysis:');
        const testCity = 'Tsin';
        const timePeriods = [
            { name: 'Last 6 hours', hours: 6 },
            { name: 'Last day', hours: 24 },
            { name: 'Last week', hours: 168 }
        ];
        for (const period of timePeriods) {
            const events = await simulation.client.getLocationHistory(testCity, { hours_back: period.hours });
            console.log(`${period.name} in ${testCity}: ${events.length} events`);
            if (events.length > 0) {
                const latestEvent = events[0];
                const timeDesc = getRelativeTimeDescription(latestEvent.hour, currentHour);
                console.log(`  Latest: "${latestEvent.description}" (${timeDesc})`);
            }
        }
        console.log('');
        // Valley-specific time analysis
        console.log('🏔️  Valley Time Analysis:');
        const valleys = [Valley.Dawn, Valley.Day, Valley.Dusk, Valley.Night];
        for (const valley of valleys) {
            const timeOfDay = calculateTimeOfDayForValley(valley, currentHour % 24);
            // Get a representative city from this valley
            const citiesInValley = {
                [Valley.Dawn]: 'Tsin',
                [Valley.Day]: 'Beitsa',
                [Valley.Dusk]: 'Jouy',
                [Valley.Night]: 'Palwede'
            };
            const cityName = citiesInValley[valley];
            const cityState = await simulation.client.getLocationState(cityName);
            console.log(`${valley} Valley (${cityName}):`);
            console.log(`  Time of Day: ${timeOfDay}`);
            console.log(`  Population Activity: ${cityState.population} residents`);
            console.log(`  Active Buildings: ${cityState.active_buildings}`);
        }
        console.log('');
        // Future time predictions
        console.log('🔮 Future Time Predictions:');
        if (timeStatus.auto_tick_enabled) {
            const hoursAhead = [1, 6, 24, 168]; // 1 hour, 6 hours, 1 day, 1 week
            console.log('Predicted future times:');
            hoursAhead.forEach(hours => {
                const futureHour = currentHour + hours;
                const futureDate = formatSimulationDate(futureHour);
                const realTimeMs = hours * timeStatus.tick_interval_ms;
                const realTimeFormatted = formatRealTimeDuration(realTimeMs);
                console.log(`+${hours}h: ${futureDate} (in ${realTimeFormatted} real time)`);
            });
        }
        else {
            console.log('Auto-tick is disabled - time advancement predictions not available');
        }
    }
    catch (error) {
        console.error('❌ Error:', error);
    }
    finally {
        simulation.disconnect();
        console.log('\n📴 Disconnected from simulation');
    }
}
// Utility function for AI agents to get time context
export async function getTimeContext() {
    const simulation = new WorldSimulation();
    await simulation.connect();
    try {
        const currentHour = await simulation.client.getCurrentHour();
        const status = await simulation.client.getSimulationStatus();
        const timeZones = getValleyTimeZones(currentHour % 24);
        return {
            current_hour: currentHour,
            formatted_date: formatSimulationDate(currentHour),
            auto_tick_enabled: status.auto_tick_enabled,
            tick_interval_ms: status.tick_interval_ms,
            valley_time_zones: timeZones,
            simulation_status: status
        };
    }
    finally {
        simulation.disconnect();
    }
}
// Run the example
if (import.meta.url === `file://${process.argv[1]}`) {
    timeManagementExample().catch(console.error);
}
export { timeManagementExample };
//# sourceMappingURL=time-management.js.map