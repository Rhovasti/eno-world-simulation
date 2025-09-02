#!/usr/bin/env node

/**
 * Test the built library
 * Demonstrates importing and using the compiled TypeScript library
 */

import { WorldSimulation } from './dist/simple-build.js';

async function testBuiltLibrary() {
  console.log('🌍 Testing Built World Simulation Client Library\n');

  try {
    // Create simulation instance
    const simulation = new WorldSimulation('ws://localhost:3001');
    console.log('✅ Created simulation instance');

    // Test connection
    await simulation.connect();
    console.log('✅ Connected to simulation\n');

    // Test simulation status
    console.log('📊 Testing simulation status...');
    const status = await simulation.getSimulationStatus();
    console.log(`   Current Hour: ${status.current_hour}`);
    console.log(`   Auto-tick: ${status.auto_tick_enabled ? 'Enabled' : 'Disabled'}`);
    console.log(`   Total Days: ${status.total_days}\n`);

    // Test city information
    console.log('🏙️ Testing city information...');
    const cityState = await simulation.getLocationState('Tsin');
    console.log(`   City: ${cityState.city}`);
    console.log(`   Valley: ${cityState.valley}`);
    console.log(`   Population: ${cityState.population.toLocaleString()}`);
    console.log(`   Time of Day: ${cityState.time_of_day}`);
    console.log(`   Active Buildings: ${cityState.active_buildings}\n`);

    // Test events
    console.log('📰 Testing location history...');
    const events = await simulation.getLocationHistory('Tsin', { hours_back: 24 });
    console.log(`   Found ${events.length} recent events:`);
    events.forEach((event, index) => {
      console.log(`   ${index + 1}. ${event.description}`);
    });

    // Test world statistics
    console.log('\n🌍 Testing world statistics...');
    const worldStats = simulation.getWorldStats();
    console.log(`   Total Cities: ${worldStats.total_cities}`);
    console.log('   Valley Distribution:');
    Object.entries(worldStats.valley_distribution).forEach(([valley, count]) => {
      console.log(`     ${valley}: ${count} cities`);
    });

    // Test narrative generation
    console.log('\n📖 Testing narrative generation...');
    const narrative = await simulation.generateNarrative('Citadel of Utaia');
    console.log('Generated Narrative:');
    console.log('─'.repeat(50));
    console.log(narrative);
    console.log('─'.repeat(50));

    // Disconnect
    simulation.disconnect();
    console.log('\n✅ All tests passed! Library is working correctly.');
    
    console.log('\n🎯 Next Steps:');
    console.log('1. Open demo: http://localhost:3002/demo/simple-demo.html');
    console.log('2. Use library in your projects: import { WorldSimulation } from "world-simulation-client"');
    console.log('3. See comprehensive examples in /src/examples/');

  } catch (error) {
    console.error('❌ Test failed:', error.message);
    process.exit(1);
  }
}

testBuiltLibrary();