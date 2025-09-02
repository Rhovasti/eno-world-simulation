/**
 * AI Narrative Example for World Simulation Client
 * Demonstrates AI-friendly narrative generation and story context
 */
import { WorldSimulation } from '../index.js';
async function aiNarrativeExample() {
    console.log('🤖 World Simulation Client - AI Narrative Example\n');
    const simulation = new WorldSimulation('ws://localhost:3001');
    try {
        await simulation.connect();
        console.log('✅ Connected to simulation\n');
        // Example 1: Get story context for narrative generation
        console.log('📖 Story Context Example:');
        console.log('Getting story context for the Citadel of Utaia...\n');
        const storyContext = await simulation.narrative.getContextForStory('Citadel of Utaia');
        console.log(`📍 Location: ${storyContext.location.city}`);
        console.log(`🏔️  Valley: ${storyContext.location.valley}`);
        console.log(`🌅 Time: ${storyContext.location.time_of_day}`);
        console.log(`👥 Population: ${storyContext.location.population.toLocaleString()}`);
        console.log(`🏢 Active Buildings: ${storyContext.location.active_buildings}`);
        console.log(`👤 Characters Available: ${storyContext.characters.length}`);
        console.log(`📰 Recent Events: ${storyContext.recent_events.length}\n`);
        // Show narrative suggestions
        if (storyContext.suggested_narratives.length > 0) {
            console.log('💡 AI Narrative Suggestions:');
            storyContext.suggested_narratives.forEach((suggestion, index) => {
                console.log(`${index + 1}. ${suggestion}`);
            });
            console.log('');
        }
        // Example 2: Generate a complete location narrative
        console.log('📝 Generated Location Narrative:');
        console.log('─'.repeat(50));
        const narrative = await simulation.narrative.generateLocationNarrative('Tsin', 12, true);
        console.log(narrative);
        console.log('─'.repeat(50) + '\n');
        // Example 3: Compare two locations
        console.log('⚖️  Comparative Analysis:');
        const comparison = await simulation.narrative.getComparativeNarrative('Tsin', 'Beitsa', 6);
        console.log(comparison.comparison_narrative + '\n');
        // Example 4: Get narrative suggestions for different themes
        console.log('🎭 Theme-Based Story Ideas:');
        const themes = ['adventure', 'mystery', 'drama', 'slice-of-life'];
        for (const theme of themes) {
            console.log(`\n🎨 ${theme.toUpperCase()} Theme:`);
            const suggestions = await simulation.narrative.getNarrativeSuggestions(theme, 'Citadel of Utaia');
            if (suggestions.suggestions.length > 0) {
                const idea = suggestions.suggestions[0];
                console.log(`📚 "${idea.title}"`);
                console.log(`📖 ${idea.premise}`);
                console.log(`🎭 Characters: ${idea.characters.join(', ')}`);
                console.log(`📍 Setting: ${idea.setting}`);
                console.log(`⚡ Conflicts: ${idea.potential_conflicts.join(', ')}`);
            }
        }
        // Example 5: Time-specific narrative
        console.log('\n🕰️  Historical Moment:');
        const currentHour = await simulation.client.getCurrentHour();
        const targetHour = Math.max(0, currentHour - 3); // 3 hours ago
        const timeNarrative = await simulation.narrative.getTimeSpecificNarrative('Jouy', targetHour, 2 // 2 hours of context
        );
        console.log(`📅 ${timeNarrative.target_time}`);
        console.log(`📍 ${timeNarrative.location}`);
        console.log(`📝 ${timeNarrative.narrative_description}\n`);
    }
    catch (error) {
        console.error('❌ Error:', error);
    }
    finally {
        simulation.disconnect();
        console.log('📴 Disconnected from simulation');
    }
}
// Utility function for AI agents to get contextual story data
async function getAIStoryDataImpl(cityName) {
    const simulation = new WorldSimulation();
    await simulation.connect();
    try {
        const context = await simulation.narrative.getContextForStory(cityName);
        const narrative = await simulation.narrative.generateLocationNarrative(cityName, 6);
        return {
            location_state: context.location,
            available_characters: context.characters.slice(0, 5), // Top 5 characters
            recent_events: context.recent_events,
            narrative_summary: narrative,
            story_suggestions: context.suggested_narratives
        };
    }
    finally {
        simulation.disconnect();
    }
}
// Run the example
if (import.meta.url === `file://${process.argv[1]}`) {
    aiNarrativeExample().catch(console.error);
}
export { aiNarrativeExample, getAIStoryDataImpl as getAIStoryData };
//# sourceMappingURL=ai-narrative-example.js.map