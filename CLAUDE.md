# CLAUDE.md - AI Assistant Instructions

## Project Overview
This is a World Simulation project that models a hierarchical system with three interconnected levels:
1. **Individual Level**: Simulated humans with physiological and psychological needs
2. **Building Level**: Structures that serve and depend on human occupants  
3. **City Level**: Urban systems emerging from building and human interactions

The simulation runs continuously in the background and can be queried for status checks.

## Key Project Guidelines
1. **Keep it simple**: Do not over-engineer. Make elegant solutions. Exercise Occam's razor.
2. **Do not add until it works**: Only add functionality when existing one IS functional.
3. **Serve a purpose**: Every feature must serve a purpose in the finished product.
4. **Kill your babies**: Destroy any intermediate steps or scripts that are not needed anymore.
5. **Use the simplest software solution**: Using file-based systems or simple scripts is OK if it fulfills its purpose.

## Core Mechanics Summary
- **Unified Need System**: All entities share 5 fundamental needs (Environment, Consumption, Connection, Rest, Waste)
- **Time System**: Ticker moves every hour
  - Individual needs tick every hour
  - Building needs tick every day (24 hours)
  - City needs tick every week (168 hours)
- **Hierarchical Needs**: Higher level needs can only be fulfilled if lower levels are at least 50% adequate

## Current Status
- **Phase**: Alpha Phase
- **Version**: Not specified
- **Next Phase Requirements**: To be determined

## Known Issues & Gaps
### Documentation Gaps
- API documentation missing
- No architecture diagrams
- Missing setup instructions
- No contribution guidelines
- Gameplay mechanics undocumented
- No performance targets documented

### Testing Gaps
- No automated testing
- No performance benchmarks
- No stress testing
- No compatibility testing
- No playtesting feedback loop

## Implementation Notes
- The example directory contains `needs_simulation.rs` which likely contains Rust implementation
- Technical architecture details are not yet documented
- Dependencies are not yet specified
- Detailed modifiers and rates are documented in `modifiers.md`

## When Working on This Project
1. Focus on implementing the core need fulfillment loop first
2. Start with individual level needs before moving to building/city levels
3. Ensure each feature is fully functional before adding new ones
4. Keep solutions simple and avoid over-engineering
5. Document any architectural decisions as you make them
6. Consider implementing automated tests for core mechanics
7. Refer to `modifiers.md` for all numerical values and rates

## Priority Order for Development
1. Complete individual level need system implementation
2. Implement basic building level operations (Home and Workplace)
3. Add location and movement mechanics
4. Implement time ticker system
5. Create basic query interface for status checks
6. Add city level mechanics with complete metrics
7. Implement cascading effects between levels
8. Add priority calculation system
9. Implement save/load functionality
10. Performance optimization and testing

## Key Improvements Made
- Fixed all typos and grammar issues in project.md
- Completed city level metric definitions
- Created comprehensive modifiers.md with exact rates
- Established clear connections between all three levels
- Removed redundant metrics (merged Safeplace into Safety)
- Aligned all metrics with the unified need system