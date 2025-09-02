/**
 * Location Explorer Example for World Simulation Client
 * Demonstrates location-based queries and exploration features
 */
import { Valley } from '../index.js';
declare function locationExplorerExample(): Promise<void>;
export declare function getLocationOverview(valleyFilter?: Valley): Promise<{
    world_stats: import("../index.js").LocationStats;
    sample_cities: any;
    total_cities_available: number;
    capital_cities: import("../index.js").CityInfo[];
}>;
export declare function findInterestingLocations(criteria: {
    minPopulation?: number;
    maxPopulation?: number;
    valley?: Valley;
    includeCapitals?: boolean;
    includeRecentActivity?: boolean;
}): Promise<{
    name: string;
    valley: any;
    population: any;
    time_of_day: any;
    active_buildings: any;
    recent_events: number;
}[]>;
export { locationExplorerExample, getLocationOverview, findInterestingLocations };
//# sourceMappingURL=location-explorer.d.ts.map