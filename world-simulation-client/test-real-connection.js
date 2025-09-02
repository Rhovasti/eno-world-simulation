/**
 * Test real SpacetimeDB connection
 */

import { testRealConnection } from './dist/spacetimedb-connection.js';

console.log('Starting real connection test...\n');

testRealConnection()
  .then(() => console.log('\nTest completed!'))
  .catch(error => console.error('Test failed:', error));