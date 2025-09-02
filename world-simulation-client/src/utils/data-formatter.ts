/**
 * Data formatting utilities for World Simulation
 * Converts raw simulation data into human-readable and AI-friendly formats
 */

import {
  Individual,
  Building,
  City,
  HistoricalEvent,
  TimeOfDay,
  Valley,
  FundamentalNeed,
  IndividualAction,
  BuildingType,
  JobType
} from '../lib/types';
import { formatSimulationDate, getRelativeTimeDescription } from './time-helpers';
import { getCityValley } from './location-mapper';

/**
 * Format individual information for narrative use
 */
export function formatIndividual(individual: Individual, includeStats: boolean = true): string {
  const name = individual.name;
  const age = individual.age;
  const activity = formatIndividualActivity(individual.current_activity);
  
  let description = `${name} (age ${age}) is currently ${activity}`;
  
  if (includeStats) {
    const needsDesc = formatNeeds({
      environment: individual.environment_need,
      consumption: individual.consumption_need,
      connection: individual.connection_need,
      rest: individual.rest_need,
      waste: individual.waste_need
    });
    
    const wellness = formatWellness(individual.energy, individual.happiness, individual.health);
    description += `. ${needsDesc}. ${wellness}`;
  }
  
  return description;
}

/**
 * Format building information
 */
export function formatBuilding(building: Building, includeDetails: boolean = true): string {
  const name = building.name;
  const type = formatBuildingType(building.building_type);
  const occupancy = `${building.occupancy}/${building.max_occupancy}`;
  
  let description = `${name} is a ${type} with ${occupancy} occupancy`;
  
  if (includeDetails) {
    const condition = formatCondition(building.condition);
    const efficiency = formatEfficiency(building.efficiency);
    description += `. Building condition: ${condition}, efficiency: ${efficiency}`;
  }
  
  return description;
}

/**
 * Format city information
 */
export function formatCity(city: City, includeMetrics: boolean = true): string {
  const name = city.name;
  const valley = city.valley;
  const population = city.population.toLocaleString();
  const buildings = city.total_buildings;
  
  let description = `${name} in the ${valley} Valley has ${population} residents and ${buildings} buildings`;
  
  if (includeMetrics) {
    const metrics = formatCityMetrics({
      stability: city.stability,
      culture: city.culture,
      prosperity: city.prosperity,
      safety: city.safety,
      sustainability: city.sustainability
    });
    description += `. ${metrics}`;
  }
  
  return description;
}

/**
 * Format historical event for narrative use
 */
export function formatHistoricalEvent(
  event: HistoricalEvent, 
  currentHour: number,
  includeContext: boolean = true
): string {
  const timeDesc = getRelativeTimeDescription(event.hour, currentHour);
  const participants = event.participants.join(' and ');
  
  let description = `${timeDesc.charAt(0).toUpperCase() + timeDesc.slice(1)}: ${event.description}`;
  
  if (includeContext && event.impact.magnitude > 0.1) {
    description += ` (${event.impact.description})`;
  }
  
  if (event.location && event.location !== 'Unknown') {
    description += ` at ${event.location}`;
  }
  
  return description;
}

/**
 * Format individual activity
 */
export function formatIndividualActivity(activity: IndividualAction): string {
  switch (activity) {
    case IndividualAction.Working:
      return 'working';
    case IndividualAction.Socializing:
      return 'socializing with others';
    case IndividualAction.Resting:
      return 'resting';
    case IndividualAction.Traveling:
      return 'traveling';
    case IndividualAction.Consuming:
      return 'consuming resources';
    case IndividualAction.Managing:
      return 'managing tasks';
    default:
      return 'engaged in daily activities';
  }
}

/**
 * Format building type
 */
export function formatBuildingType(buildingType: BuildingType): string {
  if ('Home' in buildingType) {
    const capacity = buildingType.Home.capacity;
    const rent = buildingType.Home.rent;
    return `residential home (capacity: ${capacity}, rent: ${rent})`;
  }
  
  if ('Workplace' in buildingType) {
    const jobType = formatJobType(buildingType.Workplace.job_type);
    const positions = buildingType.Workplace.positions;
    return `${jobType} workplace (${positions} positions)`;
  }
  
  if ('Restaurant' in buildingType) return 'restaurant';
  if ('Park' in buildingType) return 'park';
  if ('Hospital' in buildingType) return 'hospital';
  if ('PoliceStation' in buildingType) return 'police station';
  if ('School' in buildingType) return 'school';
  if ('ResearchLab' in buildingType) return 'research laboratory';
  if ('CultureCenter' in buildingType) return 'culture center';
  if ('CityHall' in buildingType) return 'city hall';
  
  return 'building';
}

/**
 * Format job type
 */
export function formatJobType(jobType: JobType): string {
  switch (jobType) {
    case JobType.Factory: return 'factory';
    case JobType.Office: return 'office';
    case JobType.Retail: return 'retail';
    case JobType.Healthcare: return 'healthcare';
    case JobType.Education: return 'education';
    case JobType.Research: return 'research';
    case JobType.Culture: return 'cultural';
    case JobType.Utilities: return 'utilities';
    case JobType.Government: return 'government';
    default: return 'general';
  }
}

/**
 * Format needs as human-readable text
 */
export function formatNeeds(needs: Record<string, number>): string {
  const needDescriptions: string[] = [];
  
  Object.entries(needs).forEach(([need, value]) => {
    const level = getNeedLevel(value);
    const needName = need.toLowerCase();
    needDescriptions.push(`${needName} needs are ${level}`);
  });
  
  return needDescriptions.join(', ');
}

/**
 * Get need level description
 */
export function getNeedLevel(value: number): string {
  if (value >= 0.8) return 'very high';
  if (value >= 0.6) return 'high';
  if (value >= 0.4) return 'moderate';
  if (value >= 0.2) return 'low';
  return 'very low';
}

/**
 * Format wellness stats
 */
export function formatWellness(energy: number, happiness: number, health: number): string {
  const energyDesc = getNeedLevel(energy);
  const happinessDesc = getNeedLevel(happiness);
  const healthDesc = getNeedLevel(health);
  
  return `Energy is ${energyDesc}, happiness is ${happinessDesc}, health is ${healthDesc}`;
}

/**
 * Format condition level
 */
export function formatCondition(condition: number): string {
  if (condition >= 0.9) return 'excellent';
  if (condition >= 0.7) return 'good';
  if (condition >= 0.5) return 'fair';
  if (condition >= 0.3) return 'poor';
  return 'very poor';
}

/**
 * Format efficiency level
 */
export function formatEfficiency(efficiency: number): string {
  if (efficiency >= 0.9) return 'very efficient';
  if (efficiency >= 0.7) return 'efficient';
  if (efficiency >= 0.5) return 'moderate';
  if (efficiency >= 0.3) return 'inefficient';
  return 'very inefficient';
}

/**
 * Format city metrics
 */
export function formatCityMetrics(metrics: {
  stability: number;
  culture: number;
  prosperity: number;
  safety: number;
  sustainability: number;
}): string {
  const descriptions: string[] = [];
  
  if (metrics.stability >= 0.7) {
    descriptions.push('politically stable');
  } else if (metrics.stability < 0.3) {
    descriptions.push('experiencing instability');
  }
  
  if (metrics.prosperity >= 0.7) {
    descriptions.push('economically prosperous');
  } else if (metrics.prosperity < 0.3) {
    descriptions.push('economically struggling');
  }
  
  if (metrics.culture >= 0.7) {
    descriptions.push('culturally vibrant');
  }
  
  if (metrics.safety >= 0.7) {
    descriptions.push('very safe');
  } else if (metrics.safety < 0.3) {
    descriptions.push('unsafe');
  }
  
  if (metrics.sustainability >= 0.7) {
    descriptions.push('environmentally sustainable');
  } else if (metrics.sustainability < 0.3) {
    descriptions.push('environmentally stressed');
  }
  
  return descriptions.length > 0 
    ? `The city is ${descriptions.join(', ')}`
    : 'The city shows moderate metrics across all areas';
}

/**
 * Format time of day with description
 */
export function formatTimeOfDay(timeOfDay: TimeOfDay, valley: Valley): string {
  const valleyName = valley.toLowerCase();
  
  switch (timeOfDay) {
    case TimeOfDay.Dawn:
      return `dawn in the ${valleyName} valley (early morning light)`;
    case TimeOfDay.Day:
      return `daytime in the ${valleyName} valley (full sunlight)`;
    case TimeOfDay.Dusk:
      return `dusk in the ${valleyName} valley (evening twilight)`;
    case TimeOfDay.Night:
      return `nighttime in the ${valleyName} valley (darkness)`;
  }
}

/**
 * Format valley description
 */
export function formatValleyDescription(valley: Valley): string {
  switch (valley) {
    case Valley.Dawn:
      return 'Valley of the Dawn - the eastern lands where each day begins';
    case Valley.Day:
      return 'Valley of the Day - the central heartlands in perpetual daylight';
    case Valley.Dusk:
      return 'Valley of the Dusk - the western territories of eternal twilight';
    case Valley.Night:
      return 'Valley of the Night - the far lands shrouded in endless darkness';
  }
}

/**
 * Create narrative summary for location
 */
export function createLocationNarrative(
  cityName: string,
  population: number,
  timeOfDay: TimeOfDay,
  valley: Valley,
  recentEvents: HistoricalEvent[],
  currentHour: number
): string {
  const valleyDesc = formatValleyDescription(valley);
  const timeDesc = formatTimeOfDay(timeOfDay, valley);
  const popDesc = population.toLocaleString();
  
  let narrative = `${cityName} lies in the ${valleyDesc}. `;
  narrative += `Currently, it is ${timeDesc}, and the city's ${popDesc} residents are going about their daily lives.`;
  
  if (recentEvents.length > 0) {
    narrative += '\n\nRecent happenings in the city:\n';
    const recentEventDescriptions = recentEvents
      .slice(0, 5) // Top 5 events
      .map(event => `• ${formatHistoricalEvent(event, currentHour, false)}`)
      .join('\n');
    narrative += recentEventDescriptions;
  } else {
    narrative += ' The city has been quiet recently, with residents following their usual routines.';
  }
  
  return narrative;
}

/**
 * Create character profile for narrative use
 */
export function createCharacterProfile(individual: Individual, context?: string): string {
  const profile = formatIndividual(individual, true);
  
  if (context) {
    return `${profile} Context: ${context}`;
  }
  
  return profile;
}

/**
 * Format event timeline
 */
export function formatEventTimeline(
  events: HistoricalEvent[],
  currentHour: number,
  maxEvents: number = 10
): string {
  if (events.length === 0) {
    return 'No recent events to display.';
  }
  
  const sortedEvents = events
    .sort((a, b) => b.hour - a.hour)
    .slice(0, maxEvents);
  
  const timeline = sortedEvents
    .map(event => {
      const timeDesc = getRelativeTimeDescription(event.hour, currentHour);
      return `${timeDesc}: ${event.description}`;
    })
    .join('\n');
  
  return `Recent Timeline:\n${timeline}`;
}

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

export function createSummaryStats(
  individuals: Individual[],
  buildings: Building[],
  events: HistoricalEvent[]
): SummaryStats {
  const population = individuals.length;
  const activeBuildings = buildings.filter(b => b.occupancy > 0).length;
  const recentEvents = events.length;
  
  // Calculate average wellness
  const totalWellness = individuals.reduce((sum, ind) => 
    sum + (ind.energy + ind.happiness + ind.health) / 3, 0
  );
  const averageWellness = population > 0 ? totalWellness / population : 0;
  
  // Find dominant activity
  const activityCounts: Record<string, number> = {};
  individuals.forEach(ind => {
    const activity = formatIndividualActivity(ind.current_activity);
    activityCounts[activity] = (activityCounts[activity] || 0) + 1;
  });
  
  const dominantActivity = Object.entries(activityCounts)
    .sort(([,a], [,b]) => b - a)[0]?.[0] || 'various activities';
  
  return {
    population,
    active_buildings: activeBuildings,
    recent_events: recentEvents,
    average_wellness: averageWellness,
    dominant_activity: dominantActivity
  };
}

/**
 * Format summary stats as narrative
 */
export function formatSummaryStats(stats: SummaryStats): string {
  const wellnessDesc = getNeedLevel(stats.average_wellness);
  
  return `The area has ${stats.population.toLocaleString()} residents across ${stats.active_buildings} active buildings. ` +
    `Most people are currently ${stats.dominant_activity}. ` +
    `Overall wellness levels are ${wellnessDesc}. ` +
    `There have been ${stats.recent_events} notable events recently.`;
}