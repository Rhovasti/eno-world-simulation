/**
 * AI-friendly narrative API for World Simulation
 * Provides structured data and context for AI storytelling systems
 */

import { WorldSimulationClient } from '../lib/query-interface';
import {
  StoryContext,
  LocationState,
  HistoricalEvent,
  TimeContext,
  Individual,
  Building,
  Valley,
  TimeOfDay,
  QueryOptions
} from '../lib/types';
import {
  formatIndividual,
  formatBuilding,
  formatCity,
  createLocationNarrative,
  createCharacterProfile,
  formatEventTimeline,
  createSummaryStats,
  formatSummaryStats
} from '../utils/data-formatter';
import { getCityValley, isValidCity } from '../utils/location-mapper';
import { formatSimulationDate, calculateTimeOfDayForValley } from '../utils/time-helpers';

/**
 * Narrative-focused API for AI agents and storytelling systems
 */
export class NarrativeAPI {
  constructor(private client: WorldSimulationClient) {}

  /**
   * Get comprehensive story context for a location
   * Perfect for AI agents generating location-based narratives
   */
  async getContextForStory(
    location: string,
    characterTypes: string[] = [],
    options: QueryOptions = {}
  ): Promise<StoryContext> {
    // Validate location
    if (!isValidCity(location)) {
      throw new Error(`Invalid city name: ${location}. Please use a valid city from the simulation.`);
    }

    // Get location state
    const locationState = await this.client.getLocationState(location);
    
    // Get characters (individuals in the area)
    const allIndividuals = await this.client.getIndividualsAtLocation(location);
    
    // Filter characters by type if specified
    let characters = allIndividuals;
    if (characterTypes.length > 0) {
      // For now, we'll use all individuals - in the future this could filter by job type, etc.
      characters = allIndividuals.slice(0, Math.min(10, allIndividuals.length));
    }
    
    // Get recent events
    const recentEvents = await this.client.getLocationHistory(location, { hours_back: 24 });
    
    // Get time context
    const timeContext = await this.client.getSimulationStatus();
    
    // Generate suggested narratives
    const suggestedNarratives = this.generateNarrativeSuggestions(
      locationState,
      characters,
      recentEvents,
      timeContext
    );

    return {
      location: locationState,
      characters,
      recent_events: recentEvents,
      time_context: timeContext,
      suggested_narratives: suggestedNarratives
    };
  }

  /**
   * Generate a complete location narrative
   * Returns a human-readable story about what's happening at a location
   */
  async generateLocationNarrative(
    location: string,
    timeRangeHours: number = 6,
    includeCharacters: boolean = true
  ): Promise<string> {
    const context = await this.getContextForStory(location);
    const recentEvents = await this.client.getLocationHistory(location, { hours_back: timeRangeHours });
    
    let narrative = createLocationNarrative(
      context.location.city,
      context.location.population,
      context.location.time_of_day,
      context.location.valley,
      recentEvents,
      context.time_context.current_hour
    );
    
    if (includeCharacters && context.characters.length > 0) {
      narrative += '\n\nNotable residents:\n';
      const characterDescriptions = context.characters
        .slice(0, 5) // Top 5 characters
        .map(char => `• ${createCharacterProfile(char)}`)
        .join('\n');
      narrative += characterDescriptions;
    }
    
    const stats = createSummaryStats(context.characters, context.location.active_buildings, recentEvents);
    narrative += `\n\n${formatSummaryStats(stats)}`;
    
    return narrative;
  }

  /**
   * Get character-focused story context
   * Perfect for narratives centered around specific individuals
   */
  async getCharacterStoryContext(
    characterId: number,
    hoursBack: number = 24
  ): Promise<{
    character: Individual;
    story_events: HistoricalEvent[];
    location_context: LocationState | null;
    narrative_summary: string;
  }> {
    // Get character story events
    const storyEvents = await this.client.getIndividualStory(characterId, hoursBack);
    
    // Get character details
    const allIndividuals = await this.client.spacetimeClient.getIndividuals();
    const character = allIndividuals.find(ind => ind.id === characterId);
    
    if (!character) {
      throw new Error(`Character with ID ${characterId} not found`);
    }
    
    // Get location context if character has a home
    let locationContext: LocationState | null = null;
    if (character.home_id) {
      const buildings = await this.client.spacetimeClient.getBuildings();
      const homeBuilding = buildings.find(b => b.id === character.home_id);
      
      if (homeBuilding) {
        const cities = await this.client.spacetimeClient.getCities();
        const homeCity = cities.find(c => c.id === homeBuilding.city_id);
        
        if (homeCity) {
          locationContext = await this.client.getLocationState(homeCity.name);
        }
      }
    }
    
    // Generate narrative summary
    const narrativeSummary = this.generateCharacterNarrative(character, storyEvents, locationContext);
    
    return {
      character,
      story_events: storyEvents,
      location_context: locationContext,
      narrative_summary: narrativeSummary
    };
  }

  /**
   * Get building-focused story context
   * Perfect for narratives about specific buildings or locations
   */
  async getBuildingStoryContext(
    buildingId: number,
    hoursBack: number = 24
  ): Promise<{
    building: Building;
    story_events: HistoricalEvent[];
    occupants: Individual[];
    narrative_summary: string;
  }> {
    // Get building story events
    const storyEvents = await this.client.getBuildingStory(buildingId, hoursBack);
    
    // Get building details
    const buildings = await this.client.spacetimeClient.getBuildings();
    const building = buildings.find(b => b.id === buildingId);
    
    if (!building) {
      throw new Error(`Building with ID ${buildingId} not found`);
    }
    
    // Get occupants
    const allIndividuals = await this.client.spacetimeClient.getIndividuals();
    const occupants = allIndividuals.filter(ind => 
      ind.home_id === buildingId || ind.workplace_id === buildingId
    );
    
    // Generate narrative summary
    const narrativeSummary = this.generateBuildingNarrative(building, storyEvents, occupants);
    
    return {
      building,
      story_events: storyEvents,
      occupants,
      narrative_summary: narrativeSummary
    };
  }

  /**
   * Get time-specific narrative
   * Perfect for "what was happening at this time" queries
   */
  async getTimeSpecificNarrative(
    location: string,
    targetHour: number,
    contextHours: number = 3
  ): Promise<{
    target_time: string;
    location: string;
    events_around_time: HistoricalEvent[];
    narrative_description: string;
  }> {
    const currentHour = await this.client.getCurrentHour();
    const hoursBack = currentHour - targetHour;
    
    if (hoursBack < 0) {
      throw new Error('Cannot query future events');
    }
    
    // Get events around the target time
    const allEvents = await this.client.getLocationHistory(location, { 
      hours_back: hoursBack + contextHours 
    });
    
    const eventsAroundTime = allEvents.filter(event => 
      Math.abs(event.hour - targetHour) <= contextHours
    );
    
    // Generate narrative description
    const targetTimeFormatted = formatSimulationDate(targetHour);
    const valley = getCityValley(location);
    const timeOfDay = valley ? calculateTimeOfDayForValley(valley, targetHour % 24) : TimeOfDay.Day;
    
    let narrative = `At ${targetTimeFormatted} in ${location}, it was ${timeOfDay.toLowerCase()}. `;
    
    if (eventsAroundTime.length > 0) {
      narrative += 'During this time:\n';
      narrative += formatEventTimeline(eventsAroundTime, currentHour, 5);
    } else {
      narrative += 'The city was quiet during this period, with residents following their normal routines.';
    }
    
    return {
      target_time: targetTimeFormatted,
      location,
      events_around_time: eventsAroundTime,
      narrative_description: narrative
    };
  }

  /**
   * Get comparative narrative between locations
   * Perfect for "compare these two places" queries
   */
  async getComparativeNarrative(
    location1: string,
    location2: string,
    hoursBack: number = 6
  ): Promise<{
    location1_context: StoryContext;
    location2_context: StoryContext;
    comparison_narrative: string;
  }> {
    // Get context for both locations
    const [context1, context2] = await Promise.all([
      this.getContextForStory(location1),
      this.getContextForStory(location2)
    ]);
    
    // Generate comparative narrative
    const comparisonNarrative = this.generateComparativeNarrative(context1, context2);
    
    return {
      location1_context: context1,
      location2_context: context2,
      comparison_narrative: comparisonNarrative
    };
  }

  /**
   * Get narrative suggestions for AI agents
   * Returns story ideas based on current simulation state
   */
  async getNarrativeSuggestions(
    theme?: 'adventure' | 'slice-of-life' | 'mystery' | 'drama' | 'comedy',
    location?: string
  ): Promise<{
    theme: string;
    suggestions: Array<{
      title: string;
      premise: string;
      characters: string[];
      setting: string;
      potential_conflicts: string[];
    }>;
  }> {
    let contexts: StoryContext[] = [];
    
    if (location) {
      const context = await this.getContextForStory(location);
      contexts = [context];
    } else {
      // Get a few random locations for variety
      const allIndividuals = await this.client.spacetimeClient.getIndividuals();
      const allBuildings = await this.client.spacetimeClient.getBuildings();
      const allCities = await this.client.spacetimeClient.getCities();
      
      // Pick a few interesting cities
      const selectedCities = allCities.slice(0, 3);
      contexts = await Promise.all(
        selectedCities.map(city => this.getContextForStory(city.name))
      );
    }
    
    const suggestions = this.generateStoryIdeas(contexts, theme || 'slice-of-life');
    
    return {
      theme: theme || 'slice-of-life',
      suggestions
    };
  }

  // ===== PRIVATE HELPER METHODS =====

  /**
   * Generate narrative suggestions based on context
   */
  private generateNarrativeSuggestions(
    location: LocationState,
    characters: Individual[],
    events: HistoricalEvent[],
    timeContext: TimeContext
  ): string[] {
    const suggestions: string[] = [];
    
    // Time-based suggestions
    const timeOfDay = location.time_of_day;
    switch (timeOfDay) {
      case TimeOfDay.Dawn:
        suggestions.push('A new day begins as the first light touches the valley');
        break;
      case TimeOfDay.Day:
        suggestions.push('The busy activity of midday life in the city');
        break;
      case TimeOfDay.Dusk:
        suggestions.push('Evening routines as people wind down from the day');
        break;
      case TimeOfDay.Night:
        suggestions.push('Nighttime mysteries and quiet contemplation');
        break;
    }
    
    // Population-based suggestions
    if (location.population > 1000) {
      suggestions.push('The bustling energy of a major city center');
    } else {
      suggestions.push('Intimate small-town connections and relationships');
    }
    
    // Event-based suggestions
    if (events.length > 5) {
      suggestions.push('Recent dramatic events have stirred up the community');
    } else {
      suggestions.push('A peaceful period perfect for character development');
    }
    
    // Character-based suggestions
    if (characters.length > 0) {
      const avgEnergy = characters.reduce((sum, char) => sum + char.energy, 0) / characters.length;
      if (avgEnergy > 0.7) {
        suggestions.push('High energy levels suggest active adventures');
      } else if (avgEnergy < 0.3) {
        suggestions.push('Low energy suggests introspective, quiet stories');
      }
    }
    
    // Valley-specific suggestions
    switch (location.valley) {
      case Valley.Dawn:
        suggestions.push('Eastern lands where new beginnings take root');
        break;
      case Valley.Day:
        suggestions.push('Central heartlands of activity and commerce');
        break;
      case Valley.Dusk:
        suggestions.push('Western territories of reflection and endings');
        break;
      case Valley.Night:
        suggestions.push('Mysterious far lands full of secrets');
        break;
    }
    
    return suggestions.slice(0, 5); // Return top 5 suggestions
  }

  /**
   * Generate character-focused narrative
   */
  private generateCharacterNarrative(
    character: Individual,
    events: HistoricalEvent[],
    locationContext: LocationState | null
  ): string {
    let narrative = `${createCharacterProfile(character)}. `;
    
    if (locationContext) {
      narrative += `They live in ${locationContext.city} in the ${locationContext.valley} Valley. `;
    }
    
    if (events.length > 0) {
      narrative += `Recent activities include: `;
      const eventDescriptions = events
        .slice(0, 3)
        .map(event => event.description.toLowerCase())
        .join(', ');
      narrative += eventDescriptions + '.';
    } else {
      narrative += 'They have been following their regular routine lately.';
    }
    
    return narrative;
  }

  /**
   * Generate building-focused narrative
   */
  private generateBuildingNarrative(
    building: Building,
    events: HistoricalEvent[],
    occupants: Individual[]
  ): string {
    let narrative = `${formatBuilding(building, true)}. `;
    
    if (occupants.length > 0) {
      narrative += `It houses ${occupants.length} residents. `;
    }
    
    if (events.length > 0) {
      narrative += `Recent building activities include: `;
      const eventDescriptions = events
        .slice(0, 3)
        .map(event => event.description.toLowerCase())
        .join(', ');
      narrative += eventDescriptions + '.';
    } else {
      narrative += 'The building has been operating normally.';
    }
    
    return narrative;
  }

  /**
   * Generate comparative narrative between two locations
   */
  private generateComparativeNarrative(context1: StoryContext, context2: StoryContext): string {
    const loc1 = context1.location;
    const loc2 = context2.location;
    
    let narrative = `Comparing ${loc1.city} in the ${loc1.valley} Valley with ${loc2.city} in the ${loc2.valley} Valley:\n\n`;
    
    // Population comparison
    if (loc1.population > loc2.population * 1.5) {
      narrative += `${loc1.city} is significantly larger with ${loc1.population.toLocaleString()} residents compared to ${loc2.city}'s ${loc2.population.toLocaleString()}. `;
    } else if (loc2.population > loc1.population * 1.5) {
      narrative += `${loc2.city} is significantly larger with ${loc2.population.toLocaleString()} residents compared to ${loc1.city}'s ${loc1.population.toLocaleString()}. `;
    } else {
      narrative += `Both cities are similar in size, with ${loc1.city} having ${loc1.population.toLocaleString()} residents and ${loc2.city} having ${loc2.population.toLocaleString()}. `;
    }
    
    // Time of day comparison
    if (loc1.time_of_day !== loc2.time_of_day) {
      narrative += `While it's ${loc1.time_of_day} in ${loc1.city}, it's ${loc2.time_of_day} in ${loc2.city} due to their different valleys. `;
    } else {
      narrative += `Both cities are experiencing ${loc1.time_of_day} simultaneously. `;
    }
    
    // Activity comparison
    const events1 = context1.recent_events.length;
    const events2 = context2.recent_events.length;
    
    if (events1 > events2 * 2) {
      narrative += `${loc1.city} has been much more active recently with ${events1} notable events compared to ${loc2.city}'s ${events2}. `;
    } else if (events2 > events1 * 2) {
      narrative += `${loc2.city} has been much more active recently with ${events2} notable events compared to ${loc1.city}'s ${events1}. `;
    } else {
      narrative += `Both cities show similar activity levels with ${events1} and ${events2} recent events respectively. `;
    }
    
    return narrative;
  }

  /**
   * Generate story ideas based on current simulation state
   */
  private generateStoryIdeas(
    contexts: StoryContext[],
    theme: string
  ): Array<{
    title: string;
    premise: string;
    characters: string[];
    setting: string;
    potential_conflicts: string[];
  }> {
    const ideas: Array<{
      title: string;
      premise: string;
      characters: string[];
      setting: string;
      potential_conflicts: string[];
    }> = [];
    
    contexts.forEach(context => {
      const location = context.location;
      const characters = context.characters.slice(0, 3).map(c => c.name);
      
      switch (theme) {
        case 'adventure':
          ideas.push({
            title: `The ${location.valley} Valley Expedition`,
            premise: `A group discovers ancient secrets in ${location.city}`,
            characters,
            setting: `${location.city}, ${location.valley} Valley`,
            potential_conflicts: ['Ancient guardians', 'Rival explorers', 'Natural disasters']
          });
          break;
          
        case 'mystery':
          ideas.push({
            title: `Mystery in ${location.city}`,
            premise: `Strange events plague the ${location.valley} Valley settlement`,
            characters,
            setting: `${location.city}, ${location.valley} Valley`,
            potential_conflicts: ['Hidden motives', 'False accusations', 'Missing evidence']
          });
          break;
          
        case 'drama':
          ideas.push({
            title: `Lives Intertwined`,
            premise: `Personal relationships tested in ${location.city}`,
            characters,
            setting: `${location.city}, ${location.valley} Valley`,
            potential_conflicts: ['Family disputes', 'Economic pressures', 'Social changes']
          });
          break;
          
        default: // slice-of-life
          ideas.push({
            title: `A Day in ${location.city}`,
            premise: `Following the daily lives of residents in the ${location.valley} Valley`,
            characters,
            setting: `${location.city}, ${location.valley} Valley`,
            potential_conflicts: ['Work challenges', 'Community issues', 'Personal growth']
          });
          break;
      }
    });
    
    return ideas.slice(0, 5); // Return top 5 ideas
  }
}