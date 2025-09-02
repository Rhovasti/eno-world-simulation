# Testing Guide for World Simulation Client

This guide provides comprehensive instructions for testing the TypeScript client library for the World Simulation.

## Prerequisites

### 1. Start the World Simulation Server

Before testing the client, ensure the simulation server is running:

```bash
cd /root/Eno/simulation2
./start-simulation.sh start
```

Verify the server is accessible:
```bash
curl http://localhost:3001/status
```

### 2. Install Dependencies

```bash
cd /root/Eno/simulation2/world-simulation-client
npm install
```

### 3. Build the Library

```bash
npm run build
```

## Testing Methods

### Method 1: Interactive Demo Website 🌐

The easiest way to test functionality is through the interactive demo website:

```bash
# Start the demo server
npm run demo

# Open browser to http://localhost:3002
```

**Features:**
- 🔗 **Connection Management**: Test connection to simulation server
- 🚀 **Basic Operations**: Status queries, city information, events
- 🤖 **AI Narrative**: Story generation and AI-friendly APIs
- 🗺️ **Location Explorer**: Valley exploration and city discovery
- 👤 **Character Analysis**: Individual tracking and story generation
- 🧪 **Testing Guide**: Built-in instructions and troubleshooting

### Method 2: Command Line Examples 💻

Run comprehensive examples from the command line:

```bash
# Run all examples in sequence
npm run example:all

# Run specific examples
npm run example:basic      # Basic client operations
npm run example:ai         # AI narrative generation
npm run example:locations  # Location exploration
npm run example:characters # Character stories
npm run example:time       # Time management

# Quick AI integration demo
npm run example:quick
```

### Method 3: Direct Node.js Execution 🔧

Execute examples directly with Node.js:

```bash
# Build first
npm run build

# Run examples
node dist/examples/index.js all        # All examples
node dist/examples/index.js quick      # Quick demo
node dist/examples/index.js basic      # Basic usage
node dist/examples/index.js ai         # AI narrative
node dist/examples/index.js characters # Character stories
node dist/examples/index.js time       # Time management
node dist/examples/index.js location   # Location explorer
```

### Method 4: Integration Testing 🔗

Test the library in your own projects:

```typescript
import { WorldSimulation, getAIStoryData, getTimeContext } from 'world-simulation-client';

async function testIntegration() {
  // Basic client usage
  const simulation = new WorldSimulation('ws://localhost:3001');
  await simulation.connect();
  
  // Get current status
  const status = await simulation.client.getSimulationStatus();
  console.log(`Current hour: ${status.current_hour}`);
  
  // AI narrative generation
  const storyData = await simulation.narrative.getContextForStory('Tsin');
  console.log(`Story context for ${storyData.location.city}`);
  
  // Utility functions
  const timeContext = await getTimeContext();
  console.log(`Formatted time: ${timeContext.formatted_date}`);
  
  simulation.disconnect();
}

testIntegration().catch(console.error);
```

## Test Coverage

### 🔌 Connection & Status Tests

- [x] **Connection Management**: Connect/disconnect to simulation server
- [x] **Status Queries**: Get simulation time, auto-tick status, configuration
- [x] **Error Handling**: Connection failures, timeouts, invalid responses

### 🏙️ Location & Geography Tests

- [x] **City Information**: Population, buildings, time of day, valley
- [x] **Valley Exploration**: Time zones, city distribution, characteristics
- [x] **Location Search**: City name validation, search functionality
- [x] **World Statistics**: Total cities, capitals, valley distribution

### ⏰ Time Management Tests

- [x] **Time Calculations**: Valley time zones, relative time descriptions
- [x] **Historical Queries**: Event timelines, time range filtering
- [x] **Calendar System**: Custom 360-day year, leap occurrences
- [x] **Real-time Conversion**: Tick rate calculations, duration formatting

### 👥 Character & Individual Tests

- [x] **Character Discovery**: Individual listings, filtering, statistics
- [x] **Character Stories**: Personal narratives, activity tracking
- [x] **Social Analysis**: Interaction tracking, relationship mapping
- [x] **Character Profiles**: Needs, wellness, demographic analysis

### 📚 Events & History Tests

- [x] **Event Queries**: Location history, character stories, world events
- [x] **Timeline Analysis**: Chronological ordering, relative descriptions
- [x] **Event Filtering**: Time ranges, location filters, participant filters
- [x] **Impact Analysis**: Event significance, cascading effects

### 🤖 AI Narrative Tests

- [x] **Story Context**: Location-based narratives, character contexts
- [x] **Theme Generation**: Adventure, mystery, drama, slice-of-life stories
- [x] **Narrative Formatting**: Human-readable output, AI-friendly structure
- [x] **Comparative Analysis**: Multi-location comparisons, temporal analysis

### 🏗️ Building & Infrastructure Tests

- [x] **Building Information**: Types, occupancy, efficiency, condition
- [x] **Building Stories**: Activity tracking, occupant analysis
- [x] **Infrastructure Analysis**: Building distribution, capacity metrics

## Performance Testing

### Load Testing

Test with multiple concurrent connections:

```bash
# Run multiple examples simultaneously
npm run example:basic & npm run example:ai & npm run example:locations
```

### Memory Usage

Monitor memory usage during extended operations:

```bash
# Run comprehensive test with memory monitoring
node --inspect dist/examples/index.js all
```

### Network Performance

Test with network latency simulation:

```bash
# Add artificial latency to test robustness
# (Use network simulation tools as needed)
```

## Error Testing

### Connection Failures

Test behavior when simulation server is unavailable:

1. Stop simulation server: `./start-simulation.sh stop`
2. Run client examples - should handle gracefully
3. Restart server and test reconnection

### Invalid Data

Test with invalid inputs:

```typescript
// Test invalid city names
await simulation.client.getLocationState('InvalidCity');

// Test invalid time ranges
await simulation.client.getLocationHistory('Tsin', { hours_back: -1 });

// Test invalid character IDs
await simulation.narrative.getCharacterStoryContext(999999);
```

### Rate Limiting

Test rapid successive calls:

```bash
# Run examples rapidly to test rate handling
for i in {1..10}; do npm run example:basic & done
```

## Troubleshooting

### Common Issues

**Connection Refused (ECONNREFUSED)**
- Ensure simulation server is running on port 3001
- Check firewall settings
- Verify server configuration

**WebSocket Connection Failed**
- Confirm WebSocket support in environment
- Check for proxy/firewall blocking WebSocket connections
- Verify correct server URL format

**Module Import Errors**
- Run `npm run build` to ensure latest compilation
- Check Node.js version compatibility (requires Node 16+)
- Verify all dependencies are installed

**Empty Data Responses**
- Simulation may not be populated with data yet
- Allow time for simulation to generate individuals/events
- Check simulation auto-tick status

### Debug Mode

Enable detailed logging:

```typescript
// Set debug environment variable
process.env.DEBUG = 'world-simulation:*';

// Or use verbose logging in examples
const simulation = new WorldSimulation('ws://localhost:3001', { debug: true });
```

### Performance Issues

If experiencing slow responses:

1. Check simulation server performance
2. Reduce query scope (shorter time ranges, fewer entities)
3. Use pagination for large datasets
4. Monitor network latency

## Success Criteria

A successful test run should demonstrate:

✅ **Connectivity**: Successful connection and disconnection  
✅ **Data Retrieval**: Valid responses for all query types  
✅ **Error Handling**: Graceful handling of invalid inputs  
✅ **Performance**: Reasonable response times (< 5 seconds)  
✅ **Consistency**: Repeatable results for same queries  
✅ **AI Integration**: Narrative generation produces readable output  
✅ **Documentation**: Examples run without modification  

## Automated Testing

For continuous integration, add to your pipeline:

```bash
#!/bin/bash
# CI test script

# Start simulation server
./start-simulation.sh start

# Wait for server startup
sleep 10

# Run client tests
cd world-simulation-client
npm install
npm run build
npm run test

# Run examples
npm run example:quick

# Cleanup
cd ..
./start-simulation.sh stop
```

## Getting Help

If you encounter issues:

1. **Check Logs**: Review simulation server logs and client error messages
2. **Verify Setup**: Ensure all prerequisites are met
3. **Test Isolation**: Run individual examples to isolate problems
4. **Documentation**: Review API documentation and examples
5. **Community**: Report issues with detailed reproduction steps

---

Happy testing! 🚀 The World Simulation Client provides a comprehensive interface for AI agents and narrative systems to interact with the simulation data.