# World simulation

## Project Description

Model a World that Works on multiple hierarchical levels. Lower level entities are used as agents by a higher level entities. All actors have needs they need fullfilled and that motivates them to act. Goal is to make an ongoing simulation that runs on background and can be queryed for status checks.

### Core Concept

The simulation operates on three interconnected levels:
1. **Individual Level**: Simulated humans with physiological and psychological needs
2. **Building Level**: Structures that serve and depend on human occupants
3. **City Level**: Urban systems emerging from building and human interactions

## Core Mechanics

### Unified Need System
All entities share five fundamental needs:
- **Environment**: Safety, comfort, and livability
- **Consumption**: Resource intake and usage
- **Connection**: Social bonds and network effects
- **Rest**: Recovery and maintenance
- **Waste**: Byproduct management and disposal

### Need Fulfillment Loop
1. Need depletes over time
2. Entity seeks entity/location/action to fulfill a need
3. Once a need is fulfilled the cycle continues with priority shifts

### Individual level hierarchical needs: Higher level need can be fulfilled if a lower level is at least adequate level (50%): ticks every hour

#### Level 1 needs: Physiological
- Food and Water: Simple metric that depletes every tick
- Environment: Location affects the depletion rate of metric. Can be positive if in healing environment. More hazardous environments deplete the metric faster
- Intimacy: A location and condition based need. Another human level entity needs to be at the same location. Requires an action. Depletes slowly. Fills with an action.
- Rest: A location based need. Can be fulfilled with an action in a location that allows resting
- Waste: A location based need. Can be fulfilled with an action. If on unsuitable location affects adversely some metrics.

#### Level 2: Safety & Security
- Threat: Location affects the depletion rate through environmental threat assessment
- Income/Economic stability: Doing act "work" at a suitable location fulfills income metric. If income is depleted then security is lowered faster
- Stress: If metric is depleted it causes the rest meter to drop faster.
- Safety: Can be filled in a location that is considered safe. Usually home. Safety locations have a higher positive modifier than unsafe locations have negative modifiers, so safety metric can always be fulfilled by staying in a safe location

#### Level 3: Love & Belonging
- Relationship: Ongoing relationship modifier if fulfilled then 1/3 of this metric is filled. Does not deplete over time
- Social interaction: Ongoing friendship modifier if fulfilled then 1/3 of metric is fulfilled. Does not deplete over time
- Community participation: Acting on joined projects fulfills this metric. 1/3 of love and belonging is depleted over time and is filled with acts at a location. 

#### Level 4: Self-Esteem
- Achievement system: Achievements are gained when requirements are fulfilled. 1/5 per achievement.
- Value-based goals: Available achievements depend on individual values.
- Recognition mechanics: An achievement can be given by a building and city level entity as a reward. 

#### Level 5: Self-Actualization
- Long-term progression paths: Is fulfilled with acts, achievements and time. 
- Specialized roles (artist, scientist, leader): Different paths have different requirements.

### Building level hierarchical needs: ticks every day (every 24 hours): The type of buildings dictates the needs. Buildings can have multiple functions

#### Level 1 needs: Operations

##### Home

- Rent: Simple metric that depletes overtime. Filled with act of pay rent
- Occupancy: How many people live at the Building. Increases the rate of depletion of maintenance. Bigger houses can house more people.
- Maintenance: Depletes over time. Filled with act of Maintenance work. Affects the location's environmental factors.
- Cleanliness: Depletes over time. Filled with act of Cleaning the house. Affects the location's environmental factors and waste management.

##### Workplace

- Cost: Simple metric that depletes every tick. Filled when deliveries are made. Overflow is marked as profit.
- Consumption: Workplaces consume resources: every individual working on a workplace increases the rate of consumption. resets every day: Depletes stockpile
- Produce: Workplaces produce resources: every individual working on a workplace increases the production. Resets every day: Inflates inventory
- Inventory: How many of the produce has been made. Depletes when the produce is delivered to another location. If full operations halt
- Stockpile: How much resources is stored in the Building. Inflates with incoming deliveries. Deflates with ongoing production.

#### Level 2 needs: Living standards: Buildings can be of higher standard of living and can be upgraded

- Efficiency: Can be upgraded through work. Counter or 5 stages. Resets over completion and makes the relevant modifier more favorable. (for workplace makes the ratio of resources to produce more favorable)
- Prestige: Can be upgraded through work. Counter of 5 stages. Resets over completion and makes the relevant modifier more favorable. Makes the building of higher standard and contributes to city prestige

### City level hierarchical needs: ticks every week (every 168 hours)

#### Level 1 needs: Infrastructure and Economy

- Public Works and Utilities: Infrastructure health affecting all buildings' efficiency. Depletes based on total population (0.01 per person per tick). Filled by utility workers performing maintenance actions. Low levels cause building maintenance to deplete faster.
- Imports: Resources brought from outside when local production < consumption. Each import costs 10 tax units. Automatic when city stockpiles are low.
- Exports: Surplus resources sent outside when local production > consumption. Each export generates 15 tax units. Automatic when city inventory exceeds capacity.
- Tax base: Sum of all individual incomes and business profits. Depletes by funding public services (1 per 100 population). Fills from work actions and profitable businesses.

#### Level 2 needs: Safety & Social Cohesion

- Stability: Social order metric. Depletes when average individual stress > 70% (0.1 per stressed individual). Filled by community events, low unemployment, and high average happiness. Affects individual stress modifiers.
- Health: Average health of population calculated from individual environment metrics. Low health (<40%) reduces workplace productivity by 50%. Improved by healthcare buildings and clean environments.
- Safety: Inverse of average individual threat levels. Maintained by police stations and emergency services. Low safety increases individual threat depletion rates.

#### Level 3 needs: Culture & Development

- Culture: Cultural vibrancy from artists and entertainers (each contributes 0.5 per tick). Attracts new residents (1 per 100 culture points). Enables community events that boost stability.
- Science: Innovation output from scientists and research facilities (each scientist contributes 0.3 per tick). Enables efficiency upgrades for buildings. Required for advanced building types.
- Prestige: City reputation built from achievements (10 points each), high-prestige buildings (1 point per level 5 building), and fulfilled level 5 individuals (5 points each). Attracts skilled workers and investors.

### Global modifiers

1. **Time**
   - a Ticker is moved every hour.
   - affects all meters with their assigned modifiers

2. **Location**
   - Local modifiers affect the need fulfillment meter
   - Moving between locations is an action
   - Locations inherit environmental factors from their building
   - Buildings inherit infrastructure quality from their city

The ultimate goal is a living world where every action has meaningful consequences, every agent has a story, and every simulation run reveals new insights into the complexity of human societies.