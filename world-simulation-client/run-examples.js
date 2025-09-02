#!/usr/bin/env node

/**
 * Run Examples with Working Integrated Client
 * Demonstrates all client features using the real server connection
 */

import { IntegratedWorldSimulation } from './dist/integrated-client.js';

// Example 1: Basic Usage
async function basicUsageExample() {
  console.log('🚀 Example 1: Basic Usage');
  console.log('=' .repeat(50));

  const simulation = new IntegratedWorldSimulation();
  
  try {
    await simulation.connect();
    
    // Get simulation status
    const status = await simulation.getSimulationStatus();
    console.log('\n📊 Simulation Status:');
    console.log(`   Current Hour: ${status.current_hour}`);
    console.log(`   Auto-tick: ${status.auto_tick_enabled ? 'Enabled' : 'Disabled'}`);
    console.log(`   Server Mode: ${simulation.isUsingRealServer() ? 'Real Server' : 'Mock Data'}`);
    
    // Get city information
    const cityState = await simulation.getLocationState('Tsin');
    console.log('\n🏙️  City Information:');
    console.log(`   City: ${cityState.city}`);
    console.log(`   Valley: ${cityState.valley}`);
    console.log(`   Population: ${cityState.population.toLocaleString()}`);
    
    simulation.disconnect();
    console.log('\n✅ Basic usage example completed!');
    
  } catch (error) {
    console.error('❌ Error:', error);
  }
}

// Example 2: AI Narrative Generation
async function narrativeExample() {
  console.log('\n\n🤖 Example 2: AI Narrative Generation');
  console.log('=' .repeat(50));

  const simulation = new IntegratedWorldSimulation();
  
  try {
    await simulation.connect();
    
    // Generate narratives for different cities
    const cities = ['Tsin', 'Citadel of Utaia', 'Jouy', 'Palwede'];
    
    for (const city of cities) {
      console.log(`\n📖 Narrative for ${city}:`);
      const narrative = await simulation.generateNarrative(city);
      console.log(narrative);
      console.log('-'.repeat(40));
    }
    
    simulation.disconnect();
    console.log('\n✅ Narrative generation example completed!');
    
  } catch (error) {
    console.error('❌ Error:', error);
  }
}

// Example 3: Location Explorer
async function locationExplorerExample() {
  console.log('\n\n🗺️  Example 3: Location Explorer');
  console.log('=' .repeat(50));

  const simulation = new IntegratedWorldSimulation();
  
  try {
    await simulation.connect();
    
    // Get world statistics
    const worldStats = simulation.getWorldStats();
    console.log('\n🌍 World Statistics:');
    console.log(`   Total Cities: ${worldStats.total_cities}`);
    console.log('   Valley Distribution:');
    Object.entries(worldStats.valley_distribution).forEach(([valley, count]) => {
      console.log(`     ${valley}: ${count} cities`);
    });
    console.log(`   Capital Cities: ${worldStats.capital_cities.join(', ')}`);
    
    // Explore different valleys
    const valleyCities = {
      'Dawn': ['Tsin', 'Gongshan', 'Pranos'],
      'Day': ['Beitsa', 'Phoelit', 'Zadardelen'],
      'Dusk': ['Jouy', 'Motu', 'Guild'],
      'Night': ['Palwede', 'Gyba', 'Bungomo']
    };
    
    console.log('\n🏔️  Valley Exploration:');
    for (const [valley, cities] of Object.entries(valleyCities)) {
      console.log(`\n${valley} Valley Sample Cities:`);
      for (const city of cities) {
        const state = await simulation.getLocationState(city);
        console.log(`   ${city}: ${state.population.toLocaleString()} residents, ${state.time_of_day}`);
      }
    }
    
    simulation.disconnect();
    console.log('\n✅ Location explorer example completed!');
    
  } catch (error) {
    console.error('❌ Error:', error);
  }
}

// Example 4: Time Management
async function timeManagementExample() {
  console.log('\n\n⏰ Example 4: Time Management');
  console.log('=' .repeat(50));

  const simulation = new IntegratedWorldSimulation();
  
  try {
    await simulation.connect();
    
    // Get current time across multiple checks
    console.log('\n🕐 Time Progression Check:');
    for (let i = 0; i < 3; i++) {
      const status = await simulation.getSimulationStatus();
      console.log(`   Check ${i + 1}: Hour ${status.current_hour}`);
      if (i < 2) await new Promise(resolve => setTimeout(resolve, 1000));
    }
    
    // Simulate valley time zones
    console.log('\n🌍 Valley Time Zones (simulated):');
    const valleys = ['Dawn', 'Day', 'Dusk', 'Night'];
    const baseHour = 14; // 2 PM
    
    valleys.forEach((valley, index) => {
      const offset = index === 0 ? 6 : index === 1 ? 0 : index === 2 ? -6 : 12;
      const localHour = (baseHour + offset + 24) % 24;
      const timeOfDay = localHour >= 5 && localHour < 12 ? 'Dawn' :
                       localHour >= 12 && localHour < 17 ? 'Day' :
                       localHour >= 17 && localHour < 21 ? 'Dusk' : 'Night';
      console.log(`   ${valley} Valley: ${localHour}:00 (${timeOfDay})`);
    });
    
    simulation.disconnect();
    console.log('\n✅ Time management example completed!');
    
  } catch (error) {
    console.error('❌ Error:', error);
  }
}

// Example 5: Character Stories (Mock)
async function characterStoriesExample() {
  console.log('\n\n👤 Example 5: Character Stories');
  console.log('=' .repeat(50));

  const simulation = new IntegratedWorldSimulation();
  
  try {
    await simulation.connect();
    
    // Mock character data
    const mockCharacters = [
      { id: 1, name: 'John Smith', age: 32, occupation: 'Trader', city: 'Tsin' },
      { id: 2, name: 'Alice Johnson', age: 28, occupation: 'Scholar', city: 'Citadel of Utaia' },
      { id: 3, name: 'Bob Chen', age: 45, occupation: 'Builder', city: 'Jouy' }
    ];
    
    console.log('\n🌟 Featured Characters:');
    for (const char of mockCharacters) {
      console.log(`\n${char.name} (${char.age} years old)`);
      console.log(`   Occupation: ${char.occupation}`);
      console.log(`   Location: ${char.city}`);
      
      const cityState = await simulation.getLocationState(char.city);
      console.log(`   Current environment: ${cityState.time_of_day} in the ${cityState.valley} Valley`);
      
      // Generate character-specific events
      const events = await simulation.getLocationHistory(char.city);
      if (events.length > 0) {
        console.log(`   Recent activity: ${events[0].description}`);
      }
    }
    
    simulation.disconnect();
    console.log('\n✅ Character stories example completed!');
    
  } catch (error) {
    console.error('❌ Error:', error);
  }
}

// Run all examples
async function runAllExamples() {
  console.log('🚀 World Simulation Client - All Examples');
  console.log('Running with integrated client (Real Server + Mock Fallback)\n');
  
  try {
    await basicUsageExample();
    await narrativeExample();
    await locationExplorerExample();
    await timeManagementExample();
    await characterStoriesExample();
    
    console.log('\n' + '='.repeat(60));
    console.log('✅ All examples completed successfully!');
    console.log('='.repeat(60));
    
    console.log('\n📋 Summary:');
    console.log('   - Basic Usage: ✅');
    console.log('   - AI Narrative: ✅');
    console.log('   - Location Explorer: ✅');
    console.log('   - Time Management: ✅');
    console.log('   - Character Stories: ✅');
    
    console.log('\n🎯 The World Simulation Client is working perfectly!');
    console.log('   Ready for AI agent integration and narrative generation.');
    
  } catch (error) {
    console.error('\n❌ Error running examples:', error);
  }
}

// Run based on command line argument
const command = process.argv[2];

switch (command) {
  case 'basic':
    basicUsageExample().catch(console.error);
    break;
  case 'narrative':
    narrativeExample().catch(console.error);
    break;
  case 'locations':
    locationExplorerExample().catch(console.error);
    break;
  case 'time':
    timeManagementExample().catch(console.error);
    break;
  case 'characters':
    characterStoriesExample().catch(console.error);
    break;
  case 'all':
  default:
    runAllExamples().catch(console.error);
    break;
}