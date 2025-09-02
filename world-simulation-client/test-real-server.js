#!/usr/bin/env node

/**
 * Test connection to real SpacetimeDB simulation server
 * This tests actual server connectivity instead of mock data
 */

console.log('🌍 World Simulation Client - Real Server Test\n');

// For now, let's test basic connectivity to the server
async function testRealServer() {
  console.log('📡 Testing connection to real SpacetimeDB server...\n');

  try {
    // Test 1: Check HTTP endpoint
    console.log('1️⃣ Testing HTTP endpoint...');
    const response = await fetch('http://localhost:3001/database/list');
    if (response.ok) {
      console.log('✅ HTTP endpoint is accessible');
      const data = await response.text();
      console.log(`   Response: ${data.substring(0, 100)}...`);
    } else {
      console.log(`❌ HTTP endpoint returned: ${response.status}`);
    }
  } catch (error) {
    console.log(`❌ HTTP connection failed: ${error.message}`);
  }

  // Test 2: Check WebSocket connectivity
  console.log('\n2️⃣ Testing WebSocket connectivity...');
  try {
    // Import the built client
    const { WorldSimulation } = await import('./dist/simple-build.js');
    
    const simulation = new WorldSimulation('ws://localhost:3001');
    console.log('   Created client instance');
    
    await simulation.connect();
    console.log('✅ WebSocket connection successful!');
    
    // Test basic queries
    console.log('\n3️⃣ Testing simulation queries...');
    
    // Get simulation status
    const status = await simulation.getSimulationStatus();
    console.log('✅ Simulation status query successful');
    console.log(`   Current Hour: ${status.current_hour}`);
    console.log(`   Auto-tick: ${status.auto_tick_enabled}`);
    
    // Get city info
    const cityState = await simulation.getLocationState('Tsin');
    console.log('\n✅ City query successful');
    console.log(`   City: ${cityState.city}`);
    console.log(`   Valley: ${cityState.valley}`);
    console.log(`   Population: ${cityState.population.toLocaleString()}`);
    
    simulation.disconnect();
    console.log('\n✅ Disconnected successfully');
    
  } catch (error) {
    console.log(`❌ WebSocket test failed: ${error.message}`);
  }

  // Test 3: Direct SpacetimeDB commands
  console.log('\n4️⃣ Testing SpacetimeDB commands...');
  const { exec } = await import('child_process');
  const { promisify } = await import('util');
  const execPromise = promisify(exec);
  
  try {
    // List databases
    const { stdout: dbList } = await execPromise('docker-compose exec -T spacetimedb spacetime database list 2>/dev/null');
    console.log('✅ Database list command successful');
    console.log(`   Databases: ${dbList.trim()}`);
    
    // Check if worldsim database exists
    if (dbList.includes('worldsim')) {
      console.log('✅ worldsim database found!');
      
      // Try to get current hour
      const { stdout: currentHour } = await execPromise('docker-compose exec -T spacetimedb spacetime call worldsim get_current_hour 2>/dev/null');
      console.log('✅ get_current_hour reducer called');
      console.log(`   Result: ${currentHour.trim()}`);
    } else {
      console.log('⚠️  worldsim database not found - may need to run demo');
    }
    
  } catch (error) {
    console.log(`⚠️  SpacetimeDB command test failed: ${error.message}`);
  }

  console.log('\n📋 Summary:');
  console.log('- HTTP endpoint: Check the response above');
  console.log('- WebSocket: Using mock data currently');
  console.log('- SpacetimeDB: Check if worldsim database exists');
  console.log('\n💡 To initialize with demo data: ./start-simulation.sh demo');
}

// Run the test
testRealServer().catch(console.error);