/**
 * Location mapping utilities for the World Simulation
 * Maps cities to valleys and provides location-based queries
 */
import { Valley } from '../lib/types';
export declare const CITY_VALLEY_MAP: Record<string, Valley>;
/**
 * Get the valley for a given city name
 */
export declare function getCityValley(cityName: string): Valley | null;
/**
 * Get all cities in a specific valley
 */
export declare function getCitiesInValley(valley: Valley): string[];
/**
 * Get all available valleys
 */
export declare function getAllValleys(): Valley[];
/**
 * Get total number of cities
 */
export declare function getTotalCityCount(): number;
/**
 * Get city count by valley
 */
export declare function getCityCountByValley(): Record<Valley, number>;
/**
 * Search for cities by partial name match
 */
export declare function searchCities(query: string): string[];
/**
 * Get neighboring valleys (for time zone calculations)
 */
export declare function getNeighboringValleys(valley: Valley): Valley[];
/**
 * Get the opposite valley (for day/night cycle)
 */
export declare function getOppositeValley(valley: Valley): Valley;
/**
 * Validate if a city name exists
 */
export declare function isValidCity(cityName: string): boolean;
/**
 * Get random city from a valley
 */
export declare function getRandomCityFromValley(valley: Valley): string | null;
/**
 * Get random city from any valley
 */
export declare function getRandomCity(): string;
/**
 * Get city information with valley
 */
export interface CityInfo {
    name: string;
    valley: Valley;
    isCapital: boolean;
}
export declare function getCityInfo(cityName: string): CityInfo | null;
/**
 * Get all capital cities (Citadels)
 */
export declare function getCapitalCities(): CityInfo[];
/**
 * Get location statistics
 */
export interface LocationStats {
    total_cities: number;
    valley_distribution: Record<Valley, number>;
    capital_cities: string[];
    largest_valley: Valley;
    smallest_valley: Valley;
}
export declare function getLocationStats(): LocationStats;
//# sourceMappingURL=location-mapper.d.ts.map