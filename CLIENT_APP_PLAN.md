# Client Application Development Plan
**Phase**: Alpha → Beta  
**Target**: Browser-based AI agent integration

## 🎯 Objective
Develop a TypeScript client application using the SpacetimeDB SDK to enable AI agents in browser-based narrative games to query simulation data in real-time.

## 🎮 Use Case Scenarios

### Primary Use Case: AI Narrative Agent
An AI agent running a browser-based narrative game needs to:
- **Query Current State**: "What's happening in the Valley of the Dawn right now?"
- **Historical Queries**: "What happened in Tsin 3 hours ago?"
- **Location Context**: "Who lives and works in this building?"
- **Event Streams**: "What social interactions occurred in this area today?"
- **Time-based Stories**: "Generate a story about the past 24 hours in this city"

### Example Narrative Queries
```typescript
// Current state
const currentState = await client.queryLocationNow("Tsin", "Valley of the Dawn");

// Historical state  
const pastEvents = await client.queryLocationHistory("Citadel of Utaia", hoursAgo: 24);

// Individual stories
const characterStory = await client.getIndividualStory(characterId, timeRange);

// Building activity
const buildingActivity = await client.getBuildingActivity(buildingId, currentHour);
```

## 🏗️ Technical Architecture

### Stack
- **Language**: TypeScript
- **SDK**: `@clockworklabs/spacetimedb-sdk`
- **Platform**: Browser (Web)
- **Build Tool**: Vite/Webpack
- **Package Manager**: npm/yarn

### Project Structure
```
world-simulation-client/
├── src/
│   ├── lib/
│   │   ├── spacetime-client.ts    # SDK wrapper
│   │   ├── query-interface.ts     # Query abstraction layer
│   │   └── types.ts              # Type definitions
│   ├── api/
│   │   ├── location-queries.ts    # Location-based queries
│   │   ├── time-queries.ts        # Time-based queries
│   │   ├── entity-queries.ts      # Individual/building/city queries
│   │   └── narrative-api.ts       # AI-friendly interfaces
│   ├── utils/
│   │   ├── time-helpers.ts        # Time conversion utilities
│   │   ├── location-mapper.ts     # Valley/city mapping
│   │   └── data-formatter.ts      # Output formatting
│   └── examples/
│       ├── basic-usage.ts         # Simple examples
│       ├── ai-integration.ts      # AI agent examples
│       └── narrative-demo.ts      # Narrative game demo
├── tests/
├── docs/
└── dist/
```

## 📡 API Design

### Core Client Interface
```typescript
interface WorldSimulationClient {
  // Connection management
  connect(serverUrl: string): Promise<void>;
  disconnect(): void;
  
  // Real-time queries
  getCurrentHour(): Promise<number>;
  getLocationState(location: string, valley?: string): Promise<LocationState>;
  
  // Historical queries
  getLocationHistory(location: string, timeRange: TimeRange): Promise<HistoricalEvent[]>;
  getIndividualStory(individualId: number, hoursBack: number): Promise<NarrativeEvent[]>;
  getBuildingStory(buildingId: number, hoursBack: number): Promise<NarrativeEvent[]>;
  
  // Entity queries
  getIndividualsAtLocation(location: string): Promise<Individual[]>;
  getBuildingsInCity(cityName: string): Promise<Building[]>;
  getCitySummary(cityId: number): Promise<CitySummary>;
  
  // AI-friendly interfaces
  generateLocationNarrative(location: string, timeRange: TimeRange): Promise<string>;
  getContextForStory(location: string, characters: string[]): Promise<StoryContext>;
}
```

### Data Types
```typescript
interface LocationState {
  city: string;
  valley: string;
  currentHour: number;
  timeOfDay: 'dawn' | 'day' | 'dusk' | 'night';
  population: number;
  activeBuildings: Building[];
  recentEvents: Event[];
}

interface HistoricalEvent {
  type: 'movement' | 'work' | 'social' | 'building' | 'city';
  timestamp: number;
  location: string;
  participants: string[];
  description: string;
  impact: EventImpact;
}

interface StoryContext {
  location: LocationState;
  characters: Individual[];
  recentEvents: HistoricalEvent[];
  timeContext: TimeContext;
  suggestedNarratives: string[];
}
```

## 🚀 Implementation Phases

### Phase 1: Basic SDK Integration (Week 1)
- [ ] Set up TypeScript project with SpacetimeDB SDK
- [ ] Establish connection to simulation server
- [ ] Implement basic table subscriptions
- [ ] Test real-time data retrieval
- [ ] Create basic query examples

### Phase 2: Query Interface Development (Week 2)
- [ ] Implement location-based queries
- [ ] Add time-based query functionality
- [ ] Create entity-specific query methods
- [ ] Build time conversion utilities
- [ ] Add comprehensive error handling

### Phase 3: AI Integration Layer (Week 3)
- [ ] Design AI-friendly data structures
- [ ] Implement narrative generation helpers
- [ ] Create context aggregation functions
- [ ] Add story formatting utilities
- [ ] Build example AI integrations

### Phase 4: Browser Optimization (Week 4)
- [ ] Optimize for browser performance
- [ ] Add connection management
- [ ] Implement caching strategies
- [ ] Create demo web application
- [ ] Add comprehensive documentation

## 🎨 Example Implementations

### Basic Location Query
```typescript
const client = new WorldSimulationClient();
await client.connect('http://localhost:3001');

// Get current state of Tsin in Valley of the Dawn
const tsinState = await client.getLocationState('Tsin', 'Valley of the Dawn');
console.log(`Tsin currently has ${tsinState.population} residents`);
console.log(`Time of day: ${tsinState.timeOfDay}`);
```

### AI Narrative Integration
```typescript
// AI agent requests context for story generation
const storyContext = await client.getContextForStory(
  'Citadel of Utaia',
  ['merchant', 'guard', 'scholar']
);

// AI uses context to generate narrative
const narrative = aiAgent.generateStory({
  setting: storyContext.location,
  characters: storyContext.characters,
  recentEvents: storyContext.recentEvents,
  timeOfDay: storyContext.timeContext.timeOfDay
});
```

### Historical Event Query
```typescript
// Get all events in the past 6 hours at a location
const events = await client.getLocationHistory('Beitsa', {
  hoursBack: 6,
  eventTypes: ['social', 'work', 'movement']
});

// Format for AI consumption
const narrativeEvents = events.map(event => ({
  what: event.description,
  when: `${event.hoursAgo} hours ago`,
  who: event.participants.join(', '),
  impact: event.impact.description
}));
```

## 🔧 Technical Considerations

### Performance
- **Efficient Subscriptions**: Subscribe only to needed tables
- **Query Optimization**: Batch related queries
- **Caching Strategy**: Cache frequently accessed data
- **Connection Management**: Handle reconnections gracefully

### Browser Compatibility
- **WebSocket Support**: Ensure broad browser support
- **Bundle Size**: Optimize for fast loading
- **Memory Management**: Prevent memory leaks in long-running sessions
- **Error Recovery**: Robust error handling and reconnection

### AI Integration
- **Structured Data**: Consistent, predictable data formats
- **Natural Language**: Human-readable descriptions
- **Context Aggregation**: Combine related data points
- **Narrative Helpers**: Pre-built story generation utilities

## 📋 Success Metrics

### Functionality
- [ ] Successfully queries all simulation data types
- [ ] Real-time updates work reliably
- [ ] Historical queries return accurate results
- [ ] AI integration examples work as expected

### Performance
- [ ] Initial connection < 2 seconds
- [ ] Query response time < 500ms
- [ ] Memory usage stable over time
- [ ] Bundle size < 1MB

### Developer Experience
- [ ] Clear, comprehensive documentation
- [ ] Working examples for all use cases
- [ ] Type safety throughout
- [ ] Intuitive API design

## 🎯 Deliverables

1. **TypeScript Client Library**: Complete SDK wrapper with query interface
2. **AI Integration Examples**: Demonstration of narrative game integration
3. **Demo Web Application**: Browser-based example showing all features
4. **Comprehensive Documentation**: API docs, tutorials, and examples
5. **Testing Suite**: Unit and integration tests for reliability

## 🔮 Future Enhancements

### Advanced Features
- **Real-time Subscriptions**: Live updates for dynamic narratives
- **Query Builder**: Visual interface for complex queries
- **Narrative Templates**: Pre-built story frameworks
- **Multi-agent Support**: Handle multiple AI agents simultaneously

### Integration Options
- **REST API Wrapper**: For non-WebSocket clients
- **GraphQL Interface**: Alternative query language
- **WebAssembly Client**: High-performance client option
- **Mobile SDK**: React Native/Flutter support

---

**Next Step**: Initialize the client application project and begin Phase 1 implementation.

This client application will bridge the gap between the continuously running world simulation and AI-driven narrative experiences, enabling rich, dynamic storytelling based on real simulation data.