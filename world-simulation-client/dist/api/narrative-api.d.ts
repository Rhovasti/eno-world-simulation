/**
 * AI-friendly narrative API for World Simulation
 * Provides structured data and context for AI storytelling systems
 */
import { WorldSimulationClient } from '../lib/query-interface';
import { StoryContext, LocationState, HistoricalEvent, Individual, Building, QueryOptions } from '../lib/types';
/**
 * Narrative-focused API for AI agents and storytelling systems
 */
export declare class NarrativeAPI {
    private client;
    constructor(client: WorldSimulationClient);
    /**
     * Get comprehensive story context for a location
     * Perfect for AI agents generating location-based narratives
     */
    getContextForStory(location: string, characterTypes?: string[], options?: QueryOptions): Promise<StoryContext>;
    /**
     * Generate a complete location narrative
     * Returns a human-readable story about what's happening at a location
     */
    generateLocationNarrative(location: string, timeRangeHours?: number, includeCharacters?: boolean): Promise<string>;
    /**
     * Get character-focused story context
     * Perfect for narratives centered around specific individuals
     */
    getCharacterStoryContext(characterId: number, hoursBack?: number): Promise<{
        character: Individual;
        story_events: HistoricalEvent[];
        location_context: LocationState | null;
        narrative_summary: string;
    }>;
    /**
     * Get building-focused story context
     * Perfect for narratives about specific buildings or locations
     */
    getBuildingStoryContext(buildingId: number, hoursBack?: number): Promise<{
        building: Building;
        story_events: HistoricalEvent[];
        occupants: Individual[];
        narrative_summary: string;
    }>;
    /**
     * Get time-specific narrative
     * Perfect for "what was happening at this time" queries
     */
    getTimeSpecificNarrative(location: string, targetHour: number, contextHours?: number): Promise<{
        target_time: string;
        location: string;
        events_around_time: HistoricalEvent[];
        narrative_description: string;
    }>;
    /**
     * Get comparative narrative between locations
     * Perfect for "compare these two places" queries
     */
    getComparativeNarrative(location1: string, location2: string, hoursBack?: number): Promise<{
        location1_context: StoryContext;
        location2_context: StoryContext;
        comparison_narrative: string;
    }>;
    /**
     * Get narrative suggestions for AI agents
     * Returns story ideas based on current simulation state
     */
    getNarrativeSuggestions(theme?: 'adventure' | 'slice-of-life' | 'mystery' | 'drama' | 'comedy', location?: string): Promise<{
        theme: string;
        suggestions: Array<{
            title: string;
            premise: string;
            characters: string[];
            setting: string;
            potential_conflicts: string[];
        }>;
    }>;
    /**
     * Generate narrative suggestions based on context
     */
    private generateNarrativeSuggestions;
    /**
     * Generate character-focused narrative
     */
    private generateCharacterNarrative;
    /**
     * Generate building-focused narrative
     */
    private generateBuildingNarrative;
    /**
     * Generate comparative narrative between two locations
     */
    private generateComparativeNarrative;
    /**
     * Generate story ideas based on current simulation state
     */
    private generateStoryIdeas;
}
//# sourceMappingURL=narrative-api.d.ts.map