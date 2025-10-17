# Frontend Integration API for World Simulation

## Overview

This document describes how to integrate the World Simulation Background Processor with the Eno-Frontend for async narrative generation.

## Architecture

```
Eno-Frontend (Node.js) <---> SpacetimeDB <---> World Simulation (Rust)
                         |
                         v
                    Narrative Pipeline (AI)
```

## Integration Service Setup

### 1. Create Simulation Service

Create `/root/Eno/Eno-Frontend/js/services/simulationService.js`:

```javascript
const fetch = require('node-fetch');

class SimulationService {
    constructor() {
        this.spacetimeUrl = process.env.SPACETIME_URL || 'http://localhost:3000';
        this.databaseName = process.env.SIMULATION_DB || 'world-sim-narrative';
    }

    // Get narrative events for a game
    async getNarrativeEvents(gameId, maxEvents = 10, minImportance = 3) {
        try {
            const response = await fetch(`${this.spacetimeUrl}/database/${this.databaseName}/call`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    reducer: 'get_unconsumed_events',
                    args: [gameId, maxEvents, minImportance]
                })
            });

            const result = await response.json();
            if (result.success) {
                return result.result || [];
            } else {
                throw new Error(result.error || 'Failed to get narrative events');
            }
        } catch (error) {
            console.error('Error fetching narrative events:', error);
            return [];
        }
    }

    // Mark events as consumed
    async consumeEvents(eventIds) {
        try {
            const response = await fetch(`${this.spacetimeUrl}/database/${this.databaseName}/call`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    reducer: 'consume_narrative_events',
                    args: [eventIds]
                })
            });

            const result = await response.json();
            return result.success;
        } catch (error) {
            console.error('Error consuming events:', error);
            return false;
        }
    }

    // Create a new game world
    async createGameWorld(name, climateZone, initialPopulation) {
        try {
            const response = await fetch(`${this.spacetimeUrl}/database/${this.databaseName}/call`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    reducer: 'create_game_world',
                    args: [name, climateZone, initialPopulation]
                })
            });

            const result = await response.json();
            return result.success ? result.result : null;
        } catch (error) {
            console.error('Error creating game world:', error);
            return null;
        }
    }

    // Link a game to a world
    async linkGameToWorld(gameId, worldId) {
        try {
            // Store the mapping in your database
            const query = `
                INSERT INTO game_world_mappings (game_id, world_id, created_at)
                VALUES (?, ?, datetime('now'))
                ON CONFLICT(game_id) DO UPDATE SET world_id = excluded.world_id
            `;
            // Execute with your database connection
            return true;
        } catch (error) {
            console.error('Error linking game to world:', error);
            return false;
        }
    }

    // Get world ID for a game
    async getWorldIdForGame(gameId) {
        try {
            const query = `SELECT world_id FROM game_world_mappings WHERE game_id = ?`;
            // Execute with your database connection
            // Return world_id or null
            return null;
        } catch (error) {
            console.error('Error getting world ID for game:', error);
            return null;
        }
    }

    // Get world status
    async getWorldStatus(worldId) {
        try {
            const response = await fetch(`${this.spacetimeUrl}/database/${this.databaseName}/query`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    query: `SELECT * FROM game_world WHERE id = ${worldId}`
                })
            });

            const result = await response.json();
            return result.rows && result.rows.length > 0 ? result.rows[0] : null;
        } catch (error) {
            console.error('Error getting world status:', error);
            return null;
        }
    }

    // Force scheduler run (for testing)
    async runScheduler() {
        try {
            const response = await fetch(`${this.spacetimeUrl}/database/${this.databaseName}/call`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    reducer: 'run_world_simulation_batch',
                    args: []
                })
            });

            const result = await response.json();
            return result.success;
        } catch (error) {
            console.error('Error running scheduler:', error);
            return false;
        }
    }
}

module.exports = new SimulationService();
```

### 2. Create Database Schema

Add to your SQLite schema (`/root/Eno/Eno-Frontend/js/server_sqlite_new.js`):

```sql
-- Link games to simulation worlds
CREATE TABLE IF NOT EXISTS game_world_mappings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    game_id INTEGER NOT NULL UNIQUE,
    world_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (game_id) REFERENCES games(id) ON DELETE CASCADE
);

-- Cache for narrative events
CREATE TABLE IF NOT EXISTS narrative_event_cache (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_id INTEGER NOT NULL,
    game_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    importance INTEGER NOT NULL,
    category TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    consumed BOOLEAN DEFAULT 0,
    FOREIGN KEY (game_id) REFERENCES games(id) ON DELETE CASCADE
);
```

### 3. Integrate with Game Creation

Update game creation to include world simulation:

```javascript
// In your game creation endpoint
app.post('/api/games', authenticateJWT, async (req, res) => {
    try {
        const { title, description, location_type, location_id } = req.body;

        // Create game in database
        const gameId = await createGame(req.body);

        // Create simulation world
        const simulationService = require('./services/simulationService');
        const worldId = await simulationService.createGameWorld(
            `World for ${title}`,
            'Temperate', // Default climate
            1000 // Initial population
        );

        if (worldId) {
            // Link game to world
            await simulationService.linkGameToWorld(gameId, worldId);
            console.log(`Game ${gameId} linked to simulation world ${worldId}`);
        }

        res.json({ success: true, gameId, worldId });
    } catch (error) {
        console.error('Error creating game:', error);
        res.status(500).json({ error: 'Failed to create game' });
    }
});
```

### 4. Narrative Generation Integration

Create narrative processing endpoint:

```javascript
// Process narrative events for a game
app.post('/api/games/:gameId/process-narrative', authenticateJWT, async (req, res) => {
    try {
        const gameId = parseInt(req.params.gameId);
        const simulationService = require('./services/simulationService');

        // Get world ID for this game
        const worldId = await simulationService.getWorldIdForGame(gameId);
        if (!worldId) {
            return res.status(404).json({ error: 'No simulation world found for game' });
        }

        // Get narrative events
        const events = await simulationService.getNarrativeEvents(gameId);

        if (events.length === 0) {
            return res.json({ narrative: null, message: 'No new events to process' });
        }

        // Transform events for AI context
        const context = {
            gameId,
            worldId,
            events: events.map(e => ({
                title: e.title,
                description: e.description,
                category: e.event_category,
                importance: e.importance,
                temporalContext: JSON.parse(e.temporal_context || '{}'),
                participants: JSON.parse(e.participants || '[]')
            })),
            worldStatus: await simulationService.getWorldStatus(worldId)
        };

        // Generate narrative with AI (integrate with your existing AI system)
        const narrative = await generateNarrative(context);

        // Mark events as consumed
        const eventIds = events.map(e => e.id);
        await simulationService.consumeEvents(eventIds);

        res.json({ narrative, eventsProcessed: events.length });
    } catch (error) {
        console.error('Error processing narrative:', error);
        res.status(500).json({ error: 'Failed to process narrative' });
    }
});
```

### 5. AI Integration Helper

```javascript
async function generateNarrative(context) {
    try {
        // Build prompt for AI
        const prompt = buildNarrativePrompt(context);

        // Call your AI service (OpenAI, Anthropic, etc.)
        const response = await callAI(prompt);

        // Process and format response
        return {
            content: response.content,
            worldEvents: context.events.length,
            worldCycle: context.worldStatus?.current_cycle || 0,
            worldDay: context.worldStatus?.current_day || 0,
            generatedAt: new Date().toISOString()
        };
    } catch (error) {
        console.error('Error generating narrative:', error);
        return null;
    }
}

function buildNarrativePrompt(context) {
    const { events, worldStatus } = context;

    let prompt = `Generate a narrative for an async RPG game based on these world events:\n\n`;

    prompt += `World Status:\n`;
    prompt += `- Cycle ${worldStatus.current_cycle}, Day ${worldStatus.current_day}\n`;
    prompt += `- Season: ${worldStatus.season}\n`;
    prompt += `- Population: ${worldStatus.total_population}\n\n`;

    prompt += `Recent Events:\n`;
    events.forEach((event, i) => {
        prompt += `${i + 1}. ${event.title} (${event.category}, Importance: ${event.importance})\n`;
        prompt += `   ${event.description}\n\n`;
    });

    prompt += `Create an engaging narrative that:\n`;
    prompt += `- Incorporates these events naturally\n`;
    prompt += `- Provides hooks for player interaction\n`;
    prompt += `- Maintains consistency with the world state\n`;
    prompt += `- Is 200-500 words long\n`;

    return prompt;
}
```

## API Endpoints Summary

### Game Management
- `POST /api/games` - Create game with simulation world
- `GET /api/games/:id/world-status` - Get simulation world status
- `POST /api/games/:id/process-narrative` - Generate narrative from events

### Simulation Control
- `POST /api/simulation/run-scheduler` - Force scheduler run (admin)
- `GET /api/simulation/status` - Get scheduler status
- `POST /api/simulation/worlds` - Create new world
- `GET /api/simulation/worlds/:id` - Get world details

### Narrative Events
- `GET /api/games/:id/events` - Get narrative events for game
- `POST /api/games/:id/events/consume` - Mark events as consumed
- `GET /api/games/:id/events/history` - Get consumed events history

## Configuration

Add to your `.env` file:

```env
# SpacetimeDB Configuration
SPACETIME_URL=http://localhost:3000
SIMULATION_DB=world-sim-narrative

# Simulation Settings
SIMULATION_ENABLED=true
SIMULATION_AUTO_CREATE_WORLDS=true
SIMULATION_DEFAULT_CLIMATE=Temperate
SIMULATION_DEFAULT_POPULATION=1000
```

## Testing

### 1. Test World Creation
```bash
curl -X POST http://localhost:3000/api/simulation/worlds \
  -H "Content-Type: application/json" \
  -d '{"name": "Test World", "climate": "Temperate", "population": 1000}'
```

### 2. Test Event Retrieval
```bash
curl -X GET http://localhost:3000/api/games/1/events
```

### 3. Test Narrative Generation
```bash
curl -X POST http://localhost:3000/api/games/1/process-narrative \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

## Performance Considerations

1. **Caching**: Cache events locally to reduce SpacetimeDB queries
2. **Batching**: Process multiple games together when possible
3. **Rate Limiting**: Limit narrative generation frequency per game
4. **Background Processing**: Use job queues for heavy operations

## Error Handling

```javascript
const handleSimulationError = (error, context) => {
    console.error(`Simulation error in ${context}:`, error);

    // Log to your monitoring system
    // Fallback to default behavior
    // Notify administrators if critical

    return {
        success: false,
        error: 'Simulation temporarily unavailable',
        fallback: true
    };
};
```

## Next Steps

1. Implement the service and endpoints
2. Test with a single game world
3. Scale to multiple concurrent games
4. Add monitoring and performance metrics
5. Optimize based on real usage patterns