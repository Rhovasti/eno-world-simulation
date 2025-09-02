/**
 * Character Stories Example for World Simulation Client
 * Demonstrates character-focused narrative generation and individual tracking
 */
declare function characterStoriesExample(): Promise<void>;
export declare function getCharacterData(characterId?: number, locationFilter?: string): Promise<{
    total_characters: any;
    filtered_characters: any;
    character_stories: any;
    age_distribution: {
        young: number;
        adult: number;
        senior: number;
    };
    activity_distribution: {
        [k: string]: number;
    };
    wellness_stats: {
        avgEnergy: number;
        avgHappiness: number;
        avgHealth: number;
    };
}>;
export { characterStoriesExample, getCharacterData };
//# sourceMappingURL=character-stories.d.ts.map