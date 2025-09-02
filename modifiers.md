# Modifiers and Rates

## Time-Based Depletion Rates

### Individual Level (per hour)
#### Level 1: Physiological
- **Food and Water**: -2.0 base depletion
  - Strenuous work: -3.0
  - Resting: -1.5
  - Maximum: 100, Critical: <20
  
- **Environment**: -1.0 base depletion
  - Hazardous location: -3.0
  - Neutral location: -1.0
  - Healing location: +0.5
  - Maximum: 100, Critical: <30
  
- **Intimacy**: -0.5 base depletion
  - With partner: +10 per interaction
  - Alone: -0.5
  - Maximum: 100, Critical: <10
  
- **Rest**: -1.5 base depletion
  - Sleeping (bed): +8.0
  - Resting (chair): +2.0
  - Working: -2.5
  - Maximum: 100, Critical: <20
  
- **Waste**: +2.0 base accumulation
  - Using facilities: -50 per use
  - Emergency release: -100 (environment -20)
  - Maximum: 100, Critical: >80

#### Level 2: Safety & Security (requires Level 1 >50%)
- **Threat**: -0.5 base
  - Dangerous area: -2.0
  - Safe building: +0.5
  - With security: +1.0
  - Maximum: 100, Critical: <30
  
- **Income**: -0.2 base (living costs)
  - Working: +5.0 per hour
  - Unemployed: -0.5
  - Maximum: 1000, Critical: <10
  
- **Stress**: -0.3 base
  - High workload: -1.0
  - Recreation: +2.0
  - Low income: -0.5
  - Affects Rest: -0.1 per 10 stress
  - Maximum: 100, Critical: >70
  
- **Safety**: -0.2 base
  - At home: +1.0
  - Safe location: +0.5
  - Unsafe area: -2.0
  - Maximum: 100, Critical: <40

#### Level 3: Love & Belonging (requires Level 2 >50%)
- **Relationship**: No depletion
  - Active relationship: 33.3 constant
  - No relationship: 0
  
- **Social Interaction**: No depletion
  - Active friendships (3+): 33.3 constant
  - Few friends (1-2): 16.6
  - No friends: 0
  
- **Community**: -0.3 base
  - Community project: +5.0
  - Social event: +3.0
  - Isolation: -0.5
  - Maximum: 33.4, Critical: <10

#### Level 4: Self-Esteem (requires Level 3 >50%)
- **Achievements**: No depletion
  - Each achievement: +20 permanent
  - Maximum: 100 (5 achievements)
  
#### Level 5: Self-Actualization (requires Level 4 >50%)
- **Progression**: No depletion
  - Meaningful work: +0.5 per hour
  - Achievement unlocked: +10
  - Milestone reached: +20
  - Maximum: 100

### Building Level (per day/24 hours)

#### Home Buildings
- **Rent**: -10 base
  - Paid by tenant: +rent_amount
  - Maximum: varies, Critical: <0
  
- **Maintenance**: -2 base
  - Per occupant: -0.5
  - Maintenance work: +20
  - Poor city infrastructure: -1.0 extra
  - Maximum: 100, Critical: <20
  
- **Cleanliness**: -3 base
  - Per occupant: -1.0
  - Cleaning action: +30
  - Affects Environment: -0.5 per 10 below 50
  - Maximum: 100, Critical: <20

#### Workplace Buildings
- **Cost**: -50 base (operational costs)
  - Per worker: -5
  - Revenue from sales: +varies
  - Maximum: varies, Critical: <0
  
- **Consumption**: Resource usage
  - Base: 10 units/day
  - Per worker: +5 units
  - Efficiency modifier: ×(1 - 0.1×efficiency_level)
  
- **Production**: Resource output
  - Base: 5 units/day
  - Per worker: +10 units
  - Efficiency modifier: ×(1 + 0.2×efficiency_level)
  
- **Inventory**: Storage
  - Delivery out: -amount
  - Production: +daily_production
  - Maximum: 1000 units
  
- **Stockpile**: Raw materials
  - Delivery in: +amount
  - Consumption: -daily_consumption
  - Maximum: 1000 units

#### Building Upgrades
- **Efficiency**: 5 stages (0-5)
  - Work hours needed per level: 100
  - Production bonus: +20% per level
  - Consumption reduction: -10% per level
  
- **Prestige**: 5 stages (0-5)
  - Work hours needed per level: 200
  - Rent multiplier: ×1.2 per level
  - Attracts higher-skill workers
  - City prestige: +1 per level 5 building

### City Level (per week/168 hours)

#### Infrastructure & Economy
- **Public Works**: -0.01 per citizen
  - Utility worker hour: +0.5
  - Affects all building maintenance: ×(2.0 - utilities/100)
  - Maximum: 100, Critical: <30
  
- **Tax Base**: Variable
  - Individual income tax: sum(incomes × 0.2)
  - Business tax: sum(profits × 0.3)
  - Public service cost: -1 per 100 citizens
  - Import cost: -10 per import
  - Export revenue: +15 per export
  
#### Safety & Social
- **Stability**: -0.1 per stressed citizen
  - Community event: +5
  - Low unemployment (<5%): +10
  - High happiness (>70%): +0.05 per happy citizen
  - Affects individual stress: ×(2.0 - stability/100)
  - Maximum: 100, Critical: <40
  
- **Health**: Average of individual environments
  - Healthcare facility: +0.1 per citizen served
  - Poor sanitation: -0.2 per affected citizen
  - Affects productivity: ×(health/100)
  
- **Safety**: 100 - average(threat levels)
  - Police station coverage: +20 per station
  - Emergency services: +10 per station
  - Crime rate modifier: ×(safety/100)

#### Culture & Development
- **Culture**: Cumulative
  - Artist/entertainer: +0.5 per hour worked
  - Cultural building: +1.0 per visitor
  - Population growth: +1 resident per 100 culture
  - Enables events: Requires 50+ culture
  
- **Science**: Cumulative
  - Scientist: +0.3 per hour worked
  - Research facility: ×2.0 multiplier
  - Unlocks: Building upgrades at 100, 250, 500
  
- **Prestige**: Cumulative
  - Achievement: +10
  - Level 5 building: +1
  - Self-actualized citizen: +5
  - Skilled worker attraction: +1% per 10 prestige

## Action Costs and Effects

### Individual Actions
- **Move**: 1 hour, Rest -2
- **Work**: 8 hours, Rest -16, Stress +5, Income +40
- **Sleep**: 8 hours, Rest +64, must be in bed
- **Eat**: 1 hour, Food +25, must have food access
- **Socialize**: 2 hours, Social +10, Stress -5
- **Maintain Building**: 4 hours, Building Maintenance +20
- **Clean**: 2 hours, Building Cleanliness +30

### Location Modifiers
- **Home**: Safety +1.0, Stress -0.2, Rest +0.5
- **Workplace**: Income enabled, Stress +0.3
- **Park**: Environment +0.5, Stress -0.5
- **Restaurant**: Food access, Social enabled
- **Hospital**: Environment +2.0, Health services
- **Dangerous Area**: Threat -2.0, Stress +1.0

## Cascading Effects

### Individual → Building
- Occupancy affects maintenance (-0.5 per person)
- Workers affect production (+10 units per worker)
- Rent payments affect building finances
- Individual waste affects cleanliness

### Building → City
- Building prestige contributes to city prestige
- Business taxes contribute to tax base
- Production/consumption affects imports/exports
- Building quality affects resident satisfaction

### City → Individual
- Infrastructure affects building maintenance rates
- Safety affects individual threat levels
- Culture provides entertainment options
- Science enables better job opportunities
- Stability affects stress modifiers

## Priority Calculations

When multiple needs are low, priority = (100 - current_value) × urgency_weight

Urgency weights:
1. Waste (>80): 10.0
2. Food (<20): 8.0
3. Rest (<20): 7.0
4. Safety (<30): 6.0
5. Income (<10): 5.0
6. Environment (<30): 4.0
7. Stress (>70): 3.0
8. Social needs: 2.0
9. Higher level needs: 1.0

## Threshold Effects

- **Starvation**: Food = 0 for 24 hours → Death
- **Exhaustion**: Rest = 0 for 48 hours → Collapse (forced rest)
- **Bankruptcy**: Income < 0 for 7 days → Eviction
- **Building Collapse**: Maintenance = 0 for 30 days → Uninhabitable
- **City Decline**: Tax base < 0 for 4 weeks → Service shutdown
- **Social Unrest**: Stability < 20 for 2 weeks → Riot events