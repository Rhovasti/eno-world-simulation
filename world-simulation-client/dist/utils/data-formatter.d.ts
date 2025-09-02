/**
 * Data formatting utilities for World Simulation
 * Converts raw simulation data into human-readable and AI-friendly formats
 */
import { Individual, Building, City, HistoricalEvent, TimeOfDay, Valley, IndividualAction, BuildingType, JobType } from '../lib/types';
/**
 * Format individual information for narrative use
 */
export declare function formatIndividual(individual: Individual, includeStats?: boolean): string;
/**
 * Format building information
 */
export declare function formatBuilding(building: Building, includeDetails?: boolean): string;
/**
 * Format city information
 */
export declare function formatCity(city: City, includeMetrics?: boolean): string;
/**
 * Format historical event for narrative use
 */
export declare function formatHistoricalEvent(event: HistoricalEvent, currentHour: number, includeContext?: boolean): string;
/**
 * Format individual activity
 */
export declare function formatIndividualActivity(activity: IndividualAction): string;
/**
 * Format building type
 */
export declare function formatBuildingType(buildingType: BuildingType): string;
/**
 * Format job type
 */
export declare function formatJobType(jobType: JobType): string;
/**
 * Format needs as human-readable text
 */
export declare function formatNeeds(needs: Record<string, number>): string;
/**
 * Get need level description
 */
export declare function getNeedLevel(value: number): string;
/**
 * Format wellness stats
 */
export declare function formatWellness(energy: number, happiness: number, health: number): string;
/**
 * Format condition level
 */
export declare function formatCondition(condition: number): string;
/**
 * Format efficiency level
 */
export declare function formatEfficiency(efficiency: number): string;
/**
 * Format city metrics
 */
export declare function formatCityMetrics(metrics: {
    stability: number;
    culture: number;
    prosperity: number;
    safety: number;
    sustainability: number;
}): string;
/**
 * Format time of day with description
 */
export declare function formatTimeOfDay(timeOfDay: TimeOfDay, valley: Valley): string;
/**
 * Format valley description
 */
export declare function formatValleyDescription(valley: Valley): string;
/**
 * Create narrative summary for location
 */
export declare function createLocationNarrative(cityName: string, population: number, timeOfDay: TimeOfDay, valley: Valley, recentEvents: HistoricalEvent[], currentHour: number): string;
/**
 * Create character profile for narrative use
 */
export declare function createCharacterProfile(individual: Individual, context?: string): string;
/**
 * Format event timeline
 */
export declare function formatEventTimeline(events: HistoricalEvent[], currentHour: number, maxEvents?: number): string;
/**
 * Create summary statistics
 */
export interface SummaryStats {
    population: number;
    active_buildings: number;
    recent_events: number;
    average_wellness: number;
    dominant_activity: string;
}
export declare function createSummaryStats(individuals: Individual[], buildings: Building[], events: HistoricalEvent[]): SummaryStats;
/**
 * Format summary stats as narrative
 */
export declare function formatSummaryStats(stats: SummaryStats): string;
//# sourceMappingURL=data-formatter.d.ts.map