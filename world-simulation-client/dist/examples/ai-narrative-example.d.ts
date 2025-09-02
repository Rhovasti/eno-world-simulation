/**
 * AI Narrative Example for World Simulation Client
 * Demonstrates AI-friendly narrative generation and story context
 */
declare function aiNarrativeExample(): Promise<void>;
declare function getAIStoryDataImpl(cityName: string): Promise<{
    location_state: any;
    available_characters: any;
    recent_events: any;
    narrative_summary: any;
    story_suggestions: any;
}>;
export { aiNarrativeExample, getAIStoryDataImpl as getAIStoryData };
//# sourceMappingURL=ai-narrative-example.d.ts.map