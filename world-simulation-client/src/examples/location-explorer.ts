/**
 * Location Explorer Example for World Simulation Client
 * Demonstrates location-based queries and exploration features
 */

import { 
  WorldSimulation,
  Valley,
  getCitiesInValley,
  getLocationStats,
  getCapitalCities,
  searchCities,
  getRandomCity,
  getRandomCityFromValley,
  getCityInfo,
  isValidCity
} from '../index.js';

async function locationExplorerExample() {
  console.log('🗺️  World Simulation Client - Location Explorer Example\n');

  const simulation = new WorldSimulation('ws://localhost:3001');

  try {
    await simulation.connect();
    console.log('✅ Connected to simulation\n');

    // Display world statistics
    console.log('🌐 World Statistics:');
    const stats = getLocationStats();
    console.log(`- Total Cities: ${stats.total_cities}`);
    console.log(`- Capital Cities: ${stats.capital_cities.length}`);
    console.log(`- Largest Valley: ${stats.largest_valley} (${stats.valley_distribution[stats.largest_valley]} cities)`);
    console.log(`- Smallest Valley: ${stats.smallest_valley} (${stats.valley_distribution[stats.smallest_valley]} cities)\n`);

    // Explore each valley
    console.log('🏔️  Valley Exploration:');
    const valleys = [Valley.Dawn, Valley.Day, Valley.Dusk, Valley.Night];
    
    for (const valley of valleys) {
      console.log(`\n${valley.toUpperCase()} VALLEY:`);
      const citiesInValley = getCitiesInValley(valley);
      console.log(`Cities: ${citiesInValley.length}`);
      
      // Get 3 example cities from this valley
      const exampleCities = citiesInValley.slice(0, 3);
      console.log(`Examples: ${exampleCities.join(', ')}`);
      
      // Get detailed info for the first city
      if (exampleCities.length > 0) {
        const cityName = exampleCities[0];
        const cityState = await simulation.client.getLocationState(cityName);
        console.log(`🏙️  ${cityName}:`);
        console.log(`  Population: ${cityState.population.toLocaleString()}`);
        console.log(`  Time of Day: ${cityState.time_of_day}`);
        console.log(`  Active Buildings: ${cityState.active_buildings}`);
        
        // Get recent activity
        const recentEvents = await simulation.client.getLocationHistory(cityName, { hours_back: 12 });
        console.log(`  Recent Events: ${recentEvents.length}`);
      }
    }

    // Explore capital cities
    console.log('\n👑 Capital Cities (Citadels):');
    const capitals = getCapitalCities();
    
    for (const capital of capitals) {
      console.log(`\n🏛️  ${capital.name} (${capital.valley} Valley):`);
      const capitalState = await simulation.client.getLocationState(capital.name);
      console.log(`  Population: ${capitalState.population.toLocaleString()}`);
      console.log(`  Buildings: ${capitalState.active_buildings}`);
      console.log(`  Current Time: ${capitalState.time_of_day}`);
      
      // Get capital-specific events
      const capitalEvents = await simulation.client.getLocationHistory(capital.name, { hours_back: 24 });
      if (capitalEvents.length > 0) {
        const latestEvent = capitalEvents[0];
        console.log(`  Latest Event: "${latestEvent.description}"`);
      }
    }

    // City search functionality
    console.log('\n🔍 City Search Examples:');
    const searchTerms = ['Citadel', 'Tsin', 'Ba'];
    
    for (const term of searchTerms) {
      const results = searchCities(term);
      console.log(`Search "${term}": ${results.length} results`);
      if (results.length > 0) {
        console.log(`  Examples: ${results.slice(0, 3).join(', ')}`);
      }
    }

    // Random exploration
    console.log('\n🎲 Random Exploration:');
    const randomCity = getRandomCity();
    console.log(`Random City: ${randomCity}`);
    
    const cityInfo = getCityInfo(randomCity);
    if (cityInfo) {
      console.log(`Valley: ${cityInfo.valley}`);
      console.log(`Is Capital: ${cityInfo.isCapital ? 'Yes' : 'No'}`);
      
      const randomCityState = await simulation.client.getLocationState(randomCity);
      console.log(`Population: ${randomCityState.population.toLocaleString()}`);
    }

    // Valley-specific random exploration
    console.log('\nRandom city from each valley:');
    for (const valley of valleys) {
      const randomCityFromValley = getRandomCityFromValley(valley);
      if (randomCityFromValley) {
        console.log(`${valley}: ${randomCityFromValley}`);
      }
    }

    // Location comparison
    console.log('\n⚖️  Location Comparison:');
    const city1 = 'Tsin';
    const city2 = 'Palwede';
    
    const [state1, state2] = await Promise.all([
      simulation.client.getLocationState(city1),
      simulation.client.getLocationState(city2)
    ]);
    
    console.log(`Comparing ${city1} vs ${city2}:`);
    console.log(`Population: ${state1.population.toLocaleString()} vs ${state2.population.toLocaleString()}`);
    console.log(`Valley: ${state1.valley} vs ${state2.valley}`);
    console.log(`Time of Day: ${state1.time_of_day} vs ${state2.time_of_day}`);
    console.log(`Buildings: ${state1.active_buildings} vs ${state2.active_buildings}`);

    // Location validation
    console.log('\n✅ Location Validation:');
    const testNames = ['Tsin', 'InvalidCity', 'Citadel of Utaia', 'NonExistent'];
    
    testNames.forEach(name => {
      const isValid = isValidCity(name);
      console.log(`"${name}": ${isValid ? '✅ Valid' : '❌ Invalid'}`);
    });

  } catch (error) {
    console.error('❌ Error:', error);
  } finally {
    simulation.disconnect();
    console.log('\n📴 Disconnected from simulation');
  }
}

// Utility function for AI agents to get location overview
export async function getLocationOverview(valleyFilter?: Valley) {
  const simulation = new WorldSimulation();
  await simulation.connect();
  
  try {
    const stats = getLocationStats();
    let cities: string[];
    
    if (valleyFilter) {
      cities = getCitiesInValley(valleyFilter);
    } else {
      cities = Object.keys(stats.valley_distribution);
    }
    
    // Get state for a sample of cities
    const sampleCities = cities.slice(0, 5);
    const cityStates = await Promise.all(
      sampleCities.map(city => simulation.client.getLocationState(city))
    );
    
    return {
      world_stats: stats,
      sample_cities: cityStates,
      total_cities_available: cities.length,
      capital_cities: getCapitalCities()
    };
  } finally {
    simulation.disconnect();
  }
}

// Utility function to find interesting locations for AI narratives
export async function findInterestingLocations(criteria: {
  minPopulation?: number;
  maxPopulation?: number;
  valley?: Valley;
  includeCapitals?: boolean;
  includeRecentActivity?: boolean;
}) {
  const simulation = new WorldSimulation();
  await simulation.connect();
  
  try {
    let candidateCities: string[];
    
    if (criteria.valley) {
      candidateCities = getCitiesInValley(criteria.valley);
    } else {
      candidateCities = Object.keys(getLocationStats().valley_distribution);
    }
    
    // Filter by capital status
    if (criteria.includeCapitals === false) {
      candidateCities = candidateCities.filter(city => !city.startsWith('Citadel of'));
    } else if (criteria.includeCapitals === true) {
      candidateCities = candidateCities.filter(city => city.startsWith('Citadel of'));
    }
    
    // Get detailed information for filtering
    const cityDetails = await Promise.all(
      candidateCities.slice(0, 20).map(async city => {
        const state = await simulation.client.getLocationState(city);
        let recentEvents = 0;
        
        if (criteria.includeRecentActivity) {
          const events = await simulation.client.getLocationHistory(city, { hours_back: 24 });
          recentEvents = events.length;
        }
        
        return { city, state, recentEvents };
      })
    );
    
    // Apply population filters
    let filtered = cityDetails;
    
    if (criteria.minPopulation) {
      filtered = filtered.filter(item => item.state.population >= criteria.minPopulation);
    }
    
    if (criteria.maxPopulation) {
      filtered = filtered.filter(item => item.state.population <= criteria.maxPopulation);
    }
    
    // Sort by recent activity if requested
    if (criteria.includeRecentActivity) {
      filtered.sort((a, b) => b.recentEvents - a.recentEvents);
    }
    
    return filtered.map(item => ({
      name: item.city,
      valley: item.state.valley,
      population: item.state.population,
      time_of_day: item.state.time_of_day,
      active_buildings: item.state.active_buildings,
      recent_events: item.recentEvents
    }));
    
  } finally {
    simulation.disconnect();
  }
}

// Run the example
if (import.meta.url === `file://${process.argv[1]}`) {
  locationExplorerExample().catch(console.error);
}

export { locationExplorerExample, getLocationOverview, findInterestingLocations };