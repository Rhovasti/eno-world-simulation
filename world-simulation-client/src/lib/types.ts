/**
 * TypeScript interfaces for World Simulation data structures
 * Generated from SpacetimeDB table schemas
 */

// ===== CORE SIMULATION TYPES =====

export interface SimulationTime {
  id: number;
  current_hour: number;
  day_of_week: number; // 0-6
  hour_of_day: number; // 0-23
  total_days: number;
  is_running: boolean;
  auto_tick_enabled: boolean;
  tick_interval_ms: number;
}

export interface AutotickerConfig {
  id: number;
  last_tick_time: number; // timestamp in ms
  next_tick_time: number; // timestamp in ms
}

// ===== ENTITY TYPES =====

export interface Individual {
  id: number;
  name: string;
  age: number;
  home_id?: number;
  workplace_id?: number;
  location_x: number;
  location_y: number;
  money: number;
  
  // Needs (0.0 - 1.0)
  environment_need: number;
  consumption_need: number;
  connection_need: number;
  rest_need: number;
  waste_need: number;
  
  // Status
  energy: number;
  happiness: number;
  health: number;
  current_activity: IndividualAction;
}

export interface Building {
  id: number;
  name: string;
  city_id: number;
  building_type: BuildingType;
  x: number;
  y: number;
  occupancy: number;
  max_occupancy: number;
  
  // Needs (0.0 - 1.0)
  environment_need: number;
  consumption_need: number;
  connection_need: number;
  rest_need: number;
  waste_need: number;
  
  // Status
  condition: number;
  efficiency: number;
  resources: number;
}

export interface City {
  id: number;
  name: string;
  valley: Valley;
  population: number;
  total_buildings: number;
  
  // Needs (0.0 - 1.0)
  environment_need: number;
  consumption_need: number;
  connection_need: number;
  rest_need: number;
  waste_need: number;
  
  // Metrics
  stability: number;
  culture: number;
  prosperity: number;
  safety: number;
  sustainability: number;
}

// ===== EVENT TYPES =====

export interface MovementEvent {
  id: number;
  individual_id: number;
  from_location_id: number;
  to_location_id: number;
  hour: number;
  reason: FundamentalNeed;
  travel_time: number;
}

export interface WorkEvent {
  id: number;
  individual_id: number;
  building_id: number;
  hour: number;
  hours_worked: number;
  wage_earned: number;
  productivity: number;
  resources_consumed: number;
  resources_produced: number;
}

export interface SocialEvent {
  id: number;
  individual1_id: number;
  individual2_id: number;
  location_id: number;
  hour: number;
  interaction_type: SocialInteractionType;
  relationship_change: number;
}

export interface BuildingEvent {
  id: number;
  building_id: number;
  hour: number;
  event_type: BuildingEventType;
  description: string;
  impact_value: number;
}

export interface CityEvent {
  id: number;
  city_id: number;
  hour: number;
  event_type: CityEventType;
  description: string;
  participants: number;
  impact_stability: number;
  impact_culture: number;
}

export interface NeedFulfillmentEvent {
  id: number;
  individual_id: number;
  location_id: number;
  hour: number;
  need_type: FundamentalNeed;
  amount_fulfilled: number;
  action_taken: IndividualAction;
}

// ===== ENUM TYPES =====

export enum Valley {
  Dawn = "Dawn",
  Day = "Day", 
  Dusk = "Dusk",
  Night = "Night"
}

export enum FundamentalNeed {
  Environment = "Environment",
  Consumption = "Consumption",
  Connection = "Connection",
  Rest = "Rest",
  Waste = "Waste"
}

export enum IndividualAction {
  Working = "Working",
  Socializing = "Socializing",
  Resting = "Resting",
  Traveling = "Traveling",
  Consuming = "Consuming",
  Managing = "Managing"
}

export enum SocialInteractionType {
  Conversation = "Conversation",
  SharedMeal = "SharedMeal",
  Collaboration = "Collaboration",
  Romance = "Romance",
  Conflict = "Conflict",
  CommunityEvent = "CommunityEvent"
}

export enum BuildingEventType {
  Upgraded = "Upgraded",
  MaintenancePerformed = "MaintenancePerformed",
  Cleaned = "Cleaned",
  CapacityReached = "CapacityReached",
  ResourceShortage = "ResourceShortage",
  ProductionCompleted = "ProductionCompleted",
  RentCollected = "RentCollected"
}

export enum CityEventType {
  Festival = "Festival",
  Election = "Election",
  Emergency = "Emergency",
  PolicyChange = "PolicyChange",
  MilestoneReached = "MilestoneReached",
  TradeAgreement = "TradeAgreement",
  InfrastructureProject = "InfrastructureProject"
}

export type BuildingType = 
  | { Home: { capacity: number; rent: number } }
  | { Workplace: { job_type: JobType; positions: number } }
  | { Restaurant: {} }
  | { Park: {} }
  | { Hospital: {} }
  | { PoliceStation: {} }
  | { School: {} }
  | { ResearchLab: {} }
  | { CultureCenter: {} }
  | { CityHall: {} };

export enum JobType {
  Factory = "Factory",
  Office = "Office", 
  Retail = "Retail",
  Healthcare = "Healthcare",
  Education = "Education",
  Research = "Research",
  Culture = "Culture",
  Utilities = "Utilities",
  Government = "Government"
}

// ===== CLIENT-SPECIFIC TYPES =====

export interface LocationState {
  city: string;
  valley: Valley;
  current_hour: number;
  time_of_day: TimeOfDay;
  population: number;
  active_buildings: Building[];
  recent_events: HistoricalEvent[];
  city_metrics: {
    stability: number;
    culture: number;
    prosperity: number;
    safety: number;
    sustainability: number;
  };
}

export interface HistoricalEvent {
  type: 'movement' | 'work' | 'social' | 'building' | 'city' | 'need_fulfillment';
  timestamp: number;
  hour: number;
  location: string;
  participants: string[];
  description: string;
  impact: EventImpact;
  raw_data: any; // Original event data
}

export interface EventImpact {
  description: string;
  magnitude: number;
  affected_entities: string[];
}

export interface StoryContext {
  location: LocationState;
  characters: Individual[];
  recent_events: HistoricalEvent[];
  time_context: TimeContext;
  suggested_narratives: string[];
}

export interface TimeContext {
  current_hour: number;
  time_of_day: TimeOfDay;
  day_of_week: number;
  total_days: number;
  simulation_running: boolean;
}

export enum TimeOfDay {
  Dawn = "dawn",
  Day = "day",
  Dusk = "dusk", 
  Night = "night"
}

export interface TimeRange {
  start_hour?: number;
  end_hour?: number;
  hours_back?: number;
  event_types?: string[];
}

export interface QueryOptions {
  include_details?: boolean;
  max_results?: number;
  sort_by?: 'time' | 'relevance';
  format?: 'raw' | 'narrative';
}

// ===== CONNECTION TYPES =====

export interface ConnectionConfig {
  url: string;
  auto_reconnect?: boolean;
  reconnect_delay?: number;
  max_reconnect_attempts?: number;
  timeout?: number;
}

export interface ClientStatus {
  connected: boolean;
  server_url: string;
  last_ping: number;
  subscription_count: number;
  query_count: number;
}

// ===== ERROR TYPES =====

export class SimulationClientError extends Error {
  constructor(
    message: string,
    public code: string,
    public details?: any
  ) {
    super(message);
    this.name = 'SimulationClientError';
  }
}

export interface ErrorResponse {
  error: string;
  code: string;
  details?: any;
  timestamp: number;
}