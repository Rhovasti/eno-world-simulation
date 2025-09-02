/**
 * Time conversion and calculation utilities for World Simulation
 * Handles simulation time, real time, and valley-specific time zones
 */
import { Valley, TimeOfDay } from '../lib/types';
export declare const HOURS_PER_DAY = 24;
export declare const DAYS_PER_WEEK = 6;
export declare const DAYS_PER_MONTH = 30;
export declare const DAYS_PER_YEAR = 360;
export declare const WEEKS_PER_YEAR = 60;
/**
 * Convert simulation hour to day/week/month/year
 */
export interface SimulationDate {
    hour: number;
    day_of_week: number;
    hour_of_day: number;
    day_of_month: number;
    month_of_year: number;
    year: number;
    total_days: number;
    total_weeks: number;
}
/**
 * Convert simulation hour to comprehensive date
 */
export declare function simulationHourToDate(hour: number): SimulationDate;
/**
 * Format simulation date as human-readable string
 */
export declare function formatSimulationDate(hour: number, includeTime?: boolean): string;
/**
 * Calculate time of day based on hour (0-23)
 */
export declare function calculateTimeOfDay(hour: number): TimeOfDay;
/**
 * Calculate time of day for a specific valley
 * Each valley has a different time zone based on continental rotation
 */
export declare function calculateTimeOfDayForValley(valley: Valley, baseHour: number): TimeOfDay;
/**
 * Get opposite time of day
 */
export declare function getOppositeTimeOfDay(timeOfDay: TimeOfDay): TimeOfDay;
/**
 * Get next time of day in cycle
 */
export declare function getNextTimeOfDay(timeOfDay: TimeOfDay): TimeOfDay;
/**
 * Get previous time of day in cycle
 */
export declare function getPreviousTimeOfDay(timeOfDay: TimeOfDay): TimeOfDay;
/**
 * Calculate time difference between valleys
 */
export declare function getValleyTimeOffset(fromValley: Valley, toValley: Valley): number;
/**
 * Convert time from one valley to another
 */
export declare function convertTimeToValley(hour: number, fromValley: Valley, toValley: Valley): number;
/**
 * Check if it's a leap year occurrence
 * Every year has 2 leap occurrences (60 hours each)
 * Every 4 years the leap occurrence is 72 hours
 */
export declare function isLeapOccurrence(hour: number): boolean;
/**
 * Get leap occurrence duration
 */
export declare function getLeapOccurrenceDuration(hour: number): number;
/**
 * Calculate real-time equivalent based on tick rate
 */
export interface TickRateConfig {
    name: string;
    sim_hour_to_real_ms: number;
    description: string;
}
export declare const TICK_RATES: Record<string, TickRateConfig>;
/**
 * Calculate real time duration for simulation time
 */
export declare function calculateRealTimeDuration(simulationHours: number, tickRate?: string): number;
/**
 * Format real time duration
 */
export declare function formatRealTimeDuration(milliseconds: number): string;
/**
 * Calculate estimated time to reach target hour
 */
export declare function calculateTimeToTarget(currentHour: number, targetHour: number, tickRate?: string): {
    simulation_hours: number;
    real_time_ms: number;
    formatted: string;
};
/**
 * Get time range for queries
 */
export interface TimeRangeHelper {
    current_hour: number;
    hours_back: number;
    start_hour: number;
    end_hour: number;
    date_range: {
        start: SimulationDate;
        end: SimulationDate;
    };
}
export declare function createTimeRange(currentHour: number, hoursBack: number): TimeRangeHelper;
/**
 * Check if hour is within time range
 */
export declare function isHourInRange(hour: number, range: TimeRangeHelper): boolean;
/**
 * Get relative time description
 */
export declare function getRelativeTimeDescription(targetHour: number, currentHour: number): string;
/**
 * Time zone information for valleys
 */
export interface ValleyTimeZone {
    valley: Valley;
    offset_hours: number;
    current_time_of_day: TimeOfDay;
    description: string;
}
export declare function getValleyTimeZones(baseHour: number): ValleyTimeZone[];
//# sourceMappingURL=time-helpers.d.ts.map