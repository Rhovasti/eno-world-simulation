/**
 * Time conversion and calculation utilities for World Simulation
 * Handles simulation time, real time, and valley-specific time zones
 */

import { Valley, TimeOfDay } from '../lib/types';

// Simulation time constants
export const HOURS_PER_DAY = 24;
export const DAYS_PER_WEEK = 6;  // Custom calendar
export const DAYS_PER_MONTH = 30;
export const DAYS_PER_YEAR = 360;
export const WEEKS_PER_YEAR = 60; // 360/6

/**
 * Convert simulation hour to day/week/month/year
 */
export interface SimulationDate {
  hour: number;
  day_of_week: number; // 0-5
  hour_of_day: number; // 0-23
  day_of_month: number; // 1-30
  month_of_year: number; // 1-12
  year: number;
  total_days: number;
  total_weeks: number;
}

/**
 * Convert simulation hour to comprehensive date
 */
export function simulationHourToDate(hour: number): SimulationDate {
  const totalDays = Math.floor(hour / HOURS_PER_DAY);
  const totalWeeks = Math.floor(totalDays / DAYS_PER_WEEK);
  const year = Math.floor(totalDays / DAYS_PER_YEAR);
  const dayOfYear = totalDays % DAYS_PER_YEAR;
  const monthOfYear = Math.floor(dayOfYear / DAYS_PER_MONTH) + 1;
  const dayOfMonth = (dayOfYear % DAYS_PER_MONTH) + 1;

  return {
    hour,
    day_of_week: totalDays % DAYS_PER_WEEK,
    hour_of_day: hour % HOURS_PER_DAY,
    day_of_month: dayOfMonth,
    month_of_year: monthOfYear,
    year,
    total_days: totalDays,
    total_weeks: totalWeeks
  };
}

/**
 * Format simulation date as human-readable string
 */
export function formatSimulationDate(hour: number, includeTime: boolean = true): string {
  const date = simulationHourToDate(hour);
  const dayNames = ['Solday', 'Lunday', 'Marday', 'Merday', 'Jovday', 'Venday'];
  const monthNames = [
    'Primos', 'Secundos', 'Tertios', 'Quartos', 'Quintos', 'Sextos',
    'Septimos', 'Octavos', 'Novenos', 'Decimios', 'Undecimos', 'Decimoseg'
  ];

  const dayName = dayNames[date.day_of_week];
  const monthName = monthNames[date.month_of_year - 1];
  
  if (includeTime) {
    const timeStr = `${date.hour_of_day.toString().padStart(2, '0')}:00`;
    return `${dayName}, ${date.day_of_month} ${monthName} Year ${date.year}, ${timeStr}`;
  } else {
    return `${dayName}, ${date.day_of_month} ${monthName} Year ${date.year}`;
  }
}

/**
 * Calculate time of day based on hour (0-23)
 */
export function calculateTimeOfDay(hour: number): TimeOfDay {
  const hourOfDay = hour % 24;
  
  if (hourOfDay >= 5 && hourOfDay < 12) return TimeOfDay.Dawn;
  if (hourOfDay >= 12 && hourOfDay < 17) return TimeOfDay.Day;
  if (hourOfDay >= 17 && hourOfDay < 21) return TimeOfDay.Dusk;
  return TimeOfDay.Night;
}

/**
 * Calculate time of day for a specific valley
 * Each valley has a different time zone based on continental rotation
 */
export function calculateTimeOfDayForValley(valley: Valley, baseHour: number): TimeOfDay {
  const baseTimeOfDay = calculateTimeOfDay(baseHour);
  
  switch (valley) {
    case Valley.Day:
      return baseTimeOfDay;
      
    case Valley.Night:
      // Opposite side of continent (12 hours offset)
      return getOppositeTimeOfDay(baseTimeOfDay);
      
    case Valley.Dawn:
      // 6 hours ahead of Day valley
      return getNextTimeOfDay(baseTimeOfDay);
      
    case Valley.Dusk:
      // 6 hours behind Day valley (opposite of Dawn)
      return getPreviousTimeOfDay(baseTimeOfDay);
  }
}

/**
 * Get opposite time of day
 */
export function getOppositeTimeOfDay(timeOfDay: TimeOfDay): TimeOfDay {
  switch (timeOfDay) {
    case TimeOfDay.Dawn: return TimeOfDay.Dusk;
    case TimeOfDay.Day: return TimeOfDay.Night;
    case TimeOfDay.Dusk: return TimeOfDay.Dawn;
    case TimeOfDay.Night: return TimeOfDay.Day;
  }
}

/**
 * Get next time of day in cycle
 */
export function getNextTimeOfDay(timeOfDay: TimeOfDay): TimeOfDay {
  switch (timeOfDay) {
    case TimeOfDay.Dawn: return TimeOfDay.Day;
    case TimeOfDay.Day: return TimeOfDay.Dusk;
    case TimeOfDay.Dusk: return TimeOfDay.Night;
    case TimeOfDay.Night: return TimeOfDay.Dawn;
  }
}

/**
 * Get previous time of day in cycle
 */
export function getPreviousTimeOfDay(timeOfDay: TimeOfDay): TimeOfDay {
  switch (timeOfDay) {
    case TimeOfDay.Dawn: return TimeOfDay.Night;
    case TimeOfDay.Day: return TimeOfDay.Dawn;
    case TimeOfDay.Dusk: return TimeOfDay.Day;
    case TimeOfDay.Night: return TimeOfDay.Dusk;
  }
}

/**
 * Calculate time difference between valleys
 */
export function getValleyTimeOffset(fromValley: Valley, toValley: Valley): number {
  const offsets: Record<Valley, number> = {
    [Valley.Day]: 0,     // Base time zone
    [Valley.Dawn]: 6,    // 6 hours ahead
    [Valley.Dusk]: -6,   // 6 hours behind  
    [Valley.Night]: 12   // 12 hours ahead (opposite)
  };
  
  return offsets[toValley] - offsets[fromValley];
}

/**
 * Convert time from one valley to another
 */
export function convertTimeToValley(hour: number, fromValley: Valley, toValley: Valley): number {
  const offset = getValleyTimeOffset(fromValley, toValley);
  return hour + offset;
}

/**
 * Check if it's a leap year occurrence
 * Every year has 2 leap occurrences (60 hours each)
 * Every 4 years the leap occurrence is 72 hours
 */
export function isLeapOccurrence(hour: number): boolean {
  const date = simulationHourToDate(hour);
  const dayOfYear = date.total_days % DAYS_PER_YEAR;
  
  // Leap occurrences happen at day 120 and day 240 of each year
  return dayOfYear === 120 || dayOfYear === 240;
}

/**
 * Get leap occurrence duration
 */
export function getLeapOccurrenceDuration(hour: number): number {
  const date = simulationHourToDate(hour);
  const isQuadrennialYear = date.year % 4 === 0;
  
  return isQuadrennialYear ? 72 : 60; // Hours
}

/**
 * Calculate real-time equivalent based on tick rate
 */
export interface TickRateConfig {
  name: string;
  sim_hour_to_real_ms: number;
  description: string;
}

export const TICK_RATES: Record<string, TickRateConfig> = {
  realtime: {
    name: 'Real-time',
    sim_hour_to_real_ms: 3600000, // 1 hour = 1 hour
    description: '1 simulation hour = 1 real hour'
  },
  fast: {
    name: 'Fast',
    sim_hour_to_real_ms: 60000, // 1 hour = 1 minute
    description: '1 simulation hour = 1 real minute'
  },
  very_fast: {
    name: 'Very Fast',
    sim_hour_to_real_ms: 10000, // 1 hour = 10 seconds
    description: '1 simulation hour = 10 real seconds'
  },
  test: {
    name: 'Test',
    sim_hour_to_real_ms: 1000, // 1 hour = 1 second
    description: '1 simulation hour = 1 real second'
  },
  slow: {
    name: 'Slow',
    sim_hour_to_real_ms: 300000, // 1 hour = 5 minutes
    description: '1 simulation hour = 5 real minutes'
  }
};

/**
 * Calculate real time duration for simulation time
 */
export function calculateRealTimeDuration(
  simulationHours: number, 
  tickRate: string = 'realtime'
): number {
  const config = TICK_RATES[tickRate];
  if (!config) {
    throw new Error(`Unknown tick rate: ${tickRate}`);
  }
  
  return simulationHours * config.sim_hour_to_real_ms;
}

/**
 * Format real time duration
 */
export function formatRealTimeDuration(milliseconds: number): string {
  const seconds = Math.floor(milliseconds / 1000);
  const minutes = Math.floor(seconds / 60);
  const hours = Math.floor(minutes / 60);
  const days = Math.floor(hours / 24);

  if (days > 0) {
    return `${days}d ${hours % 24}h ${minutes % 60}m`;
  } else if (hours > 0) {
    return `${hours}h ${minutes % 60}m`;
  } else if (minutes > 0) {
    return `${minutes}m ${seconds % 60}s`;
  } else {
    return `${seconds}s`;
  }
}

/**
 * Calculate estimated time to reach target hour
 */
export function calculateTimeToTarget(
  currentHour: number,
  targetHour: number,
  tickRate: string = 'realtime'
): { simulation_hours: number; real_time_ms: number; formatted: string } {
  const simulationHours = targetHour - currentHour;
  const realTimeMs = calculateRealTimeDuration(simulationHours, tickRate);
  
  return {
    simulation_hours: simulationHours,
    real_time_ms: realTimeMs,
    formatted: formatRealTimeDuration(realTimeMs)
  };
}

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

export function createTimeRange(currentHour: number, hoursBack: number): TimeRangeHelper {
  const startHour = Math.max(0, currentHour - hoursBack);
  
  return {
    current_hour: currentHour,
    hours_back: hoursBack,
    start_hour: startHour,
    end_hour: currentHour,
    date_range: {
      start: simulationHourToDate(startHour),
      end: simulationHourToDate(currentHour)
    }
  };
}

/**
 * Check if hour is within time range
 */
export function isHourInRange(hour: number, range: TimeRangeHelper): boolean {
  return hour >= range.start_hour && hour <= range.end_hour;
}

/**
 * Get relative time description
 */
export function getRelativeTimeDescription(targetHour: number, currentHour: number): string {
  const diff = currentHour - targetHour;
  
  if (diff === 0) return 'now';
  if (diff === 1) return '1 hour ago';
  if (diff > 1 && diff < 24) return `${diff} hours ago`;
  
  const days = Math.floor(diff / 24);
  if (days === 1) return '1 day ago';
  if (days < 7) return `${days} days ago`;
  
  const weeks = Math.floor(days / 6); // 6-day weeks
  if (weeks === 1) return '1 week ago';
  return `${weeks} weeks ago`;
}

/**
 * Time zone information for valleys
 */
export interface ValleyTimeZone {
  valley: Valley;
  offset_hours: number;
  current_time_of_day: TimeOfDay;
  description: string;
}

export function getValleyTimeZones(baseHour: number): ValleyTimeZone[] {
  return [
    {
      valley: Valley.Day,
      offset_hours: 0,
      current_time_of_day: calculateTimeOfDay(baseHour),
      description: 'Base time zone - center of the continent'
    },
    {
      valley: Valley.Dawn,
      offset_hours: 6,
      current_time_of_day: calculateTimeOfDayForValley(Valley.Dawn, baseHour),
      description: '6 hours ahead - eastern region'
    },
    {
      valley: Valley.Dusk,
      offset_hours: -6,
      current_time_of_day: calculateTimeOfDayForValley(Valley.Dusk, baseHour),
      description: '6 hours behind - western region'
    },
    {
      valley: Valley.Night,
      offset_hours: 12,
      current_time_of_day: calculateTimeOfDayForValley(Valley.Night, baseHour),
      description: 'Opposite side - 12 hours offset'
    }
  ];
}