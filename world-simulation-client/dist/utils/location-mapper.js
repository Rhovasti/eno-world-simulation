/**
 * Location mapping utilities for the World Simulation
 * Maps cities to valleys and provides location-based queries
 */
import { Valley } from '../lib/types';
// City to Valley mapping based on the status.md file
export const CITY_VALLEY_MAP = {
    // Valley of the Dawn
    'Tsin': Valley.Dawn,
    'Gongshan': Valley.Dawn,
    'Pranos': Valley.Dawn,
    'Orve': Valley.Dawn,
    'Alebuo': Valley.Dawn,
    'Jiafeng': Valley.Dawn,
    'Tsanghom': Valley.Dawn,
    'Aurola': Valley.Dawn,
    'Dautong': Valley.Dawn,
    'Bernala': Valley.Dawn,
    'Alos': Valley.Dawn,
    'Pigna': Valley.Dawn,
    'Pornoli': Valley.Dawn,
    'Mintenomarci': Valley.Dawn,
    'Murmium': Valley.Dawn,
    'Chingsan': Valley.Dawn,
    'Jinzhou': Valley.Dawn,
    'Pogliaferte': Valley.Dawn,
    'Eliusila': Valley.Dawn,
    'Jining': Valley.Dawn,
    'Laizhou': Valley.Dawn,
    'Fonte': Valley.Dawn,
    'Waishan': Valley.Dawn,
    'Pinkang': Valley.Dawn,
    'Patrellasa': Valley.Dawn,
    'Huang': Valley.Dawn,
    'Monte': Valley.Dawn,
    'Modurelius': Valley.Dawn,
    'Sera': Valley.Dawn,
    'Kukar': Valley.Dawn,
    'Sa': Valley.Dawn,
    'Citadel of Utaia': Valley.Dawn,
    'Shaxing': Valley.Dawn,
    // Valley of the Day
    'Beitsa': Valley.Day,
    'Phoelit': Valley.Day,
    'Zadardelen': Valley.Day,
    'Tonkhanhad': Valley.Day,
    'Avesia': Valley.Day,
    'Makad': Valley.Day,
    'Fanha': Valley.Day,
    'Mical': Valley.Day,
    'Mevus': Valley.Day,
    'Likzib': Valley.Day,
    'Kushk': Valley.Day,
    'Luquti': Valley.Day,
    'Regoes': Valley.Day,
    'Gyt No': Valley.Day,
    'Hasamut': Valley.Day,
    'Shfanaan': Valley.Day,
    'Vialiranave': Valley.Day,
    'Mernah': Valley.Day,
    'Reriro': Valley.Day,
    'Sobriro': Valley.Day,
    'Bulekh Orov': Valley.Day,
    'Malveiba': Valley.Day,
    'Veren': Valley.Day,
    'Darvangor': Valley.Day,
    'Cajunara': Valley.Day,
    'Euata': Valley.Day,
    'Urir': Valley.Day,
    'Roparia': Valley.Day,
    'Auera': Valley.Day,
    'Ieye': Valley.Day,
    'Oatue': Valley.Day,
    'Uiaria': Valley.Day,
    'Aneroa': Valley.Day,
    'Atar': Valley.Day,
    'Iinasia': Valley.Day,
    'Ouiar': Valley.Day,
    // Valley of the Dusk
    'Jouy': Valley.Dusk,
    'Motu': Valley.Dusk,
    'Guild': Valley.Dusk,
    'Jeong': Valley.Dusk,
    'Riroku': Valley.Dusk,
    'Bafke': Valley.Dusk,
    'Mahyapak': Valley.Dusk,
    'Mohi': Valley.Dusk,
    'Enyhazto': Valley.Dusk,
    'Briarviles': Valley.Dusk,
    'Jargeroy': Valley.Dusk,
    'Engar': Valley.Dusk,
    'Bifjorda': Valley.Dusk,
    'Borgloy': Valley.Dusk,
    'Zidesun': Valley.Dusk,
    'Hajsala': Valley.Dusk,
    'Kushimaki': Valley.Dusk,
    'Tisvarmend': Valley.Dusk,
    'Nyirfalmasle': Valley.Dusk,
    'Aiya': Valley.Dusk,
    'Bazujduzeszeg': Valley.Dusk,
    'Szargony': Valley.Dusk,
    'Korestad': Valley.Dusk,
    'Sunker': Valley.Dusk,
    'Kikoupaupo': Valley.Dusk,
    'Bancik': Valley.Dusk,
    'Ottengenburg': Valley.Dusk,
    'Balashee': Valley.Dusk,
    'Moki': Valley.Dusk,
    'Tasa': Valley.Dusk,
    'Tetonykut': Valley.Dusk,
    'Uyaria': Valley.Dusk,
    'Citadel of Almo': Valley.Dusk,
    'Ian': Valley.Dusk,
    'Vul': Valley.Dusk,
    'Teveh': Valley.Dusk,
    'Vea': Valley.Dusk,
    'Tlida': Valley.Dusk,
    'Harruresh': Valley.Dusk,
    'Ubahmia': Valley.Dusk,
    'Fibeon': Valley.Dusk,
    'Shma': Valley.Dusk,
    // Valley of the Night
    'Palwede': Valley.Night,
    'Gyba': Valley.Night,
    'Bungomo': Valley.Night,
    'Ithemate': Valley.Night,
    'Kudina': Valley.Night,
    'Asuyan': Valley.Night,
    'Dûr-Tu': Valley.Night,
    'Binh Ninh': Valley.Night,
    'Rambedamkur': Valley.Night,
    'Doros': Valley.Night,
    'Ergoigoibar': Valley.Night,
    'Bellikavima': Valley.Night,
    'Valvi': Valley.Night,
    'Kyenga': Valley.Night,
    'Mbale': Valley.Night,
    'Kudchna': Valley.Night,
    'Citadel of the Pass': Valley.Night,
    'Kuoruvaa': Valley.Night,
    'Castri': Valley.Night,
    'Kolli': Valley.Night,
    'Hamapurara': Valley.Night,
    'Chitna': Valley.Night,
    'Aira': Valley.Night,
    'Otety': Valley.Night,
    'Ithiu': Valley.Night,
    'Atsa': Valley.Night,
    'Eneya': Valley.Night,
    'Kure': Valley.Night,
    'Itia': Valley.Night
};
/**
 * Get the valley for a given city name
 */
export function getCityValley(cityName) {
    return CITY_VALLEY_MAP[cityName] || null;
}
/**
 * Get all cities in a specific valley
 */
export function getCitiesInValley(valley) {
    return Object.entries(CITY_VALLEY_MAP)
        .filter(([_, v]) => v === valley)
        .map(([city, _]) => city);
}
/**
 * Get all available valleys
 */
export function getAllValleys() {
    return [Valley.Dawn, Valley.Day, Valley.Dusk, Valley.Night];
}
/**
 * Get total number of cities
 */
export function getTotalCityCount() {
    return Object.keys(CITY_VALLEY_MAP).length;
}
/**
 * Get city count by valley
 */
export function getCityCountByValley() {
    const counts = {
        [Valley.Dawn]: 0,
        [Valley.Day]: 0,
        [Valley.Dusk]: 0,
        [Valley.Night]: 0
    };
    Object.values(CITY_VALLEY_MAP).forEach(valley => {
        counts[valley]++;
    });
    return counts;
}
/**
 * Search for cities by partial name match
 */
export function searchCities(query) {
    const lowerQuery = query.toLowerCase();
    return Object.keys(CITY_VALLEY_MAP).filter(city => city.toLowerCase().includes(lowerQuery));
}
/**
 * Get neighboring valleys (for time zone calculations)
 */
export function getNeighboringValleys(valley) {
    const valleyOrder = [Valley.Dawn, Valley.Day, Valley.Dusk, Valley.Night];
    const currentIndex = valleyOrder.indexOf(valley);
    const previousIndex = (currentIndex - 1 + valleyOrder.length) % valleyOrder.length;
    const nextIndex = (currentIndex + 1) % valleyOrder.length;
    return [valleyOrder[previousIndex], valleyOrder[nextIndex]];
}
/**
 * Get the opposite valley (for day/night cycle)
 */
export function getOppositeValley(valley) {
    switch (valley) {
        case Valley.Dawn: return Valley.Dusk;
        case Valley.Day: return Valley.Night;
        case Valley.Dusk: return Valley.Dawn;
        case Valley.Night: return Valley.Day;
    }
}
/**
 * Validate if a city name exists
 */
export function isValidCity(cityName) {
    return cityName in CITY_VALLEY_MAP;
}
/**
 * Get random city from a valley
 */
export function getRandomCityFromValley(valley) {
    const cities = getCitiesInValley(valley);
    if (cities.length === 0)
        return null;
    const randomIndex = Math.floor(Math.random() * cities.length);
    return cities[randomIndex];
}
/**
 * Get random city from any valley
 */
export function getRandomCity() {
    const allCities = Object.keys(CITY_VALLEY_MAP);
    const randomIndex = Math.floor(Math.random() * allCities.length);
    return allCities[randomIndex];
}
export function getCityInfo(cityName) {
    const valley = getCityValley(cityName);
    if (!valley)
        return null;
    return {
        name: cityName,
        valley,
        isCapital: cityName.startsWith('Citadel of')
    };
}
/**
 * Get all capital cities (Citadels)
 */
export function getCapitalCities() {
    return Object.keys(CITY_VALLEY_MAP)
        .filter(city => city.startsWith('Citadel of'))
        .map(city => getCityInfo(city))
        .filter(info => info !== null);
}
export function getLocationStats() {
    const valleyDistribution = getCityCountByValley();
    const capitalCities = getCapitalCities().map(info => info.name);
    const valleys = Object.entries(valleyDistribution);
    const largestValley = valleys.reduce((a, b) => a[1] > b[1] ? a : b)[0];
    const smallestValley = valleys.reduce((a, b) => a[1] < b[1] ? a : b)[0];
    return {
        total_cities: getTotalCityCount(),
        valley_distribution: valleyDistribution,
        capital_cities: capitalCities,
        largest_valley: largestValley,
        smallest_valley: smallestValley
    };
}
//# sourceMappingURL=location-mapper.js.map