/**
 * World Simulation Client Examples
 * Collection of practical usage examples for AI agents and developers
 */

// Basic Usage Examples
export { basicUsageExample } from './basic-usage.js';
export { timeManagementExample, getTimeContext } from './time-management.js';
export { locationExplorerExample, getLocationOverview, findInterestingLocations } from './location-explorer.js';

// AI Integration Examples  
export { aiNarrativeExample, getAIStoryData } from './ai-narrative-example.js';
export { characterStoriesExample, getCharacterData } from './character-stories.js';

/**
 * Run all examples in sequence
 */
export async function runAllExamples() {
  console.log('🚀 Running All World Simulation Client Examples\n');
  console.log('=' .repeat(80));
  
  try {
    const { basicUsageExample } = await import('./basic-usage.js');
    await basicUsageExample();
    
    console.log('\n' + '='.repeat(80));
    
    const { timeManagementExample } = await import('./time-management.js');
    await timeManagementExample();
    
    console.log('\n' + '='.repeat(80));
    
    const { locationExplorerExample } = await import('./location-explorer.js');
    await locationExplorerExample();
    
    console.log('\n' + '='.repeat(80));
    
    const { aiNarrativeExample } = await import('./ai-narrative-example.js');
    await aiNarrativeExample();
    
    console.log('\n' + '='.repeat(80));
    
    const { characterStoriesExample } = await import('./character-stories.js');
    await characterStoriesExample();
    
    console.log('\n' + '='.repeat(80));
    console.log('✅ All examples completed successfully!');
    
  } catch (error) {
    console.error('❌ Error running examples:', error);
  }
}

/**
 * Quick AI integration demo
 * Demonstrates the most useful functions for AI agents
 */
export async function quickAIDemo() {
  console.log('🤖 Quick AI Integration Demo\n');
  
  try {
    // Import utility functions
    const { getTimeContext } = await import('./time-management.js');
    const { getAIStoryData } = await import('./ai-narrative-example.js');
    const { getLocationOverview } = await import('./location-explorer.js');
    
    // Get current time context
    console.log('⏰ Getting time context...');
    const timeContext = await getTimeContext();
    console.log(`Current time: ${timeContext.formatted_date}`);
    console.log(`Auto-tick: ${timeContext.auto_tick_enabled ? 'Running' : 'Stopped'}\n`);
    
    // Get location overview
    console.log('🗺️  Getting location overview...');
    const locationOverview = await getLocationOverview();
    console.log(`Total cities: ${locationOverview.world_stats.total_cities}`);
    console.log(`Sample cities: ${locationOverview.sample_cities.map(c => c.city).join(', ')}\n`);
    
    // Get story data for an interesting location
    console.log('📖 Getting story data...');
    const storyData = await getAIStoryData('Citadel of Utaia');
    console.log(`Location: ${storyData.location_state.city}`);
    console.log(`Characters available: ${storyData.available_characters.length}`);
    console.log(`Recent events: ${storyData.recent_events.length}`);
    console.log(`Story suggestions: ${storyData.story_suggestions.length}\n`);
    
    console.log('✅ Quick demo completed!');
    console.log('👉 Use these utility functions in your AI agent implementation');
    
  } catch (error) {
    console.error('❌ Error in quick demo:', error);
  }
}

// Export example metadata for documentation
export const EXAMPLES_METADATA = {
  'basic-usage': {
    title: 'Basic Usage',
    description: 'Fundamental client operations and connection management',
    complexity: 'Beginner',
    estimated_runtime: '< 30 seconds'
  },
  'time-management': {
    title: 'Time Management', 
    description: 'Time queries, calculations, and valley time zones',
    complexity: 'Intermediate',
    estimated_runtime: '< 1 minute'
  },
  'location-explorer': {
    title: 'Location Explorer',
    description: 'Location-based queries and world exploration',
    complexity: 'Intermediate', 
    estimated_runtime: '1-2 minutes'
  },
  'ai-narrative': {
    title: 'AI Narrative Generation',
    description: 'AI-friendly narrative generation and story context',
    complexity: 'Advanced',
    estimated_runtime: '2-3 minutes'
  },
  'character-stories': {
    title: 'Character Stories',
    description: 'Character-focused narrative and individual tracking',
    complexity: 'Advanced',
    estimated_runtime: '2-4 minutes'
  }
};

// Run examples from command line
if (import.meta.url === `file://${process.argv[1]}`) {
  const command = process.argv[2];
  
  switch (command) {
    case 'all':
      runAllExamples().catch(console.error);
      break;
    case 'quick':
      quickAIDemo().catch(console.error);
      break;
    case 'basic':
      import('./basic-usage.js').then(m => m.basicUsageExample()).catch(console.error);
      break;
    case 'time':
      import('./time-management.js').then(m => m.timeManagementExample()).catch(console.error);
      break;
    case 'location':
      import('./location-explorer.js').then(m => m.locationExplorerExample()).catch(console.error);
      break;
    case 'ai':
      import('./ai-narrative-example.js').then(m => m.aiNarrativeExample()).catch(console.error);
      break;
    case 'characters':
      import('./character-stories.js').then(m => m.characterStoriesExample()).catch(console.error);
      break;
    default:
      console.log('World Simulation Client Examples');
      console.log('Usage: node examples/index.js [command]');
      console.log('');
      console.log('Commands:');
      console.log('  all        - Run all examples');
      console.log('  quick      - Quick AI integration demo');
      console.log('  basic      - Basic usage example');
      console.log('  time       - Time management example');
      console.log('  location   - Location explorer example');
      console.log('  ai         - AI narrative example');
      console.log('  characters - Character stories example');
      break;
  }
}