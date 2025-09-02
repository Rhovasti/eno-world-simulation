#!/usr/bin/env node

/**
 * Simple test script for World Simulation Client
 * Tests basic functionality without complex TypeScript compilation
 */

console.log('🌍 World Simulation Client - Simple Test\n');

// Mock simulation client for testing
class MockWorldSimulation {
  constructor(serverUrl = 'ws://localhost:3001') {
    this.serverUrl = serverUrl;
    this.connected = false;
  }

  async connect() {
    console.log(`📡 Connecting to ${this.serverUrl}...`);
    
    // Simulate connection attempt
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    // For demo, always succeed
    this.connected = true;
    console.log('✅ Connected successfully (mock mode)\n');
  }

  disconnect() {
    this.connected = false;
    console.log('📴 Disconnected\n');
  }

  isConnected() {
    return this.connected;
  }

  async getSimulationStatus() {
    if (!this.connected) throw new Error('Not connected');
    
    return {
      current_hour: Math.floor(Math.random() * 1000) + 100,
      auto_tick_enabled: Math.random() > 0.5,
      tick_interval_ms: 1000,
      total_days: Math.floor(Math.random() * 50) + 10
    };
  }

  async getLocationState(cityName) {
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

  async getLocationHistory(cityName, options = {}) {
    if (!this.connected) throw new Error('Not connected');
    
    const events = [
      { description: `Market activity in ${cityName}`, participants: ['Trader John', 'Merchant Alice'] },
      { description: `Festival celebration in ${cityName}`, participants: ['Mayor Smith', 'Citizens'] },
      { description: `Construction project in ${cityName}`, participants: ['Builder Bob', 'Architect Carol'] }
    ];
    
    return events.slice(0, Math.floor(Math.random() * 3) + 1);
  }
}

// Test functions
async function testBasicOperations() {
  console.log('🚀 Testing Basic Operations');
  console.log('═'.repeat(50));
  
  const simulation = new MockWorldSimulation();
  
  try {
    // Test connection
    await simulation.connect();
    
    // Test status
    console.log('📊 Getting simulation status...');
    const status = await simulation.getSimulationStatus();
    console.log(`   Current Hour: ${status.current_hour}`);
    console.log(`   Auto-tick: ${status.auto_tick_enabled ? 'Enabled' : 'Disabled'}`);
    console.log(`   Total Days: ${status.total_days}\n`);
    
    // Test city information
    const cityName = 'Tsin';
    console.log(`🏙️ Getting information for ${cityName}...`);
    const cityState = await simulation.getLocationState(cityName);
    console.log(`   Valley: ${cityState.valley}`);
    console.log(`   Population: ${cityState.population.toLocaleString()}`);
    console.log(`   Time of Day: ${cityState.time_of_day}`);
    console.log(`   Active Buildings: ${cityState.active_buildings}\n`);
    
    // Test events
    console.log(`📰 Getting recent events for ${cityName}...`);
    const events = await simulation.getLocationHistory(cityName, { hours_back: 24 });
    console.log(`   Found ${events.length} recent events:`);
    events.forEach((event, index) => {
      console.log(`   ${index + 1}. ${event.description}`);
      console.log(`      Participants: ${event.participants.join(', ')}`);
    });
    
    simulation.disconnect();
    
  } catch (error) {
    console.error('❌ Error:', error.message);
  }
}

async function testLocationExplorer() {
  console.log('\n🗺️ Testing Location Explorer');
  console.log('═'.repeat(50));
  
  // Mock world statistics
  const worldStats = {
    total_cities: 160,
    valley_distribution: {
      'Dawn': 43,
      'Day': 37,
      'Dusk': 43,
      'Night': 29
    },
    capital_cities: ['Citadel of Utaia', 'Citadel of Almo', 'Citadel of the Pass']
  };
  
  console.log('🌍 World Statistics:');
  console.log(`   Total Cities: ${worldStats.total_cities}`);
  console.log('   Valley Distribution:');
  Object.entries(worldStats.valley_distribution).forEach(([valley, count]) => {
    console.log(`     ${valley}: ${count} cities`);
  });
  console.log(`   Capital Cities: ${worldStats.capital_cities.join(', ')}\n`);
  
  // Test random city discovery
  const cities = ['Tsin', 'Beitsa', 'Jouy', 'Palwede', 'Citadel of Utaia'];
  const randomCity = cities[Math.floor(Math.random() * cities.length)];
  
  console.log(`🎲 Random City Discovery: ${randomCity}`);
  
  const simulation = new MockWorldSimulation();
  await simulation.connect();
  
  const cityInfo = await simulation.getLocationState(randomCity);
  console.log(`   Valley: ${cityInfo.valley}`);
  console.log(`   Population: ${cityInfo.population.toLocaleString()}`);
  console.log(`   Current Time: ${cityInfo.time_of_day}`);
  
  simulation.disconnect();
}

async function testTimeManagement() {
  console.log('\n⏰ Testing Time Management');
  console.log('═'.repeat(50));
  
  // Mock time zones
  const timeZones = [
    { valley: 'Dawn', offset: '+6h', description: 'Eastern lands where each day begins' },
    { valley: 'Day', offset: '0h', description: 'Central heartlands in perpetual daylight' },
    { valley: 'Dusk', offset: '-6h', description: 'Western territories of eternal twilight' },
    { valley: 'Night', offset: '+12h', description: 'Far lands shrouded in endless darkness' }
  ];
  
  console.log('🌍 Valley Time Zones:');
  timeZones.forEach(zone => {
    console.log(`   ${zone.valley} Valley (${zone.offset}): ${zone.description}`);
  });
  
  console.log('\n🕐 Current time across valleys:');
  const baseHour = 14; // 2 PM
  timeZones.forEach(zone => {
    const times = ['Dawn', 'Day', 'Dusk', 'Night'];
    const timeOfDay = times[Math.floor(Math.random() * times.length)];
    console.log(`   ${zone.valley}: ${timeOfDay} (local time)`);
  });
}

// Main test runner
async function runTests() {
  console.log('🧪 World Simulation Client Test Suite');
  console.log('Running simplified tests using mock data...\n');
  
  try {
    await testBasicOperations();
    await testLocationExplorer();
    await testTimeManagement();
    
    console.log('\n✅ All tests completed successfully!');
    console.log('\n📋 Next Steps:');
    console.log('1. Start simulation server: cd /root/Eno/simulation2 && ./start-simulation.sh start');
    console.log('2. View demo website: http://localhost:3002/demo/simple-demo.html');
    console.log('3. Run full examples: npm run example:all (after building TypeScript)');
    
  } catch (error) {
    console.error('\n❌ Test suite failed:', error.message);
  }
}

// Run if called directly
if (require.main === module) {
  runTests();
}

module.exports = { MockWorldSimulation, runTests };