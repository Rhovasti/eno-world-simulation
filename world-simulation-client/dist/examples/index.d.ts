/**
 * World Simulation Client Examples
 * Collection of practical usage examples for AI agents and developers
 */
export { basicUsageExample } from './basic-usage.js';
export { timeManagementExample, getTimeContext } from './time-management.js';
export { locationExplorerExample, getLocationOverview, findInterestingLocations } from './location-explorer.js';
export { aiNarrativeExample, getAIStoryData } from './ai-narrative-example.js';
export { characterStoriesExample, getCharacterData } from './character-stories.js';
/**
 * Run all examples in sequence
 */
export declare function runAllExamples(): Promise<void>;
/**
 * Quick AI integration demo
 * Demonstrates the most useful functions for AI agents
 */
export declare function quickAIDemo(): Promise<void>;
export declare const EXAMPLES_METADATA: {
    'basic-usage': {
        title: string;
        description: string;
        complexity: string;
        estimated_runtime: string;
    };
    'time-management': {
        title: string;
        description: string;
        complexity: string;
        estimated_runtime: string;
    };
    'location-explorer': {
        title: string;
        description: string;
        complexity: string;
        estimated_runtime: string;
    };
    'ai-narrative': {
        title: string;
        description: string;
        complexity: string;
        estimated_runtime: string;
    };
    'character-stories': {
        title: string;
        description: string;
        complexity: string;
        estimated_runtime: string;
    };
};
//# sourceMappingURL=index.d.ts.map