/**
 * Character Stories Example for World Simulation Client
 * Demonstrates character-focused narrative generation and individual tracking
 */
import { WorldSimulation, createCharacterProfile, formatIndividualActivity, formatNeeds, formatWellness } from '../index.js';
async function characterStoriesExample() {
    console.log('👤 World Simulation Client - Character Stories Example\n');
    const simulation = new WorldSimulation('ws://localhost:3001');
    try {
        await simulation.connect();
        console.log('✅ Connected to simulation\n');
        // Get all individuals in the simulation
        console.log('🔍 Finding Characters...');
        const allIndividuals = await simulation.client.spacetimeClient.getIndividuals();
        console.log(`Found ${allIndividuals.length} individuals in the simulation\n`);
        if (allIndividuals.length === 0) {
            console.log('⚠️  No individuals found in the simulation. The simulation may need to be started or populated.');
            return;
        }
        // Select interesting characters for stories
        console.log('📊 Character Analysis:');
        // Sort by different criteria
        const youngCharacters = allIndividuals
            .filter(ind => ind.age < 30)
            .sort((a, b) => a.age - b.age)
            .slice(0, 3);
        const energeticCharacters = allIndividuals
            .sort((a, b) => b.energy - a.energy)
            .slice(0, 3);
        const socialCharacters = allIndividuals
            .sort((a, b) => b.connection_need - a.connection_need)
            .slice(0, 3);
        console.log('👶 Youngest Characters:');
        youngCharacters.forEach((char, index) => {
            console.log(`${index + 1}. ${char.name} (age ${char.age}) - ${formatIndividualActivity(char.current_activity)}`);
        });
        console.log('\n⚡ Most Energetic Characters:');
        energeticCharacters.forEach((char, index) => {
            console.log(`${index + 1}. ${char.name} (energy: ${(char.energy * 100).toFixed(0)}%) - ${formatIndividualActivity(char.current_activity)}`);
        });
        console.log('\n🤝 Most Social Characters:');
        socialCharacters.forEach((char, index) => {
            console.log(`${index + 1}. ${char.name} (connection: ${(char.connection_need * 100).toFixed(0)}%) - ${formatIndividualActivity(char.current_activity)}`);
        });
        // Generate detailed character stories
        console.log('\n📖 Detailed Character Stories:');
        // Pick one character from each category for detailed analysis
        const featuredCharacters = [
            { character: youngCharacters[0], category: 'Youngest' },
            { character: energeticCharacters[0], category: 'Most Energetic' },
            { character: socialCharacters[0], category: 'Most Social' }
        ].filter(item => item.character); // Remove any undefined characters
        for (const { character, category } of featuredCharacters) {
            console.log(`\n🌟 Featured Character (${category}): ${character.name}`);
            console.log('─'.repeat(60));
            // Get character story context
            const storyContext = await simulation.narrative.getCharacterStoryContext(character.id, 48);
            console.log('Basic Information:');
            console.log(`  Name: ${character.name}`);
            console.log(`  Age: ${character.age}`);
            console.log(`  Current Activity: ${formatIndividualActivity(character.current_activity)}`);
            // Location information
            if (storyContext.location_context) {
                console.log(`  Location: ${storyContext.location_context.city} (${storyContext.location_context.valley} Valley)`);
                console.log(`  Local Time: ${storyContext.location_context.time_of_day}`);
            }
            // Needs and wellness
            const needs = {
                environment: character.environment_need,
                consumption: character.consumption_need,
                connection: character.connection_need,
                rest: character.rest_need,
                waste: character.waste_need
            };
            console.log(`  Needs: ${formatNeeds(needs)}`);
            console.log(`  Wellness: ${formatWellness(character.energy, character.happiness, character.health)}`);
            // Recent story events
            console.log('\nRecent Story Events:');
            if (storyContext.story_events.length > 0) {
                storyContext.story_events.slice(0, 5).forEach((event, index) => {
                    console.log(`  ${index + 1}. ${event.description}`);
                    if (event.participants.length > 1) {
                        const otherParticipants = event.participants.filter(p => p !== character.name);
                        if (otherParticipants.length > 0) {
                            console.log(`     (with ${otherParticipants.join(', ')})`);
                        }
                    }
                });
            }
            else {
                console.log('  No recent story events recorded');
            }
            // AI-generated narrative summary
            console.log('\nAI Narrative Summary:');
            console.log(`"${storyContext.narrative_summary}"`);
        }
        // Character interaction analysis
        console.log('\n🤝 Character Interaction Analysis:');
        // Find characters who appear in events together
        const recentEvents = await simulation.client.getWorldHistory({ hours_back: 24 });
        const interactions = new Map();
        recentEvents.forEach(event => {
            if (event.participants.length > 1) {
                event.participants.forEach(participant1 => {
                    event.participants.forEach(participant2 => {
                        if (participant1 !== participant2) {
                            if (!interactions.has(participant1)) {
                                interactions.set(participant1, new Set());
                            }
                            interactions.get(participant1).add(participant2);
                        }
                    });
                });
            }
        });
        // Show most social connections
        const socialConnections = Array.from(interactions.entries())
            .map(([character, connections]) => ({ character, connectionCount: connections.size }))
            .sort((a, b) => b.connectionCount - a.connectionCount)
            .slice(0, 5);
        if (socialConnections.length > 0) {
            console.log('Most Socially Connected Characters (based on recent events):');
            socialConnections.forEach((connection, index) => {
                console.log(`${index + 1}. ${connection.character} (${connection.connectionCount} recent interactions)`);
            });
        }
        // Character location distribution
        console.log('\n📍 Character Location Distribution:');
        const locationCounts = new Map();
        for (const individual of allIndividuals.slice(0, 50)) { // Sample of 50 characters
            try {
                const characterLocation = await getCharacterLocation(simulation, individual);
                if (characterLocation) {
                    locationCounts.set(characterLocation, (locationCounts.get(characterLocation) || 0) + 1);
                }
            }
            catch (error) {
                // Skip characters without clear location
            }
        }
        const topLocations = Array.from(locationCounts.entries())
            .sort((a, b) => b[1] - a[1])
            .slice(0, 5);
        topLocations.forEach(([location, count]) => {
            console.log(`${location}: ${count} characters`);
        });
    }
    catch (error) {
        console.error('❌ Error:', error);
    }
    finally {
        simulation.disconnect();
        console.log('\n📴 Disconnected from simulation');
    }
}
// Helper function to determine character location
async function getCharacterLocation(simulation, individual) {
    if (!individual.home_id)
        return null;
    try {
        const buildings = await simulation.client.spacetimeClient.getBuildings();
        const homeBuilding = buildings.find(b => b.id === individual.home_id);
        if (!homeBuilding)
            return null;
        const cities = await simulation.client.spacetimeClient.getCities();
        const homeCity = cities.find(c => c.id === homeBuilding.city_id);
        return homeCity ? homeCity.name : null;
    }
    catch {
        return null;
    }
}
// Utility function for AI agents to get character data
export async function getCharacterData(characterId, locationFilter) {
    const simulation = new WorldSimulation();
    await simulation.connect();
    try {
        const allIndividuals = await simulation.client.spacetimeClient.getIndividuals();
        let characters = allIndividuals;
        // Filter by specific character
        if (characterId) {
            characters = characters.filter(char => char.id === characterId);
        }
        // Filter by location (if possible to determine)
        if (locationFilter) {
            const filteredCharacters = [];
            for (const char of characters) {
                const location = await getCharacterLocation(simulation, char);
                if (location === locationFilter) {
                    filteredCharacters.push(char);
                }
            }
            characters = filteredCharacters;
        }
        // Get story context for featured characters
        const characterStories = await Promise.all(characters.slice(0, 10).map(async (char) => {
            try {
                const storyContext = await simulation.narrative.getCharacterStoryContext(char.id, 24);
                return {
                    character: char,
                    story_context: storyContext,
                    formatted_profile: createCharacterProfile(char)
                };
            }
            catch {
                return {
                    character: char,
                    story_context: null,
                    formatted_profile: createCharacterProfile(char)
                };
            }
        }));
        return {
            total_characters: allIndividuals.length,
            filtered_characters: characters.length,
            character_stories: characterStories,
            age_distribution: getAgeDistribution(allIndividuals),
            activity_distribution: getActivityDistribution(allIndividuals),
            wellness_stats: getWellnessStats(allIndividuals)
        };
    }
    finally {
        simulation.disconnect();
    }
}
// Helper functions for character analysis
function getAgeDistribution(individuals) {
    const ageGroups = { young: 0, adult: 0, senior: 0 };
    individuals.forEach(ind => {
        if (ind.age < 30)
            ageGroups.young++;
        else if (ind.age < 60)
            ageGroups.adult++;
        else
            ageGroups.senior++;
    });
    return ageGroups;
}
function getActivityDistribution(individuals) {
    const activities = new Map();
    individuals.forEach(ind => {
        const activity = formatIndividualActivity(ind.current_activity);
        activities.set(activity, (activities.get(activity) || 0) + 1);
    });
    return Object.fromEntries(activities);
}
function getWellnessStats(individuals) {
    if (individuals.length === 0)
        return { avgEnergy: 0, avgHappiness: 0, avgHealth: 0 };
    const totals = individuals.reduce((acc, ind) => ({
        energy: acc.energy + ind.energy,
        happiness: acc.happiness + ind.happiness,
        health: acc.health + ind.health
    }), { energy: 0, happiness: 0, health: 0 });
    return {
        avgEnergy: totals.energy / individuals.length,
        avgHappiness: totals.happiness / individuals.length,
        avgHealth: totals.health / individuals.length
    };
}
// Run the example
if (import.meta.url === `file://${process.argv[1]}`) {
    characterStoriesExample().catch(console.error);
}
export { characterStoriesExample };
//# sourceMappingURL=character-stories.js.map