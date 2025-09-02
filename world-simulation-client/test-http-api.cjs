#!/usr/bin/env node

/**
 * Test SpacetimeDB HTTP API directly
 */

const http = require('http');

async function makeRequest(path, method = 'GET', body = null) {
  return new Promise((resolve, reject) => {
    const options = {
      hostname: 'localhost',
      port: 3001,
      path: path,
      method: method,
      headers: {
        'Content-Type': 'application/json'
      }
    };

    const req = http.request(options, (res) => {
      let data = '';
      
      res.on('data', (chunk) => {
        data += chunk;
      });
      
      res.on('end', () => {
        resolve({
          status: res.statusCode,
          headers: res.headers,
          body: data
        });
      });
    });

    req.on('error', (error) => {
      reject(error);
    });

    if (body) {
      req.write(JSON.stringify(body));
    }
    
    req.end();
  });
}

async function testAPI() {
  console.log('🌍 Testing SpacetimeDB HTTP API\n');

  try {
    // Test 1: Root endpoint
    console.log('1️⃣ Testing root endpoint...');
    const root = await makeRequest('/');
    console.log(`   Status: ${root.status}`);
    console.log(`   Response: ${root.body.substring(0, 100)}...\n`);

    // Test 2: Database info
    console.log('2️⃣ Testing database info...');
    const dbInfo = await makeRequest('/v1/database/worldsim');
    console.log(`   Status: ${dbInfo.status}`);
    console.log(`   Response: ${dbInfo.body.substring(0, 200)}...\n`);

    // Test 3: Call reducer
    console.log('3️⃣ Testing reducer call (get_current_hour)...');
    const reducerCall = await makeRequest('/v1/database/worldsim/call/get_current_hour', 'POST', []);
    console.log(`   Status: ${reducerCall.status}`);
    console.log(`   Response: ${reducerCall.body}\n`);

    // Test 4: SQL query
    console.log('4️⃣ Testing SQL query...');
    const sqlQuery = await makeRequest('/v1/database/worldsim/sql', 'POST', {
      query: "SELECT 1 as test"
    });
    console.log(`   Status: ${sqlQuery.status}`);
    console.log(`   Response: ${sqlQuery.body}\n`);

  } catch (error) {
    console.error('❌ Error:', error);
  }
}

testAPI();