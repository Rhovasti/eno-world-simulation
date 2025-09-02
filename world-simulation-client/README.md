# World Simulation Client

A comprehensive TypeScript client library for accessing the SpacetimeDB World Simulation. Perfect for AI agents, narrative games, and interactive storytelling systems.

## 🌟 Features

- **🤖 AI-Friendly APIs**: Narrative generation and story context for AI systems
- **🗺️ Location Intelligence**: 160+ cities across 4 valleys with time zones
- **👥 Character Tracking**: Individual analysis and story generation
- **⏰ Time Management**: Custom calendar system with valley-specific time zones
- **📚 Event History**: Comprehensive historical event tracking
- **🎭 Narrative Generation**: Theme-based story suggestions and comparative analysis
- **🔄 Real-time Updates**: WebSocket connection with live data
- **📦 TypeScript**: Full type safety and excellent IDE support

## 🚀 Quick Start

### Installation

```bash
npm install world-simulation-client
```

### Basic Usage

```typescript
import { WorldSimulation } from 'world-simulation-client';

const simulation = new WorldSimulation('ws://localhost:3001');
await simulation.connect();

// Get current simulation status
const status = await simulation.client.getSimulationStatus();
console.log(`Current hour: ${status.current_hour}`);

// Get city information
const cityState = await simulation.client.getLocationState('Tsin');
console.log(`${cityState.city}: ${cityState.population} residents`);

// Generate AI narrative
const narrative = await simulation.narrative.generateLocationNarrative('Tsin');
console.log(narrative);

simulation.disconnect();
```

### AI Integration Example

```typescript
import { getAIStoryData, getTimeContext } from 'world-simulation-client';

// Perfect for AI agents generating narratives
const storyData = await getAIStoryData('Citadel of Utaia');
const timeContext = await getTimeContext();

// Use story data for AI narrative generation
console.log(`Setting: ${storyData.location_state.city}`);
console.log(`Characters: ${storyData.available_characters.length}`);
console.log(`Time: ${timeContext.formatted_date}`);
```

## 🧪 Testing & Demo

### Interactive Demo Website

Start the interactive demo to test all functionality:

```bash
npm run demo
# Open http://localhost:3002
```

### Command Line Examples

```bash
# Run all examples
npm run example:all

# Specific examples
npm run example:basic      # Basic operations
npm run example:ai         # AI narrative generation
npm run example:locations  # Location exploration
npm run example:characters # Character stories
npm run example:time       # Time management

# Quick demo for AI integration
npm run example:quick
```

### Prerequisites

Ensure the World Simulation server is running:

```bash
cd /path/to/simulation
./start-simulation.sh start
```

## 📖 API Reference

### Core Classes

#### `WorldSimulation`
All-in-one client combining query interface with narrative API.

```typescript
const simulation = new WorldSimulation('ws://localhost:3001');
await simulation.connect();

// Access the query client
const status = await simulation.client.getSimulationStatus();

// Access the narrative API
const context = await simulation.narrative.getContextForStory('Tsin');
```

#### `WorldSimulationClient`
Main query interface for simulation data.

```typescript
import { WorldSimulationClient } from 'world-simulation-client';

const client = new WorldSimulationClient({ url: 'ws://localhost:3001' });
await client.connect();

// Query methods
const currentHour = await client.getCurrentHour();
const cityState = await client.getLocationState('Tsin');
const events = await client.getLocationHistory('Tsin', { hours_back: 24 });
```

#### `NarrativeAPI`
AI-friendly narrative generation and story context.

```typescript
import { NarrativeAPI } from 'world-simulation-client';

const narrative = new NarrativeAPI(client);

// Get story context for AI agents
const context = await narrative.getContextForStory('Citadel of Utaia');

// Generate complete narratives
const story = await narrative.generateLocationNarrative('Tsin', 12, true);

// Get character-focused stories
const characterStory = await narrative.getCharacterStoryContext(characterId, 24);
```

### Utility Functions

#### Location Utilities

```typescript
import { 
  getCitiesInValley, 
  getLocationStats, 
  searchCities,
  isValidCity,
  getRandomCity 
} from 'world-simulation-client';

const dawnCities = getCitiesInValley('Dawn');
const stats = getLocationStats();
const searchResults = searchCities('Citadel');
```

#### Time Utilities

```typescript
import { 
  formatSimulationDate,
  calculateTimeOfDayForValley,
  getValleyTimeZones,
  createTimeRange 
} from 'world-simulation-client';

const formatted = formatSimulationDate(currentHour);
const timeOfDay = calculateTimeOfDayForValley('Dawn', 14);
const timeZones = getValleyTimeZones(currentHour);
```

#### Data Formatting

```typescript
import { 
  formatIndividual,
  formatBuilding,
  formatCity,
  createLocationNarrative 
} from 'world-simulation-client';

const characterDesc = formatIndividual(individual, true);
const buildingDesc = formatBuilding(building, true);
const narrative = createLocationNarrative(city, population, timeOfDay, valley, events, currentHour);
```

## 🌍 World Structure

### Valleys & Time Zones

The simulation features 4 valleys with different time zones:

- **🌅 Dawn Valley**: Eastern lands (+6 hours)
- **☀️ Day Valley**: Central heartlands (base time)
- **🌇 Dusk Valley**: Western territories (-6 hours)  
- **🌙 Night Valley**: Far lands (+12 hours)

### Cities

160+ cities distributed across valleys:
- **43 cities** in Dawn Valley (including Citadel of Utaia)
- **37 cities** in Day Valley (including major trade centers)
- **43 cities** in Dusk Valley (including Citadel of Almo)
- **29 cities** in Night Valley (including Citadel of the Pass)

## 🎯 Use Cases

### AI Narrative Games

```typescript
// Get story suggestions by theme
const suggestions = await narrative.getNarrativeSuggestions('adventure', 'Tsin');

// Generate character-focused stories
const characterContext = await narrative.getCharacterStoryContext(characterId);

// Compare locations for story variety
const comparison = await narrative.getComparativeNarrative('Tsin', 'Palwede');
```

### Interactive Storytelling

```typescript
// Get real-time location state
const locationState = await client.getLocationState('Citadel of Utaia');

// Track character activities
const characters = await client.getIndividualsAtLocation('Tsin');

// Follow historical events
const timeline = await client.getLocationHistory('Tsin', { hours_back: 48 });
```

### World Building & Analysis

```typescript
// Explore world statistics
const worldStats = getLocationStats();

// Find interesting locations
const locations = await findInterestingLocations({
  minPopulation: 1000,
  includeRecentActivity: true
});

// Analyze character demographics
const characterData = await getCharacterData();
```

## 🔧 Configuration

### Connection Options

```typescript
const client = new WorldSimulationClient({
  url: 'ws://localhost:3001',
  reconnect: true,
  maxReconnectAttempts: 5,
  reconnectInterval: 1000
});
```

### Query Options

```typescript
// Time range queries
const events = await client.getLocationHistory('Tsin', {
  hours_back: 24,
  max_events: 50
});

// Character filtering
const context = await narrative.getContextForStory('Tsin', ['workers', 'residents']);
```

## 📚 Examples

The library includes comprehensive examples:

- **Basic Usage**: Connection, status queries, city information
- **AI Narrative**: Story generation, theme-based narratives
- **Location Explorer**: Valley exploration, city discovery
- **Character Stories**: Individual tracking, social analysis
- **Time Management**: Time zones, historical queries

See the [examples directory](./src/examples/) for complete implementations.

## 🐛 Troubleshooting

### Common Issues

**Connection Failed**
```bash
# Ensure simulation server is running
./start-simulation.sh start

# Check server status
curl http://localhost:3001/status
```

**Module Not Found**
```bash
# Build the library
npm run build

# Install dependencies
npm install
```

**Empty Data**
- Allow time for simulation to populate with data
- Check if auto-tick is enabled
- Verify simulation is running properly

See [TESTING.md](./TESTING.md) for comprehensive troubleshooting.

## 📄 License

MIT License - see [LICENSE](./LICENSE) file for details.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Run the test suite: `npm test`
5. Submit a pull request

## 📞 Support

- **Documentation**: See examples and API reference
- **Issues**: Report bugs and feature requests
- **Testing**: Use the interactive demo for testing

---

Built with ❤️ for AI agents and narrative systems. Happy storytelling! 🎭